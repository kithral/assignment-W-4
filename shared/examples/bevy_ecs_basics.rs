//! Bevy ECS Basics Example
//!
//! This example demonstrates Bevy's Entity Component System.
//! Run with: cargo run --example bevy_ecs_basics --package shared

use bevy::prelude::*;

// Define components (pure data)
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement_system, print_positions_system))
        .run();
}

// Startup system - runs once
fn setup(mut commands: Commands) {
    println!("=== Setting up entities ===\n");

    // Spawn an entity with multiple components
    commands.spawn((
        Name("Player".to_string()),
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 0.5 },
    ));

    commands.spawn((
        Name("Enemy".to_string()),
        Position { x: 10.0, y: 5.0 },
        Velocity { x: -0.5, y: 0.2 },
    ));

    // Entity with only position (no velocity - won't move)
    commands.spawn((
        Name("Static Object".to_string()),
        Position { x: 5.0, y: 5.0 },
    ));
}

// System that updates positions based on velocities
fn movement_system(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

// System that prints all positions
fn print_positions_system(query: Query<(&Name, &Position)>) {
    for (name, pos) in &query {
        println!("{}: ({:.1}, {:.1})", name.0, pos.x, pos.y);
    }
    println!();
}
