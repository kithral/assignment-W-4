# Multi-stage Dockerfile for script-executor service

FROM rust:1.83-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/script-executor ./services/script-executor

WORKDIR /app/services/script-executor

# Build with Rhai by default (change to --features lua-scripting to use Lua)
RUN cargo build --release --features rhai-scripting

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/services/script-executor/target/release/script-executor /

EXPOSE 8081

CMD ["/script-executor"]
