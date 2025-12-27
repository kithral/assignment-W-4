FROM rust:1.83-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/chat-service ./services/chat-service

WORKDIR /app/services/chat-service
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/services/chat-service/target/release/chat-service /
EXPOSE 8086
CMD ["/chat-service"]
