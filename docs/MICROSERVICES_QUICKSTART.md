# Microservices Quick Start Guide

## Overview

The project has been restructured into a microservices architecture with 9 independent services. This guide will help you get started with local development.

## Structure

```
services/
├── world-state/         ✅ Core ECS service (implemented)
├── script-executor/     ✅ Rhai/Lua scripting (implemented with Rhai)
├── graphics-gateway/    📋 UDP client gateway (template)
├── text-gateway/        📋 TCP/Telnet gateway (template)
├── auth-service/        📋 Authentication (template)
├── physics-service/     📋 Physics simulation (template)
├── chat-service/        📋 Chat and messaging (template)
├── persistence-service/ 📋 Database ops (template)
└── asset-service/       📋 Asset delivery (template)
```

## Quick Start

### 1. Check Everything Compiles

```bash
cargo check --workspace
```

Expected output: `Finished \`dev\` profile [optimized + debuginfo] target(s) in X.XXs`

### 2. Run Individual Services Locally

```bash
# Run world-state service
cargo run -p world-state

# Run script-executor service
cargo run -p script-executor

# Run any service by package name
cargo run -p <service-name>
```

### 3. Run with Docker Compose (Infrastructure + Services)

```bash
# Start infrastructure only (Redis, PostgreSQL, RabbitMQ)
docker-compose -f docker-compose.dev.yml up redis postgres rabbitmq

# Start everything
docker-compose -f docker-compose.dev.yml up

# Start specific services
docker-compose -f docker-compose.dev.yml up world-state script-executor redis
```

### 4. Check Service Health

```bash
# World State
curl http://localhost:8080/health
curl http://localhost:8080/ready

# Script Executor
curl http://localhost:8081/health
curl http://localhost:8081/ready
```

## Service Ports

| Service | Health Port | Service Port | Protocol |
|---------|-------------|--------------|----------|
| world-state | 8080 | 50051 (future) | gRPC |
| script-executor | 8081 | - | Worker |
| graphics-gateway | 8082 | 5000 | UDP |
| text-gateway | 8083 | 4201 | TCP |
| auth-service | 8084 | - | gRPC |
| physics-service | 8085 | - | gRPC |
| chat-service | 8086 | - | gRPC |
| persistence-service | 8087 | - | gRPC |
| asset-service | 8088 | 8089 | HTTP |

## Scripting: Rhai vs Lua

The script-executor service supports both Rhai (default) and Lua (placeholder).

### Using Rhai (Current Default)

```bash
# Build with Rhai
cargo build -p script-executor --features rhai-scripting

# Run tests
cargo test -p script-executor
```

### Switching to Lua (Future)

When ready to switch to Lua:

1. Edit `services/script-executor/Cargo.toml`:

   ```toml
   # Uncomment this line
   mlua = { version = "0.9", features = ["lua54", "vendored", "async", "serialize"] }
   ```

2. Build with Lua feature:

   ```bash
   cargo build -p script-executor --features lua-scripting --no-default-features
   ```

3. Update Dockerfile:

   ```dockerfile
   RUN cargo build --release --features lua-scripting --no-default-features
   ```

See [services/script-executor/src/lua_executor.rs](../services/script-executor/src/lua_executor.rs) for implementation details.

## Development Workflow

### Working on a Single Service

```bash
# 1. Start infrastructure
docker-compose -f docker-compose.dev.yml up redis postgres rabbitmq -d

# 2. Run the service locally for fast iteration
cd services/world-state
cargo watch -x run

# 3. Make changes, see instant feedback
```

### Building Docker Images

```bash
# Build a specific service
docker build -t world-state:dev -f services/world-state/Dockerfile .

# Build all services (takes a while)
for service in services/*/; do
  name=$(basename $service)
  docker build -t $name:dev -f $service/Dockerfile .
done
```

## Environment Variables

Create a `.env` file in the project root:

```bash
# Database
DATABASE_URL=postgres://dev:devpassword@localhost:5432/worldengine

# Redis
REDIS_URL=redis://localhost:6379

# RabbitMQ
RABBITMQ_URL=amqp://dev:devpassword@localhost:5672

# Logging
RUST_LOG=debug
```

Services will automatically load this via `dotenvy`.

## GitHub Actions (Currently Disabled)

The GitHub Actions workflow has been disabled during the microservices migration:

- File: [.github/workflows/deploy.yml](../.github/workflows/deploy.yml)
- Status: `if: false` prevents execution
- To re-enable: Remove the `if: false` line or set to `if: true`

## Next Steps

1. **Implement Service Logic**
   - Start with world-state: Add Bevy ECS
   - Add gRPC definitions in `shared/proto/`
   - Implement service-to-service communication

2. **Create Integration Tests**
   - Test service interactions
   - Test health checks
   - Test scripting execution

3. **Add Monitoring**
   - Prometheus metrics
   - Distributed tracing
   - Log aggregation

4. **Deploy to K3s**
   - Create Kubernetes manifests in `deploy/k3s/`
   - Configure auto-scaling
   - Set up Traefik ingress

## Troubleshooting

### Service won't start

```bash
# Check logs
docker-compose -f docker-compose.dev.yml logs world-state

# Check if port is in use
sudo netstat -tlnp | grep 8080
```

### Can't connect to infrastructure

```bash
# Verify services are healthy
docker-compose -f docker-compose.dev.yml ps

# Restart infrastructure
docker-compose -f docker-compose.dev.yml restart redis postgres rabbitmq
```

### Compilation errors

```bash
# Clean build
cargo clean

# Update dependencies
cargo update

# Check specific service
cargo check -p world-state
```

## Resources

- [Microservices Architecture](microservices-architecture.md) - Complete design
- [Deployment Strategy](deployment-strategy.md) - K3s deployment
- [Scripting Research](scripting-research.md) - Rhai vs Lua comparison
- [Project Status](../PROJECT_STATUS.md) - Current state and roadmap

---

**Ready to build!** Start with the world-state service and add functionality incrementally. 🦀
