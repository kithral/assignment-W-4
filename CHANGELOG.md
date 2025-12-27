# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Documentation Capstone (2025-12-26)

- **PROJECT_STATUS.md** - Comprehensive 480-line status report
  - Complete architecture overview with 9 microservices
  - Documentation inventory (4,500+ lines across 22 files)
  - 5-phase implementation roadmap
  - Immediate next steps with 3 options
  - Success metrics and learning achievements
  - Recommended path: Text MUD → Scripting → Graphics → Microservices
- **.claude/CONVERSATION_SUMMARY.md** - Session handoff document for continuity
- Updated README.md documentation section for better navigation

### Changed - Project Scope (2025-12-26)

**Major Direction Change:** Evolved from space simulation to programmable world engine with microservices architecture

- **New Vision:** Hybrid MUD/MUSH + graphical game engine
  - Inspired by TinyMUSH's programmability + modern game engines
  - Support for both text-based and graphical clients
  - In-world scripting language for user-created content
  - Content-agnostic platform (not tied to any specific genre)

- **Microservices Architecture:**
  - Independent scaling of services (graphics, scripting, persistence, etc.)
  - Fast deployments - update one service without redeploying everything
  - Fault isolation - one service failure doesn't crash entire system
  - gRPC for service-to-service communication
  - RabbitMQ for async processing
  - Redis for caching and pub/sub
  - K3s orchestration with auto-scaling

- **Updated Documentation:**
  - `README.md` - Reflects new programmable engine vision and microservices architecture
  - `docs/vision.md` - Comprehensive vision document
  - `docs/scripting-research.md` - Scripting language evaluation (Rhai recommended)
  - `docs/microservices-architecture.md` - Complete microservices design with 9 services
  - `docs/deployment-strategy.md` - Fast deployment workflows and rollback strategies

- **New Technical Goals:**
  - Embedded scripting language (Rhai planned)
  - Multi-protocol server (UDP for graphics, TCP for text)
  - Database persistence for user-created content
  - Sandboxed script execution
  - Content hot-reloading
  - Independent service scaling (2-minute deployments)
  - Blue/green and canary deployments
  - Distributed tracing and monitoring

### Added (2025-12-26)

- Project structure improvements for better Rust learning
- Modular organization in `shared/src/` with separate modules for:
  - `components.rs` - Shared ECS components
  - `protocol.rs` - Network protocol definitions
  - `systems.rs` - Shared game logic
  - `physics.rs` - Physics constants and utilities
- Development tooling configuration:
  - `rustfmt.toml` - Code formatting standards
  - `rust-toolchain.toml` - Rust version management
  - `.clippy.toml` - Linter configuration for learning best practices
- Enhanced VSCode configuration:
  - Comprehensive settings for rust-analyzer
  - Debug configurations for client and server
  - Task definitions for common operations
  - Recommended extensions list
- `justfile` - Task runner with 25+ useful commands
- Docker improvements:
  - `.dockerignore` for faster builds
  - `docker-compose.yml` for local testing
- Comprehensive documentation:
  - `docs/architecture.md` - System architecture overview
  - `docs/rust-learnings.md` - Rust concepts tracking document
  - `docs/development-workflow.md` - Daily development guide
  - `CONTRIBUTING.md` - Development standards and workflow
- Example programs:
  - `shared/examples/ownership_basics.rs` - Ownership demonstration
  - `shared/examples/bevy_ecs_basics.rs` - ECS fundamentals
- Integration test structure in `tests/`
- Directory structure for future examples in each crate

### Fixed

- Corrected Rust edition from invalid "2024" to "2021" in all `Cargo.toml` files
- Renamed `.github/` directory (was incorrectly named `,github/`)
- Generated and committed `Cargo.lock` for reproducible builds

### Changed

- Commented out unused workspace dependencies (serde, tokio) with notes for future use
- Enhanced `.cargo/config.toml` with:
  - Build parallelization settings
  - Cargo aliases for common commands
  - Documentation for cross-compilation to ARM64
- Updated `.gitignore` to properly exclude build artifacts
- Improved workspace structure for better code organization

## [0.1.0] - Initial Setup

### Added

- Basic Cargo workspace with three crates: client, server, shared
- Bevy and Lightyear dependencies
- Docker configuration for server deployment
- GitHub Actions workflow for CI/CD
- Mold linker configuration for fast builds on Linux
- Development profile optimizations
