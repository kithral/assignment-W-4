# Justfile - Task runner for common commands
# Install just: cargo install just
# Run: just <command>

# List all available commands
default:
    @just --list

# Build all workspace members
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run the server
server:
    cargo run --bin server

# Run the client
client:
    cargo run --bin client

# Run server in release mode for testing performance
server-release:
    cargo run --release --bin server

# Run client in release mode
client-release:
    cargo run --release --bin client

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run clippy (Rust linter) with strict settings
lint:
    cargo clippy --all-targets -- -D warnings

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run cargo check (faster than build, just type checks)
check:
    cargo check --all-targets

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# Run clippy, format, and test - full quality check
qa: lint fmt test
    @echo "✅ Quality assurance passed!"

# Build Docker image for server
docker-build:
    docker build -t assignment-w4-server:latest -f server/Dockerfile .

# Run Docker container locally
docker-run:
    docker run -d --name game-server --restart unless-stopped -p 5000:5000/udp assignment-w4-server:latest

# Stop and remove Docker container
docker-stop:
    docker stop game-server || true
    docker rm game-server || true

# Full Docker rebuild and run
docker-refresh: docker-stop docker-build docker-run
    @echo "🐳 Docker container refreshed!"

# Watch for changes and run checks (requires cargo-watch)
watch:
    cargo watch -x check -x test

# Generate documentation and open in browser
docs:
    cargo doc --no-deps --open

# Run benchmarks (when you add them to benches/)
bench:
    cargo bench

# Show outdated dependencies
outdated:
    cargo outdated

# Audit dependencies for security vulnerabilities
audit:
    cargo audit

# Install useful development tools
install-tools:
    cargo install cargo-watch cargo-audit cargo-outdated

# Install and setup pre-commit hooks
install-hooks:
    pip install pre-commit
    pre-commit install
    @echo "✅ Pre-commit hooks installed!"

# Run pre-commit on all files
pre-commit-all:
    pre-commit run --all-files

# Update pre-commit hooks to latest versions
update-hooks:
    pre-commit autoupdate

# Learning: Run an example from shared crate
example NAME:
    cargo run --example {{NAME}} --package shared

# Learning: Run clippy with all lints (educational)
clippy-pedantic:
    cargo clippy --all-targets -- -W clippy::pedantic

# ARM64 cross-compile setup (for Orange Pi target)
setup-cross:
    rustup target add aarch64-unknown-linux-gnu
