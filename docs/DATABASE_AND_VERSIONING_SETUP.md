# Database and Versioning Setup Guide

Quick start guide for setting up database migrations and versioning workflows.

## Prerequisites

```bash
# Rust tooling
cargo install sqlx-cli --no-default-features --features postgres
cargo install cargo-release

# Python tooling (optional but recommended for commit helpers)
pip install pre-commit commitizen

# Node.js tooling (optional)
npm install -g @commitlint/cli @commitlint/config-conventional
```

## Database Setup

### 1. Start PostgreSQL

```bash
# Using Docker Compose
docker-compose -f docker-compose.dev.yml up postgres -d

# Verify it's running
docker-compose -f docker-compose.dev.yml ps
```

### 2. Set Environment Variable

```bash
# Add to ~/.bashrc or ~/.zshrc
export DATABASE_URL="postgres://dev:devpassword@localhost:5432/worldengine"

# Or create .env file in project root
echo 'DATABASE_URL=postgres://dev:devpassword@localhost:5432/worldengine' > .env
```

### 3. Create First Migration

```bash
# For persistence-service (owns the schema)
cd services/persistence-service

# Create migration directory
mkdir -p migrations

# Create initial schema migration
sqlx migrate add -r initial_schema

# This creates:
# migrations/TIMESTAMP_initial_schema.up.sql
# migrations/TIMESTAMP_initial_schema.down.sql
```

### 4. Write Migration

Edit `migrations/TIMESTAMP_initial_schema.up.sql`:

```sql
-- Create worlds table
CREATE TABLE IF NOT EXISTS worlds (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create world_objects table
CREATE TABLE IF NOT EXISTS world_objects (
    id BIGSERIAL PRIMARY KEY,
    world_id BIGINT NOT NULL REFERENCES worlds(id) ON DELETE CASCADE,
    object_type VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    position JSONB NOT NULL,
    properties JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_world_objects_world_id ON world_objects(world_id);
```

Edit `migrations/TIMESTAMP_initial_schema.down.sql`:

```sql
DROP TABLE IF EXISTS world_objects;
DROP TABLE IF EXISTS worlds;
```

### 5. Run Migration

```bash
# Run all pending migrations
sqlx migrate run

# Check migration status
sqlx migrate info

# Rollback last migration
sqlx migrate revert
```

### 6. Add SQLx to Service

Edit `services/persistence-service/Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "migrate", "macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 7. Use in Code

Create `services/persistence-service/src/main.rs`:

```rust
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Connect to database
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations on startup
    println!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    println!("Migrations complete!");

    // Your service logic here...

    Ok(())
}
```

### 8. Prepare for Offline Builds

```bash
# Generate query metadata for CI/CD
cargo sqlx prepare

# This creates .sqlx/ directory
# Commit this to Git
git add .sqlx
git commit -m "chore(persistence-service): add SQLx query metadata"
```

## Versioning Setup

### 1. Install Pre-commit Hook

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install commit-msg hook
pre-commit install --hook-type commit-msg

# Verify installation
pre-commit run --all-files
```

### 2. Test Conventional Commits

```bash
# Good commit messages (will pass)
git commit -m "feat(auth-service): add OAuth2 authentication"
git commit -m "fix(world-state): resolve entity duplication bug"
git commit -m "docs: update deployment guide"

# Bad commit messages (will fail)
git commit -m "updated stuff"  # ❌ No type
git commit -m "feat: Add Feature"  # ❌ Subject not lowercase
git commit -m "fix(random): something"  # ❌ Invalid scope
```

### 3. Using Commitizen (Interactive)

```bash
# Install
pip install commitizen

# Use interactive commit
cz commit

# Or shorter
cz c

# Follow the prompts:
# 1. Select type (feat, fix, etc.)
# 2. Select scope (auth-service, etc.)
# 3. Write description
# 4. Write body (optional)
# 5. Breaking changes? (optional)
```

### 4. Manual Version Bump

```bash
# Install cargo-release
cargo install cargo-release

# Dry run to see what would happen
cargo release patch --dry-run

# Actually bump version (patch: 0.1.0 → 0.1.1)
cargo release patch --execute

# Or bump minor (0.1.0 → 0.2.0)
cargo release minor --execute

# Or bump major (0.1.0 → 1.0.0)
cargo release major --execute
```

### 5. Automated Releases (GitHub Actions)

Trigger a release manually via GitHub:
1. Go to Actions → Release workflow
2. Click "Run workflow"
3. Choose version bump type or "auto" to detect from commits
4. Optionally enable dry run to preview

The workflow will:
1. Analyze commits since last release (if auto)
2. Determine version bump (MAJOR/MINOR/PATCH)
3. Update Cargo.toml versions
4. Update CHANGELOG.md
5. Create Git tag
6. Create GitHub release

## Workflow Example

### Scenario: Add New Feature

```bash
# 1. Create feature branch
git checkout -b feat/add-inventory-system

# 2. Create database migration
cd services/persistence-service
sqlx migrate add add_inventory_tables

# 3. Write migration
vim migrations/TIMESTAMP_add_inventory_tables.up.sql

# 4. Run migration
sqlx migrate run

# 5. Update code
vim src/inventory.rs

# 6. Prepare SQLx metadata
cargo sqlx prepare

# 7. Commit with conventional commit
git add .
git commit -m "feat(persistence-service): add inventory system

Added inventory tables and queries for item management.

- Created items table
- Created player_inventories table
- Added CRUD operations for inventory"

# 8. Push and create PR
git push origin feat/add-inventory-system
gh pr create

# 9. Merge to main
# GitHub Actions will:
#   - Bump MINOR version (0.1.0 → 0.2.0)
#   - Update CHANGELOG.md
#   - Create release tag
#   - Build and deploy
```

## Common Commands

### Database

```bash
# Create migration
sqlx migrate add <name>

# Run migrations
sqlx migrate run

# Rollback
sqlx migrate revert

# Check status
sqlx migrate info

# Prepare metadata
cargo sqlx prepare

# Check queries (requires running DB)
cargo sqlx prepare --check
```

### Versioning

```bash
# Interactive commit
cz commit

# Manual version bump (local)
cargo release patch --execute  # 0.1.0 → 0.1.1
cargo release minor --execute  # 0.1.0 → 0.2.0
cargo release major --execute  # 0.1.0 → 1.0.0

# Dry run (see what would happen)
cargo release patch --dry-run

# View current version
grep '^version = ' Cargo.toml

# Or use cargo metadata
cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'
```

### Pre-commit

```bash
# Run all hooks manually
pre-commit run --all-files

# Run specific hook
pre-commit run cargo-fmt

# Update hook versions
pre-commit autoupdate

# Bypass hooks (use sparingly)
git commit --no-verify
```

## Troubleshooting

### SQLx Prepare Fails

```bash
# Ensure database is running
docker-compose -f docker-compose.dev.yml up postgres -d

# Ensure migrations are run
sqlx migrate run

# Try prepare again
cargo sqlx prepare
```

### Commit Message Rejected

```bash
# Check the error message
# Common issues:
# - Type not in allowed list (feat, fix, docs, etc.)
# - Scope not in allowed list (see .commitlintrc.yaml)
# - Subject not lowercase
# - Subject ends with period
# - Line too long (>100 characters)

# Use commitizen to avoid mistakes
cz commit
```

### Version Not Bumping

```bash
# Ensure commits follow Conventional Commits
git log --oneline

# Manually trigger release workflow
gh workflow run release.yaml
```

## Best Practices

### Database

✅ **DO:**
- Run migrations on service startup
- Use reversible migrations (up/down)
- Test migrations on copy of production data
- Use transactions (SQLx does this automatically)
- Commit .sqlx metadata to Git

❌ **DON'T:**
- Modify existing migrations after deployment
- Run migrations manually in production
- Store large files in PostgreSQL
- Use `SELECT *` in queries

### Versioning

✅ **DO:**
- Use Conventional Commits format
- Write descriptive commit messages
- Break large changes into smaller commits
- Use commitizen for interactive help
- Let automation handle version bumps

❌ **DON'T:**
- Use vague commit messages ("fix stuff")
- Mix multiple changes in one commit
- Manually edit version numbers
- Skip pre-commit hooks without reason
- Force push to main

## References

- [SQLx Documentation](https://docs.rs/sqlx/)
- [Semantic Versioning 2.0](https://semver.org/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Commitizen](https://commitizen-tools.github.io/commitizen/)
- [release-plz](https://release-plz.ieni.dev/)

See also:
- [DATABASE_STRATEGY.md](DATABASE_STRATEGY.md) - Detailed database architecture
- [VERSIONING_STRATEGY.md](VERSIONING_STRATEGY.md) - Detailed versioning approach
