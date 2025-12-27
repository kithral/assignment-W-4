FROM rust:1.83-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/graphics-gateway ./services/graphics-gateway

WORKDIR /app/services/graphics-gateway
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/services/graphics-gateway/target/release/graphics-gateway /
EXPOSE 8082
CMD ["/graphics-gateway"]
