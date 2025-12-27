# Pre-commit Hooks Guide

This project uses [pre-commit](https://pre-commit.com/) to automatically enforce quality standards before code is committed.

## What Are Pre-commit Hooks?

Pre-commit hooks are scripts that run automatically when you try to commit code. They check your changes and **block the commit** if any checks fail. This helps you:

- Catch mistakes before they enter version control
- Build good habits (format code, run tests)
- Maintain consistent code quality
- Learn from automated feedback

## Installation

### 1. Install pre-commit

```bash
# Using pip (Python package manager)
pip install pre-commit

# Or using pipx (recommended - isolated install)
pipx install pre-commit

# On Arch Linux
sudo pacman -S pre-commit
```

### 2. Install the git hooks

```bash
# From the project root
cd /home/pakishi/Projects/rust/assignment-W-4
pre-commit install
```

This adds the hooks to your `.git/hooks/` directory. You only need to do this once per clone.

### 3. Verify installation

```bash
pre-commit --version
```

## What Gets Checked

Every time you commit, these checks run automatically:

### Rust Quality Checks

1. **Cargo Check** - Ensures code compiles
2. **Cargo Format** - Checks code formatting (rustfmt)
3. **Cargo Clippy** - Runs linter with strict settings
4. **Cargo Test** - Runs all tests

### General File Checks

1. **Large Files** - Prevents committing files > 1MB
2. **Case Conflicts** - Checks for filename case issues
3. **End of File** - Ensures newline at end of files
4. **Merge Conflicts** - Detects unresolved conflicts
5. **YAML/TOML Syntax** - Validates config files
6. **Trailing Whitespace** - Removes extra spaces
7. **Private Keys** - Detects accidentally committed secrets
8. **Markdown Linting** - Checks documentation quality

### Informational Checks

1. **TODO Comments** - Warns about TODO/FIXME (doesn't block)

## Usage

### Automatic (Recommended)

Just commit normally - the hooks run automatically:

```bash
git add .
git commit -m "Add new feature"
```

If checks fail, the commit is blocked and you'll see error messages.

### Manual Run

Run checks manually without committing:

```bash
# Check all files
pre-commit run --all-files

# Check only staged files
pre-commit run

# Run a specific hook
pre-commit run cargo-clippy --all-files
```

### Skip Hooks (Use Sparingly!)

Sometimes you need to commit work-in-progress code:

```bash
# Skip all hooks
git commit --no-verify -m "WIP: testing approach"

# Better: Use WIP branch instead
git checkout -b wip/experimental-feature
git commit -m "Experimental: trying new approach"
```

**Important:** Only skip hooks when you have a good reason. They exist to help you!

## Common Scenarios

### Scenario 1: Format Check Fails

```
Cargo Format Check...................................................Failed
- hook id: cargo-fmt
- exit code: 1

Diff in /path/to/file.rs at line 42...
```

**Solution:** Run `cargo fmt` or `just fmt` and re-commit:

```bash
cargo fmt
git add .
git commit -m "Add new feature"
```

### Scenario 2: Clippy Warnings

```
Cargo Clippy.........................................................Failed
- hook id: cargo-clippy

warning: unnecessary clone
  --> src/main.rs:15:10
   |
15 |     let x = y.clone();
   |          ^^^^^^^^^ help: remove this
```

**Solution:** Fix the issue flagged by Clippy:

```bash
# Edit the file to fix the warning
# Then re-commit
git add .
git commit -m "Add new feature"
```

### Scenario 3: Tests Fail

```
Cargo Test...........................................................Failed
- hook id: cargo-test

running 5 tests
test tests::it_works ... FAILED
```

**Solution:** Fix the failing test before committing:

```bash
# Fix the test
cargo test  # Verify locally
git add .
git commit -m "Add new feature"
```

### Scenario 4: Markdown Linting

```
markdownlint.........................................................Failed
- hook id: markdownlint

README.md:42 MD029/ol-prefix Ordered list item prefix
```

**Solution:** The hook auto-fixes many issues. Just add the changes:

```bash
git add README.md
git commit -m "Update documentation"
```

## Configuration Files

- **[.pre-commit-config.yaml](../.pre-commit-config.yaml)** - Main configuration
- **[.markdownlint.yaml](../.markdownlint.yaml)** - Markdown rules

## Customization

### Disable Specific Hooks Temporarily

Edit `.pre-commit-config.yaml` and comment out hooks:

```yaml
# - id: cargo-test  # Disabled temporarily
```

Then update hooks:

```bash
pre-commit install
```

### Add Custom Hooks

Add to the `local` repo section in `.pre-commit-config.yaml`:

```yaml
- repo: local
  hooks:
    - id: my-custom-check
      name: My Custom Check
      entry: ./scripts/custom-check.sh
      language: system
      files: \.rs$
```

## Best Practices

### Do's ✅

- Let hooks run on every commit (build the habit)
- Fix issues immediately when hooks fail
- Run `pre-commit run --all-files` before pushing
- Use `just qa` for a comprehensive local check

### Don'ts ❌

- Don't use `--no-verify` unless absolutely necessary
- Don't commit broken code "to fix later"
- Don't disable hooks permanently without good reason
- Don't bypass hooks to meet arbitrary deadlines

## Learning Benefits

Pre-commit hooks are educational because they:

1. **Provide Immediate Feedback** - Learn what's wrong right away
2. **Enforce Best Practices** - Can't commit bad patterns
3. **Build Muscle Memory** - You'll start writing correct code naturally
4. **Prevent Bad Habits** - Catch issues before they become habits

## Integration with Other Tools

Pre-commit works alongside:

- **[justfile](../justfile)** - `just qa` runs similar checks
- **[VSCode settings](../.vscode/settings.json)** - Format on save
- **[GitHub Actions](../.github/workflows/deploy.yml)** - CI/CD runs same checks
- **[Claude settings](../.claude/settings.json)** - AI suggests compliant code

## Troubleshooting

### Hooks Not Running

```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install

# Check hook is installed
ls -la .git/hooks/pre-commit
```

### Hooks Taking Too Long

```bash
# Set fail_fast: true in .pre-commit-config.yaml
# This stops on first failure instead of running all hooks

# Or skip specific slow hooks
SKIP=cargo-test git commit -m "Quick fix"
```

### Update Hooks to Latest Versions

```bash
# Update to latest hook versions
pre-commit autoupdate

# Then commit the updated .pre-commit-config.yaml
git add .pre-commit-config.yaml
git commit -m "Update pre-commit hooks"
```

### False Positives

If a hook blocks a legitimate commit:

1. Check if there's a real issue first
2. If it's truly a false positive, use `--no-verify` with a comment
3. Consider filing an issue to improve the hook

## Performance Tips

Hooks run fast on incremental changes but can be slow on large changesets:

- **Commit frequently** - Smaller commits = faster hooks
- **Run manually first** - `pre-commit run --all-files` before committing
- **Use `fail_fast`** - Stop on first error instead of running all

## Learning Resources

- [Pre-commit Documentation](https://pre-commit.com/)
- [Git Hooks Guide](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
- [Supported Hooks List](https://pre-commit.com/hooks.html)

## Summary

Pre-commit hooks are your **first line of defense** against committing broken or poorly formatted code. They're especially valuable for learning because they provide instant, actionable feedback.

Think of them as a helpful pair programming partner who always catches your mistakes! 🦀

---

**Quick Start:**

```bash
pip install pre-commit
pre-commit install
git commit -m "Test hooks"  # They run automatically!
```
