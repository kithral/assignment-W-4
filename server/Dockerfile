# 1. Build Stage
FROM rust:1.83-slim-bookworm AS builder

# Create a dummy project to cache dependencies
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY server ./server
COPY client ./client
# Note: We copy client just to satisfy the workspace,
# even though we only build the server.

# Build the server specifically
RUN cargo build --release --bin server

# 2. Runtime Stage (The tiny part)
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/server /app/server

WORKDIR /app
ENV RUST_LOG=info
# Expose the Lightyear UDP port
EXPOSE 5000/udp

ENTRYPOINT ["./server"]
