# Development Tools Setup

This guide helps you install all the development tools needed for this project.

## Required Tools

### 1. Rust Toolchain

Already installed via `rust-toolchain.toml`, but verify:

```bash
rustc --version
cargo --version
rustfmt --version
cargo clippy --version
```

If not installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Just (Task Runner)

Makes running commands easier than remembering cargo flags.

```bash
# Using cargo
cargo install just

# Or on Arch Linux
sudo pacman -S just

# Verify
just --version
```

### 3. Mold Linker (Fast Linking)

Already configured in `.cargo/config.toml`, just needs installation:

```bash
# Arch Linux
sudo pacman -S mold

# Ubuntu/Debian
sudo apt install mold

# Or build from source
cargo install mold

# Verify
mold --version
```

## Recommended Tools

### 4. Pre-commit (Code Quality Hooks)

Automatically enforces quality checks before commits.

```bash
# Using pip
pip install pre-commit

# Using pipx (isolated install - recommended)
pipx install pre-commit

# Or on Arch Linux
sudo pacman -S pre-commit

# Install hooks in this repo
cd /home/pakishi/Projects/rust/assignment-W-4
pre-commit install

# Verify
pre-commit --version
```

**Learn more:** [docs/pre-commit-guide.md](pre-commit-guide.md)

### 5. Cargo Watch (Auto-reload)

Automatically runs checks when files change.

```bash
cargo install cargo-watch

# Use with
just watch
```

### 6. Cargo Audit (Security Checking)

Checks for known vulnerabilities in dependencies.

```bash
cargo install cargo-audit

# Use with
just audit
```

### 7. Cargo Outdated (Dependency Updates)

Shows outdated dependencies.

```bash
cargo install cargo-outdated

# Use with
just outdated
```

### Install All Cargo Tools at Once

```bash
just install-tools
```

This installs: cargo-watch, cargo-audit, cargo-outdated

## Optional Tools

### 8. Docker & Docker Compose

For containerization and deployment.

```bash
# Arch Linux
sudo pacman -S docker docker-compose

# Ubuntu/Debian
sudo apt install docker.io docker-compose

# Start Docker service
sudo systemctl start docker
sudo systemctl enable docker

# Add user to docker group (no sudo needed)
sudo usermod -aG docker $USER
# Log out and back in for this to take effect

# Verify
docker --version
docker-compose --version
```

### 9. Cargo Flamegraph (Performance Profiling)

For performance analysis.

```bash
cargo install flamegraph

# May need perf on Linux
sudo pacman -S perf  # Arch
sudo apt install linux-tools-generic  # Ubuntu

# Use with
cargo flamegraph --bin server
```

### 10. Bacon (Background Code Checker)

Alternative to cargo-watch with better UI.

```bash
cargo install bacon

# Run with
bacon
```

## VSCode Extensions (If Using VSCode)

The project already recommends these in `.vscode/extensions.json`:

1. **rust-analyzer** - Essential Rust support
2. **vadimcn.vscode-lldb** - Debugging
3. **tamasfe.even-better-toml** - TOML syntax
4. **serayuzgur.crates** - Dependency management
5. **usernamehw.errorlens** - Inline errors
6. **eamodio.gitlens** - Git integration

Install by opening VSCode and accepting the extension recommendations.

## Verification Checklist

Run these commands to verify everything is set up:

```bash
# Rust toolchain
rustc --version          # Should show 1.x.x
cargo --version          # Should show 1.x.x
rustfmt --version        # Should show nightly-...
cargo clippy --version   # Should show 0.1.x

# Task runner
just --version           # Should show 1.x.x

# Fast linker
mold --version           # Should show 1.x.x or 2.x.x

# Pre-commit (optional but recommended)
pre-commit --version     # Should show 3.x.x

# Cargo tools (optional)
cargo watch --version    # Should show 8.x.x
cargo audit --version    # Should show 0.18.x
cargo outdated --version # Should show 0.14.x

# Docker (optional)
docker --version         # Should show 24.x.x or newer
docker-compose --version # Should show 2.x.x or newer

# Project compiles
cargo check              # Should succeed
cargo test               # Should pass
```

## Post-Installation Setup

### 1. Configure Pre-commit

```bash
cd /home/pakishi/Projects/rust/assignment-W-4
just install-hooks
```

### 2. Test Pre-commit

```bash
# Run on all files to verify it works
pre-commit run --all-files
```

### 3. Verify Development Workflow

```bash
# Start auto-checking
just watch

# In another terminal, make a change and save
# Watch terminal should show checks running
```

## Troubleshooting

### Mold Not Found

If cargo complains about mold:

```bash
# Check if installed
which mold

# If not, install it
sudo pacman -S mold

# Or temporarily disable in .cargo/config.toml
# Comment out the mold linker lines
```

### Pre-commit Not Running

```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install

# Verify hook file exists
ls -la .git/hooks/pre-commit
```

### Cargo Tools Taking Too Long to Install

```bash
# Install them one at a time
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-outdated
```

### Permission Denied (Docker)

```bash
# Add user to docker group
sudo usermod -aG docker $USER

# Log out and back in, or use:
newgrp docker

# Test
docker run hello-world
```

## Quick Setup Script

For a fresh setup, run these commands in order:

```bash
# 1. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Install system tools
sudo pacman -S just mold pre-commit docker docker-compose

# 3. Install Cargo tools
just install-tools

# 4. Setup pre-commit hooks
just install-hooks

# 5. Verify everything
cargo check
pre-commit run --all-files

# 6. You're ready!
echo "✅ Development environment ready!"
```

## Next Steps

Once tools are installed:

1. Read [QUICK_START.md](../QUICK_START.md) for daily workflow
2. Try running examples: `just example ownership_basics`
3. Start the watch mode: `just watch`
4. Make a change and commit to see pre-commit in action

## Tool Reference

| Tool | Purpose | Install | Use |
|------|---------|---------|-----|
| Rust | Programming language | rustup | `cargo build` |
| Just | Task runner | `cargo install just` | `just <command>` |
| Mold | Fast linker | `pacman -S mold` | Automatic |
| Pre-commit | Git hooks | `pip install pre-commit` | `just install-hooks` |
| Cargo Watch | Auto-reload | `cargo install cargo-watch` | `just watch` |
| Cargo Audit | Security check | `cargo install cargo-audit` | `just audit` |
| Cargo Outdated | Dep updates | `cargo install cargo-outdated` | `just outdated` |
| Docker | Containers | `pacman -S docker` | `just docker-build` |

---

**Questions?** Check the [development-workflow.md](development-workflow.md) guide or ask Claude!
