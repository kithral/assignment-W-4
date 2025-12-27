//! Physics Constants and Shared Logic
//!
//! Defines physics constants and helper functions that must be identical
//! on both client and server for deterministic physics simulation.
//!
//! # Learning Note
//! Constants in Rust use `const` (compile-time) or `static` (runtime).
//! For game physics, compile-time constants are preferred for performance.

/// Maximum velocity for any entity (units per second)
pub const MAX_VELOCITY: f32 = 500.0;

/// Gravity constant (if applicable)
pub const GRAVITY: f32 = 9.81;

/// Physics timestep (fixed update rate in seconds)
pub const PHYSICS_TIMESTEP: f32 = 1.0 / 60.0;

// TODO: Add physics helper functions here
