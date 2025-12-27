# Rust Learning Notes

This document tracks Rust concepts learned during development, with code examples from this project.

## Core Concepts

### Ownership & Borrowing

**Status:** 🟡 Learning in progress

**Key Rules:**

1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped
3. You can borrow with `&` (immutable) or `&mut` (mutable)
4. Only one mutable borrow OR multiple immutable borrows at a time

**Where to see it:**

- Check ECS queries in Bevy systems (uses borrowing extensively)
- Look at how `Query<&Position>` borrows immutably
- Look at how `Query<&mut Velocity>` borrows mutably

**Resources:**

- [The Rust Book - Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

---

### Trait System

**Status:** 🔴 Not yet explored

**What are traits:**

- Like interfaces in other languages
- Define shared behavior across types
- Enable polymorphism without inheritance

**Where to see it:**

- Component types implement Bevy's `Component` trait
- Protocol messages implement `Serialize` and `Deserialize`

**Resources:**

- [The Rust Book - Chapter 10](https://doc.rust-lang.org/book/ch10-02-traits.html)

---

### Error Handling

**Status:** 🔴 Not yet explored

**Key types:**

- `Result<T, E>` for operations that can fail
- `Option<T>` for values that might not exist
- The `?` operator for error propagation

**Where to see it:**

- Network operations (connection failures)
- File I/O (missing assets)

**Resources:**

- [The Rust Book - Chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

### Concurrency

**Status:** 🔴 Not yet explored

**Key concepts:**

- `Send` trait: can transfer ownership between threads
- `Sync` trait: can share references between threads
- Channels for thread communication
- Arc and Mutex for shared state

**Where to see it:**

- Bevy's parallel ECS systems
- Network thread handling in Lightyear

**Resources:**

- [The Rust Book - Chapter 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

---

### Lifetime Annotations

**Status:** 🔴 Not yet explored

**What are lifetimes:**

- Tell the compiler how long references are valid
- Prevent dangling pointers
- Usually inferred, sometimes need explicit annotations

**When you'll need them:**

- Complex struct definitions with references
- Function signatures with multiple borrowed parameters

**Resources:**

- [The Rust Book - Chapter 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

---

### Pattern Matching

**Status:** 🔴 Not yet explored

**Key syntax:**

- `match` expressions (exhaustive)
- `if let` for simple cases
- Destructuring in function parameters

**Where to see it:**

- Handling different message types in protocol
- Processing game events

**Resources:**

- [The Rust Book - Chapter 6](https://doc.rust-lang.org/book/ch06-00-enums.html)

---

## Bevy-Specific Concepts

### ECS Architecture

**Status:** 🔴 Not yet explored

**Components:**

- Pure data structs
- Implement the `Component` trait

**Systems:**

- Functions that query components
- Run in parallel when possible

**Resources:**

- Queries: `Query<&ComponentA, With<ComponentB>>`
- Commands: Spawn/despawn entities
- Events: Communication between systems

**Resources:**

- [Bevy ECS Guide](https://bevyengine.org/learn/book/getting-started/ecs/)

---

### Bevy Plugins

**Status:** 🔴 Not yet explored

**What are plugins:**

- Modular way to organize systems and resources
- Can be added to App with `add_plugins()`

**Where to see it:**

- Your server and client will likely use custom plugins
- Lightyear provides networking plugins

**Resources:**

- [Bevy Plugins Guide](https://bevyengine.org/learn/book/getting-started/plugins/)

---

## Networking Concepts

### Serialization with Bincode

**Status:** 🔴 Not yet explored

**What is serialization:**

- Converting Rust structs to bytes
- Sending over network
- Deserializing back to structs

**Where to see it:**

- Protocol messages in `shared/src/protocol.rs`
- Lightyear handles this automatically

---

### Client-Side Prediction

**Status:** 🔴 Not yet explored

**Why it matters:**

- Makes game feel responsive despite network latency
- Client simulates actions immediately
- Reconciles with server updates later

**Where to see it:**

- Client movement system runs before server confirms

---

## Performance Concepts

### Zero-Cost Abstractions

**Status:** 🔴 Not yet explored

**Key idea:**

- Rust's abstractions compile down to efficient machine code
- No runtime overhead for things like iterators, generics

**Where to see it:**

- Bevy's ECS queries are highly optimized
- Iterator chains instead of manual loops

---

### Compile-Time Guarantees

**Status:** 🔴 Not yet explored

**What Rust checks:**

- Memory safety (no null pointers, no dangling references)
- Thread safety (no data races)
- Type safety (no unexpected type conversions)

**Why it matters:**

- Bugs caught before runtime
- Confidence in refactoring

---

## Learning Exercises

### Suggested Order

1. ✅ Set up project structure
2. 🔲 Create a simple component (Position)
3. 🔲 Write a system that modifies Position
4. 🔲 Define a protocol message
5. 🔲 Implement basic client-server connection
6. 🔲 Send a message from client to server
7. 🔲 Spawn an entity based on network message
8. 🔲 Implement client-side prediction for movement

### Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Bevy Documentation](https://docs.rs/bevy/latest/bevy/)
- [Lightyear Examples](https://github.com/cBournhonesque/lightyear/tree/main/examples)

---

## Questions to Explore

- How does Bevy parallelize systems safely?
- What's the difference between `&str` and `String`?
- When should I use `Vec<T>` vs `&[T]`?
- How does Lightyear handle packet loss?
- What's the performance impact of cloning vs borrowing?

(Add your own questions as they come up!)
