# Assignment W-4

**Status:** Active Development  
**Classification:** Internal Learning / Simulation Framework

## 📋 Project Overview

Assignment W-4 is a **programmable multiplayer world engine** that bridges the gap between classic MUD/MUSH servers and modern graphical games. Inspired by the programmability of TinyMUSH and the visual richness of modern game engines, this platform allows users to create their own persistent worlds with custom rules, art, and gameplay—all while leveraging a standardized scripting language.

### Vision

A hybrid platform where:

- **Players** interact through both graphical clients and text-based interfaces
- **Administrators** program game logic in-world using an embedded scripting language
- **World builders** define custom content, rules, and systems without forking the codebase
- **The engine** remains content-agnostic, supporting any genre (fantasy, sci-fi, modern, etc.)

### Learning Objectives

This project serves as a rigorous testbed for:

- **Memory Safety & Concurrency:** Leveraging Rust's borrow checker to handle massive entity counts and concurrent script execution without race conditions.
- **Network Replication:** Implementing client-side prediction, interpolation, and server reconciliation over UDP.
- **Language Design:** Building a safe, sandboxed scripting language embedded in Rust.
- **DevOps Engineering:** Establishing a zero-touch CI/CD pipeline targeting ARM64 infrastructure (Orange Pi 5+).
- **ECS Architecture:** Using Bevy's data-oriented design for flexible, performant game systems.

## 🛠 Tech Stack

### Core Application

- **Language:** [Rust](https://www.rust-lang.org/) (2021 Edition)
- **Engine:** [Bevy](https://bevyengine.org/) (Data-Oriented ECS)
- **Networking:** [Lightyear](https://github.com/carrascomj/lightyear) (Netcode for reliable/unreliable UDP channels & state sync)
- **Scripting:** Embedded scripting language for in-world programming *[Planned]*
  - Options: [Rhai](https://rhai.rs/), [Lua](https://github.com/mlua-rs/mlua), or custom DSL
- **Physics:** [Avian](https://github.com/Jondolf/avian) (ECS-native physics engine) *[Planned]*
- **Protocol:** Bincode (Binary serialization) + Text-based MUSH protocol

### Infrastructure & Operations

- **Architecture:** Microservices with independent scaling
- **Containerization:** Docker (Multi-stage builds → Distroless runtime)
- **Orchestration:** K3s (Lightweight Kubernetes) on ARM64
- **Service Mesh:** Traefik for load balancing and routing
- **Message Queue:** RabbitMQ for async processing
- **Cache/Pub-Sub:** Redis for state and events
- **Database:** PostgreSQL with read replicas
- **CI/CD:** GitHub Actions (Self-hosted runners)
- **Monitoring:** Prometheus + Grafana
- **Target Hardware:** Orange Pi 5 Plus (expandable cluster)

## 📂 Architecture

The project uses a **microservices architecture** where each service can scale independently based on demand.

```text
assignment-W-4/
├── services/                # Microservices (each independently scalable)
│   ├── world-state/        ✅ Core ECS service (implemented)
│   ├── script-executor/    ✅ Rhai/Lua scripting (implemented)
│   ├── graphics-gateway/   📋 UDP client connections (template)
│   ├── text-gateway/       📋 Telnet/WebSocket (template)
│   ├── auth-service/       📋 Authentication (template)
│   ├── physics-service/    📋 Physics simulation (template)
│   ├── chat-service/       📋 Chat and messaging (template)
│   ├── persistence-service/📋 Database operations (template)
│   └── asset-service/      📋 Asset delivery (template)
├── shared/                 # Shared code across services
│   ├── src/               # Common modules
│   └── examples/          # Learning examples
├── client/                # Graphical client (Bevy)
├── server/                # Legacy monolith (being deprecated)
├── docker-compose.dev.yml # Local development
└── docs/                  # 4,500+ lines of documentation
```

**Quick Start:** See [docs/MICROSERVICES_QUICKSTART.md](docs/MICROSERVICES_QUICKSTART.md)
**Architecture:** See [docs/microservices-architecture.md](docs/microservices-architecture.md)

### Key Concepts

**Multi-Protocol Server:**

- Graphical clients connect via UDP (Lightyear)
- Text clients connect via Telnet/WebSocket
- Both interact with the same world state

**Programmable Worlds:**

- In-world scripting language for game logic
- Permission system for player/admin capabilities
- Content stored in database, not hardcoded

**Content Agnostic:**

- No hardcoded "space" or "fantasy" theme
- World builders define their own:
  - Art assets and models
  - Game rules and mechanics
  - Custom commands and verbs
  - Economy and progression systems

## 🚀 Getting Started

### Prerequisites

- Rust & Cargo (Latest Stable) - Automatically configured via `rust-toolchain.toml`
- Just (Task Runner) - `cargo install just`
- Mold Linker (Recommended for fast iteration on Linux) - `sudo pacman -S mold`
- Pre-commit (Optional, for code quality hooks) - `pip install pre-commit`
- Docker (Optional, for release builds) - `sudo pacman -S docker`

**Quick setup:** See [docs/tools-setup.md](docs/tools-setup.md) for detailed installation instructions.

### Development

#### 1. Clone and Setup

```bash
git clone https://github.com/kithral/assignment-W-4.git
cd assignment-W-4

# Install development tools
just install-tools

# Setup pre-commit hooks (optional but recommended)
just install-hooks

# Verify everything works
cargo check
```

#### 2. Run the Services

**Option A: Run locally (recommended for development)**

```bash
# Run infrastructure
docker-compose -f docker-compose.dev.yml up redis postgres -d

# Run a service
cargo run -p world-state
cargo run -p script-executor
```

**Option B: Run everything with Docker**

```bash
docker-compose -f docker-compose.dev.yml up
```

**Option C: Run legacy monolith (deprecated)**

```bash
just server  # Terminal 1
just client  # Terminal 2
```

**Quick Reference:** See [QUICK_START.md](QUICK_START.md) for daily workflow commands.

## 📚 Documentation

### Quick Start

- **[docs/MICROSERVICES_QUICKSTART.md](docs/MICROSERVICES_QUICKSTART.md)** - Microservices guide (START HERE!)
- **[PROJECT_STATUS.md](PROJECT_STATUS.md)** - Current status and implementation roadmap
- **[QUICK_START.md](QUICK_START.md)** - Daily workflow and essential commands
- **[docs/MIGRATION_TO_MICROSERVICES.md](docs/MIGRATION_TO_MICROSERVICES.md)** - What changed
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development standards and guidelines

### Architecture & Design

- **[docs/vision.md](docs/vision.md)** - Complete project vision
- **[docs/microservices-architecture.md](docs/microservices-architecture.md)** - 9 services design
- **[docs/deployment-strategy.md](docs/deployment-strategy.md)** - Deployment workflows
- **[docs/architecture.md](docs/architecture.md)** - System architecture overview
- **[docs/scripting-research.md](docs/scripting-research.md)** - Scripting language evaluation

### GitOps & Deployment

- **[docs/ARGOCD_SUMMARY.md](docs/ARGOCD_SUMMARY.md)** - ArgoCD + GitOps overview
- **[docs/ARGOCD_SETUP.md](docs/ARGOCD_SETUP.md)** - ArgoCD installation guide
- **[docs/DEPLOYMENT_WORKFLOW.md](docs/DEPLOYMENT_WORKFLOW.md)** - Daily deployment operations
- **[docs/ORANGEPI_SETUP_CHECKLIST.md](docs/ORANGEPI_SETUP_CHECKLIST.md)** - Orange Pi setup checklist

### Database & Versioning

- **[docs/DATABASE_STRATEGY.md](docs/DATABASE_STRATEGY.md)** - PostgreSQL + SQLx migration strategy
- **[docs/VERSIONING_STRATEGY.md](docs/VERSIONING_STRATEGY.md)** - SemVer 2.0 + Conventional Commits
- **[docs/DATABASE_AND_VERSIONING_SETUP.md](docs/DATABASE_AND_VERSIONING_SETUP.md)** - Quick setup guide

### Development Guides

- **[docs/rust-learnings.md](docs/rust-learnings.md)** - Rust concepts tracker
- **[docs/development-workflow.md](docs/development-workflow.md)** - Detailed workflow guide
- **[docs/pre-commit-guide.md](docs/pre-commit-guide.md)** - Pre-commit hooks setup
- **[docs/tools-setup.md](docs/tools-setup.md)** - Development tools installation

## 🔧 Development Tools

This project includes comprehensive tooling:

- **Pre-commit Hooks** - Automatic code quality checks before commits
- **Claude AI Integration** - Best practices enforcement in `.claude/settings.json`
- **VSCode Configuration** - Complete IDE setup with debugging
- **Just Task Runner** - 30+ commands for common operations
- **Docker Support** - Containerization with multi-stage builds

Run `just` to see all available commands.

## ⚙️ Deployment Pipeline

This project uses **GitOps** with ArgoCD for automated deployments:

1. **Developer pushes code** to main branch
2. **GitHub Actions** detects changes (path-based filtering)
3. **Only affected services** are built and pushed to ghcr.io
4. **Kustomize manifests** updated with new image tags
5. **ArgoCD** automatically syncs changes to Kubernetes
6. **Services deployed** to local K3s cluster on Orange Pi

**Key Features:**
- ✅ Only changed services rebuild (monorepo optimization)
- ✅ GitOps - Git as single source of truth
- ✅ Auto-sync with rollback capabilities
- ✅ Self-hosted on ARM64 infrastructure

See [docs/ARGOCD_SUMMARY.md](docs/ARGOCD_SUMMARY.md) for complete GitOps setup.

## 🎓 Learning Resources

This is a learning project with educational features:

- Structured documentation tracking Rust concepts
- Example programs demonstrating ownership and ECS
- Claude AI configured for educational responses
- Pre-commit hooks providing immediate feedback
- Comprehensive inline comments and documentation

**New to Rust?** Start with [docs/rust-learnings.md](docs/rust-learnings.md) and run the examples:

```bash
cargo run --example ownership_basics --package shared
cargo run --example bevy_ecs_basics --package shared
```

---
*Note: This repository contains experimental code for educational purposes. The primary goal is learning Rust and systems programming concepts.*
