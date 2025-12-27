# Scripting Language Research

Research document for choosing an embedded scripting language for in-world programming.

## Requirements

### Must Have

1. **Safety** - Cannot crash the server or escape sandbox
2. **Rust Integration** - Clean FFI or native Rust implementation
3. **Performance** - Fast enough for real-time game logic
4. **Sandboxing** - Limit access to system resources
5. **Interruptibility** - Can timeout long-running scripts

### Nice to Have

1. **Familiar Syntax** - Either Rust-like or widely known
2. **Hot Reload** - Reload scripts without restart
3. **Debugging** - Stack traces, error messages
4. **Standard Library** - Common data structures and functions
5. **Documentation** - Good learning resources

## Options

### 1. Rhai

**Website:** https://rhai.rs/

**Overview:**

- Native Rust scripting language
- Designed specifically for embedding in Rust apps
- Syntax similar to Rust + JavaScript

**Example Code:**

```rhai
// Rhai script for a magic door
fn on_use(player) {
    if player.has_item("ancient_key") {
        emit("The door swings open!");
        teleport(player, "secret_room");
        return true;
    } else {
        emit("The door is locked.");
        return false;
    }
}
```

**Pros:**

- ✅ **Native Rust** - No FFI needed, pure Rust
- ✅ **Sandboxed by default** - No file I/O or network access
- ✅ **Interruptible** - Built-in operation limits
- ✅ **Type-safe** - Rust types map cleanly
- ✅ **Hot reload** - Scripts are just strings
- ✅ **Good docs** - Well-documented for Rust devs
- ✅ **Active development** - Regular updates

**Cons:**

- ❌ **Newer ecosystem** - Less mature than Lua
- ❌ **Custom syntax** - Players must learn it
- ❌ **No JIT** - Interpreted only (but still fast)

**Performance:**

- ~100-500ns per simple operation
- AOT compilation planned for future

**Integration Complexity:** ⭐⭐⭐⭐⭐ (Excellent)

**Recommendation:** **Best choice for Rust-first development**

---

### 2. Lua (via mlua)

**Website:** https://github.com/mlua-rs/mlua

**Overview:**

- Industry-standard scripting language
- Used in World of Warcraft, Roblox, many games
- LuaJIT provides excellent performance

**Example Code:**

```lua
-- Lua script for a magic door
function on_use(player)
    if player:has_item("ancient_key") then
        emit("The door swings open!")
        teleport(player, "secret_room")
        return true
    else
        emit("The door is locked.")
        return false
    end
end
```

**Pros:**

- ✅ **Proven** - Decades of game industry use
- ✅ **Fast** - LuaJIT is extremely fast (JIT compilation)
- ✅ **Familiar** - Many programmers know Lua
- ✅ **Sandboxing** - Well-understood sandbox techniques
- ✅ **Ecosystem** - Lots of libraries and tools
- ✅ **mlua** - Excellent Rust bindings

**Cons:**

- ❌ **FFI overhead** - Crossing Rust/Lua boundary
- ❌ **Different syntax** - Not Rust-like
- ❌ **Manual sandboxing** - Must disable io, os modules
- ❌ **GC pauses** - Garbage collection can cause hitches

**Performance:**

- ~10-50ns per operation with LuaJIT
- Faster than Rhai for compute-heavy tasks

**Integration Complexity:** ⭐⭐⭐⭐ (Good)

**Recommendation:** **Best choice for performance-critical scripting**

---

### 3. JavaScript (via Boa/Deno)

**Website:** https://github.com/boa-dev/boa

**Overview:**

- Full ECMAScript engine in Rust
- Familiar syntax for web developers

**Example Code:**

```javascript
// JavaScript for a magic door
function on_use(player) {
    if (player.hasItem("ancient_key")) {
        emit("The door swings open!");
        teleport(player, "secret_room");
        return true;
    } else {
        emit("The door is locked.");
        return false;
    }
}
```

**Pros:**

- ✅ **Extremely familiar** - Most know JavaScript
- ✅ **Native Rust** - Boa is pure Rust
- ✅ **Modern features** - async/await, classes, etc.
- ✅ **Good tooling** - VSCode, linters exist

**Cons:**

- ❌ **Heavy** - Large engine, slow startup
- ❌ **Complex** - Full spec is overkill for scripting
- ❌ **Immature** - Boa not production-ready
- ❌ **Performance** - Slower than Lua or Rhai

**Performance:**

- ~1-10µs per operation
- Not suitable for hot paths

**Integration Complexity:** ⭐⭐⭐ (Moderate)

**Recommendation:** **Not recommended** - too heavy for game scripting

---

### 4. Python (via PyO3)

**Website:** https://github.com/PyO3/pyo3

**Overview:**

- Full Python interpreter embedded in Rust
- Very familiar to many programmers

**Example Code:**

```python
# Python for a magic door
def on_use(player):
    if player.has_item("ancient_key"):
        emit("The door swings open!")
        teleport(player, "secret_room")
        return True
    else:
        emit("The door is locked.")
        return False
```

**Pros:**

- ✅ **Very familiar** - Extremely popular language
- ✅ **Rich ecosystem** - Lots of libraries
- ✅ **Easy to learn** - Simple syntax

**Cons:**

- ❌ **GIL** - Global interpreter lock limits concurrency
- ❌ **Heavy** - Full Python runtime
- ❌ **Sandboxing** - Very difficult to sandbox properly
- ❌ **Performance** - Much slower than alternatives
- ❌ **Deployment** - Need Python installed

**Performance:**

- ~10-100µs per operation
- Not suitable for real-time

**Integration Complexity:** ⭐⭐ (Difficult)

**Recommendation:** **Not recommended** - too slow and heavy

---

### 5. Custom DSL

**Overview:**

- Build a domain-specific language from scratch
- Tailored exactly to our needs

**Example Code:**

```
# Custom DSL for a magic door
on use by @player:
    if @player has "ancient_key":
        emit "The door swings open!"
        teleport @player to "secret_room"
        return true
    else:
        emit "The door is locked."
        return false
```

**Pros:**

- ✅ **Perfect fit** - Exactly what we need, no more
- ✅ **Simple** - No complex features we don't use
- ✅ **Learning** - Best learning opportunity
- ✅ **Controlled** - We control every aspect

**Cons:**

- ❌ **Time** - Significant development time
- ❌ **Bugs** - We must fix all issues
- ❌ **Features** - Must implement everything ourselves
- ❌ **Unfamiliar** - Players must learn our syntax
- ❌ **Tooling** - No IDE support, linters, etc.

**Performance:**

- Depends on implementation
- Could be very fast if well-designed

**Integration Complexity:** ⭐ (Very difficult)

**Recommendation:** **Phase 2 consideration** - start with proven solution first

---

## Comparison Matrix

| Feature | Rhai | Lua | JavaScript | Python | Custom DSL |
|---------|------|-----|------------|--------|------------|
| **Rust Integration** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐ | ⭐⭐⭐⭐⭐ |
| **Safety** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Familiarity** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐ |
| **Sandboxing** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐ | ⭐⭐⭐⭐⭐ |
| **Dev Time** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐ |
| **Ecosystem** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐ |
| **Learning Value** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐⭐ |

## Recommendations

### For This Project: **Rhai**

**Why:**

1. **Best Rust Integration** - Native, no FFI overhead
2. **Safety First** - Sandboxed by default, perfect for learning
3. **Good Performance** - Fast enough for game logic
4. **Learning Opportunity** - Teaches language embedding in Rust
5. **Simplicity** - Easy to get started

**Trade-offs:**

- Less familiar than Lua to most users
- Slightly slower than LuaJIT for heavy compute
- Newer ecosystem

**Implementation Plan:**

```rust
// Example Rhai integration in Rust
use rhai::{Engine, Scope};

fn main() {
    let engine = Engine::new();
    let mut scope = Scope::new();

    // Register Rust functions for scripts to call
    engine.register_fn("emit", |msg: &str| {
        println!("Script says: {}", msg);
    });

    engine.register_fn("teleport", |player: Player, room: &str| {
        // Teleport logic
    });

    // Run user script
    let script = r#"
        fn on_use(player) {
            if player.has_item("key") {
                emit("Door opens!");
                teleport(player, "next_room");
            }
        }
    "#;

    engine.eval::<()>(script)?;
}
```

### Alternative: **Lua** (if performance becomes critical)

Consider switching to Lua if:

- Scripts become a performance bottleneck
- Users demand LuaJIT speed
- We need the mature ecosystem

## Next Steps

1. **Prototype with Rhai**
   - Add `rhai` to `Cargo.toml`
   - Create basic script execution system
   - Expose core API to scripts

2. **Design Script API**
   - What functions can scripts call?
   - What data can scripts access?
   - How are entities represented?

3. **Build Sandbox**
   - Set operation limits
   - Restrict dangerous operations
   - Add timeout handling

4. **Create Examples**
   - Door example
   - NPC behavior example
   - Custom command example

5. **Document for Users**
   - Script syntax guide
   - API reference
   - Best practices

## Learning Resources

### Rhai

- Official Book: https://rhai.rs/book/
- Examples: https://github.com/rhaiscript/rhai/tree/master/examples
- Playground: https://rhai.rs/playground/

### Lua (if needed later)

- mlua Book: https://github.com/mlua-rs/mlua
- Programming in Lua: https://www.lua.org/pil/
- Lua 5.4 Reference: https://www.lua.org/manual/5.4/

### Language Implementation

- Crafting Interpreters: https://craftinginterpreters.com/
- Writing An Interpreter In Go: https://interpreterbook.com/

---

**Decision:** Start with **Rhai** for its Rust-first design, safety guarantees, and learning value. Evaluate Lua later if performance requirements demand it.
