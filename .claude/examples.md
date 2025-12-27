# Claude Settings Examples

This document shows how the `.claude/settings.json` configuration affects Claude's responses.

## Example 1: Error Handling

### ❌ Without Settings (Common Mistake)

```rust
fn read_config(path: &str) -> Config {
    let content = fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
}
```

### ✅ With Settings (Recommended)

```rust
fn read_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)
        .map_err(ConfigError::IoError)?;

    let config = serde_json::from_str(&content)
        .map_err(ConfigError::ParseError)?;

    Ok(config)
}

#[derive(Debug)]
enum ConfigError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
}
```

**Why this is better:**

- No panics - errors are handled gracefully
- Caller can decide how to handle errors
- Errors are descriptive and typed
- Follows Rust's Result-based error handling idiom

---

## Example 2: Ownership & Borrowing

### ❌ Without Settings (Inefficient)

```rust
fn process_player_name(player: Player) -> String {
    let name = player.name.clone();
    format!("Player: {}", name)
}
```

### ✅ With Settings (Idiomatic)

```rust
fn process_player_name(player: &Player) -> String {
    format!("Player: {}", player.name)
}
```

**Why this is better:**

- Borrows instead of taking ownership
- No unnecessary cloning
- More flexible for callers
- Zero-cost abstraction

**Learning Note:** The settings will explain when to use `&T`, `&mut T`, or `T`.

---

## Example 3: Bevy ECS Components

### ❌ Without Settings (Anti-pattern)

```rust
#[derive(Component)]
struct Player {
    position: Vec3,
    velocity: Vec3,
    health: f32,
    inventory: Vec<Item>,
    ai_state: Option<AIController>,
}
```

### ✅ With Settings (Idiomatic Bevy)

```rust
#[derive(Component)]
struct Position(Vec3);

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Inventory(Vec<Item>);

#[derive(Component)]
struct AIController {
    state: AIState,
}
```

**Why this is better:**

- Fine-grained components enable better query patterns
- Systems can query only what they need
- Better cache locality
- Easier to add/remove behavior

**Learning Note:** The settings enforce "components are pure data" principle.

---

## Example 4: Iterator Chains vs Loops

### ❌ Without Settings (Imperative)

```rust
fn sum_positive_evens(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        if num > 0 && num % 2 == 0 {
            sum += num;
        }
    }
    sum
}
```

### ✅ With Settings (Functional)

```rust
fn sum_positive_evens(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n > 0 && n % 2 == 0)
        .sum()
}
```

**Why this is better:**

- More declarative and easier to read
- Composable and chainable
- Optimized by the compiler
- Idiomatic Rust

---

## Example 5: Network Input Validation

### ❌ Without Settings (Dangerous)

```rust
fn handle_move_input(input: MoveInput) {
    player.position += input.delta;
}
```

### ✅ With Settings (Secure)

```rust
fn handle_move_input(input: MoveInput) -> Result<(), ValidationError> {
    // Server is authority - validate everything
    if input.delta.length() > MAX_MOVE_DISTANCE {
        return Err(ValidationError::MoveTooFar);
    }

    if !is_valid_position(player.position + input.delta) {
        return Err(ValidationError::InvalidPosition);
    }

    player.position += input.delta;
    Ok(())
}
```

**Why this is better:**

- Prevents cheating
- Server validates all client input
- Clear error types
- Follows "never trust the client" principle

---

## Example 6: String Handling

### ❌ Without Settings (Inefficient in Loops)

```rust
fn format_names(names: &[String]) -> String {
    let mut result = String::new();
    for name in names {
        result = result + &format!("{}, ", name);  // Allocates each iteration!
    }
    result
}
```

### ✅ With Settings (Efficient)

```rust
fn format_names(names: &[String]) -> String {
    names
        .iter()
        .map(|name| name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

// Or with capacity hint:
fn format_names_optimized(names: &[String]) -> String {
    let capacity = names.iter().map(|s| s.len()).sum::<usize>() + names.len() * 2;
    let mut result = String::with_capacity(capacity);

    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(name);
    }
    result
}
```

**Why this is better:**

- Avoids repeated allocations
- Pre-allocates when possible
- More idiomatic

---

## Example 7: Documentation

### ❌ Without Settings

```rust
// Gets player health
fn get_health(player: &Player) -> f32 {
    player.health
}
```

### ✅ With Settings

```rust
/// Returns the current health value of the player.
///
/// # Examples
///
/// ```
/// let player = Player::new();
/// assert_eq!(player.health(), 100.0);
/// ```
fn health(&self) -> f32 {
    self.health
}
```

**Why this is better:**

- Proper doc comments (///)
- Includes example
- Documents public API
- Testable via doc tests

---

## How Claude Uses These Settings

When you ask Claude to write code, it will:

1. **Suggest the idiomatic version first**
2. **Explain why it's better**
3. **Link to relevant documentation**
4. **Compare with common mistakes**
5. **Update your learning docs**

For example:

**Your request:** "Add a function to move a player"

**Claude's response will:**

- Use proper error handling (Result type)
- Borrow instead of clone where possible
- Follow Bevy ECS patterns if applicable
- Validate inputs for network code
- Include documentation
- Suggest where to add tests
- Explain the Rust concepts used
- Link to relevant sections of The Rust Book

This ensures every interaction is a learning opportunity! 🦀
