FROM rust:1.83-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/text-gateway ./services/text-gateway

WORKDIR /app/services/text-gateway
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/services/text-gateway/target/release/text-gateway /
EXPOSE 8083
CMD ["/text-gateway"]
