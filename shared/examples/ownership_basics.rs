//! Ownership Basics Example
//!
//! This example demonstrates Rust's ownership rules.
//! Run with: cargo run --example ownership_basics --package shared

fn main() {
    println!("=== Ownership Basics ===\n");

    // OWNERSHIP RULE 1: Each value has exactly one owner
    let s1 = String::from("hello");
    println!("s1 owns the string: {}", s1);

    // This MOVES ownership from s1 to s2
    let s2 = s1;
    println!("s2 now owns the string: {}", s2);

    // Uncommenting this line would cause a compile error:
    // println!("s1: {}", s1);  // ERROR: value borrowed after move

    println!("\n=== Borrowing (References) ===\n");

    // Borrowing with & creates a reference without taking ownership
    let s3 = String::from("world");
    let len = calculate_length(&s3); // Borrow s3
    println!("The length of '{}' is {}.", s3, len); // s3 still valid!

    println!("\n=== Mutable References ===\n");

    let mut s4 = String::from("hello");
    change_string(&mut s4);
    println!("Changed string: {}", s4);

    println!("\n=== Copy vs Move ===\n");

    // Simple types like integers implement Copy, so they're copied, not moved
    let x = 5;
    let y = x; // Copy, not move
    println!("x: {}, y: {} (both still valid!)", x, y);
}

// This function borrows a string (doesn't take ownership)
fn calculate_length(s: &str) -> usize {
    s.len()
} // s goes out of scope, but since we don't own it, nothing happens

// This function takes a mutable reference
fn change_string(s: &mut String) {
    s.push_str(", world");
}
