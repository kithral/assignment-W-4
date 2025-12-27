# Pre-Push Checklist

Quick checklist before pushing baseline to main.

## ✅ Automated Checks

```bash
# 1. Verify compilation
cargo check --workspace

# 2. Run formatter
cargo fmt --all -- --check

# 3. Run linter
cargo clippy --workspace -- -D warnings

# 4. Run tests (optional but recommended)
cargo test --workspace
```

## ✅ Manual Verification

- [ ] README.md is up to date
- [ ] CHANGELOG.md reflects recent changes
- [ ] No sensitive data in commits (.env files, credentials)
- [ ] .gitignore properly excludes build artifacts
- [ ] GitHub Actions disabled (if desired)
- [ ] All new files are tracked (`git status`)

## ✅ Git Commands

```bash
# Review what will be committed
git status
git diff

# Stage all changes
git add .

# Verify staged changes
git diff --staged

# Create baseline commit
git commit -m "feat: microservices architecture baseline

- Convert server to microservices (9 services)
- Implement world-state service with health checks
- Implement script-executor with Rhai (default) and Lua (placeholder)
- Add docker-compose.dev.yml for local development
- Disable GitHub Actions during migration
- Add comprehensive documentation (4,500+ lines)
- All services compile successfully

Closes #[issue-number] (if applicable)

🤖 Generated with Claude Code"

# Push to main
git push origin main
```

## 📋 Post-Push

After pushing:

1. Create development branch:

   ```bash
   git checkout -b feature/world-state-ecs
   ```

2. Verify GitHub shows the push

3. Check that GitHub Actions didn't trigger (should be disabled)

4. Start implementing!

## 🔄 Alternative: Protected Main Branch

If you want to protect main:

```bash
# Push to a feature branch first
git checkout -b baseline/microservices
git push -u origin baseline/microservices

# Then merge via PR on GitHub
```

---

**Current Status:** Ready to push ✅

- All services compile
- Documentation complete
- No compilation errors
