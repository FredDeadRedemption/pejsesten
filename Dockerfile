# Build wasm client
FROM rust:1.87-slim AS wasm-builder
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
COPY Cargo.toml ./
COPY shared shared
COPY client client
COPY server server
RUN cargo build --release -p client --target wasm32-unknown-unknown

# Build server
FROM rust:1.87-slim AS server-builder
WORKDIR /app
COPY Cargo.toml ./
COPY shared shared
COPY client client
COPY server server
RUN cargo build --release -p server

# Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=server-builder /app/target/release/server ./server
COPY --from=wasm-builder /app/target/wasm32-unknown-unknown/release/client.wasm ./dist/client.wasm
COPY client/index.html ./dist/index.html
COPY client/mq_js_bundle.js ./dist/mq_js_bundle.js
COPY client/app_ws.js ./dist/app_ws.js
COPY client/app_storage.js ./dist/app_storage.js
COPY client/static/favicon.png ./dist/favicon.png
COPY client/static/media ./dist/media
EXPOSE 3000
CMD ["./server"]
