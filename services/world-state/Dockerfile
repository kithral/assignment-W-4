# Multi-stage Dockerfile for world-state service
# Optimized for fast builds and small runtime images

# Stage 1: Build
FROM rust:1.83-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/world-state ./services/world-state

# Build only the world-state service
WORKDIR /app/services/world-state
RUN cargo build --release

# Stage 2: Runtime
FROM gcr.io/distroless/cc-debian12

# Copy the binary
COPY --from=builder /app/services/world-state/target/release/world-state /

# Expose health check port
EXPOSE 8080

# Run the service
CMD ["/world-state"]
