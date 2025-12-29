# Versioning Strategy

This document defines the versioning strategy for Assignment W-4 following **SemVer 2.0** and **Conventional Commits**.

## Overview

We use a **hybrid versioning approach**:
- **Monorepo version**: Single version for the entire project
- **Service versions**: Individual versions per microservice
- **Image tags**: Docker images tagged with version + commit hash

## Semantic Versioning 2.0

Following [SemVer 2.0](https://semver.org/):

```
MAJOR.MINOR.PATCH+metadata

Example: 1.2.3+abc1234
```

### Version Components

- **MAJOR**: Incompatible API changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)
- **Metadata**: Git commit hash (optional but recommended)

### Version Bumping Rules

| Change Type | Version Bump | Example |
|-------------|--------------|---------|
| Breaking change | MAJOR | 1.0.0 → 2.0.0 |
| New feature | MINOR | 1.0.0 → 1.1.0 |
| Bug fix | PATCH | 1.0.0 → 1.0.1 |
| Documentation | PATCH | 1.0.0 → 1.0.1 |
| Refactor (no API change) | PATCH | 1.0.0 → 1.0.1 |

### Special Versions

- **0.x.x**: Pre-release (anything can change)
- **1.0.0**: First stable release
- **x.y.0-alpha.1**: Alpha release
- **x.y.0-beta.1**: Beta release
- **x.y.0-rc.1**: Release candidate

## Conventional Commits

Following [Conventional Commits 1.0.0](https://www.conventionalcommits.org/):

### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description | Version Bump |
|------|-------------|--------------|
| `feat` | New feature | MINOR |
| `fix` | Bug fix | PATCH |
| `docs` | Documentation only | PATCH |
| `style` | Code style (formatting) | PATCH |
| `refactor` | Code refactor | PATCH |
| `perf` | Performance improvement | PATCH |
| `test` | Add/update tests | PATCH |
| `build` | Build system changes | PATCH |
| `ci` | CI/CD changes | PATCH |
| `chore` | Maintenance tasks | PATCH |
| `revert` | Revert previous commit | PATCH |

### Breaking Changes

**Any type** with `!` or `BREAKING CHANGE:` footer → **MAJOR** bump

```bash
# Example 1: Using !
feat!: change API response format

# Example 2: Using footer
feat: add new authentication endpoint

BREAKING CHANGE: removed legacy /auth endpoint
```

### Scopes

Scopes identify which component changed:

```bash
feat(auth-service): add OAuth2 support
fix(world-state): resolve entity duplication bug
docs(deployment): update ArgoCD setup guide
chore(deps): update bevy to 0.15
```

**Common scopes:**
- Service names: `auth-service`, `world-state`, etc.
- Components: `client`, `shared`, `migrations`
- Infrastructure: `deployment`, `ci`, `docker`
- Cross-cutting: `deps`, `config`, `security`

### Examples

```bash
# New feature (MINOR bump)
feat(world-state): add spatial indexing for entities

# Bug fix (PATCH bump)
fix(auth-service): prevent duplicate user registration

# Breaking change (MAJOR bump)
feat(shared)!: change protocol to use MessagePack

BREAKING CHANGE: Clients must upgrade to new protocol

# Multiple changes in one commit (avoid this)
# Instead: make separate commits per logical change
```

## Versioning Strategies

### Strategy 1: Monorepo Version (Recommended)

**Single version** for entire project, all services share it.

**Advantages:**
- ✅ Simple - one version to track
- ✅ All services released together
- ✅ Easy to see what's compatible
- ✅ Matches monorepo philosophy

**Disadvantages:**
- ❌ All services get version bump even if unchanged
- ❌ Can't deploy services independently at different versions

**Implementation:**

```toml
# Cargo.toml (workspace root)
[workspace.package]
version = "0.1.0"

[workspace.dependencies]
shared = { path = "./shared", version = "0.1.0" }

# services/auth-service/Cargo.toml
[package]
name = "auth-service"
version.workspace = true
```

### Strategy 2: Independent Service Versions

**Separate versions** per service.

**Advantages:**
- ✅ Each service versioned independently
- ✅ Clear which services changed
- ✅ Can deploy different service versions

**Disadvantages:**
- ❌ Complex - track 10+ versions
- ❌ Compatibility matrix needed
- ❌ Shared library version conflicts

**Implementation:**

```toml
# services/auth-service/Cargo.toml
[package]
version = "1.2.3"

# services/world-state/Cargo.toml
[package]
version = "2.0.1"
```

### Strategy 3: Hybrid (Our Choice)

**Workspace version + Git metadata** for traceability.

**Format:**

```
Workspace: 0.1.0
Image tags: 0.1.0+sha-abc1234
           0.1.0+main-abc1234
```

**Advantages:**
- ✅ Simple workspace versioning
- ✅ Git commit hash for exact traceability
- ✅ Can identify which code built which image
- ✅ Works with monorepo + GitOps

**Implementation:**

```toml
# Cargo.toml
[workspace.package]
version = "0.1.0"

# All services inherit workspace version
# Docker tags include commit hash
```

## Automated Versioning

### Using cargo-release

```bash
# Install
cargo install cargo-release

# Dry run (see what would happen)
cargo release patch --dry-run

# Release patch version
cargo release patch --execute

# Release minor version
cargo release minor --execute

# Release with custom version
cargo release 1.0.0 --execute
```

### Using release-plz (Recommended)

[release-plz](https://github.com/MarcoIeni/release-plz) automates versioning based on Conventional Commits.

```bash
# Install
cargo install release-plz

# Generate changelog and determine next version
release-plz update

# Create GitHub release
release-plz release
```

**GitHub Actions integration:**

```yaml
# .github/workflows/release.yaml
name: Release

on:
  push:
    branches:
      - main

jobs:
  release-plz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Run release-plz
        uses: MarcoIeni/release-plz-action@v0.5
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**How it works:**
1. Scans commits since last release
2. Analyzes Conventional Commits
3. Determines version bump (MAJOR/MINOR/PATCH)
4. Updates Cargo.toml versions
5. Generates CHANGELOG.md
6. Creates Git tag
7. Creates GitHub release

## Docker Image Tagging

### Tag Strategy

```
ghcr.io/USERNAME/SERVICE:TAG
```

**Tags to create:**
- `latest` - Latest build from main
- `0.1.0` - Semantic version
- `0.1.0+sha-abc1234` - Version + commit
- `main-abc1234` - Branch + commit
- `pr-123` - Pull request number

### GitHub Actions Implementation

```yaml
# In build-deploy.yaml
- name: Extract metadata
  id: meta
  uses: docker/metadata-action@v5
  with:
    images: ghcr.io/${{ github.repository_owner }}/${{ matrix.service }}
    tags: |
      # Semantic version from Cargo.toml
      type=semver,pattern={{version}}
      type=semver,pattern={{major}}.{{minor}}

      # Git commit hash
      type=sha,prefix={{branch}}-,format=short

      # Branch name
      type=ref,event=branch

      # PR number
      type=ref,event=pr

      # Latest (only on main)
      type=raw,value=latest,enable={{is_default_branch}}
```

**Result:**

```
ghcr.io/yourname/auth-service:0.1.0
ghcr.io/yourname/auth-service:0.1
ghcr.io/yourname/auth-service:main-abc1234
ghcr.io/yourname/auth-service:latest
```

## Version Management Workflow

### Development Workflow

```bash
# 1. Make changes
git checkout -b feat/add-oauth

# 2. Commit with conventional commits
git commit -m "feat(auth-service): add OAuth2 authentication"

# 3. Push and create PR
git push origin feat/add-oauth
gh pr create

# 4. Merge to main
# GitHub Actions automatically:
#   - Analyzes commits
#   - Bumps version (0.1.0 → 0.2.0)
#   - Updates CHANGELOG.md
#   - Creates Git tag
#   - Builds Docker images with new version
#   - Deploys via ArgoCD
```

### Manual Release

```bash
# 1. Ensure you're on main
git checkout main
git pull

# 2. Run release-plz
release-plz update

# 3. Review changes
git diff

# 4. Commit version bump
git commit -am "chore: release v0.2.0"
git tag v0.2.0
git push origin main --tags

# 5. CI/CD handles the rest
```

## Version Information at Runtime

### Embed Version in Binary

```toml
# Cargo.toml
[package.metadata.version]
git-commit-hash = true
build-timestamp = true
```

```rust
// build.rs
fn main() {
    // Embed git commit hash
    println!("cargo:rustc-env=GIT_HASH={}",
        std::process::Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .unwrap()
            .stdout
            .as_slice()
    );

    // Embed build timestamp
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}",
        chrono::Utc::now().to_rfc3339()
    );
}

// src/main.rs
const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: &str = env!("GIT_HASH");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

#[derive(Serialize)]
struct VersionInfo {
    version: String,
    git_hash: String,
    build_timestamp: String,
}

// Expose via health endpoint
async fn version() -> Json<VersionInfo> {
    Json(VersionInfo {
        version: VERSION.to_string(),
        git_hash: GIT_HASH.to_string(),
        build_timestamp: BUILD_TIMESTAMP.to_string(),
    })
}

// GET /version
// {
//   "version": "0.1.0",
//   "git_hash": "abc1234",
//   "build_timestamp": "2024-01-26T12:00:00Z"
// }
```

### Version Compatibility Checks

```rust
// shared/src/protocol.rs
pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub protocol_version: u32,
    pub client_version: String,
}

// Server validates client version
if handshake.protocol_version != PROTOCOL_VERSION {
    return Err("Incompatible protocol version");
}
```

## CHANGELOG Generation

Using [git-cliff](https://git-cliff.org/) or release-plz:

```toml
# cliff.toml
[changelog]
header = "# Changelog\n\nAll notable changes to this project will be documented in this file.\n"
body = """
{% for group, commits in commits | group_by(attribute="group") %}
    ### {{ group | upper_first }}
    {% for commit in commits %}
        - {{ commit.message | upper_first }} ([{{ commit.id | truncate(length=7, end="") }}]({{ commit.link }}))\
    {% endfor %}
{% endfor %}
"""

[git]
conventional_commits = true
filter_unconventional = true
commit_parsers = [
    { message = "^feat", group = "Features" },
    { message = "^fix", group = "Bug Fixes" },
    { message = "^doc", group = "Documentation" },
    { message = "^perf", group = "Performance" },
    { message = "^refactor", group = "Refactor" },
    { message = "^style", group = "Styling" },
    { message = "^test", group = "Testing" },
    { message = "^chore\\(release\\): prepare for", skip = true },
    { message = "^chore", group = "Miscellaneous Tasks" },
    { body = ".*security", group = "Security" },
]
```

**Generated CHANGELOG.md:**

```markdown
# Changelog

## [0.2.0] - 2024-01-26

### Features
- Add OAuth2 authentication (abc1234)
- Implement spatial indexing (def5678)

### Bug Fixes
- Prevent duplicate user registration (ghi9012)

### Performance
- Optimize entity queries (jkl3456)

## [0.1.0] - 2024-01-20

### Features
- Initial microservices architecture
```

## Pre-commit Hook for Conventional Commits

```bash
# Install commitizen
pip install commitizen

# Add to .pre-commit-config.yaml
- repo: https://github.com/commitizen-tools/commitizen
  rev: v3.13.0
  hooks:
    - id: commitizen
```

Or use [cocogitto](https://github.com/cocogitto/cocogitto):

```bash
cargo install cocogitto

# .cog.toml
[changelog]
path = "CHANGELOG.md"
template = "remote"
remote = "github.com"
repository = "assignment-W-4"
owner = "yourname"

# Check commit messages
cog check

# Generate changelog
cog changelog
```

## Version Strategy Summary

| Aspect | Strategy |
|--------|----------|
| **Monorepo version** | Single version (workspace.package.version) |
| **Service versions** | Inherit workspace version |
| **Docker tags** | version+sha (0.1.0+abc1234) |
| **Git tags** | v0.1.0 (SemVer 2.0) |
| **Commits** | Conventional Commits 1.0.0 |
| **Automation** | release-plz or cargo-release |
| **Changelog** | Auto-generated from commits |
| **Runtime info** | Embedded in binary via build.rs |

## Tools Installation

```bash
# Core tools
cargo install cargo-release
cargo install release-plz
cargo install git-cliff
cargo install cocogitto

# Commit helpers
pip install commitizen
npm install -g @commitlint/cli @commitlint/config-conventional
```

## Next Steps

1. Choose automation tool (release-plz recommended)
2. Set up GitHub Actions for automated releases
3. Configure pre-commit hooks for commit validation
4. Start using Conventional Commits
5. Automate CHANGELOG generation

See [DATABASE_STRATEGY.md](DATABASE_STRATEGY.md) for database migrations.
