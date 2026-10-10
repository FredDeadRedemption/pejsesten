// Persistent player identity for macroquad/miniquad wasm.
// The token outlives a page reload, so the server can put a returning socket back in its game.
(function () {
    var KEY = 'pejsesten.player_token';
    var pending = null;

    function mem_view() {
        return new Uint8Array(wasm_memory.buffer);
    }

    function fresh_token() {
        var bytes = new Uint8Array(16);
        // getRandomValues needs a secure context; whoever holds the token is the player
        if (globalThis.crypto && crypto.getRandomValues) {
            crypto.getRandomValues(bytes);
        } else {
            console.error('[app_session] no crypto, falling back to a guessable token');
            for (var i = 0; i < bytes.length; i++) bytes[i] = Math.floor(Math.random() * 256);
        }
        var hex = '';
        for (var j = 0; j < bytes.length; j++) hex += bytes[j].toString(16).padStart(2, '0');
        return hex;
    }

    // an exception here would unwind into wasm and kill the app, and storage
    // is blocked outright in some browser contexts
    function token() {
        try {
            var stored = localStorage.getItem(KEY);
            if (stored) return stored;
        } catch (e) {
            console.error('[app_session] get failed:', e);
            return fresh_token();
        }
        var created = fresh_token();
        try { localStorage.setItem(KEY, created); } catch (e) { console.error('[app_session] set failed:', e); }
        return created;
    }

    function register_plugin(importObject) {
        importObject.env.app_session_token_len = function () {
            pending = new TextEncoder().encode(token());
            return pending.length;
        };

        importObject.env.app_session_token_take = function (buf_ptr) {
            if (pending === null) return 0;
            mem_view().set(pending, buf_ptr);
            var len = pending.length;
            pending = null;
            return len;
        };
    }

    miniquad_add_plugin({ register_plugin: register_plugin, version: 1, name: "app_session" });
})();
