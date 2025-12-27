# Microservices Architecture - Summary

## What Changed

Your project now uses a **microservices architecture** where the server is split into **9 independent services** that can scale and deploy independently.

## Why This Is Better

### Before (Monolithic Server)

```
┌─────────────────────────────┐
│      Entire Server          │
│  - Graphics handling        │
│  - Text handling            │
│  - Scripting                │
│  - Physics                  │
│  - Database                 │
│  - Authentication           │
│  - Chat                     │
└─────────────────────────────┘
```

**Problems:**

- ❌ Update anything = redeploy everything
- ❌ Graphics spike = entire server needs more resources
- ❌ Script crash = entire server crashes
- ❌ ~10 minute deployments with downtime

### After (Microservices)

```
┌──────────┐ ┌──────────┐ ┌──────────┐
│ Graphics │ │   Text   │ │  Script  │
│ Gateway  │ │ Gateway  │ │ Executor │
└──────────┘ └──────────┘ └──────────┘
┌──────────┐ ┌──────────┐ ┌──────────┐
│  World   │ │   Auth   │ │ Physics  │
│  State   │ │ Service  │ │ Service  │
└──────────┘ └──────────┘ └──────────┘
┌──────────┐ ┌──────────┐ ┌──────────┐
│   Chat   │ │Persistence│ │  Asset   │
│ Service  │ │  Service  │ │ Service  │
└──────────┘ └──────────┘ └──────────┘
```

**Benefits:**

- ✅ Update one service = ~2 minute deployment
- ✅ Graphics spike = scale only graphics-gateway
- ✅ Script crash = only script-executor restarts
- ✅ Zero downtime deployments

## The 9 Services

| Service | Purpose | Scales When |
|---------|---------|-------------|
| **Graphics Gateway** | UDP graphical clients | Many 3D players |
| **Text Gateway** | TCP/Telnet clients | Many text players |
| **World State** | Core ECS and game state | (Vertical scaling) |
| **Script Executor** | Run user scripts | Heavy scripting load |
| **Auth Service** | Login and sessions | Login spikes |
| **Physics Service** | Physics simulation | Many physics entities |
| **Chat Service** | Chat and messaging | High chat volume |
| **Persistence Service** | Database operations | Heavy DB load |
| **Asset Service** | Serve assets (CDN) | Asset downloads |

## Real-World Example

### Scenario: Graphics Client Spike

**Situation:** 50 new graphical players join

**Monolithic Response:**

```
1. Entire server CPU maxes out
2. All players lag (text + graphics)
3. Admin manually adds more server capacity
4. Redeploys entire server
5. Brief downtime for all players
Time: 15-20 minutes, affects everyone
```

**Microservices Response:**

```
1. Graphics-gateway CPU reaches 70%
2. K8s auto-scales graphics-gateway from 2 → 6 instances
3. New instances start in 30 seconds
4. Load distributed across 6 instances
5. Text players and other services unaffected
Time: 30 seconds, zero downtime, surgical scaling
```

## Fast Deployment Example

### Scenario: Bug Fix in Script Executor

**Problem:** Scripts sometimes timeout incorrectly

**Old Way (Monolith):**

```bash
1. Fix bug in code
2. Rebuild entire server                 [~5 min]
3. Run all tests                          [~2 min]
4. Stop server                            [downtime starts]
5. Deploy new binary                      [~1 min]
6. Restart server                         [~2 min]
7. Verify all services work               [~1 min]

Total: ~11 minutes, ~3 minutes downtime
```

**New Way (Microservices):**

```bash
1. Fix bug in services/script-executor/
2. CI builds only script-executor         [~1 min]
3. Run script-executor tests              [~30 sec]
4. Rolling update:
   - Start new pod                        [~10 sec]
   - Health check passes                  [~5 sec]
   - Route traffic to new pod             [instant]
   - Terminate old pod                    [~5 sec]
   - Repeat for other replicas            [~20 sec]

Total: ~2 minutes, 0 seconds downtime
Services affected: 1/9 (script-executor only)
```

## Project Structure

Your repository now has this structure:

```
assignment-W-4/
├── services/                    # Each service is independent
│   ├── graphics-gateway/
│   │   ├── Cargo.toml          # Own dependencies
│   │   ├── Dockerfile          # Own container
│   │   └── src/main.rs         # Own codebase
│   ├── text-gateway/
│   ├── world-state/
│   ├── script-executor/
│   ├── auth-service/
│   ├── physics-service/
│   ├── chat-service/
│   ├── persistence-service/
│   └── asset-service/
├── shared/                      # Shared code
│   ├── proto/                  # gRPC definitions
│   ├── types/                  # Common types
│   └── components/             # ECS components
├── client/                     # Graphical client
├── deploy/                     # Deployment configs
│   ├── k3s/                   # Kubernetes manifests
│   │   ├── graphics-gateway.yaml
│   │   ├── text-gateway.yaml
│   │   └── ...
│   ├── docker-compose.dev.yml  # Local development
│   └── docker-compose.prod.yml # Production
└── docs/
    ├── microservices-architecture.md  # Full design
    └── deployment-strategy.md         # Deploy workflows
```

## Communication Between Services

Services talk to each other via:

### 1. gRPC (Synchronous)

**For:** Immediate responses needed

```rust
// Graphics Gateway calls World State
let response = world_state_client
    .move_player(MoveRequest {
        player_id: 123,
        x: 10.0,
        y: 5.0,
    })
    .await?;
```

### 2. Message Queue (Asynchronous)

**For:** Background tasks

```rust
// World State sends script execution job
queue.publish(ScriptExecutionJob {
    script_id: "door_on_use",
    player_id: 123,
});

// Script Executor picks it up
let job = queue.consume().await?;
execute_script(job.script_id, job.player_id)?;
```

### 3. Redis Pub/Sub (Events)

**For:** Broadcasting events

```rust
// World State publishes event
redis.publish("player_events", PlayerDiedEvent {
    player_id: 123,
    killer_id: 456,
});

// Chat Service subscribes
let event = redis.subscribe("player_events").await?;
broadcast_death_message(event.player_id, event.killer_id);
```

## Development Workflow

### Local Development

```bash
# Run all services locally
docker-compose up

# Or just the ones you're working on
docker-compose up world-state redis postgres
cargo run --bin graphics-gateway
```

### Staging (Orange Pi)

```bash
# Deploy to K3s cluster
just deploy graphics-gateway staging

# Check status
kubectl get pods -n staging
```

### Production

```bash
# Deploy with rolling update
just deploy graphics-gateway production

# Or canary (gradual rollout)
just deploy-canary graphics-gateway
```

## Auto-Scaling Example

Kubernetes configuration:

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: graphics-gateway
spec:
  scaleTargetRef:
    kind: Deployment
    name: graphics-gateway
  minReplicas: 2      # Always at least 2
  maxReplicas: 10     # Scale up to 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70  # Scale at 70% CPU
```

**Behavior:**

- **Low load:** 2 instances running
- **Medium load (70% CPU):** Scales to 4 instances
- **High load:** Scales up to 10 instances
- **Load drops:** Scales back down to 2

**Time to scale:** ~30 seconds from trigger to new instance ready

## Deployment Strategies

### 1. Rolling Update (Default)

- Gradual replacement of old with new
- Zero downtime
- Easy rollback

### 2. Blue/Green

- Two complete environments
- Instant traffic switch
- Instant rollback

### 3. Canary

- 10% traffic to new version
- Gradual increase
- Minimal blast radius

**See [docs/deployment-strategy.md](deployment-strategy.md) for details.**

## Learning Opportunities

This architecture adds significant learning:

| Topic | What You'll Learn |
|-------|-------------------|
| **Microservices** | Service boundaries, communication patterns |
| **gRPC** | Protocol buffers, bi-directional streaming |
| **Message Queues** | RabbitMQ, async processing, job queues |
| **Kubernetes** | Pods, deployments, services, autoscaling |
| **Observability** | Distributed tracing, metrics, logging |
| **DevOps** | CI/CD, blue/green deploys, canary releases |
| **Distributed Systems** | CAP theorem, eventual consistency |
| **Load Balancing** | Traefik, service mesh |

## Cost Efficiency

### Development

- **All-in-one docker-compose**
- Minimal resources
- Fast iteration

### Staging (Orange Pi)

- **Single K3s node**
- 1-2 replicas per service
- ~4GB RAM total
- Fits on Orange Pi 5+

### Production (Multi-node)

- **Scale only what's needed**
- Graphics spike? Scale only graphics-gateway
- Database load? Scale only persistence-service
- More cost-efficient than scaling entire monolith

## Migration Path

### Phase 1: Modular Monolith (Start Here)

```
Single binary with clean module boundaries
Prepare for service extraction
```

### Phase 2: Extract First Service

```
Extract graphics-gateway
Test dual deployment
Learn K8s basics
```

### Phase 3: Gradual Extraction

```
Extract services one by one
Add monitoring and tracing
Build confidence
```

### Phase 4: Full Microservices

```
All 9 services deployed
Auto-scaling configured
Production-ready
```

## Key Takeaways

1. **Independent Scaling**
   - Graphics spike? Scale graphics-gateway
   - Script load? Scale script-executor
   - Don't scale what doesn't need it

2. **Fast Deployments**
   - ~2 minutes to deploy one service
   - Zero downtime with rolling updates
   - Easy rollback in seconds

3. **Fault Isolation**
   - Script crash? Only script-executor affected
   - Database issue? Other services continue
   - Better reliability overall

4. **Technology Flexibility**
   - Use Rust for performance-critical services
   - Use Python for admin tools (if needed)
   - Right tool for each job

5. **Development Velocity**
   - Teams can work on different services
   - No coordination needed for deploys
   - Faster iteration

## Documentation

- **[microservices-architecture.md](microservices-architecture.md)** - Complete design (717 lines)
- **[deployment-strategy.md](deployment-strategy.md)** - Deploy workflows (655 lines)
- **[vision.md](vision.md)** - Overall vision (374 lines)

## Next Steps

1. **Learn the architecture**
   - Read [microservices-architecture.md](microservices-architecture.md)
   - Understand service boundaries

2. **Start simple**
   - Begin with modular monolith
   - Clean interfaces between modules

3. **Extract first service**
   - Graphics-gateway is easiest
   - Learn K8s basics
   - Setup monitoring

4. **Iterate and expand**
   - Extract services gradually
   - Add auto-scaling
   - Build production readiness

This architecture ensures your project can scale from a learning exercise on a single Orange Pi to a production system handling thousands of concurrent players! 🚀

---

**Total Documentation:** 4,142 lines across 12 files covering architecture, deployment, scripting, vision, and development workflows.
