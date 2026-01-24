//! Verus Verification Example
//!
//! Demonstrates using Verus SMT-based contracts with elicitation.
//! Verus uses Z3 SMT solver for verification.
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example verification_verus_example --features verification,verify-verus
//! ```
//!
//! # Verifying with Verus
//!
//! ```bash
//! # Requires Verus toolchain installed
//! verus --features verify-verus
//! ```

#![cfg(all(feature = "verification", feature = "verify-verus"))]

use elicitation::verification::Contract;
use elicitation::verification::contracts::verus::{
    VerusI32Positive, VerusOptionIsSome, VerusResultIsOk, VerusStringNonEmpty, VerusVecNonEmpty,
};

fn main() {
    println!("=== Verus Verification Example ===\n");

    // Example 1: String contracts
    println!("1. String Contracts (Verus)");
    let hello = String::from("hello");
    println!("   Input: {:?}", hello);
    println!(
        "   VerusStringNonEmpty precondition: {}",
        VerusStringNonEmpty::requires(&hello)
    );
    println!(
        "   VerusStringNonEmpty postcondition: {}",
        VerusStringNonEmpty::ensures(&hello, &hello)
    );
    println!();

    // Example 2: Positive integer contracts
    println!("2. Positive Integer Contracts (Verus)");
    let positive = 42i32;
    let negative = -1i32;
    println!("   Positive input: {}", positive);
    println!(
        "   VerusI32Positive precondition: {}",
        VerusI32Positive::requires(&positive)
    );
    println!("   Negative input: {}", negative);
    println!(
        "   VerusI32Positive precondition: {}",
        VerusI32Positive::requires(&negative)
    );
    println!();

    // Example 3: Option contracts
    println!("3. Option<T> Contracts (Verus)");
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    println!(
        "   Some(42): {}",
        VerusOptionIsSome::<i32>::requires(&some_value)
    );
    println!(
        "   None: {}",
        VerusOptionIsSome::<i32>::requires(&none_value)
    );
    println!();

    // Example 4: Result contracts
    println!("4. Result<T, E> Contracts (Verus)");
    let ok_value: Result<i32, String> = Ok(42);
    let err_value: Result<i32, String> = Err("error".to_string());
    println!(
        "   Ok(42): {}",
        VerusResultIsOk::<i32, String>::requires(&ok_value)
    );
    println!(
        "   Err(\"error\"): {}",
        VerusResultIsOk::<i32, String>::requires(&err_value)
    );
    println!();

    // Example 5: Vec contracts
    println!("5. Vec<T> Contracts (Verus)");
    let non_empty_vec = vec![1, 2, 3];
    let empty_vec: Vec<i32> = vec![];
    println!(
        "   vec![1, 2, 3]: {}",
        VerusVecNonEmpty::<i32>::requires(&non_empty_vec)
    );
    println!(
        "   vec![]: {}",
        VerusVecNonEmpty::<i32>::requires(&empty_vec)
    );
    println!();

    println!("=== Why Choose Verus? ===\n");
    println!("✓ SMT-based: Powerful automated reasoning via Z3");
    println!("✓ Fast: Generally faster than deductive verification");
    println!("✓ Ghost code: Separate spec and proof from exec code");
    println!("✓ Modes: spec/proof/exec separation for clarity");
    println!("✓ Microsoft-backed: Active development and support");
    println!();

    println!("=== Verus Limitations ===\n");
    println!("✗ Limited Rust: Subset of Rust language supported");
    println!("✗ Requires rewrite: Need to structure code for Verus");
    println!("✗ New toolchain: Separate verus command, not cargo");
    println!("✗ Documentation: Still evolving, fewer examples");
    println!();

    println!("=== Verus Verification Modes ===\n");
    println!("In actual Verus verification, you would structure:");
    println!();
    println!("  spec fn is_positive(x: i32) -> bool {{");
    println!("      x > 0");
    println!("  }}");
    println!();
    println!("  exec fn increment(input: i32) -> (result: i32)");
    println!("      requires(is_positive(input))");
    println!("      ensures(is_positive(result))");
    println!("  {{");
    println!("      input + 1");
    println!("  }}");
    println!();
    println!("Verus separates specification (spec), proof, and execution (exec).");
}
