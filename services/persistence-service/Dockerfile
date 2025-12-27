FROM rust:1.83-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/persistence-service ./services/persistence-service

WORKDIR /app/services/persistence-service
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/services/persistence-service/target/release/persistence-service /
EXPOSE 8087
CMD ["/persistence-service"]
