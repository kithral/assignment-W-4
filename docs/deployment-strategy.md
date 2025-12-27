# Deployment Strategy

## Overview

This document describes the deployment strategy for the microservices architecture, focusing on fast deployments, zero-downtime updates, and independent service scaling.

## Deployment Environments

### Local Development

**Purpose:** Fast iteration during development

**Setup:**

```bash
# Run all services with docker-compose
docker-compose -f deploy/docker-compose.dev.yml up

# Or run individual services
docker-compose up world-state redis postgres
```

**Characteristics:**

- All services in one compose file
- Shared volumes for hot reload
- Debug builds for fast compilation
- Local PostgreSQL and Redis

---

### Staging (Orange Pi)

**Purpose:** Pre-production testing

**Setup:**

```bash
# Deploy to K3s on Orange Pi
just deploy-staging

# Or individual service
just deploy graphics-gateway staging
```

**Characteristics:**

- Single-node K3s cluster
- Minimal replicas (1-2 per service)
- Shared infrastructure services
- Production-like environment
- Safe for breaking changes

---

### Production (Multi-node)

**Purpose:** Live environment

**Setup:**

```bash
# Deploy all services
just deploy-production

# Canary deployment for risky changes
just deploy-canary graphics-gateway
```

**Characteristics:**

- Multi-node K3s cluster (when needed)
- Auto-scaling enabled
- High availability
- Monitoring and alerting
- Gradual rollouts

---

## Deployment Patterns

### 1. Rolling Update (Default)

**What:** Gradually replace old instances with new ones

**When to use:** Standard updates, low risk

**How it works:**

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: graphics-gateway
spec:
  replicas: 4
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1        # Add 1 new pod at a time
      maxUnavailable: 1  # Allow 1 old pod down at a time
```

**Process:**

1. Deploy pod with new version
2. Wait for health check
3. Terminate one old pod
4. Repeat until all updated

**Pros:**

- ✅ Zero downtime
- ✅ Automatic rollback if health checks fail
- ✅ Simple to configure

**Cons:**

- ❌ Both versions run simultaneously (mixed traffic)
- ❌ Slower than recreate

**Example:**

```bash
# Update graphics-gateway image
kubectl set image deployment/graphics-gateway \
  graphics-gateway=assignment-w4/graphics-gateway:v2.0.0

# Watch rollout
kubectl rollout status deployment/graphics-gateway

# Rollback if needed
kubectl rollout undo deployment/graphics-gateway
```

---

### 2. Blue/Green Deployment

**What:** Run two identical environments, switch traffic instantly

**When to use:** High-risk updates, need instant rollback

**How it works:**

```yaml
# Blue environment (current production)
apiVersion: v1
kind: Service
metadata:
  name: graphics-gateway
spec:
  selector:
    app: graphics-gateway
    version: blue    # Routes to blue

---
# Green environment (new version)
apiVersion: apps/v1
kind: Deployment
metadata:
  name: graphics-gateway-green
spec:
  replicas: 4
  template:
    metadata:
      labels:
        app: graphics-gateway
        version: green
```

**Process:**

1. Deploy green environment alongside blue
2. Test green environment
3. Switch service selector to green
4. Monitor metrics
5. Keep blue for instant rollback

**Pros:**

- ✅ Instant rollback (change selector back)
- ✅ Full testing before switching
- ✅ No mixed versions

**Cons:**

- ❌ Requires 2x resources during deployment
- ❌ More complex setup

**Example:**

```bash
# Deploy green
kubectl apply -f deploy/k3s/graphics-gateway-green.yaml

# Test green
kubectl port-forward svc/graphics-gateway-green 5000:5000

# Switch traffic (edit selector: blue → green)
kubectl patch service graphics-gateway -p '{"spec":{"selector":{"version":"green"}}}'

# Rollback if needed (green → blue)
kubectl patch service graphics-gateway -p '{"spec":{"selector":{"version":"blue"}}}'
```

---

### 3. Canary Deployment

**What:** Gradually route traffic to new version

**When to use:** Very high-risk updates, gradual validation

**How it works:**

```yaml
# Main deployment (90% traffic)
apiVersion: apps/v1
kind: Deployment
metadata:
  name: graphics-gateway
spec:
  replicas: 9

---
# Canary deployment (10% traffic)
apiVersion: apps/v1
kind: Deployment
metadata:
  name: graphics-gateway-canary
spec:
  replicas: 1

---
# Service routes to both
apiVersion: v1
kind: Service
metadata:
  name: graphics-gateway
spec:
  selector:
    app: graphics-gateway  # Matches both deployments
```

**Process:**

1. Deploy canary with 10% of replicas
2. Monitor error rates, latency
3. Gradually increase canary %
4. Eventually replace all old instances
5. Or rollback if issues detected

**Pros:**

- ✅ Minimal blast radius (only 10% affected)
- ✅ Real production testing
- ✅ Gradual confidence building

**Cons:**

- ❌ Slower rollout
- ❌ Complex monitoring required

**Example with Istio/Traefik:**

```yaml
apiVersion: traefik.containo.us/v1alpha1
kind: TraefikService
metadata:
  name: graphics-gateway-weighted
spec:
  weighted:
    services:
      - name: graphics-gateway-v1
        weight: 9    # 90% traffic
        port: 5000
      - name: graphics-gateway-v2
        weight: 1    # 10% traffic
        port: 5000
```

---

## Fast Deployment Workflow

### Single Service Update

**Scenario:** Fix bug in script-executor service

**Time:** ~2 minutes from commit to production

```bash
# 1. Commit fix
git add services/script-executor/src/main.rs
git commit -m "fix: prevent script timeout race condition"
git push

# 2. CI builds container (auto-triggered)
# GitHub Actions:
# - Builds only script-executor
# - Runs tests
# - Pushes to registry

# 3. Deploy to staging
just deploy script-executor staging

# 4. Automated tests run
# Health checks pass
# Integration tests pass

# 5. Deploy to production
just deploy script-executor production

# 6. Rolling update:
# - 1 new pod starts
# - Health check passes
# - 1 old pod terminates
# - Repeat 3 more times

# Total time: ~2 minutes
# Downtime: 0 seconds
# Services updated: 1/9
```

**Comparison to Monolith:**

- **Before:** Rebuild entire server, redeploy everything, ~10 minutes, brief downtime
- **After:** Rebuild one service, update only that service, ~2 minutes, zero downtime

---

### Multi-Service Update

**Scenario:** Add new feature requiring changes to multiple services

**Time:** ~5 minutes for all services

```bash
# 1. Update services
git add services/graphics-gateway/
git add services/world-state/
git add services/chat-service/
git commit -m "feat: add proximity voice chat"
git push

# 2. CI builds all changed services in parallel
# Each runs in separate workflow

# 3. Deploy in dependency order
just deploy world-state staging     # Core service first
just deploy graphics-gateway staging
just deploy chat-service staging

# 4. Integration test suite runs

# 5. Deploy to production
just deploy-all production

# Rolling updates run in parallel
# Total time: ~5 minutes
```

---

## Health Checks

Every service implements health endpoints:

```rust
// health.rs in each service
use axum::{routing::get, Router};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    // Service-specific state
}

async fn health() -> &'static str {
    "OK"
}

async fn ready(state: Arc<AppState>) -> Result<&'static str, StatusCode> {
    // Check dependencies
    state.check_database().await?;
    state.check_redis().await?;
    Ok("READY")
}

pub fn health_routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
}
```

**Kubernetes Configuration:**

```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5
  failureThreshold: 3

readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 3
  failureThreshold: 2
```

**Behavior:**

- **Liveness fails:** Container restarts
- **Readiness fails:** No traffic routed to pod
- Both must pass for pod to receive traffic

---

## Rollback Strategies

### Automatic Rollback

**Scenario:** New version fails health checks

```yaml
spec:
  progressDeadlineSeconds: 600  # Fail if not ready in 10 min
  revisionHistoryLimit: 10      # Keep last 10 versions
```

**What happens:**

1. New pod starts
2. Health check fails repeatedly
3. After 600s, Kubernetes fails the deployment
4. Old pods remain running
5. Alert fires to notify team

### Manual Rollback

**Scenario:** New version passes health checks but has runtime issues

```bash
# See deployment history
kubectl rollout history deployment/graphics-gateway

# Rollback to previous version
kubectl rollout undo deployment/graphics-gateway

# Rollback to specific revision
kubectl rollout undo deployment/graphics-gateway --to-revision=3
```

**Time:** ~30 seconds

### Blue/Green Rollback

**Scenario:** Instant rollback needed

```bash
# Change service selector back to blue
kubectl patch service graphics-gateway \
  -p '{"spec":{"selector":{"version":"blue"}}}'
```

**Time:** ~1 second (just updates routing)

---

## Monitoring During Deployment

### Pre-Deployment Checks

```bash
# Ensure cluster healthy
kubectl get nodes
kubectl top nodes

# Check current service status
kubectl get pods -l app=graphics-gateway
kubectl logs deployment/graphics-gateway
```

### During Deployment

```bash
# Watch rollout in real-time
kubectl rollout status deployment/graphics-gateway --watch

# Stream logs
kubectl logs -f deployment/graphics-gateway --all-containers

# Monitor metrics
just metrics graphics-gateway
```

### Post-Deployment Validation

```bash
# Check all pods running
kubectl get pods -l app=graphics-gateway

# Verify health
kubectl exec deployment/graphics-gateway -- wget -qO- http://localhost:8080/health

# Check error rates in Prometheus
# Check latency dashboard in Grafana
```

---

## CI/CD Pipeline

```yaml
# .github/workflows/deploy-service.yml
name: Deploy Service

on:
  push:
    paths:
      - 'services/graphics-gateway/**'

jobs:
  build:
    runs-on: self-hosted  # Orange Pi
    steps:
      - uses: actions/checkout@v4

      - name: Build Docker image
        run: |
          cd services/graphics-gateway
          docker build -t graphics-gateway:${{ github.sha }} .

      - name: Run tests
        run: |
          docker run graphics-gateway:${{ github.sha }} cargo test

      - name: Push to registry
        run: |
          docker tag graphics-gateway:${{ github.sha }} \
            registry.local/graphics-gateway:${{ github.sha }}
          docker push registry.local/graphics-gateway:${{ github.sha }}

  deploy-staging:
    needs: build
    runs-on: self-hosted
    steps:
      - name: Deploy to staging
        run: |
          kubectl set image deployment/graphics-gateway \
            graphics-gateway=registry.local/graphics-gateway:${{ github.sha }} \
            -n staging

      - name: Wait for rollout
        run: |
          kubectl rollout status deployment/graphics-gateway -n staging

      - name: Run integration tests
        run: |
          ./tests/integration-test.sh staging

  deploy-production:
    needs: deploy-staging
    runs-on: self-hosted
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Deploy to production
        run: |
          kubectl set image deployment/graphics-gateway \
            graphics-gateway=registry.local/graphics-gateway:${{ github.sha }} \
            -n production

      - name: Monitor rollout
        run: |
          kubectl rollout status deployment/graphics-gateway -n production

      - name: Smoke test
        run: |
          ./tests/smoke-test.sh production
```

---

## Cost Optimization

### Development

- All services in one docker-compose
- Minimal resources
- Quick iteration

### Staging

- Shared infrastructure (1 Redis, 1 PostgreSQL)
- Minimal replicas (1 per service)
- Orange Pi 5+ sufficient

### Production

- Dedicated infrastructure per service (if needed)
- Auto-scaling based on load
- Add nodes only when necessary

**Example Resource Allocation:**

```yaml
# Small service
resources:
  requests:
    memory: "64Mi"
    cpu: "100m"
  limits:
    memory: "128Mi"
    cpu: "200m"

# Large service
resources:
  requests:
    memory: "512Mi"
    cpu: "500m"
  limits:
    memory: "1Gi"
    cpu: "1000m"
```

---

## Disaster Recovery

### Backup Strategy

```bash
# Automated daily backups
kubectl create cronjob backup-world-state \
  --image=backup-tool \
  --schedule="0 2 * * *" \
  -- /backup.sh
```

### Restore Process

```bash
# Restore from backup
kubectl exec -it persistence-service -- /restore.sh backup-2025-12-26.tar.gz
```

### High Availability

```yaml
# Critical services have replicas
spec:
  replicas: 3

# Anti-affinity (don't run all on same node)
affinity:
  podAntiAffinity:
    requiredDuringSchedulingIgnoredDuringExecution:
    - labelSelector:
        matchExpressions:
        - key: app
          operator: In
          values:
          - world-state
      topologyKey: kubernetes.io/hostname
```

---

## Summary

**Key Benefits:**

1. **Fast deployments** - Update one service in ~2 minutes
2. **Zero downtime** - Rolling updates with health checks
3. **Easy rollback** - Undo to previous version in seconds
4. **Independent scaling** - Scale only what needs it
5. **Fault isolation** - Service failures don't cascade
6. **Cost efficient** - Run minimal resources, scale when needed

**Best Practices:**

- Always deploy to staging first
- Monitor metrics during rollout
- Keep rollback plan ready
- Use health checks on all services
- Automate everything via CI/CD

This deployment strategy ensures fast iteration while maintaining reliability! 🚀
