# Architecture Overview

## Project Structure

This project follows a **workspace monorepo** pattern with three main crates:

### 📦 Shared (`shared/`)

**Purpose:** The single source of truth for game logic, protocol, and data structures.

- **Components:** ECS components that exist on both client and server
- **Protocol:** Network message definitions (serialized with bincode)
- **Systems:** Deterministic game logic that runs identically on both sides
- **Physics:** Constants and utilities for physics simulation

**Learning Note:** By sharing code between client and server, we ensure consistency and avoid duplication. This is a common pattern in Rust game development.

### 🖥️ Server (`server/`)

**Purpose:** Authoritative game server (headless, runs on Linux).

- **Authority:** The server has final say on all game state
- **Validation:** Receives client input, validates it, processes it
- **Broadcasting:** Sends authoritative state updates to all clients
- **Persistence:** (Future) Handle saving/loading game state

**Learning Note:** The server is "headless" (no graphics), making it perfect for running in containers on remote hardware like your Orange Pi.

### 🎮 Client (`client/`)

**Purpose:** Visual frontend with rendering and client-side prediction.

- **Rendering:** Displays the game world using Bevy's rendering engine
- **Input:** Captures player input and sends to server
- **Prediction:** Runs local simulation while waiting for server confirmation
- **Interpolation:** Smooths out network updates for visual quality

**Learning Note:** Client-side prediction makes the game feel responsive even with network latency. The client "guesses" what will happen, then reconciles with server updates.

## Data Flow

```
Player Input → Client → Server (Validates) → Game Simulation
                  ↑                                ↓
                  ←──── State Updates ──────────────
```

1. Player presses a key
2. Client sends input to server
3. Client also predicts the outcome locally
4. Server validates and processes input
5. Server broadcasts new authoritative state
6. Client reconciles prediction with server state

## Network Architecture

### Protocol: Lightyear over UDP

- **Reliable Channel:** Critical messages (spawn entity, game events)
- **Unreliable Channel:** High-frequency updates (position, velocity)
- **State Sync:** Server periodically sends full state snapshots

**Learning Note:** UDP is faster than TCP but doesn't guarantee delivery. Lightyear provides reliability where needed while keeping the speed of UDP.

## ECS Architecture (Bevy)

Bevy uses an **Entity Component System**:

- **Entities:** Unique IDs (just numbers)
- **Components:** Data attached to entities (Position, Velocity, Health)
- **Systems:** Functions that query and modify components

```rust
// Example system
fn movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Position)>
) {
    for (velocity, mut position) in &mut query {
        position.x += velocity.x * time.delta_seconds();
        position.y += velocity.y * time.delta_seconds();
    }
}
```

**Learning Note:** This data-oriented design is different from OOP. It's optimized for CPU cache performance and makes parallelism easier.

## Deployment Architecture

```
GitHub Push → Actions Runner (Orange Pi) → Docker Build → K3s Deploy
```

1. Push code to `main` branch
2. GitHub Actions webhook triggers self-hosted runner
3. Runner builds Docker image
4. Image deployed to local K3s cluster
5. Server accessible at `<orange-pi-ip>:5000/udp`

## Future Considerations

### Scaling

- **Horizontal:** Multiple server instances with load balancer
- **Spatial Partitioning:** Divide game world into zones

### Persistence

- **Database:** PostgreSQL or SQLite for game state
- **Snapshots:** Periodic full state saves

### Observability

- **Metrics:** Prometheus for server metrics
- **Logging:** Structured logging with tracing
- **Profiling:** Performance profiling tools

## Learning Path

1. **Phase 1:** Get basic client-server connection working
2. **Phase 2:** Implement simple entity movement
3. **Phase 3:** Add client-side prediction
4. **Phase 4:** Optimize network protocol
5. **Phase 5:** Deploy to Orange Pi and test in real network conditions
