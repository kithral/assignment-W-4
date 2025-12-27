FROM rust:1.83-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/asset-service ./services/asset-service

WORKDIR /app/services/asset-service
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/services/asset-service/target/release/asset-service /
EXPOSE 8088
CMD ["/asset-service"]
