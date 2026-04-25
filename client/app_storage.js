// Minimal localStorage plugin for macroquad/miniquad wasm.
// Byte-level interop: caller passes ptr+len, no sapp-jsutils.
(function () {
    function mem_view() {
        return new Uint8Array(wasm_memory.buffer);
    }

    function read_str(ptr, len) {
        var bytes = new Uint8Array(wasm_memory.buffer, ptr, len);
        return new TextDecoder('utf-8').decode(bytes);
    }

    // Cache of last get() result so peek_len + take don't re-read.
    var pending_value = null;

    function register_plugin(importObject) {
        // Begins a "get" operation: returns byte-length of the stored value, or -1 if missing.
        // The value is cached until app_storage_get_take is called or the next get_len.
        importObject.env.app_storage_get_len = function (key_ptr, key_len) {
            var key = read_str(key_ptr, key_len);
            var v = localStorage.getItem(key);
            if (v === null) {
                pending_value = null;
                return -1;
            }
            pending_value = new TextEncoder().encode(v);
            return pending_value.length;
        };

        // Copies the cached value into buf_ptr.
        importObject.env.app_storage_get_take = function (buf_ptr) {
            if (pending_value === null) return 0;
            mem_view().set(pending_value, buf_ptr);
            var len = pending_value.length;
            pending_value = null;
            return len;
        };

        importObject.env.app_storage_set = function (key_ptr, key_len, val_ptr, val_len) {
            var key = read_str(key_ptr, key_len);
            var val = read_str(val_ptr, val_len);
            try { localStorage.setItem(key, val); } catch (e) { console.error('[app_storage] set failed:', e); }
        };
    }

    miniquad_add_plugin({ register_plugin: register_plugin, version: 1, name: "app_storage" });
})();
