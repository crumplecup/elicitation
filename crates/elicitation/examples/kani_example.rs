//! Example demonstrating Kani contract verification for tool chains.
//!
//! This example shows how to:
//! 1. Define tools with formal contracts
//! 2. Compose tools into chains
//! 3. Verify chain compatibility with Kani
//!
//! Run with:
//! ```bash
//! cargo run --example kani_example --features verification
//! ```
//!
//! Verify with Kani (proofs in elicitation_kani crate):
//! ```bash
//! cargo kani -p elicitation_kani --all-features
//! ```

#[cfg(not(feature = "verification"))]
fn main() {
    eprintln!("This example requires the verification feature.");
    eprintln!("Run with: cargo run --example kani_example --features verification");
}

#[cfg(feature = "verification")]
mod kani_enabled {

    // Domain types and implementations would go here
    // For now, just a simple main that exits
    pub fn run_main() {
        println!("Kani example - feature enabled but implementation TBD");
    }
}

#[cfg(feature = "verification")]
fn main() {
    kani_enabled::run_main();
}
