# syntax=docker/dockerfile:1
FROM rust:1-slim-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src \
    && echo "pub fn _dep_cache_anchor() {}" > src/lib.rs \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/.fingerprint/serverless-llm-relay* \
       target/release/deps/*serverless_llm_relay* \
       target/release/serverless-llm-relay

COPY src ./src
RUN cargo build --release --bin serverless-llm-relay \
    && strip target/release/serverless-llm-relay
