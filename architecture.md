# Architecture

A reference for how the pieces fit together — workspace layout, how the same Rust client runs both native and in the browser, what the JS plugins do, and how to build/deploy.

## Workspace layout

```
pejsesten/
├── Cargo.toml          # workspace
├── shared/             # card catalog + game types (used by server + client)
├── server/             # axum + socketioxide; engine, bot, settings
├── client/             # macroquad app — compiles native AND wasm
│   ├── index.html      # HTML shim for the wasm build
│   ├── mq_js_bundle.js # macroquad's JS glue (vendored)
│   ├── app_ws.js       # custom plugin: WebSocket
│   ├── app_storage.js  # custom plugin: localStorage
│   └── static/         # card art, favicon
├── docs/               # design notes
├── scripts/            # dev-native.sh, dev-web.sh
└── Dockerfile          # multi-stage: builds wasm + server, ships one container
```

`shared/` exists so the card catalog (`get_collectible_cards`, `get_card_by_id`) and types (`Card`, `MinionEntity`, `GameStateClient`, etc.) are defined exactly once and consumed by both server and client. Game logic (engine, `IdGenerator`, `deck_to_cards`) stays server-side because the client never instantiates entities — it only renders what the server sends.

## How macroquad + wasm works

### The mental model

`macroquad` is a friendly API on top of `miniquad`, the platform abstraction. Miniquad's job is "give me a window, a GL context, input events, file loading, time." It has implementations for Linux/X11, Windows, macOS, Android, iOS, **and the web**.

Your code calls macroquad → miniquad → miniquad picks the right backend. One Rust codebase, all platforms.

### Native

When you run `cargo run -p client`, miniquad opens a window via X11/Win32/Cocoa, gets a real GL context, reads OS input events, calls `std::fs` to load PNGs from disk. Standard stuff.

### Wasm

A `.wasm` binary cannot do `glDrawArrays` or `fopen` itself. WebAssembly can only call functions the host hands it. So the wasm declares **imports**:

```
(import "env" "glDrawArrays" (func ...))
(import "env" "fs_load_file" (func ...))
(import "env" "now" (func ...))
```

These are just names. Something must provide implementations.

`mq_js_bundle.js` is the JavaScript file that does. When the browser loads the wasm, the bundle hands it an `importObject` with real JS implementations:

```js
importObject.env.glDrawArrays = function(mode, first, count) {
    gl.drawArrays(mode, first, count);   // real WebGL
};
importObject.env.fs_load_file = function(path_ptr, path_len) {
    fetch(...);                          // real browser fetch
};
```

So `macroquad::load_texture("Card.png")` ends up calling real WebGL `texImage2D`. Same Rust code, completely different implementation under the hood. That's the trick.

## Plugins

`mq_js_bundle.js` covers the basics: GL, input, file loading, time, audio. It does **not** cover networking, localStorage, or clipboard. Plugins are how you add more imports.

A plugin is just two pieces:

1. A JS file that defines extra functions and registers them via `miniquad_add_plugin(...)`. When the bundle instantiates the wasm, it calls each plugin's `register_plugin(importObject)` so they can add their entries to `importObject.env`.
2. Rust code with `extern "C" fn` declarations that match those names.

We have three custom plugins:

### `app_ws.js` — WebSocket

Adds these imports to `importObject.env`:

| Import                  | Rust signature                                               | What it does                                          |
|-------------------------|--------------------------------------------------------------|-------------------------------------------------------|
| `app_ws_connect`        | `fn(*const u8, u32) -> u32`                                  | Open a `new WebSocket(url)`, return a handle          |
| `app_ws_send_text`      | `fn(handle: u32, *const u8, u32)`                            | Send a text frame                                     |
| `app_ws_recv_peek_len`  | `fn(handle: u32) -> u32`                                     | Length of next queued message, or 0                   |
| `app_ws_recv_take`      | `fn(handle: u32, *mut u8) -> u32`                            | Copy next message into the buffer, dequeue it         |
| `app_ws_state`          | `fn(handle: u32) -> u32`                                     | 0=connecting, 1=open, 3=closed                        |

Strings and byte buffers cross the JS/wasm boundary as `(ptr, len)` pairs into wasm's linear memory. JS reads them via `new Uint8Array(wasm_memory.buffer, ptr, len)`. No `wasm-bindgen`, no `sapp-jsutils`.

### `app_storage.js` — localStorage

| Import                   | Rust signature                                       | What it does                              |
|--------------------------|------------------------------------------------------|-------------------------------------------|
| `app_storage_get_len`    | `fn(*const u8, u32) -> i32`                          | Returns byte length of value, or -1       |
| `app_storage_get_take`   | `fn(*mut u8) -> u32`                                 | Copies the cached value into the buffer   |
| `app_storage_set`        | `fn(*const u8, u32, *const u8, u32)`                 | `localStorage.setItem(key, value)`        |

Same pattern. The two-call get (peek length, then take) lets Rust allocate exactly the right buffer.

### `app_location.js` — page origin

| Import                          | Rust signature                | What it does                                  |
|---------------------------------|-------------------------------|-----------------------------------------------|
| `app_location_ws_origin_len`    | `fn() -> u32`                 | Computes `wss://host` (or `ws://`), returns length |
| `app_location_ws_origin_take`   | `fn(*mut u8) -> u32`          | Copies the URL into the provided buffer       |

Used by `server_url()` so the wasm connects to the same origin it was served from. Means one image works at any deploy URL — no `SERVER_URL` env needed.

### Why we wrote these instead of using `quad-net` / `quad-storage`

`quad-net` and `quad-storage` are macroquad-ecosystem crates that do the same thing. We tried them, but their JS files and Rust crates have drifted out of version-sync — the bundled JS expects an API the Rust side doesn't provide, and vice versa. Custom plugins are ~50 lines each and have no skew because we control both sides.

### Why **not** `wasm-bindgen` / `web-sys`

`wasm-bindgen` is a competing system. It generates its own JS glue with its own import-naming convention (`__wbindgen_placeholder__::*`). `mq_js_bundle.js` doesn't speak it. Two systems, one wasm binary — they conflict. So in this project: zero wasm-bindgen-using crates in the wasm dep graph. (`ewebsock` pulls wasm-bindgen on wasm, which is why it's `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` — desktop only.)

## Cfg-splits: same API, two backends

Wherever a feature needs a different implementation native vs wasm, we wrap it in a small module with two `#[cfg]`-gated submodules exporting the same surface. Example from `client/src/network.rs`:

```rust
#[cfg(not(target_arch = "wasm32"))]
mod ws {
    use ewebsock::{...};
    pub struct WebSocket { /* sender, receiver, is_open */ }
    impl WebSocket {
        pub fn connect(url: &str) -> Self { ... }
        pub fn send_text(&mut self, s: &str) { ... }
        pub fn try_recv(&mut self) -> Option<String> { ... }
        pub fn connected(&self) -> bool { ... }
    }
}

#[cfg(target_arch = "wasm32")]
mod ws {
    unsafe extern "C" {
        fn app_ws_connect(...) -> u32;
        fn app_ws_send_text(...);
        fn app_ws_recv_peek_len(...) -> u32;
        fn app_ws_recv_take(...) -> u32;
        fn app_ws_state(...) -> u32;
    }
    pub struct WebSocket { handle: u32 }
    impl WebSocket { /* same methods */ }
}
```

Everything else in `network.rs` (and `main.rs`, deckbuilder, etc.) just calls `ws::WebSocket::connect(url)` without caring which backend it gets. Cargo picks one at compile time based on the target.

`decks.rs` follows the same pattern: `std::fs` + `SystemTime` on desktop, `app_storage_*` + `miniquad::date::now()` on wasm.

`textures.rs` cfg-splits `MEDIA_ROOT`: `concat!(env!("CARGO_MANIFEST_DIR"), "/static/media/")` on desktop (absolute path baked at compile time, CWD-independent), `/media/` on wasm (URL served by the static server).

## Networking protocol

The Rust client speaks raw socket.io v4 framing over a single WebSocket. The framing (e.g. `42["eventName",{...}]`) is hand-rolled in `client/src/network.rs::handle_message`. Server uses `socketioxide` to speak the same protocol. No JSON-RPC, no custom framing layer beyond what socket.io defines.

Server URL is set via `option_env!("SERVER_URL")`, defaulting to `ws://localhost:3000`. In production it's baked at build time in the Dockerfile. The wasm and the socket.io server run on the **same origin** (one Railway service serving both), so no CORS gymnastics.

## Build & run

### Native dev loop

```sh
./scripts/dev-native.sh
```

Starts the server in the background and runs the client under `cargo watch`. Save → ~1 sec rebuild → window relaunches. Ctrl-C kills both.

### Web dev loop

```sh
./scripts/dev-web.sh
```

Builds the wasm bundle, symlinks `client/index.html`, `mq_js_bundle.js`, `app_ws.js`, `app_storage.js`, `favicon.png`, the `media/` dir, and the wasm into `dist/`, then runs the server with `STATIC_DIR=dist`. Open `http://localhost:3000`. Iterate by rebuilding the wasm in another terminal — symlinks pick up the new artifact, just refresh the tab.

WASM release rebuilds are 1.5–6 sec depending on what changed.

### Production (Railway)

The repo-root `Dockerfile` is a three-stage build:

1. **wasm-builder**: Rust + `wasm32-unknown-unknown` target, builds `client.wasm`.
2. **server-builder**: builds the server binary.
3. **runtime**: `debian:bookworm-slim`, copies the server binary plus `dist/` (wasm + index.html + JS shims + favicon + media). Runs `./server` listening on `$PORT`.

Single Railway service, single origin. The server hosts the static files via `tower-http`'s `ServeDir` as a fallback after socket.io routes.

## Cheat sheet

- "How do I add a new browser API to the wasm client?" — Write a JS plugin (~30 lines) and matching Rust `extern "C"` block. Add the JS file to `index.html`, `dev-web.sh`, and the Dockerfile.
- "Why is `ewebsock` desktop-only?" — It pulls `wasm-bindgen` transitively, which can't coexist with `mq_js_bundle.js`. We use our `app_ws.js` plugin on wasm instead.
- "Why does `MEDIA_ROOT` differ between targets?" — Native loads from disk via an absolute path baked at compile time. Wasm loads via HTTP from `/media/...`, served by the same axum server.
- "Where do decks save?" — `decks.json` in CWD on desktop, `localStorage` under key `pejsesten.decks` on wasm.
- "How do I update `mq_js_bundle.js`?" — `curl -sSL https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -o client/mq_js_bundle.js`. The hosted version is known-good; the master branch on GitHub has a strict-mode bug.
