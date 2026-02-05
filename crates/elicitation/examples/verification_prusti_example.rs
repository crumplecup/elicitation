//! Prusti verification example - Requires verify-prusti feature to run.

#[cfg(not(feature = "verify-prusti"))]
fn main() {
    eprintln!("This example requires the verify-prusti feature.");
    eprintln!("Run with: cargo run --example verification_prusti_example --features verify-prusti");
}

#[cfg(feature = "verify-prusti")]
fn main() {
    println!("Prusti verification example - implementation TBD");
}
