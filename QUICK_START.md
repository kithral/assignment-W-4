# Quick Start Guide

## First Time Setup - Done! ✅

Your project is fully configured and ready for development.

## Daily Development

### Run the Project

```bash
# Terminal 1: Start server
just server

# Terminal 2: Start client
just client
```

### While Coding

```bash
# Auto-check on file changes (recommended)
just watch

# Manual checks
just check          # Fast type checking
just test           # Run tests
just lint           # Run clippy
just fmt            # Format code
```

### Before Committing

```bash
# Run full quality assurance
just qa

# If all passes, commit
git add .
git commit -m "Your message"
git push
```

## Essential Commands

| Command | What It Does |
|---------|-------------|
| `just` | List all available commands |
| `just server` | Run server |
| `just client` | Run client |
| `just test` | Run all tests |
| `just qa` | Quality check (lint + format + test) |
| `just watch` | Auto-run checks on file changes |
| `just build` | Build all workspace members |
| `just docker-build` | Build Docker image |
| `cargo check` | Fast compilation check |
| `cargo run --example ownership_basics --package shared` | Run learning example |

## File Locations

| What | Where |
|------|-------|
| Shared game logic | `shared/src/` |
| Server code | `server/src/main.rs` |
| Client code | `client/src/main.rs` |
| Learning examples | `shared/examples/` |
| Documentation | `docs/` |
| Tests | `tests/` |

## Documentation

- **[docs/SETUP_COMPLETE.md](docs/SETUP_COMPLETE.md)** - What was set up
- **[docs/rust-learnings.md](docs/rust-learnings.md)** - Track Rust concepts
- **[docs/architecture.md](docs/architecture.md)** - System overview
- **[docs/development-workflow.md](docs/development-workflow.md)** - Detailed workflow
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development standards
- **[.claude/README.md](.claude/README.md)** - Claude Code settings & best practices

## Learning Path

1. ✅ Environment is set up
2. ⬜ Run the example programs (`just example ownership_basics`)
3. ⬜ Read `docs/rust-learnings.md`
4. ⬜ Create your first component in `shared/src/components.rs`
5. ⬜ Write your first system in `shared/src/systems.rs`
6. ⬜ Get client and server communicating
7. ⬜ Track your progress in `docs/rust-learnings.md`

## Troubleshooting

**Build fails:** `cargo clean && cargo update && cargo check`

**IDE slow:** Restart rust-analyzer (Cmd/Ctrl+Shift+P → "rust-analyzer: Restart")

**Format warnings:** Ignore nightly-only feature warnings in rustfmt - they're harmless

## Claude Code Integration

This project includes a [.claude/settings.json](.claude/settings.json) configuration that ensures Claude provides:

- ✅ **Rust best practices** - Idiomatic code, proper error handling, ownership patterns
- ✅ **Bevy ECS guidelines** - Component design, system organization, performance tips
- ✅ **Learning mode** - Explanations, comparisons, and links to resources
- ✅ **Code review** - Catches common mistakes like unwrap(), unnecessary cloning, etc.

See [.claude/examples.md](.claude/examples.md) for examples of how this works in practice.

## Get Help

- Check `docs/` for answers
- Read error messages carefully (Rust errors are very helpful!)
- Update `docs/rust-learnings.md` when you find solutions
- Ask Claude - it's configured to provide educational, best-practice responses

---

**Ready to code!** Start with `just watch` in one terminal and edit files in your editor. 🦀
