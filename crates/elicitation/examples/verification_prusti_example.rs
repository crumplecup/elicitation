//! Prusti Verification Example
//!
//! Demonstrates using Prusti separation logic contracts with elicitation.
//! Prusti uses Viper for separation logic verification.
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example verification_prusti_example --features verification,verify-prusti
//! ```
//!
//! # Verifying with Prusti
//!
//! ```bash
//! # Requires Prusti toolchain installed
//! cargo prusti --features verify-prusti
//! ```

#![cfg(all(feature = "verification", feature = "verify-prusti"))]

use elicitation::verification::Contract;
use elicitation::verification::contracts::prusti::{
    PrustiI32Positive, PrustiOptionIsSome, PrustiResultIsOk, PrustiStringNonEmpty,
    PrustiVecNonEmpty,
};

fn main() {
    println!("=== Prusti Verification Example ===\n");

    // Example 1: String contracts
    println!("1. String Contracts (Prusti)");
    let hello = String::from("hello");
    println!("   Input: {:?}", hello);
    println!(
        "   PrustiStringNonEmpty precondition: {}",
        PrustiStringNonEmpty::requires(&hello)
    );
    println!(
        "   PrustiStringNonEmpty postcondition: {}",
        PrustiStringNonEmpty::ensures(&hello, &hello)
    );
    println!();

    // Example 2: Positive integer contracts
    println!("2. Positive Integer Contracts (Prusti)");
    let positive = 42i32;
    let negative = -1i32;
    println!("   Positive input: {}", positive);
    println!(
        "   PrustiI32Positive precondition: {}",
        PrustiI32Positive::requires(&positive)
    );
    println!("   Negative input: {}", negative);
    println!(
        "   PrustiI32Positive precondition: {}",
        PrustiI32Positive::requires(&negative)
    );
    println!();

    // Example 3: Option contracts
    println!("3. Option<T> Contracts (Prusti)");
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    println!(
        "   Some(42): {}",
        PrustiOptionIsSome::<i32>::requires(&some_value)
    );
    println!(
        "   None: {}",
        PrustiOptionIsSome::<i32>::requires(&none_value)
    );
    println!();

    // Example 4: Result contracts
    println!("4. Result<T, E> Contracts (Prusti)");
    let ok_value: Result<i32, String> = Ok(42);
    let err_value: Result<i32, String> = Err("error".to_string());
    println!(
        "   Ok(42): {}",
        PrustiResultIsOk::<i32, String>::requires(&ok_value)
    );
    println!(
        "   Err(\"error\"): {}",
        PrustiResultIsOk::<i32, String>::requires(&err_value)
    );
    println!();

    // Example 5: Vec contracts
    println!("5. Vec<T> Contracts (Prusti)");
    let non_empty_vec = vec![1, 2, 3];
    let empty_vec: Vec<i32> = vec![];
    println!(
        "   vec![1, 2, 3]: {}",
        PrustiVecNonEmpty::<i32>::requires(&non_empty_vec)
    );
    println!(
        "   vec![]: {}",
        PrustiVecNonEmpty::<i32>::requires(&empty_vec)
    );
    println!();

    println!("=== Why Choose Prusti? ===\n");
    println!("✓ Separation logic: Verify memory safety and ownership");
    println!("✓ Automatic inference: Less annotations than Creusot");
    println!("✓ Rust-specific: Designed for Rust's ownership model");
    println!("✓ Viper backend: Mature verification infrastructure");
    println!("✓ Error messages: Good feedback on verification failures");
    println!();

    println!("=== Prusti Limitations ===\n");
    println!("✗ Incomplete: Not all Rust features supported");
    println!("✗ Requires attributes: Must add #[pure], #[requires]");
    println!("✗ Complex setup: Requires Java, Viper, Z3");
    println!("✗ Slow compilation: Verification adds significant time");
    println!();

    println!("=== Prusti Verification Attributes ===\n");
    println!("In actual Prusti verification, you would annotate:");
    println!();
    println!("  #[pure]");
    println!("  fn is_positive(x: i32) -> bool {{ x > 0 }}");
    println!();
    println!("  #[requires(is_positive(input))]");
    println!("  #[ensures(is_positive(result))]");
    println!("  fn increment(input: i32) -> i32 {{ input + 1 }}");
    println!();
    println!("Prusti uses Viper to verify separation logic properties.");
}
