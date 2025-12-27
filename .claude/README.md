# Claude Settings for Assignment W-4

This directory contains Claude Code configuration for this project.

## Configuration Files

### `settings.json`

This file configures Claude's behavior for this project with:

- **Rust Best Practices** - Idiomatic code, ownership patterns, error handling
- **Bevy ECS Guidelines** - Component design, system organization, performance
- **Networking Standards** - Server authority, validation, bandwidth optimization
- **Learning Mode** - Explanations, comparisons, and educational resources
- **Code Review Rules** - What to check, what to avoid, what to encourage
- **Quality Standards** - Testing, documentation, security practices

## How It Works

When working in this project, Claude will:

1. **Follow Rust Idioms** - Suggest idiomatic Rust code patterns
2. **Explain Decisions** - Explain why certain approaches are recommended
3. **Catch Common Mistakes** - Flag unwrap(), cloning, unsafe usage
4. **Enforce Best Practices** - Validate input, handle errors properly, optimize carefully
5. **Support Learning** - Link to resources, explain concepts, compare alternatives

## Key Settings

### Code Quality

- Clippy warnings treated as errors
- Rustfmt enforced
- No `unwrap()` in production code
- Prefer borrowing over cloning
- Comprehensive error handling

### Bevy ECS

- Components are pure data
- Systems are stateless
- Use queries efficiently
- Consider system ordering
- Organize with plugins

### Networking

- Server is always authoritative
- Validate all client input
- Handle packet loss gracefully
- Minimize bandwidth usage
- Implement client-side prediction

### Learning Focus

- Explain Rust concepts when they arise
- Compare alternative approaches
- Link to documentation and resources
- Highlight idioms and anti-patterns
- Update `docs/rust-learnings.md` regularly

## Customization

To modify Claude's behavior for this project, edit `settings.json`. The configuration uses these sections:

- `projectInfo` - Basic project metadata
- `codeStyle` - Language and style preferences
- `bestPractices` - Domain-specific guidelines
- `codeReview` - What to check during reviews
- `learningMode` - Educational features
- `antiPatterns` - Patterns to avoid
- `encouragedPatterns` - Patterns to use

## Examples of What This Enforces

### ✅ Good (Encouraged)

```rust
// Borrowing instead of cloning
fn process_data(data: &str) -> Result<String, Error> {
    // Proper error handling
    let parsed = data.parse()?;
    Ok(format!("Processed: {}", parsed))
}

// Idiomatic iterators
let sum: i32 = numbers.iter().filter(|&&x| x > 0).sum();
```

### ❌ Bad (Flagged)

```rust
// Unnecessary cloning
fn process_data(data: String) -> String {
    // Using unwrap (could panic)
    let parsed = data.parse().unwrap();
    format!("Processed: {}", parsed)
}

// Inefficient loops
let mut sum = 0;
for i in 0..numbers.len() {
    if numbers[i] > 0 {
        sum += numbers[i];
    }
}
```

## Integration with Development Workflow

This configuration integrates with:

- **VS Code** - Rust-analyzer settings align with these standards
- **Clippy** - Configuration matches `.clippy.toml`
- **Rustfmt** - Style aligns with `rustfmt.toml`
- **Justfile** - `just qa` enforces these standards
- **Git Hooks** - Can be configured to check standards pre-commit

## Learning Resources Referenced

The settings will suggest resources from:

- The Rust Book
- Rust by Example
- Bevy Documentation
- Lightyear Examples
- Rust API Guidelines
- Clippy Lint Documentation

## Maintenance

This configuration should be updated when:

- New Rust patterns are learned and adopted
- Project architecture evolves
- New best practices are discovered
- Dependencies change significantly
- Team conventions are established

---

These settings ensure Claude provides high-quality, educational assistance tailored to your Rust learning journey. 🦀
