#!/bin/sh
# One-shot: build the wasm bundle, symlink everything into dist/, run the server.
# Server hosts the wasm + assets at http://localhost:3000 (same origin as socket.io).
#
# To iterate:
#   1. Edit code
#   2. In another terminal: cargo build -p client --target wasm32-unknown-unknown --release
#   3. Refresh the browser tab (symlinks pick up the new wasm automatically)
set -e
cd "$(dirname "$0")/.."

cargo build -p client --target wasm32-unknown-unknown --release

mkdir -p dist
ln -sfn ../target/wasm32-unknown-unknown/release/client.wasm dist/client.wasm
ln -sfn ../client/index.html                                 dist/index.html
ln -sfn ../client/mq_js_bundle.js                            dist/mq_js_bundle.js
ln -sfn ../client/app_ws.js                                  dist/app_ws.js
ln -sfn ../client/app_storage.js                             dist/app_storage.js
ln -sfn ../client/static/favicon.png                         dist/favicon.png
ln -sfn ../client/static/media                               dist/media

echo
echo "→ http://localhost:3000"
echo
exec env STATIC_DIR=dist cargo run -p server
