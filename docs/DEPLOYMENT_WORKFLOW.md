# Deployment Workflow

Quick reference for deploying services with ArgoCD + GitHub Actions.

## How It Works

```
1. Developer pushes code
   ↓
2. GitHub Actions detects changes (path filters)
   ↓
3. Only affected services are built
   ↓
4. Docker images pushed to ghcr.io
   ↓
5. Kustomize manifests updated with new image tags
   ↓
6. ArgoCD detects manifest changes
   ↓
7. Services automatically deployed to Kubernetes
```

## Making Changes

### Single Service Change

```bash
# Edit a service
vim services/auth-service/src/main.rs

# Commit and push
git add services/auth-service
git commit -m "feat: add new auth endpoint"
git push origin main
```

**Result:** Only `auth-service` is built and deployed.

### Shared Library Change

```bash
# Edit shared code
vim shared/src/protocol.rs

# Commit and push
git add shared
git commit -m "feat: update protocol"
git push origin main
```

**Result:** All services that depend on `shared` are rebuilt and deployed.

### Configuration Change

```bash
# Edit Kubernetes config
vim deployments/auth-service/kustomization.yaml

# Commit and push
git add deployments/auth-service
git commit -m "chore: increase auth-service replicas"
git push origin main
```

**Result:** Only Kubernetes manifests are updated, no rebuild.

## Monitoring Deployments

### Watch CI/CD Pipeline

```bash
# View in GitHub
https://github.com/YOUR_USERNAME/assignment-W-4/actions

# Or use GitHub CLI
gh run watch
```

### Watch ArgoCD Sync

```bash
# List all applications
argocd app list

# Watch specific service
argocd app wait auth-service --timeout 300

# View sync status
argocd app get auth-service
```

### Check Kubernetes Pods

```bash
# Watch pods update
kubectl get pods -w

# Check specific service
kubectl get pods -l app=auth-service

# View logs
kubectl logs -l app=auth-service -f
```

## Deployment States

### Green (Healthy)
- ✅ Synced
- ✅ Healthy
- ✅ All pods running

### Yellow (Progressing)
- 🔄 Syncing
- ⏳ Pods starting
- ⏳ Health checks pending

### Red (Failed)
- ❌ Out of sync
- ❌ CrashLoopBackOff
- ❌ ImagePullBackOff

## Common Operations

### Trigger Rebuild

```bash
# Make empty commit
git commit --allow-empty -m "chore: rebuild auth-service"
git push origin main
```

### Rollback Deployment

```bash
# View history
argocd app history auth-service

# Rollback to previous version
argocd app rollback auth-service

# Or rollback to specific revision
argocd app rollback auth-service 5
```

### Manual Sync

```bash
# Sync single service
argocd app sync auth-service

# Sync all services
argocd app sync -l app.kubernetes.io/instance=microservices

# Force sync (ignore differences)
argocd app sync auth-service --force
```

### Scale Service

```bash
# Edit kustomization
cd deployments/auth-service
kustomize edit add patch --kind Deployment --patch '
- op: replace
  path: /spec/replicas
  value: 3
'

# Commit
git add kustomization.yaml
git commit -m "chore: scale auth-service to 3 replicas"
git push origin main
```

### Update Environment Variables

```bash
# Edit configmap generator
vim deployments/auth-service/kustomization.yaml

# Add new literal:
configMapGenerator:
- name: service-config
  behavior: merge
  literals:
  - SERVICE_NAME=auth-service
  - NEW_VAR=new_value  # Add this

# Commit and push
git add deployments/auth-service/kustomization.yaml
git commit -m "chore: add NEW_VAR to auth-service"
git push origin main
```

## Build Optimization

### Check What Will Build

Before pushing, check which services will be affected:

```bash
# Show changed files
git status

# If you changed shared/ → all services rebuild
# If you changed services/auth-service/ → only auth-service rebuilds
```

### Build Locally

Test builds before pushing:

```bash
# Build specific service
docker build \
  --build-arg SERVICE_NAME=auth-service \
  -f Dockerfile.service \
  -t auth-service:test \
  .

# Test the image
docker run -p 8080:8080 auth-service:test
```

## Debugging Failed Deployments

### Check GitHub Actions

```bash
# View recent workflows
gh run list

# View specific run
gh run view <run-id>

# View logs
gh run view <run-id> --log
```

### Check ArgoCD

```bash
# Get application details
argocd app get auth-service

# View events
kubectl get events --sort-by='.lastTimestamp' | grep auth

# Check pod logs
kubectl logs -l app=auth-service --tail=100
```

### Common Issues

**ImagePullBackOff**
- Check if image was pushed to ghcr.io
- Verify image name in kustomization.yaml
- Check if image is public or add pull secret

**CrashLoopBackOff**
- Check pod logs: `kubectl logs <pod-name>`
- Verify environment variables
- Check health check endpoints

**OutOfSync**
- Force sync: `argocd app sync auth-service --force`
- Check for manual kubectl changes (will be reverted)

## Release Strategy

### Development Branch

```bash
# Push to develop branch
git push origin develop

# Images tagged as: develop-<sha>
# ArgoCD can be configured to watch develop branch for staging
```

### Production Release

```bash
# Push to main branch
git push origin main

# Images tagged as: main-<sha> and latest
# ArgoCD auto-syncs to production
```

### Manual Approval (Optional)

Disable auto-sync for production:

```yaml
# In applicationset
syncPolicy:
  automated: null  # Disable auto-sync
```

Then manually sync:

```bash
argocd app sync auth-service
```

## Service Dependencies

When deploying, consider service dependencies:

```
graphics-gateway, text-gateway
        ↓
   world-state
        ↓
physics-service, script-executor
        ↓
persistence-service, asset-service
```

All services can be deployed in parallel as they're designed to be resilient.

## Performance Tips

### Reduce Build Time

1. **Only commit relevant files**

   ```bash
   git add services/auth-service/src/main.rs
   # Don't: git add .
   ```

2. **Use Docker build cache**
   - GitHub Actions caches layers automatically
   - Reorder Dockerfile for better caching

3. **Parallel builds**
   - Multiple services build in parallel
   - Check Actions → Build matrix

### Reduce Deployment Time

1. **Increase ArgoCD sync timeout**

   ```yaml
   syncPolicy:
     retry:
       limit: 5
       backoff:
         duration: 5s
   ```

2. **Optimize health checks**

   ```yaml
   readinessProbe:
     initialDelaySeconds: 5  # Reduce if app starts fast
   ```

## Getting Help

```bash
# ArgoCD help
argocd app --help

# Kubectl help
kubectl explain deployment.spec

# Kustomize help
kustomize build --help
```

## Summary

✅ **Push to main** → Auto-build → Auto-deploy
✅ **Only changed services** are built
✅ **ArgoCD auto-syncs** deployments
✅ **GitOps** - Git is source of truth
✅ **Rollback** anytime via ArgoCD
