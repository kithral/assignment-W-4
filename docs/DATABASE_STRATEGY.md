# Database Strategy

This document defines the database architecture, migration strategy, and best practices for Assignment W-4.

## Architecture Overview

### Database Layout

```
PostgreSQL (Primary)
├── worldengine (main database)
│   ├── public schema
│   │   ├── Persistent world data
│   │   ├── User accounts
│   │   └── Asset metadata
│   └── _sqlx_migrations (migration tracking)
│
Redis (Cache & Pub/Sub)
├── Session data (TTL: 24h)
├── Active world state cache
└── Pub/Sub channels for events
```

### Database Usage by Service

| Service | PostgreSQL | Redis | RabbitMQ |
|---------|-----------|-------|----------|
| world-state | ✅ Read/Write | ✅ Cache | ❌ |
| auth-service | ✅ Read/Write | ✅ Sessions | ❌ |
| persistence-service | ✅ Read/Write | ❌ | ❌ |
| chat-service | ❌ | ✅ Pub/Sub | ❌ |
| script-executor | ❌ | ❌ | ✅ Queue |
| asset-service | ✅ Metadata only | ❌ | ❌ |

## Migration Strategy: SQLx

**Decision:** Use [SQLx](https://github.com/launchbadge/sqlx) for migrations and database access.

### Why SQLx?

✅ **Compile-time verified SQL** - Queries checked at build time
✅ **Built-in migrations** - No external tools needed
✅ **Async/await native** - Perfect for Tokio
✅ **Type-safe** - Rust structs from SQL
✅ **Database agnostic** - Supports Postgres, MySQL, SQLite
✅ **Active maintenance** - Industry standard in Rust

### Alternatives Considered

❌ **Diesel** - More mature but sync-only, doesn't fit async microservices
❌ **SeaORM** - Good but adds complexity, SQLx is simpler
❌ **Flyway/Liquibase** - Java-based, not Rust-native
❌ **Raw SQL** - No type safety, manual migration tracking

## Implementation Guide

### 1. Add SQLx to Services

```toml
# services/persistence-service/Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "migrate", "macros"] }
tokio = { version = "1", features = ["full"] }
```

### 2. Directory Structure

```
services/
└── persistence-service/
    ├── migrations/
    │   ├── 20240101000000_initial_schema.sql
    │   ├── 20240102000000_add_users_table.sql
    │   └── 20240103000000_add_world_objects.sql
    ├── src/
    │   ├── main.rs
    │   ├── models.rs  # Rust structs matching DB schema
    │   └── queries.rs # SQL queries (compile-time verified)
    └── Cargo.toml

# Shared migrations (alternative approach)
migrations/
├── core/               # Shared schema
│   └── 20240101000000_initial_schema.sql
├── auth/               # Auth service schema
│   └── 20240102000000_users_and_sessions.sql
└── world/              # World state schema
    └── 20240103000000_world_objects.sql
```

### 3. Migration File Format

```sql
-- migrations/20240101000000_initial_schema.sql
-- Create initial schema for world engine

CREATE TABLE IF NOT EXISTS worlds (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS world_objects (
    id BIGSERIAL PRIMARY KEY,
    world_id BIGINT NOT NULL REFERENCES worlds(id) ON DELETE CASCADE,
    object_type VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    position JSONB NOT NULL,  -- {x, y, z}
    properties JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_world_objects_world_id ON world_objects(world_id);
CREATE INDEX idx_world_objects_position ON world_objects USING GIN(position);

-- Trigger for updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_worlds_updated_at BEFORE UPDATE ON worlds
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_world_objects_updated_at BEFORE UPDATE ON world_objects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

### 4. Migration Naming Convention

**Format:** `YYYYMMDDHHMMSS_description.sql`

Examples:
- `20240126120000_initial_schema.sql`
- `20240126130000_add_user_authentication.sql`
- `20240126140000_add_inventory_system.sql`

**Rules:**
- Use timestamps (not sequential numbers) to avoid conflicts
- Use descriptive snake_case names
- One logical change per migration
- Never modify existing migrations after deployment

### 5. Running Migrations

**Development:**

```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Create new migration
sqlx migrate add -r initial_schema

# Run migrations
sqlx migrate run --database-url "postgres://dev:devpassword@localhost:5432/worldengine"

# Rollback last migration
sqlx migrate revert --database-url "postgres://dev:devpassword@localhost:5432/worldengine"
```

**In Application:**

```rust
// src/main.rs
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    // Run migrations on startup
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // Your application logic...
    Ok(())
}
```

### 6. Type-Safe Queries

```rust
// src/models.rs
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct World {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// src/queries.rs
use sqlx::PgPool;

pub async fn get_world_by_name(pool: &PgPool, name: &str) -> Result<World, sqlx::Error> {
    sqlx::query_as!(
        World,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM worlds
        WHERE name = $1
        "#,
        name
    )
    .fetch_one(pool)
    .await
}

pub async fn create_world(
    pool: &PgPool,
    name: &str,
    description: Option<&str>,
) -> Result<World, sqlx::Error> {
    sqlx::query_as!(
        World,
        r#"
        INSERT INTO worlds (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at, updated_at
        "#,
        name,
        description
    )
    .fetch_one(pool)
    .await
}
```

**Compile-time verification:**

```bash
# Prepare for offline builds (stores query metadata)
cargo sqlx prepare --database-url "postgres://dev:devpassword@localhost:5432/worldengine"

# This creates .sqlx/ directory with query metadata
# Commit this to Git for CI/CD
```

## Migration Workflow

### Development Flow

```
1. Developer creates migration
   ↓
2. Run `sqlx migrate run` locally
   ↓
3. Test queries with `cargo check`
   ↓
4. Run `cargo sqlx prepare`
   ↓
5. Commit migration + .sqlx metadata
   ↓
6. Push to Git
   ↓
7. CI/CD builds (uses offline mode)
   ↓
8. Deployment runs migration on startup
```

### Migration Best Practices

✅ **DO:**
- Use transactions (SQLx does this automatically)
- Write reversible migrations when possible
- Test migrations on copy of production data
- Use `IF NOT EXISTS` for idempotent migrations
- Include indexes in initial table creation
- Document complex migrations with comments

❌ **DON'T:**
- Modify existing migrations after deployment
- Use `SELECT *` in application code
- Store large blobs in Postgres (use object storage)
- Run migrations manually in production (automate)
- Skip rollback migrations (always include `down.sql`)

### Handling Breaking Changes

**Strategy 1: Expand-Contract Pattern**

```sql
-- Step 1: Add new column (backward compatible)
ALTER TABLE users ADD COLUMN email_verified BOOLEAN DEFAULT false;

-- Deploy new code that writes to both columns
-- Wait for all instances to update

-- Step 2: Backfill data
UPDATE users SET email_verified = true WHERE verified_at IS NOT NULL;

-- Step 3: Remove old column (new migration, later)
ALTER TABLE users DROP COLUMN verified_at;
```

**Strategy 2: Feature Flags**

```rust
if feature_flags.new_auth_system {
    // Use new schema
} else {
    // Use old schema
}
```

## Redis Usage

### Data Structures

```rust
// Session data (Hash)
// Key: session:{session_id}
// TTL: 24 hours
HSET session:abc123 user_id 42 created_at 1234567890

// Active players (Set)
// Key: active_players:{world_id}
SADD active_players:1 player42 player43

// World state cache (String - JSON)
// Key: world_state:{world_id}
// TTL: 5 minutes
SET world_state:1 '{"entities": [...], "version": 123}'

// Pub/Sub channels
PUBLISH world:1:events '{"type": "player_joined", "player_id": 42}'
```

### Redis Migration Strategy

**No migrations needed** - Redis is ephemeral cache:
- Data has TTL (auto-expires)
- Application code defines structure
- Breaking changes = flush cache

```bash
# If schema changes, just clear cache
redis-cli FLUSHDB
```

## Hybrid Approach: PostgreSQL + Redis

### Write Pattern

```rust
async fn update_world_object(
    pool: &PgPool,
    redis: &RedisPool,
    object_id: i64,
    new_position: Position,
) -> Result<()> {
    // 1. Update PostgreSQL (source of truth)
    sqlx::query!(
        "UPDATE world_objects SET position = $1 WHERE id = $2",
        serde_json::to_value(&new_position)?,
        object_id
    )
    .execute(pool)
    .await?;

    // 2. Invalidate Redis cache
    redis.del(format!("object:{}", object_id)).await?;

    // 3. Publish event
    redis.publish("world:events", serde_json::to_string(&UpdateEvent {
        object_id,
        position: new_position,
    })?).await?;

    Ok(())
}
```

### Read Pattern (Cache-Aside)

```rust
async fn get_world_object(
    pool: &PgPool,
    redis: &RedisPool,
    object_id: i64,
) -> Result<WorldObject> {
    // 1. Try Redis cache first
    if let Some(cached) = redis.get(format!("object:{}", object_id)).await? {
        return Ok(serde_json::from_str(&cached)?);
    }

    // 2. Cache miss - read from PostgreSQL
    let object = sqlx::query_as!(
        WorldObject,
        "SELECT * FROM world_objects WHERE id = $1",
        object_id
    )
    .fetch_one(pool)
    .await?;

    // 3. Store in cache (TTL: 5 minutes)
    redis.setex(
        format!("object:{}", object_id),
        300,
        serde_json::to_string(&object)?
    ).await?;

    Ok(object)
}
```

## Testing Strategy

### Local Testing

```bash
# Start test database
docker-compose -f docker-compose.dev.yml up postgres -d

# Run migrations
sqlx migrate run

# Run tests
cargo test
```

### Integration Tests

```rust
#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_create_world(pool: PgPool) {
        // Migrations run automatically
        let world = create_world(&pool, "TestWorld", None).await.unwrap();
        assert_eq!(world.name, "TestWorld");
    }
}
```

## Production Deployment

### Migration on Startup

```rust
// Kubernetes Init Container
// OR
// Application startup (recommended)
sqlx::migrate!("./migrations").run(&pool).await?;
```

### Zero-Downtime Deployments

1. **Always backward compatible migrations**
2. **Deploy in phases:**
   - Phase 1: Deploy migration (expand schema)
   - Phase 2: Deploy new code
   - Phase 3: Deploy cleanup migration (contract schema)

### Backup Strategy

```bash
# Automated daily backups in Kubernetes CronJob
pg_dump -Fc worldengine > backup_$(date +%Y%m%d).dump

# Restore
pg_restore -d worldengine backup_20240126.dump
```

## Monitoring

```sql
-- Check migration status
SELECT * FROM _sqlx_migrations ORDER BY version DESC;

-- Check table sizes
SELECT
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename))
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;
```

## Summary

✅ **PostgreSQL** for persistent data (SQLx migrations)
✅ **Redis** for cache and pub/sub (no migrations)
✅ **Compile-time SQL verification** with SQLx
✅ **Automated migrations** on startup
✅ **Type-safe queries** with Rust structs
✅ **Git-based migration history**

Next: See [VERSIONING_STRATEGY.md](VERSIONING_STRATEGY.md) for semantic versioning.
