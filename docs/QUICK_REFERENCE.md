# Quick Reference Card

Essential commands and workflows for Assignment W-4 development.

## Database Commands

```bash
# Start database
docker-compose -f docker-compose.dev.yml up postgres -d

# Create migration
cd services/persistence-service
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Rollback last migration
sqlx migrate revert

# Check migration status
sqlx migrate info

# Prepare for offline builds (CI/CD)
cargo sqlx prepare

# Verify queries compile
cargo sqlx prepare --check
```

## Git Workflow (Conventional Commits)

```bash
# Create feature branch
git checkout -b feat/feature-name

# Interactive commit (recommended)
cz commit

# OR manual commit following format:
git commit -m "type(scope): description"

# Types: feat, fix, docs, style, refactor, perf, test, build, ci, chore
# Scopes: auth-service, world-state, shared, deployment, etc.

# Examples:
git commit -m "feat(auth-service): add OAuth2 support"
git commit -m "fix(world-state): resolve entity duplication"
git commit -m "docs: update deployment guide"
git commit -m "feat(shared)!: change protocol (BREAKING CHANGE)"
```

## Version Management

```bash
# View current version
cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'

# Local version bump
cargo release patch --dry-run   # Preview
cargo release patch --execute   # 0.1.0 → 0.1.1
cargo release minor --execute   # 0.1.0 → 0.2.0
cargo release major --execute   # 0.1.0 → 1.0.0

# GitHub Actions (manual trigger)
# Go to: Actions → Release → Run workflow
# Choose: auto/patch/minor/major
```

## Development

```bash
# Check code compiles
cargo check --all-targets

# Format code
cargo fmt --all

# Lint code
cargo clippy --all-targets -- -D warnings

# Run tests
cargo test --all

# Run all pre-commit checks
pre-commit run --all-files

# Run specific service
cargo run -p world-state
cargo run -p auth-service
```

## Docker

```bash
# Build specific service
docker build \
  --build-arg SERVICE_NAME=auth-service \
  -f Dockerfile.service \
  -t auth-service:latest \
  .

# Start infrastructure
docker-compose -f docker-compose.dev.yml up redis postgres -d

# View logs
docker-compose -f docker-compose.dev.yml logs -f world-state
```

## ArgoCD (After Orange Pi Setup)

```bash
# List applications
argocd app list

# Sync application
argocd app sync auth-service

# Watch sync
argocd app wait auth-service

# View application details
argocd app get auth-service

# View sync history
argocd app history auth-service

# Rollback
argocd app rollback auth-service <revision>
```

## Kubernetes

```bash
# View pods
kubectl get pods

# View pods for specific service
kubectl get pods -l app=auth-service

# View logs
kubectl logs -l app=auth-service -f

# Describe pod
kubectl describe pod <pod-name>

# View services
kubectl get services

# View deployments
kubectl get deployments

# View events
kubectl get events --sort-by='.lastTimestamp'
```

## Conventional Commit Types

| Type | Description | Version Bump |
|------|-------------|--------------|
| `feat` | New feature | MINOR (0.1.0 → 0.2.0) |
| `fix` | Bug fix | PATCH (0.1.0 → 0.1.1) |
| `docs` | Documentation | PATCH |
| `style` | Formatting | PATCH |
| `refactor` | Code refactor | PATCH |
| `perf` | Performance | PATCH |
| `test` | Tests | PATCH |
| `build` | Build system | PATCH |
| `ci` | CI/CD | PATCH |
| `chore` | Maintenance | PATCH |
| `feat!` | Breaking change | MAJOR (0.1.0 → 1.0.0) |

## Common Scopes

- Service names: `auth-service`, `world-state`, `chat-service`, etc.
- Components: `client`, `shared`, `migrations`
- Infrastructure: `deployment`, `ci`, `docker`, `argocd`
- Cross-cutting: `deps`, `config`, `security`

## File Locations

```
assignment-W-4/
├── services/               # Microservices
│   ├── auth-service/
│   ├── world-state/
│   └── ...
├── deployments/           # Kubernetes manifests
│   ├── base/             # Shared templates
│   └── auth-service/     # Service-specific
├── argocd/               # ArgoCD applications
├── docs/                 # Documentation
├── .github/workflows/    # CI/CD
├── Cargo.toml           # Workspace config
├── release.toml         # Release config
├── .commitlintrc.yaml   # Commit validation
└── CHANGELOG.md         # Version history
```

## Environment Variables

```bash
# Database
export DATABASE_URL="postgres://dev:devpassword@localhost:5432/worldengine"

# Redis
export REDIS_URL="redis://localhost:6379"

# RabbitMQ
export RABBITMQ_URL="amqp://dev:devpassword@localhost:5672"

# Or use .env file
cat > .env <<EOF
DATABASE_URL=postgres://dev:devpassword@localhost:5432/worldengine
REDIS_URL=redis://localhost:6379
RABBITMQ_URL=amqp://dev:devpassword@localhost:5672
EOF
```

## Pre-commit Hooks

```bash
# Install hooks
pre-commit install
pre-commit install --hook-type commit-msg

# Run manually
pre-commit run --all-files

# Run specific hook
pre-commit run cargo-fmt

# Update hooks
pre-commit autoupdate

# Bypass hooks (emergency only)
git commit --no-verify
```

## Troubleshooting

### Database connection failed

```bash
# Check if PostgreSQL is running
docker-compose -f docker-compose.dev.yml ps

# View logs
docker-compose -f docker-compose.dev.yml logs postgres

# Restart
docker-compose -f docker-compose.dev.yml restart postgres
```

### Commit message rejected

```bash
# Use interactive helper
cz commit

# Check format:
# type(scope): description
# - Type must be in allowed list
# - Scope must be in allowed list
# - Description must be lowercase
# - No period at end
```

### SQLx prepare fails

```bash
# Ensure database is running and migrations are current
docker-compose -f docker-compose.dev.yml up postgres -d
sqlx migrate run

# Then retry
cargo sqlx prepare
```

### Pre-commit checks fail

```bash
# Fix formatting
cargo fmt --all

# Fix clippy warnings
cargo clippy --all-targets --fix

# Run tests
cargo test --all

# Retry
pre-commit run --all-files
```

## Documentation Index

- [DATABASE_STRATEGY.md](DATABASE_STRATEGY.md) - PostgreSQL + SQLx details
- [VERSIONING_STRATEGY.md](VERSIONING_STRATEGY.md) - SemVer + Conventional Commits
- [DATABASE_AND_VERSIONING_SETUP.md](DATABASE_AND_VERSIONING_SETUP.md) - Setup guide
- [ARGOCD_SETUP.md](ARGOCD_SETUP.md) - ArgoCD installation
- [DEPLOYMENT_WORKFLOW.md](DEPLOYMENT_WORKFLOW.md) - Daily operations
- [ORANGEPI_SETUP_CHECKLIST.md](ORANGEPI_SETUP_CHECKLIST.md) - Orange Pi setup

## Support

- Issues: [GitHub Issues](https://github.com/CHANGEME/assignment-W-4/issues)
- Discussions: [GitHub Discussions](https://github.com/CHANGEME/assignment-W-4/discussions)
- Documentation: [docs/](docs/)
