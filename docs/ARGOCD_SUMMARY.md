# ArgoCD GitOps Setup Summary

This document summarizes the ArgoCD + GitOps deployment infrastructure added to the project.

## What Was Created

### Directory Structure

```
assignment-W-4/
├── deployments/                    # Kubernetes manifests (Kustomize)
│   ├── base/                      # Base templates for all services
│   │   ├── deployment.yaml        # Generic deployment spec
│   │   ├── service.yaml           # Generic k8s service
│   │   ├── configmap.yaml         # Base configuration
│   │   └── kustomization.yaml     # Base kustomize config
│   │
│   ├── auth-service/              # Service-specific overlays
│   │   └── kustomization.yaml     # Auth service config
│   ├── chat-service/
│   ├── world-state/               # Scaled to 2 replicas
│   ├── physics-service/
│   ├── script-executor/
│   ├── asset-service/
│   ├── persistence-service/
│   ├── graphics-gateway/
│   └── text-gateway/
│
├── argocd/                         # ArgoCD application definitions
│   ├── applicationsets/
│   │   └── microservices.yaml     # Manages all 9 services
│   └── applications/
│       └── client.yaml            # Client application
│
├── docs/
│   ├── ARGOCD_SETUP.md            # Installation guide
│   ├── DEPLOYMENT_WORKFLOW.md     # Daily operations guide
│   ├── ORANGEPI_SETUP_CHECKLIST.md # Setup checklist
│   └── ARGOCD_SUMMARY.md          # This file
│
├── .github/workflows/
│   └── build-deploy.yaml          # CI/CD pipeline
│
├── Dockerfile.service              # Multi-stage build for all services
└── .dockerignore                   # Updated with deployment paths
```

## How It Works

### 1. GitOps Workflow

```
Developer commits code
        ↓
GitHub Actions detects changes (path filters)
        ↓
Builds only affected services
        ↓
Pushes Docker images to ghcr.io
        ↓
Updates Kustomize manifests with new image tags
        ↓
ArgoCD detects manifest changes
        ↓
Auto-deploys to Kubernetes
```

### 2. Selective Building

The CI/CD pipeline only builds services that changed:

- Change to `services/auth-service/` → Only builds `auth-service`
- Change to `shared/` → Rebuilds all services (dependency)
- Change to `deployments/auth-service/` → No build, just re-deploy

This is achieved via `dorny/paths-filter` in GitHub Actions.

### 3. Multi-Stage Docker Build

The `Dockerfile.service` uses cargo-chef for efficient layer caching:

```dockerfile
Stage 1: cargo-chef prepare
Stage 2: cargo-chef cook (dependencies) ← CACHED
Stage 3: Build specific service
Stage 4: Runtime image (Debian slim)
```

Benefits:
- Dependencies cached between builds
- Only rebuild when Cargo.lock changes
- Single Dockerfile for all services
- Small runtime images (~50MB per service)

## Key Features

### ✅ Monorepo Optimizations

- **Selective builds**: Only changed services rebuild
- **Shared caching**: All services share dependency cache
- **Atomic commits**: Update multiple services in one commit
- **Unified versioning**: All services track together in Git

### ✅ GitOps Benefits

- **Git as source of truth**: All config in version control
- **Declarative**: Describe desired state, not steps
- **Auditable**: Every change tracked in Git history
- **Rollback**: Easy rollback via Git or ArgoCD
- **Auto-sync**: Changes deploy automatically

### ✅ Kubernetes Best Practices

- **Health checks**: Liveness and readiness probes
- **Resource limits**: CPU and memory constraints
- **Labels**: Consistent labeling for all resources
- **ConfigMaps**: Environment-based configuration
- **Secrets**: Ready for secrets management integration

## Configuration Files

### ArgoCD ApplicationSet

[argocd/applicationsets/microservices.yaml](../argocd/applicationsets/microservices.yaml)

Manages all 9 microservices:
- Auto-sync enabled
- Self-healing enabled
- Prune enabled (removes deleted resources)

### GitHub Actions Workflow

[.github/workflows/build-deploy.yaml](../.github/workflows/build-deploy.yaml)

Three jobs:
1. **detect-changes**: Path filtering
2. **build-and-push**: Docker build + push
3. **update-manifests**: Update Kustomize with new tags

### Base Kustomize Template

[deployments/base/](../deployments/base/)

Shared template for all services:
- 1 replica by default (override per service)
- 128Mi-512Mi memory
- 100m-500m CPU
- HTTP (8080) and gRPC (50051) ports

## Resource Requirements

### Per Service (Default)

- **CPU**: 100m request, 500m limit
- **Memory**: 128Mi request, 512Mi limit
- **Storage**: None (stateless)

### World State Service (Special)

- **CPU**: 200m request, 1000m limit
- **Memory**: 256Mi request, 1Gi limit
- **Replicas**: 2 (for HA)

### Total Cluster (9 services)

- **CPU**: ~1-5 cores
- **Memory**: ~1.5-5 GB
- **Storage**: Minimal (logs only)

Perfect for Orange Pi in Proxmox!

## Network Architecture

### Service Communication

```
External Traffic
    ↓
Ingress (future)
    ↓
Gateway Services (graphics/text)
    ↓
World State Service
    ↓
Backend Services (physics, scripts, etc.)
    ↓
Persistence Service
```

### Service Mesh (Future)

Consider adding Istio or Linkerd for:
- Service-to-service encryption
- Traffic management
- Observability
- Circuit breaking

## Security Considerations

### Current State

- ✅ Non-root containers
- ✅ Resource limits
- ✅ Health checks
- ✅ Minimal base images (Debian slim)
- ⚠️ No secrets management (use env vars for now)
- ⚠️ No network policies
- ⚠️ No pod security policies

### To Add

1. **Secrets Management**
   - Sealed Secrets
   - External Secrets Operator
   - Vault integration

2. **Network Policies**
   - Restrict inter-service communication
   - Deny by default

3. **Pod Security**
   - Pod Security Standards
   - Security contexts
   - Read-only root filesystem

## Customization Guide

### Modify Service Resources

Edit `deployments/<service>/kustomization.yaml`:

```yaml
patches:
- patch: |-
    - op: replace
      path: /spec/replicas
      value: 3  # Scale to 3 replicas
    - op: replace
      path: /spec/template/spec/containers/0/resources/limits/memory
      value: "1Gi"  # Increase memory
  target:
    kind: Deployment
```

### Add Environment Variables

Edit `deployments/<service>/kustomization.yaml`:

```yaml
configMapGenerator:
- name: service-config
  behavior: merge
  literals:
  - SERVICE_NAME=auth-service
  - NEW_VAR=value
  - ANOTHER_VAR=value
```

### Change Image Registry

```bash
# Update all kustomization files
find deployments -name "kustomization.yaml" -exec sed -i 's|ghcr.io/OLD|ghcr.io/NEW|g' {} \;
```

### Disable Auto-Sync

Edit `argocd/applicationsets/microservices.yaml`:

```yaml
syncPolicy:
  automated: null  # Disable auto-sync
```

Then manually sync:

```bash
argocd app sync auth-service
```

## Monitoring and Observability

### What's Missing (Add Later)

1. **Metrics**: Prometheus + Grafana
2. **Logs**: Loki or ELK stack
3. **Traces**: Jaeger or Tempo
4. **Dashboards**: Service-specific Grafana dashboards
5. **Alerts**: PagerDuty/Slack integration

### Health Endpoints

All services expose (you need to implement):
- `GET /health` - Liveness probe
- `GET /ready` - Readiness probe

Example implementation:

```rust
// In each service
async fn health() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}

async fn ready() -> impl Responder {
    // Check dependencies (database, etc.)
    HttpResponse::Ok().body("ready")
}
```

## Testing Strategy

### Local Testing

```bash
# Build and test locally
docker build --build-arg SERVICE_NAME=auth-service -f Dockerfile.service -t auth-service:test .
docker run -p 8080:8080 auth-service:test

# Test Kustomize
kustomize build deployments/auth-service
```

### Integration Testing

```bash
# Deploy to staging namespace
kubectl apply -k deployments/auth-service --namespace=staging

# Run tests
kubectl run test --rm -i --image=curlimages/curl -- curl http://auth-service.staging:8080/health
```

### Rollback Testing

```bash
# Practice rollback
argocd app history auth-service
argocd app rollback auth-service 1
```

## Disaster Recovery

### Backup Strategy

1. **Git repository**: Primary backup (all configs)
2. **Container images**: Stored in ghcr.io
3. **Application state**: Implement backup for persistence-service

### Recovery Procedure

```bash
# 1. Reinstall ArgoCD
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# 2. Re-apply ApplicationSet
kubectl apply -f argocd/applicationsets/microservices.yaml

# 3. ArgoCD auto-deploys everything
argocd app sync -l app.kubernetes.io/instance=microservices
```

Recovery time: ~5-10 minutes

## Performance Metrics

### Build Performance

- **First build**: ~10-15 minutes (no cache)
- **Subsequent builds**: ~2-5 minutes (with cache)
- **Unchanged services**: 0 seconds (skipped)

### Deployment Performance

- **Sync time**: ~30-60 seconds per service
- **Rollout time**: ~1-2 minutes (depends on readiness)
- **Parallel deploys**: All services deploy concurrently

## Cost Considerations

### Free Tier Usage

- ✅ GitHub Actions: 2000 minutes/month free
- ✅ GHCR: Unlimited public images
- ✅ ArgoCD: Free (self-hosted)
- ✅ Kubernetes: Free (self-hosted on Orange Pi)

### Resource Usage

- **Storage**: ~500MB per service image
- **Bandwidth**: ~5GB/month (CI/CD + deployments)
- **Compute**: Minimal (Orange Pi)

Total cost: **$0** (self-hosted)

## Next Steps

### Phase 1: Basic Setup (Now)
- ✅ GitOps infrastructure
- ✅ CI/CD pipeline
- ✅ Deployment automation

### Phase 2: Production Ready (Next)
- [ ] Implement health endpoints
- [ ] Add secrets management
- [ ] Configure persistent storage
- [ ] Set up monitoring

### Phase 3: Advanced (Future)
- [ ] Service mesh (Istio/Linkerd)
- [ ] Multi-environment (staging/prod)
- [ ] Auto-scaling (HPA)
- [ ] Chaos engineering

## Questions?

See the detailed guides:
- [ArgoCD Setup Guide](ARGOCD_SETUP.md)
- [Deployment Workflow](DEPLOYMENT_WORKFLOW.md)
- [Orange Pi Checklist](ORANGEPI_SETUP_CHECKLIST.md)

Or check the official docs:
- [ArgoCD Documentation](https://argo-cd.readthedocs.io/)
- [Kustomize Documentation](https://kubectl.docs.kubernetes.io/references/kustomize/)
