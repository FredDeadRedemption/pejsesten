// Exposes the current page's WebSocket origin to Rust.
// On a page served from https://example.com, returns "wss://example.com".
(function () {
    var pending = null;

    function mem_view() {
        return new Uint8Array(wasm_memory.buffer);
    }

    function register_plugin(importObject) {
        importObject.env.app_location_ws_origin_len = function () {
            var loc = window.location;
            var scheme = loc.protocol === "https:" ? "wss" : "ws";
            var url = scheme + "://" + loc.host;
            pending = new TextEncoder().encode(url);
            return pending.length;
        };

        importObject.env.app_location_ws_origin_take = function (buf_ptr) {
            if (pending === null) return 0;
            mem_view().set(pending, buf_ptr);
            var len = pending.length;
            pending = null;
            return len;
        };
    }

    miniquad_add_plugin({ register_plugin: register_plugin, version: 1, name: "app_location" });
})();
