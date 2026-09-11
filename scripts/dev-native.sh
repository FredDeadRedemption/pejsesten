#!/bin/sh
# Native dev loop: server + auto-restarting macroquad client.
set -e
cd "$(dirname "$0")/.."

cargo run -p server > /tmp/pejsesten-server.log 2>&1 &
SERVER_PID=$!
trap "kill $SERVER_PID 2>/dev/null" EXIT

echo "server: pid $SERVER_PID, log: /tmp/pejsesten-server.log"
echo "client: cargo-watch will rebuild and restart on save"
echo

exec cargo watch -x 'run -p client'
