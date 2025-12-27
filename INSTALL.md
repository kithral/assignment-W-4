# Installation Guide

Quick installation guide for all required tools on Arch Linux.

## Required Tools Installation

### 1. Just (Task Runner)

```bash
# Option 1: Via pacman (recommended)
sudo pacman -S just

# Option 2: Via cargo
cargo install just

# Verify
just --version
```

### 2. Mold (Fast Linker)

```bash
sudo pacman -S mold

# Verify
mold --version
```

### 3. Pre-commit (Git Hooks)

```bash
# Option 1: Via pacman (recommended)
sudo pacman -S pre-commit

# Option 2: Via pip
pip install pre-commit

# Option 3: Via pipx (isolated)
pipx install pre-commit

# Verify
pre-commit --version
```

## One-Line Install (All Required Tools)

```bash
sudo pacman -S just mold pre-commit
```

## Optional Tools

### Cargo Development Tools

```bash
cargo install cargo-watch cargo-audit cargo-outdated
```

### Docker (for containerization)

```bash
sudo pacman -S docker docker-compose

# Start Docker service
sudo systemctl enable docker
sudo systemctl start docker

# Add user to docker group (no sudo needed)
sudo usermod -aG docker $USER
# Log out and back in for this to take effect
```

## Setup After Installation

Once tools are installed:

```bash
# 1. Setup pre-commit hooks
pre-commit install

# 2. Verify everything works
cargo check

# 3. Run quality checks
just qa
```

## Quick Start

```bash
# See all available commands
just

# Run development workflow
just watch        # Terminal 1: Auto-check on changes
just server       # Terminal 2: Run server
just client       # Terminal 3: Run client
```

## Verification

Run these to verify everything is installed:

```bash
rustc --version          # Should show 1.x.x
cargo --version          # Should show 1.x.x
just --version           # Should show 1.x.x
mold --version           # Should show 1.x.x or 2.x.x
pre-commit --version     # Should show 3.x.x
```

## Troubleshooting

### Just not found after cargo install

Add cargo bin to PATH in `~/.zshrc`:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Mold not being used

Already configured in `.cargo/config.toml`, just ensure it's installed:

```bash
which mold  # Should show /usr/bin/mold
```

### Pre-commit not running

After installing, run in the project directory:

```bash
pre-commit install
```

## Next Steps

After installation:

1. Read [QUICK_START.md](QUICK_START.md) for daily workflow
2. Setup tools: `pre-commit install`
3. Start coding: `just watch`

---

**Need help?** See [docs/tools-setup.md](docs/tools-setup.md) for detailed information.
