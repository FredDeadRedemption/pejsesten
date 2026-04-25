// Minimal WebSocket plugin for macroquad/miniquad wasm.
// Byte-level interop: caller passes ptr+len, no sapp-jsutils.
(function () {
    var sockets = {};
    var next_handle = 1;

    function mem_view() {
        return new Uint8Array(wasm_memory.buffer);
    }

    function read_str(ptr, len) {
        var bytes = new Uint8Array(wasm_memory.buffer, ptr, len);
        return new TextDecoder('utf-8').decode(bytes);
    }

    function register_plugin(importObject) {
        importObject.env.app_ws_connect = function (url_ptr, url_len) {
            var url = read_str(url_ptr, url_len);
            var handle = next_handle++;
            var entry = { ws: null, queue: [], peek: null, state: 0 };
            try {
                entry.ws = new WebSocket(url);
                entry.ws.onopen = function () { entry.state = 1; };
                entry.ws.onmessage = function (ev) {
                    if (typeof ev.data === 'string') {
                        entry.queue.push(ev.data);
                    }
                };
                entry.ws.onclose = function () { entry.state = 3; };
                entry.ws.onerror = function () { entry.state = 3; };
            } catch (e) {
                console.error('[app_ws] connect failed:', e);
                entry.state = 3;
            }
            sockets[handle] = entry;
            return handle;
        };

        importObject.env.app_ws_send_text = function (handle, ptr, len) {
            var entry = sockets[handle];
            if (!entry || !entry.ws || entry.state !== 1) return;
            try { entry.ws.send(read_str(ptr, len)); } catch (e) { /* ignore */ }
        };

        // Returns 0 if queue empty, otherwise the byte-length of the next message.
        // The message stays queued until app_ws_recv_take is called.
        importObject.env.app_ws_recv_peek_len = function (handle) {
            var entry = sockets[handle];
            if (!entry || entry.queue.length === 0) return 0;
            if (!entry.peek) {
                entry.peek = new TextEncoder().encode(entry.queue[0]);
            }
            return entry.peek.length;
        };

        // Copies the front message into buf_ptr, removes it, returns bytes written.
        importObject.env.app_ws_recv_take = function (handle, buf_ptr) {
            var entry = sockets[handle];
            if (!entry || entry.queue.length === 0) return 0;
            if (!entry.peek) {
                entry.peek = new TextEncoder().encode(entry.queue[0]);
            }
            mem_view().set(entry.peek, buf_ptr);
            var len = entry.peek.length;
            entry.queue.shift();
            entry.peek = null;
            return len;
        };

        // 0=connecting, 1=open, 3=closed
        importObject.env.app_ws_state = function (handle) {
            var entry = sockets[handle];
            return entry ? entry.state : 3;
        };
    }

    miniquad_add_plugin({ register_plugin: register_plugin, version: 1, name: "app_ws" });
})();
