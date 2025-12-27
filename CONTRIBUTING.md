# Contributing to Assignment W-4

This document outlines the development workflow and standards for this learning project.

## Learning Philosophy

This project prioritizes **learning Rust** over shipping features quickly. Take time to:

- Understand concepts deeply before implementing
- Experiment with different approaches
- Document discoveries in `docs/rust-learnings.md`
- Ask questions and explore edge cases

## Code Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing (enforced by rustfmt.toml)
- Pass `cargo clippy` with no warnings
- Add comments explaining **why**, not **what** (code should be self-documenting for "what")

### Documentation

- Use `///` doc comments for public APIs
- Use `//!` module-level docs to explain module purpose
- Include examples in doc comments when helpful
- Keep `docs/rust-learnings.md` updated with new concepts

### Testing

- Write unit tests for shared logic in `shared/`
- Add integration tests in workspace `tests/` directory
- Test edge cases and error conditions
- Aim for meaningful tests, not just coverage percentages

### Commit Messages

```
Brief summary (50 chars or less)

More detailed explanatory text, if necessary. Wrap at 72 characters.
Explain the problem this commit solves and why this approach was chosen.

Can include multiple paragraphs.

- Bullet points are okay too
- Use present tense: "Add feature" not "Added feature"
```

## Development Workflow

### Before Starting Work

1. Pull latest changes: `git pull`
2. Ensure project builds: `just check`
3. Create a feature branch: `git checkout -b feature/descriptive-name`

### While Working

1. Make incremental commits
2. Run `just watch` to get immediate feedback
3. Document learning moments in `docs/rust-learnings.md`

### Before Committing

1. Run quality checks: `just qa`
   - This runs clippy, fmt, and tests
2. Review your changes: `git diff`
3. Stage changes: `git add -p` (interactive staging)
4. Commit with descriptive message

### Before Pushing

1. Ensure all tests pass: `just test`
2. Ensure clean build: `just build`
3. Push changes: `git push`

## Code Review Checklist

When reviewing your own code (or having others review):

- [ ] Does it compile without warnings?
- [ ] Does it pass clippy without warnings?
- [ ] Is it formatted with rustfmt?
- [ ] Are there tests for new functionality?
- [ ] Is the code idiomatic Rust?
- [ ] Are error cases handled properly?
- [ ] Is unsafe code justified and documented?
- [ ] Are dependencies necessary and up-to-date?
- [ ] Is documentation clear and accurate?

## Project-Specific Guidelines

### Shared Crate (`shared/`)

- **Purpose:** Code that MUST be identical on client and server
- **Rule:** Never use platform-specific code here
- **Rule:** All types must be serializable (for networking)
- **Rule:** Systems must be deterministic (same input = same output)

### Server Crate (`server/`)

- **Purpose:** Authoritative game logic
- **Rule:** Server is always right (validate all client input)
- **Rule:** No graphics dependencies (headless)
- **Rule:** Optimize for correctness over performance initially

### Client Crate (`client/`)

- **Purpose:** Player-facing interface
- **Rule:** Never trust client predictions (server reconciliation)
- **Rule:** Optimize for smooth UX (interpolation, prediction)
- **Rule:** Handle network errors gracefully

## Learning Exercises

### Good First Tasks

1. Add a simple component (e.g., `Health`, `Team`)
2. Write a system that operates on components
3. Add a test for shared logic
4. Create an example in `shared/examples/`

### Intermediate Tasks

1. Implement a new protocol message
2. Add client-side prediction for a feature
3. Write integration tests
4. Profile performance and optimize hot paths

### Advanced Tasks

1. Implement entity interpolation
2. Add compression to network protocol
3. Implement server reconciliation
4. Set up distributed tracing

## Common Mistakes to Avoid

### Rust-Specific

- Don't clone unnecessarily (prefer borrowing)
- Don't use `unwrap()` in production code (handle errors properly)
- Don't fight the borrow checker (redesign if it's painful)
- Don't use `unsafe` without strong justification

### Bevy-Specific

- Don't access the same resource mutably in different systems
- Don't perform I/O in hot-path systems
- Don't forget to register components/events with the app

### Networking-Specific

- Don't trust client input without validation
- Don't send more data than necessary
- Don't assume reliable delivery (even with "reliable" channels)
- Don't ignore network latency in design

## Resources

### Learning Rust

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings (exercises)](https://github.com/rust-lang/rustlings)

### Bevy Engine

- [Official Bevy Book](https://bevyengine.org/learn/book/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)
- [Bevy Discord](https://discord.gg/bevy)

### Networking

- [Lightyear Documentation](https://github.com/cBournhonesque/lightyear)
- [Gaffer on Games (networking articles)](https://gafferongames.com/)

## Questions?

This is a learning project - asking questions is encouraged!

- Check `docs/` for existing documentation
- Search closed issues for similar questions
- Open an issue for discussion
- Document the answer once you find it

Happy learning! 🦀
