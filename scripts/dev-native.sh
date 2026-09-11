#!/usr/bin/env bash
# Native dev loop: server in the background, client in the foreground.
set -e
cd "$(dirname "$0")/.."

cargo run -p server > /tmp/pejsesten-server.log 2>&1 &
SERVER_PID=$!
trap "kill $SERVER_PID 2>/dev/null" EXIT

echo "server: pid $SERVER_PID, log: /tmp/pejsesten-server.log"

# client has no reconnect, so do not start it until the port accepts
until (exec 3<>/dev/tcp/127.0.0.1/3000) 2>/dev/null; do
    kill -0 $SERVER_PID 2>/dev/null || { echo "server died, see log"; exit 1; }
    sleep 0.2
done
echo

cargo run -p client
