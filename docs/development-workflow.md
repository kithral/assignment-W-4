# Development Workflow

## Daily Development Cycle

### 1. Start Your Development Session

```bash
# Pull latest changes
git pull

# Check project compiles
just check

# Or use the faster cargo check
cargo check
```

### 2. Work on a Feature

```bash
# Run tests in watch mode (requires cargo-watch)
just watch

# Or manually run checks after changes
just check
just test
```

### 3. Test Your Changes

```bash
# Run server in one terminal
just server

# Run client in another terminal
just client

# For performance testing, use release builds
just server-release
just client-release
```

### 4. Before Committing

```bash
# Run full quality assurance check
just qa

# This runs: clippy, fmt, and test
```

### 5. Commit Your Changes

```bash
git add .
git commit -m "Brief description of changes"
git push
```

## Useful Commands

### Building

```bash
just build              # Debug build (faster compile, slower runtime)
just build-release      # Release build (slower compile, faster runtime)
just check              # Type check only (fastest)
```

### Running

```bash
just server            # Run server
just client            # Run client
just server-release    # Run server (optimized)
just client-release    # Run client (optimized)
```

### Testing & Quality

```bash
just test              # Run all tests
just test-verbose      # Show test output
just lint              # Run clippy
just fmt               # Format code
just qa                # All of the above
```

### Docker

```bash
just docker-build      # Build Docker image
just docker-run        # Run container
just docker-stop       # Stop container
just docker-refresh    # Rebuild and restart
```

### Documentation

```bash
just docs              # Generate and open docs
```

## Debugging

### Using VSCode

1. Set breakpoints by clicking left of line numbers
2. Press `F5` or use "Run and Debug" panel
3. Choose "Debug Server" or "Debug Client"
4. Step through code with `F10` (step over) or `F11` (step into)

### Using Print Debugging

```rust
// Basic print
println!("Debug: {:?}", variable);

// Pretty print
println!("Debug: {:#?}", variable);

// Bevy info logging
info!("Game state: {:?}", state);

// Conditional compilation (only in debug builds)
#[cfg(debug_assertions)]
println!("This only prints in debug mode");
```

### Using RUST_LOG

```bash
# Run with debug logging
RUST_LOG=debug cargo run --bin server

# More specific logging
RUST_LOG=assignment_w4=debug,bevy=info cargo run --bin server
```

## Common Workflows

### Adding a New Component

1. Define in `shared/src/components.rs`
2. Derive `Component` trait
3. Export in `shared/src/lib.rs` if needed
4. Use in systems on both client and server

### Adding a New Protocol Message

1. Define in `shared/src/protocol.rs`
2. Derive `Serialize`, `Deserialize`, `Clone`, `Debug`
3. Register with Lightyear (see Lightyear docs)
4. Handle in client and server

### Adding a New System

1. Define in appropriate crate (client/server/shared)
2. Add to plugin or app
3. Specify system ordering if needed
4. Test with `cargo test`

### Fixing Compilation Errors

```bash
# See detailed error messages
cargo build

# Use clippy for additional hints
cargo clippy

# Check specific package
cargo check --package server
```

### Performance Profiling

```bash
# Install profiling tools
cargo install flamegraph

# Profile the server
cargo flamegraph --bin server

# Profile with release optimizations
cargo flamegraph --release --bin server
```

## Git Workflow

### Feature Branch Workflow

```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes and commit
git add .
git commit -m "Add feature X"

# Push and create PR
git push -u origin feature/my-feature
```

### Keeping Up to Date

```bash
# Update main branch
git checkout main
git pull

# Rebase feature branch
git checkout feature/my-feature
git rebase main
```

## Troubleshooting

### Build Errors

**Problem:** Cargo can't find dependencies

```bash
# Solution: Update dependencies
cargo update
```

**Problem:** Linker errors on Linux

```bash
# Solution: Ensure mold is installed
sudo pacman -S mold  # or your package manager
```

**Problem:** Build is very slow

```bash
# Solution: Enable parallel compilation
export CARGO_BUILD_JOBS=8  # or number of cores
```

### Runtime Errors

**Problem:** Server won't start

- Check port 5000 isn't already in use: `netstat -tulpn | grep 5000`
- Check firewall settings
- Look at server logs for error messages

**Problem:** Client can't connect to server

- Verify server IP address
- Check firewall allows UDP on port 5000
- Test with local connection first (127.0.0.1)

### IDE Issues

**Problem:** rust-analyzer is slow

- Exclude `target/` directory in settings (already configured)
- Restart rust-analyzer: Command Palette → "rust-analyzer: Restart"

**Problem:** Code formatting isn't working

- Ensure `rustfmt` is installed: `rustup component add rustfmt`
- Check "Format on Save" is enabled in VSCode settings

## Learning Resources

- Run `just` to see all available commands
- Check `docs/rust-learnings.md` for Rust concepts
- See `docs/architecture.md` for system overview
- Read Bevy examples: https://github.com/bevyengine/bevy/tree/main/examples
- Read Lightyear examples: https://github.com/cBournhonesque/lightyear/tree/main/examples
