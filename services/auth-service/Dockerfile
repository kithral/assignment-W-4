FROM rust:1.83-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/auth-service ./services/auth-service

WORKDIR /app/services/auth-service
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/services/auth-service/target/release/auth-service /
EXPOSE 8084
CMD ["/auth-service"]
