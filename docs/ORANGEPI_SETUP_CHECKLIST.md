# Orange Pi Setup Checklist

Checklist for setting up the Kubernetes worker node and ArgoCD when your Orange Pi comes back online.

## Prerequisites Setup

### 1. Proxmox Worker Node

- [ ] Orange Pi powered on and accessible
- [ ] Network configured (static IP recommended)
- [ ] SSH access configured
- [ ] Joined to existing Kubernetes cluster as worker node
- [ ] Verify node status: `kubectl get nodes`

### 2. GitHub Runner (Optional)

If you want to run GitHub Actions on your own hardware:

```bash
# On the worker node
mkdir actions-runner && cd actions-runner
curl -o actions-runner-linux-arm64-2.311.0.tar.gz -L https://github.com/actions/runner/releases/download/v2.311.0/actions-runner-linux-arm64-2.311.0.tar.gz
tar xzf ./actions-runner-linux-arm64-2.311.0.tar.gz

# Configure
./config.sh --url https://github.com/YOUR_USERNAME/assignment-W-4 --token YOUR_TOKEN

# Install as service
sudo ./svc.sh install
sudo ./svc.sh start
```

- [ ] GitHub runner installed
- [ ] Runner configured for your repository
- [ ] Runner showing as online in GitHub
- [ ] Update workflow to use self-hosted runner:

  ```yaml
  runs-on: self-hosted  # Instead of ubuntu-latest
  ```

## ArgoCD Installation

### 1. Install ArgoCD

```bash
# Create namespace
kubectl create namespace argocd

# Install ArgoCD
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Wait for ready
kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=argocd-server -n argocd --timeout=300s
```

- [ ] ArgoCD namespace created
- [ ] ArgoCD installed
- [ ] All ArgoCD pods running: `kubectl get pods -n argocd`

### 2. Access ArgoCD

```bash
# Get initial password
kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d; echo

# Port forward (or set up Ingress)
kubectl port-forward svc/argocd-server -n argocd 8080:443
```

- [ ] Initial password retrieved
- [ ] Can access ArgoCD UI at localhost:8080
- [ ] Logged in successfully (admin / password)
- [ ] Changed default password

### 3. Install ArgoCD CLI

```bash
# Linux ARM64
curl -sSL -o argocd-linux-arm64 https://github.com/argoproj/argo-cd/releases/latest/download/argocd-linux-arm64
sudo install -m 555 argocd-linux-arm64 /usr/local/bin/argocd
rm argocd-linux-arm64

# Login
argocd login localhost:8080
```

- [ ] ArgoCD CLI installed
- [ ] Can execute `argocd version`
- [ ] Logged in via CLI

## Repository Configuration

### 1. Update Repository URLs

```bash
# In your local git repo
cd /home/pakishi/Projects/rust/assignment-W-4

# Update ArgoCD manifests
sed -i 's|https://github.com/CHANGEME/assignment-W-4.git|https://github.com/YOUR_USERNAME/assignment-W-4.git|g' argocd/applicationsets/microservices.yaml
sed -i 's|https://github.com/CHANGEME/assignment-W-4.git|https://github.com/YOUR_USERNAME/assignment-W-4.git|g' argocd/applications/client.yaml

# Update image registry
find deployments -name "kustomization.yaml" -exec sed -i 's|ghcr.io/CHANGEME|ghcr.io/YOUR_USERNAME|g' {} \;

# Commit changes
git add argocd/ deployments/
git commit -m "chore: update repository and registry URLs"
git push origin main
```

- [ ] Repository URL updated in ApplicationSet
- [ ] Image registry updated in all Kustomize files
- [ ] Changes committed and pushed

### 2. GitHub Container Registry

```bash
# Make packages public (or configure pull secrets)
# Go to: https://github.com/YOUR_USERNAME?tab=packages
# For each package → Package settings → Change visibility to Public
```

- [ ] GitHub Actions has write permissions enabled
- [ ] Container registry packages are public (or pull secret configured)

## Deploy Applications

### 1. Apply ApplicationSet

```bash
# Apply microservices ApplicationSet
kubectl apply -f argocd/applicationsets/microservices.yaml

# Verify applications created
kubectl get applications -n argocd
```

- [ ] ApplicationSet applied
- [ ] 9 applications created (one per service)
- [ ] All applications showing in ArgoCD UI

### 2. Initial Sync

Since images don't exist yet, applications will be OutOfSync. This is expected.

```bash
# Check status
argocd app list

# Expected: All apps showing "OutOfSync" (waiting for images)
```

- [ ] Applications created but OutOfSync (expected)

## First Deployment

### 1. Trigger Initial Build

```bash
# Make a small change to trigger builds
echo "# ArgoCD configured" >> README.md
git add README.md
git commit -m "chore: trigger initial deployment"
git push origin main
```

- [ ] Commit pushed to main
- [ ] GitHub Actions workflow started
- [ ] Can view workflow: `gh run watch` or GitHub web UI

### 2. Wait for Builds

```bash
# Watch GitHub Actions
gh run list

# This will take ~10-15 minutes for first build
# Subsequent builds are faster (cached)
```

- [ ] All service builds completed successfully
- [ ] Docker images pushed to ghcr.io
- [ ] Kustomize manifests updated with image tags

### 3. Verify ArgoCD Sync

```bash
# Applications should auto-sync
argocd app list

# Wait for sync
argocd app wait auth-service
argocd app wait world-state
# ... etc
```

- [ ] Applications synced automatically
- [ ] All applications showing "Healthy"
- [ ] Pods running: `kubectl get pods`

## Verify Deployment

### 1. Check Pods

```bash
# All pods should be running
kubectl get pods

# Check specific service
kubectl get pods -l app=auth-service
kubectl logs -l app=auth-service
```

- [ ] All service pods in Running state
- [ ] No CrashLoopBackOff or ImagePullBackOff
- [ ] Logs show services started correctly

### 2. Check Services

```bash
# List services
kubectl get services

# Test connectivity (from within cluster)
kubectl run curl --image=curlimages/curl -i --tty --rm -- sh
# Inside pod:
curl http://auth-service:8080/health
```

- [ ] All Kubernetes services created
- [ ] Services accessible within cluster
- [ ] Health endpoints responding

### 3. ArgoCD Health

```bash
# Check application health
argocd app get auth-service
argocd app get world-state

# All should show:
# - Sync Status: Synced
# - Health Status: Healthy
```

- [ ] All applications Synced
- [ ] All applications Healthy
- [ ] ArgoCD UI shows green status

## Test Deployment Workflow

### 1. Make a Change

```bash
# Edit a service
echo "// test change" >> services/auth-service/src/main.rs

# Commit and push
git add services/auth-service
git commit -m "test: verify deployment pipeline"
git push origin main
```

- [ ] Change committed
- [ ] GitHub Actions triggered
- [ ] Only auth-service built (not all services)

### 2. Verify Selective Build

```bash
# Check GitHub Actions
gh run view

# Should only build auth-service
```

- [ ] Only auth-service built
- [ ] Other services skipped
- [ ] Build completed successfully

### 3. Verify Auto-Deploy

```bash
# Watch ArgoCD sync
argocd app wait auth-service

# Check pods
kubectl get pods -l app=auth-service -w
```

- [ ] ArgoCD detected change
- [ ] auth-service synced automatically
- [ ] New pod rolled out
- [ ] Old pod terminated
- [ ] Service remained available during update

## Optional: Set Up Ingress

For external access to services:

```bash
# Install NGINX Ingress Controller
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/controller-v1.8.2/deploy/static/provider/cloud/deploy.yaml

# Create Ingress for ArgoCD
kubectl apply -f - <<EOF
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: argocd-server-ingress
  namespace: argocd
  annotations:
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  ingressClassName: nginx
  rules:
  - host: argocd.local  # Change to your domain
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: argocd-server
            port:
              number: 80
EOF
```

- [ ] Ingress controller installed
- [ ] Ingress configured
- [ ] Can access ArgoCD via domain name

## Monitoring Setup (Future)

- [ ] Install Prometheus
- [ ] Install Grafana
- [ ] Configure dashboards for microservices
- [ ] Set up alerts

## Troubleshooting

If something goes wrong:

```bash
# Check ArgoCD logs
kubectl logs -n argocd -l app.kubernetes.io/name=argocd-application-controller

# Check pod events
kubectl get events --sort-by='.lastTimestamp'

# Describe pod
kubectl describe pod <pod-name>

# Force sync
argocd app sync auth-service --force --prune
```

## Rollback Plan

If deployment fails:

```bash
# Rollback via ArgoCD
argocd app history auth-service
argocd app rollback auth-service <revision>

# Or delete and recreate
kubectl delete applicationset microservices -n argocd
kubectl apply -f argocd/applicationsets/microservices.yaml
```

## Success Criteria

✅ All services deployed and healthy
✅ GitOps workflow functioning (push → build → deploy)
✅ Only changed services rebuild
✅ ArgoCD auto-syncs deployments
✅ Can rollback if needed
✅ Monitoring (optional) configured

## Next Steps After Setup

1. Configure persistent storage for services that need it
2. Set up secrets management (Sealed Secrets or External Secrets Operator)
3. Configure resource quotas and limits
4. Set up network policies
5. Configure backup strategy
6. Document disaster recovery procedures

## Notes

- First deployment will be slow (no cache)
- Subsequent deployments much faster
- `shared` changes trigger rebuild of all services
- Consider setting up a staging environment
