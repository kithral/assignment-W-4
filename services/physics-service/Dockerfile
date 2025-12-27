FROM rust:1.83-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/physics-service ./services/physics-service

WORKDIR /app/services/physics-service
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/services/physics-service/target/release/physics-service /
EXPOSE 8085
CMD ["/physics-service"]
