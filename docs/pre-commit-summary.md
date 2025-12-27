# Pre-commit Hooks - Quick Summary

## What Was Added

✅ **[.pre-commit-config.yaml](../.pre-commit-config.yaml)** - 122 lines
✅ **[.markdownlint.yaml](../.markdownlint.yaml)** - Markdown linting config
✅ **[docs/pre-commit-guide.md](pre-commit-guide.md)** - 318 lines of documentation
✅ **[docs/tools-setup.md](tools-setup.md)** - Tool installation guide
✅ **Updated [justfile](../justfile)** - Added pre-commit commands
✅ **Updated [README.md](../README.md)** - Mentioned in setup instructions

## What Pre-commit Does

Pre-commit hooks run **automatically before every commit** and enforce:

### Rust Quality (Every Commit)

1. ✅ **Cargo Check** - Code must compile
2. ✅ **Cargo Format** - Code must be formatted
3. ✅ **Cargo Clippy** - No linter warnings allowed
4. ✅ **Cargo Test** - All tests must pass

### General Quality (Every Commit)

1. ✅ No large files (>1MB)
2. ✅ No case conflicts in filenames
3. ✅ Files end with newline
4. ✅ No merge conflict markers
5. ✅ Valid YAML/TOML syntax
6. ✅ No trailing whitespace
7. ✅ No private keys
8. ✅ Markdown linting
9. ⚠️ TODO/FIXME warning (doesn't block)

## Quick Installation

```bash
# Install pre-commit
pip install pre-commit

# Or on Arch Linux
sudo pacman -S pre-commit

# Setup hooks (one time)
cd /home/pakishi/Projects/rust/assignment-W-4
just install-hooks

# Or manually
pre-commit install
```

## How It Works

### Normal Workflow (Automatic)

```bash
# Make changes
vim shared/src/lib.rs

# Stage changes
git add .

# Commit - hooks run automatically!
git commit -m "Add new feature"
```

If checks fail, commit is blocked with helpful error messages.

### Manual Run

```bash
# Check all files
pre-commit run --all-files

# Check only staged files
pre-commit run

# Check specific hook
pre-commit run cargo-clippy
```

### Skip Hooks (Emergency Only!)

```bash
# Skip all hooks
git commit --no-verify -m "WIP: experimental"

# Better: Use WIP branch instead
git checkout -b wip/experiment
git commit -m "Trying new approach"
```

## Integration with Workflow

Pre-commit works alongside:

| Tool | When It Runs | What It Checks |
|------|--------------|----------------|
| **Pre-commit** | Before commit | Format, clippy, tests, files |
| **VSCode** | On save | Format (auto-fix) |
| **`just watch`** | On file change | Check, test |
| **`just qa`** | Manual | Clippy, format, test |
| **GitHub Actions** | On push | Full CI/CD pipeline |

Think of it as **layers of defense**:

1. VSCode catches issues as you type
2. `just watch` catches issues when you save
3. Pre-commit catches issues before commit
4. GitHub Actions catches issues before deploy

## Common Scenarios

### Scenario 1: Everything Passes ✅

```bash
$ git commit -m "Add player movement"
Cargo Check.........................................Passed
Cargo Format Check..................................Passed
Cargo Clippy........................................Passed
Cargo Test..........................................Passed
Check Added Large Files.............................Passed
Check Case Conflict.................................Passed
End of File Fixer...................................Passed
Check Merge Conflict................................Passed
Check Yaml..........................................Passed
Check Toml..........................................Passed
Trailing Whitespace.................................Passed
Detect Private Key..................................Passed
markdownlint........................................Passed
Check for TODOs.....................................Passed

[main a1b2c3d] Add player movement
 2 files changed, 45 insertions(+)
```

Commit succeeds! ✅

### Scenario 2: Format Check Fails ❌

```bash
$ git commit -m "Add feature"
Cargo Check.........................................Passed
Cargo Format Check..................................Failed

Diff in shared/src/lib.rs at line 42...

[Fix by running: cargo fmt]
```

**Solution:**

```bash
cargo fmt
git add .
git commit -m "Add feature"
```

### Scenario 3: Tests Fail ❌

```bash
$ git commit -m "Add feature"
...
Cargo Test..........................................Failed

test shared::tests::it_works ... FAILED
```

**Solution:** Fix the test first!

```bash
# Fix the failing test
cargo test  # Verify it passes
git add .
git commit -m "Add feature"
```

### Scenario 4: Work in Progress

```bash
# Don't use --no-verify unless necessary!
# Better: Use a WIP branch
git checkout -b wip/new-feature
git commit -m "WIP: Trying new approach"

# Later, when ready:
git checkout main
git merge wip/new-feature  # Hooks will run on merge commit
```

## Benefits for Learning

Pre-commit is **excellent for learning** because:

1. **Immediate Feedback** - See mistakes right away
2. **No Bad Habits** - Can't commit bad code patterns
3. **Learn by Correction** - Error messages teach you
4. **Build Muscle Memory** - Start writing correct code naturally
5. **Safety Net** - Catch issues before they become problems

## Configuration Files

- **[.pre-commit-config.yaml](../.pre-commit-config.yaml)** - Hook configuration
- **[.markdownlint.yaml](../.markdownlint.yaml)** - Markdown rules
- **[docs/pre-commit-guide.md](pre-commit-guide.md)** - Full documentation

## Just Commands

```bash
just install-hooks      # Install pre-commit hooks
just pre-commit-all     # Run on all files
just update-hooks       # Update to latest versions
```

## Customization

### Disable Specific Hooks Temporarily

Edit `.pre-commit-config.yaml`:

```yaml
# - id: cargo-test  # Disabled for now
```

Then: `pre-commit install`

### Change Behavior

In `.pre-commit-config.yaml`:

```yaml
fail_fast: true  # Stop on first failure (faster feedback)
```

## Performance

- **Fast on incremental changes** - Only checks changed files
- **Slower on large changes** - But ensures quality
- **Parallelizes where possible** - Multiple hooks run concurrently

**Tip:** Commit frequently = faster hooks!

## Troubleshooting

### Hooks not running

```bash
pre-commit uninstall
pre-commit install
ls -la .git/hooks/pre-commit  # Should exist
```

### False positives

```bash
# Use --no-verify sparingly
git commit --no-verify -m "Reason for skip"
```

### Update hooks

```bash
just update-hooks
# or
pre-commit autoupdate
```

## Next Steps

1. **Install:** `just install-hooks`
2. **Test:** Make a change and commit
3. **Experience:** See hooks in action
4. **Learn:** Read error messages carefully
5. **Iterate:** Fix issues and re-commit

## Summary

Pre-commit hooks are your **first line of defense** against:

- ❌ Unformatted code
- ❌ Compiler errors
- ❌ Linter warnings
- ❌ Failing tests
- ❌ Large files
- ❌ Security issues

They ensure every commit meets quality standards, helping you build excellent habits as you learn Rust! 🦀

---

**Install now:** `just install-hooks`
**Full guide:** [docs/pre-commit-guide.md](pre-commit-guide.md)
