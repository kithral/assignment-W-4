# ArgoCD Setup Guide

This guide covers setting up ArgoCD on your Proxmox Kubernetes cluster for GitOps-based deployments.

## Prerequisites

- Kubernetes cluster running on Proxmox
- kubectl configured to access your cluster
- Helm 3 installed (optional, for ArgoCD installation)
- GitHub repository with read access

## Architecture Overview

```
GitHub Repository (this repo)
    ↓
ArgoCD (watches for changes)
    ↓
Kubernetes Cluster
    ↓
Microservices (auto-deployed)
```

## Installation

### Option 1: Using kubectl

```bash
# Create ArgoCD namespace
kubectl create namespace argocd

# Install ArgoCD
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Wait for ArgoCD to be ready
kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=argocd-server -n argocd --timeout=300s
```

### Option 2: Using Helm

```bash
# Add ArgoCD Helm repository
helm repo add argo https://argoproj.github.io/argo-helm
helm repo update

# Install ArgoCD
helm install argocd argo/argo-cd \
  --namespace argocd \
  --create-namespace \
  --set server.service.type=LoadBalancer
```

## Access ArgoCD UI

### Get the initial admin password

```bash
# Get the password
kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d; echo
```

### Port forward to access UI

```bash
# Forward ArgoCD server to localhost
kubectl port-forward svc/argocd-server -n argocd 8080:443

# Access UI at: https://localhost:8080
# Username: admin
# Password: (from command above)
```

### Change admin password

```bash
# Login with ArgoCD CLI
argocd login localhost:8080

# Update password
argocd account update-password
```

## Install ArgoCD CLI (Optional but Recommended)

```bash
# Linux
curl -sSL -o argocd-linux-amd64 https://github.com/argoproj/argo-cd/releases/latest/download/argocd-linux-amd64
sudo install -m 555 argocd-linux-amd64 /usr/local/bin/argocd
rm argocd-linux-amd64

# Verify installation
argocd version
```

## Deploy Applications

### 1. Update Repository URLs

Before deploying, update the repository URLs in the ArgoCD manifests:

```bash
# Edit ApplicationSet
nano argocd/applicationsets/microservices.yaml

# Change:
# repoURL: https://github.com/CHANGEME/assignment-W-4.git
# To your actual repository URL

# Also update image registry in Kustomize files
find deployments -name "kustomization.yaml" -exec sed -i 's/ghcr.io\/CHANGEME/ghcr.io\/YOUR_USERNAME/g' {} \;
```

### 2. Apply ApplicationSet

```bash
# Apply the microservices ApplicationSet
kubectl apply -f argocd/applicationsets/microservices.yaml

# Verify applications are created
kubectl get applications -n argocd
```

### 3. Sync Applications

```bash
# Sync all applications
argocd app sync -l app.kubernetes.io/instance=microservices

# Or sync individual services
argocd app sync auth-service
argocd app sync world-state
```

## Verify Deployments

```bash
# Check ArgoCD applications
kubectl get applications -n argocd

# Check deployments in default namespace
kubectl get deployments

# Check pods
kubectl get pods

# Check services
kubectl get services
```

## Configure GitHub Actions

### 1. Enable GitHub Container Registry

Your GitHub Actions workflow will automatically push images to `ghcr.io`. Ensure:

1. In your GitHub repository settings → Actions → General
2. Workflow permissions → Select "Read and write permissions"
3. Save

### 2. Update Workflow Variables

Edit [.github/workflows/build-deploy.yaml](../.github/workflows/build-deploy.yaml):

```yaml
env:
  REGISTRY: ghcr.io
  IMAGE_PREFIX: ghcr.io/YOUR_GITHUB_USERNAME  # Update this
```

### 3. Test the Pipeline

```bash
# Make a change to a service
echo "// test" >> services/auth-service/src/main.rs

# Commit and push
git add .
git commit -m "test: trigger CI/CD pipeline"
git push origin main

# Watch the GitHub Actions workflow
# Watch ArgoCD sync the changes
argocd app wait auth-service
```

## ArgoCD Configuration

### Enable Auto-Sync

All applications are configured with auto-sync enabled:

```yaml
syncPolicy:
  automated:
    prune: true      # Delete resources when removed from Git
    selfHeal: true   # Revert manual changes
```

### View Sync Status

```bash
# CLI
argocd app list

# UI
# Navigate to https://localhost:8080 (or your ArgoCD URL)
```

## Monitoring and Troubleshooting

### Check Application Health

```bash
# Get detailed app info
argocd app get auth-service

# View sync history
argocd app history auth-service

# View application logs
argocd app logs auth-service
```

### Manual Sync

```bash
# Force sync
argocd app sync auth-service --force

# Sync with prune
argocd app sync auth-service --prune
```

### Rollback

```bash
# List revisions
argocd app history auth-service

# Rollback to specific revision
argocd app rollback auth-service <revision-id>
```

## Service Architecture

The ApplicationSet deploys these services:

- `auth-service` - Authentication and authorization
- `chat-service` - Chat functionality
- `world-state` - Game world state management (2 replicas)
- `physics-service` - Physics simulation
- `script-executor` - Script execution
- `asset-service` - Asset management
- `persistence-service` - Data persistence
- `graphics-gateway` - Graphics protocol gateway
- `text-gateway` - Text protocol gateway

## Networking

Each service exposes:
- Port 8080 - HTTP API
- Port 50051 - gRPC

Access services within cluster:

```
http://auth-service:8080
http://world-state-service:8080
```

## Health Checks

All services include:
- Liveness probe: `/health` endpoint
- Readiness probe: `/ready` endpoint

## Resource Limits

Default limits (adjust in `deployments/<service>/kustomization.yaml`):
- Requests: 128Mi RAM, 100m CPU
- Limits: 512Mi RAM, 500m CPU

World State service has higher limits (2 replicas, 1Gi RAM).

## Next Steps

1. Set up Ingress for external access
2. Configure TLS certificates
3. Set up monitoring (Prometheus + Grafana)
4. Configure alerts
5. Set up logging aggregation

## Troubleshooting

### Applications stuck in "Progressing"

```bash
# Check pod status
kubectl get pods -l app=auth-service

# Check events
kubectl describe pod <pod-name>
```

### Image pull errors

Ensure GitHub Container Registry is public or add image pull secret:

```bash
# Create secret
kubectl create secret docker-registry ghcr-secret \
  --docker-server=ghcr.io \
  --docker-username=YOUR_USERNAME \
  --docker-password=YOUR_PAT

# Update deployment to use secret
# Add to deployments/base/deployment.yaml:
# imagePullSecrets:
# - name: ghcr-secret
```

### Service not syncing

```bash
# Check ArgoCD logs
kubectl logs -n argocd -l app.kubernetes.io/name=argocd-application-controller

# Force refresh
argocd app get auth-service --refresh
```

## Cleanup

```bash
# Delete all applications
kubectl delete applicationset microservices -n argocd

# Uninstall ArgoCD
kubectl delete -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Delete namespace
kubectl delete namespace argocd
```

## Additional Resources

- [ArgoCD Documentation](https://argo-cd.readthedocs.io/)
- [Kustomize Documentation](https://kubectl.docs.kubernetes.io/references/kustomize/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
