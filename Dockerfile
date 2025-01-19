FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY config ./config

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/tiktok_proxy /usr/local/bin/tiktok_proxy
COPY config/config.toml /config/config.toml

ENV RUST_LOG=info

CMD ["tiktok_proxy"]
