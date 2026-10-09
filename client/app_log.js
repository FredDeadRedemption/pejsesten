// Routes the client's log lines to the browser console. Rust has no stdout on wasm.
(function () {
    function register_plugin(importObject) {
        importObject.env.app_log = function (ptr, len) {
            var bytes = new Uint8Array(wasm_memory.buffer, ptr, len);
            console.log('[pejsesten] ' + new TextDecoder('utf-8').decode(bytes));
        };
    }

    miniquad_add_plugin({ register_plugin: register_plugin, version: 1, name: "app_log" });
})();
