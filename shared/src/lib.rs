//! Shared Library
//!
//! This crate contains all code that must be identical between client and server.
//! This includes:
//! - Protocol definitions (network messages)
//! - Shared components (ECS data structures)
//! - Shared systems (deterministic game logic)
//! - Physics constants and utilities
//!
//! # Learning Note
//! Rust's module system uses `pub mod` to declare modules and `pub use` to
//! re-export items. This creates a clean public API for your library.

// Enable stricter lints to learn best practices
#![warn(clippy::all)]
#![warn(missing_docs)]

// Declare modules
pub mod components;
pub mod physics;
pub mod protocol;
pub mod systems;

// Re-export commonly used items for convenience
// As you develop, you'll add items here like:
// pub use components::{Position, Velocity};
// pub use protocol::{PlayerInput, ServerMessage};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // Keep this basic test, add more as you develop
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
