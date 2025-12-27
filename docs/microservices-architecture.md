# Microservices Architecture

## Overview

The server is designed as a **modular, containerized microservices architecture** where each service can scale independently based on demand. This approach enables:

- **Independent scaling** - Scale graphics, scripting, or persistence separately
- **Fast deployments** - Update one service without redeploying everything
- **Fault isolation** - One service crash doesn't bring down the entire server
- **Technology flexibility** - Use the best tool for each job
- **Development velocity** - Teams can work on different services independently

## Architectural Principles

### 1. Single Responsibility

Each service does **one thing well**:

- Graphics service handles only UDP clients
- Scripting service only executes scripts
- Persistence service only manages database

### 2. Stateless Services

Services should be **stateless** where possible:

- State lives in shared services (Redis, PostgreSQL)
- Any instance can handle any request
- Easy to scale horizontally

### 3. API-First Design

Services communicate via **well-defined APIs**:

- gRPC for internal service-to-service
- REST/WebSocket for external clients
- Message queues for async operations

### 4. Independent Deployment

Each service has its own:

- Docker container
- Deployment pipeline
- Version number
- Health checks

## Service Breakdown

### Core Services

```
┌─────────────────────────────────────────────────────────┐
│                    Load Balancer (Traefik/Nginx)        │
└─────────────────────────────────────────────────────────┘
         │                    │                    │
    ┌────▼────┐         ┌────▼────┐         ┌────▼────┐
    │ Graphics │         │  Text   │         │   HTTP  │
    │ Gateway  │         │ Gateway │         │   API   │
    │  (UDP)   │         │  (TCP)  │         │  (REST) │
    └────┬────┘         └────┬────┘         └────┬────┘
         │                    │                    │
         └────────────────────┼────────────────────┘
                              │
         ┌────────────────────▼────────────────────┐
         │        World State Service (Core)       │
         │  - Entity management                    │
         │  - Event distribution                   │
         │  - State synchronization                │
         └─┬─────────┬──────────┬─────────┬───────┘
           │         │          │         │
    ┌──────▼──┐  ┌──▼───┐  ┌───▼────┐  ┌▼────────┐
    │ Script  │  │ Auth │  │Physics │  │  Chat   │
    │ Executor│  │ Svc  │  │  Svc   │  │  Svc    │
    └──┬──────┘  └──┬───┘  └───┬────┘  └┬────────┘
       │            │          │         │
       └────────────┴──────────┴─────────┘
                    │
         ┌──────────▼──────────┐
         │   Shared Services   │
         ├─────────────────────┤
         │ Redis (Cache/Pub)   │
         │ PostgreSQL (DB)     │
         │ RabbitMQ (Queue)    │
         │ S3 (Assets)         │
         └─────────────────────┘
```

### Service Definitions

#### 1. Graphics Gateway Service

**Responsibility:** Handle UDP graphical clients

```toml
[package]
name = "graphics-gateway"
```

**What it does:**

- Receives UDP packets from Bevy clients
- Authenticates connections
- Routes to World State Service
- Sends state updates back to clients

**Scaling trigger:**

- High number of connected graphical clients
- Network bandwidth saturation

**Container:**

```dockerfile
FROM rust:1.83-slim AS builder
WORKDIR /app
COPY graphics-gateway/ .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/graphics-gateway /
EXPOSE 5000/udp
CMD ["/graphics-gateway"]
```

**K8s scaling:**

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: graphics-gateway
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: graphics-gateway
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

---

#### 2. Text Gateway Service

**Responsibility:** Handle TCP/Telnet text clients

```toml
[package]
name = "text-gateway"
```

**What it does:**

- Accepts Telnet/WebSocket connections
- Parses text commands
- Routes to World State Service
- Formats responses as text

**Scaling trigger:**

- High number of text clients
- Command parsing CPU usage

**Technology:**

- Tokio for async I/O
- WebSocket support via `tokio-tungstenite`

---

#### 3. World State Service (Core)

**Responsibility:** Authoritative game state and ECS

```toml
[package]
name = "world-state"
```

**What it does:**

- Runs Bevy ECS
- Maintains authoritative state
- Distributes events to other services
- Coordinates service communication

**Scaling:**

- **Vertical initially** (more CPU/RAM)
- **Sharding later** (by zone/region)

**Special considerations:**

- **Sticky sessions** - Players route to same instance
- **State replication** - For failover
- **Zone sharding** - Multiple instances for different world areas

---

#### 4. Script Executor Service

**Responsibility:** Execute user scripts safely

```toml
[package]
name = "script-executor"
```

**What it does:**

- Receives script execution requests
- Runs Rhai/Lua in sandboxed environment
- Enforces timeouts and resource limits
- Returns results to World State

**Scaling trigger:**

- High script execution load
- Long-running scripts

**Isolation:**

- Run in isolated containers
- CPU/memory limits enforced
- Timeout protection

**Technology:**

- Rhai with operation limits
- Job queue (RabbitMQ)
- Worker pool pattern

---

#### 5. Authentication Service

**Responsibility:** User authentication and sessions

```toml
[package]
name = "auth-service"
```

**What it does:**

- User login/logout
- Session management
- Permission verification
- JWT token generation

**Scaling:**

- Stateless, easy to scale horizontally
- Cache sessions in Redis

---

#### 6. Physics Service

**Responsibility:** Physics simulation

```toml
[package]
name = "physics-service"
```

**What it does:**

- Runs Avian physics engine
- Collision detection
- Movement calculations
- Returns updates to World State

**Scaling trigger:**

- High entity count
- Complex physics interactions

**Special:**

- Can use GPU acceleration
- Dedicated physics instances

---

#### 7. Chat Service

**Responsibility:** Chat and messaging

```toml
[package]
name = "chat-service"
```

**What it does:**

- Global/local/private chat
- Chat history
- Moderation
- Emotes and commands

**Scaling:**

- High message throughput
- WebSocket connections

---

#### 8. Persistence Service

**Responsibility:** Database operations

```toml
[package]
name = "persistence-service"
```

**What it does:**

- Save/load world state
- User data persistence
- Script storage
- Backup management

**Technology:**

- PostgreSQL for relational data
- Redis for cache
- S3 for assets/backups

**Scaling:**

- Read replicas for load distribution
- Write batching for performance

---

#### 9. Asset Service

**Responsibility:** Serve game assets

```toml
[package]
name = "asset-service"
```

**What it does:**

- Serve 3D models, textures, sounds
- CDN integration
- Asset versioning
- Client caching headers

**Scaling:**

- High bandwidth usage
- CDN offloading

---

## Project Structure

```
assignment-W-4/
├── services/
│   ├── graphics-gateway/
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── text-gateway/
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── world-state/
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── script-executor/
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── auth-service/
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── physics-service/
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── chat-service/
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── persistence-service/
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   └── asset-service/
│       ├── Cargo.toml
│       ├── Dockerfile
│       └── src/
├── shared/
│   ├── proto/          # gRPC definitions
│   ├── types/          # Shared types
│   └── components/     # ECS components
├── client/             # Graphical client
├── deploy/
│   ├── k3s/            # Kubernetes manifests
│   │   ├── services/
│   │   ├── deployments/
│   │   └── hpa/        # Horizontal Pod Autoscalers
│   ├── docker-compose.yml
│   └── docker-compose.prod.yml
└── infrastructure/
    ├── redis/
    ├── postgres/
    └── rabbitmq/
```

## Communication Patterns

### 1. Synchronous (gRPC)

For immediate responses:

```
Graphics Gateway → World State: "Player moved to (x,y)"
World State → Graphics Gateway: "ACK, new visible entities: [...]"
```

### 2. Asynchronous (Message Queue)

For background tasks:

```
World State → RabbitMQ → Script Executor: "Execute on_enter script"
Script Executor → RabbitMQ → World State: "Script result: success"
```

### 3. Pub/Sub (Redis)

For events:

```
World State → Redis (publish): "player_died"
Chat Service → Redis (subscribe): "Broadcast death message"
Graphics Gateways → Redis (subscribe): "Send death animation"
```

## Deployment Strategy

### Development Environment

```bash
# Run all services locally with docker-compose
just docker-dev

# Or selectively start services
docker-compose up graphics-gateway world-state redis postgres
```

### Staging Environment

- Deploy to Orange Pi K3s cluster
- All services in one cluster
- Minimal replicas (1-2 per service)

### Production Environment

- Multi-node K3s cluster
- Services scaled independently
- Load balancing with Traefik
- Monitoring with Prometheus

## Scaling Scenarios

### Scenario 1: Graphics Client Spike

**Problem:** 100 graphical clients suddenly connect

**Solution:**

```yaml
# Graphics Gateway auto-scales from 2 → 6 instances
# World State remains at 1 instance (not bottleneck)
# Other services unaffected
```

**Result:**

- Only graphics-gateway containers scale
- Fast response (seconds)
- No downtime

---

### Scenario 2: Script Execution Load

**Problem:** Admin creates computationally expensive script

**Solution:**

```yaml
# Script Executor scales from 2 → 8 instances
# Job queue distributes work
# Timeouts prevent runaway scripts
```

**Result:**

- Scripts execute in parallel
- World State not blocked
- Other players unaffected

---

### Scenario 3: Database Writes Spike

**Problem:** Periodic world save creates write spike

**Solution:**

```yaml
# Persistence Service batches writes
# Read replicas handle queries
# Write buffering in Redis
```

**Result:**

- Smooth write distribution
- No player-facing lag

---

## Fast Deployment Strategy

### Hot Reload (No Downtime)

```bash
# Update graphics gateway
just deploy graphics-gateway

# K8s rolling update:
# 1. Deploy new version alongside old
# 2. Health check passes
# 3. Route traffic to new version
# 4. Terminate old version
```

### Blue/Green Deployment

```yaml
# Deploy new version to "green" environment
# Test with subset of users
# Switch traffic if successful
# Rollback instantly if issues
```

### Canary Deployment

```yaml
# Deploy new version to 10% of users
# Monitor metrics
# Gradually increase to 100%
# Or rollback if problems detected
```

## Service Discovery

Services find each other via **Kubernetes DNS**:

```rust
// In graphics-gateway
let world_state_url = env::var("WORLD_STATE_URL")
    .unwrap_or("http://world-state:8080".to_string());

let client = WorldStateClient::connect(world_state_url).await?;
```

Kubernetes resolves `world-state` to current healthy instances.

## Health Checks

Each service exposes health endpoints:

```rust
// In every service
#[get("/health")]
async fn health() -> &'static str {
    "OK"
}

#[get("/ready")]
async fn ready(state: State<AppState>) -> Result<&'static str> {
    // Check dependencies
    state.db_pool.get().await?;
    Ok("READY")
}
```

K8s probes:

```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5

readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 3
```

## Monitoring & Observability

### Metrics (Prometheus)

- Request latency per service
- Error rates
- Active connections
- Resource usage

### Logging (Structured)

```rust
use tracing::{info, error};

info!(
    service = "graphics-gateway",
    player_id = %player.id,
    "Player connected"
);
```

### Tracing (Jaeger)

- Distributed tracing across services
- See request path through system
- Identify bottlenecks

### Alerting

```yaml
# Alert if graphics-gateway error rate > 5%
- alert: HighErrorRate
  expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.05
  for: 5m
  annotations:
    summary: "High error rate in {{ $labels.service }}"
```

## Development Workflow

### 1. Develop Service Locally

```bash
# Work on graphics-gateway
cd services/graphics-gateway
cargo watch -x run
```

### 2. Test with Dependencies

```bash
# Start dependencies with docker-compose
docker-compose up redis postgres world-state

# Run service locally
cargo run
```

### 3. Build Container

```bash
just docker-build graphics-gateway
```

### 4. Deploy to K3s

```bash
just deploy graphics-gateway

# Or deploy all
just deploy-all
```

### 5. Monitor

```bash
# Watch logs
kubectl logs -f deployment/graphics-gateway

# Check metrics
just metrics graphics-gateway
```

## Cost Optimization

### Development: All-in-One

```yaml
# Single container with all services
# Fast iteration, low resources
```

### Staging: Minimal Microservices

```yaml
# Essential services only
# 1-2 replicas each
# Orange Pi 5+ is sufficient
```

### Production: Full Scaling

```yaml
# All services
# Auto-scaling enabled
# Multi-node cluster
```

## Migration Path

### Phase 1: Monolith (Current)

```
Single server binary with all functionality
```

### Phase 2: Modular Monolith

```
Separate modules, same binary
Clean interfaces between modules
```

### Phase 3: Service Extraction

```
Extract graphics gateway first (easy to split)
Then text gateway
Then scripting (for isolation)
```

### Phase 4: Full Microservices

```
All services containerized
Independent scaling
Production-ready
```

## Learning Opportunities

This architecture teaches:

- **Service design** - Breaking monoliths into services
- **gRPC** - Service-to-service communication
- **Message queues** - Async processing
- **Container orchestration** - K8s/K3s
- **Observability** - Metrics, logs, traces
- **Distributed systems** - Consensus, CAP theorem
- **DevOps** - CI/CD pipelines, GitOps
- **Performance** - Load testing, optimization

## References

- [12-Factor App](https://12factor.net/)
- [Microservices Patterns](https://microservices.io/patterns/)
- [Kubernetes Docs](https://kubernetes.io/docs/)
- [gRPC in Rust](https://github.com/hyperium/tonic)
- [Distributed Systems for Fun and Profit](http://book.mixu.net/distsys/)

---

**Next Steps:**

1. Start with modular monolith
2. Extract first service (graphics-gateway)
3. Add service discovery
4. Implement health checks
5. Setup basic monitoring

This architecture scales with your learning and project needs! 🦀
