//! Creusot Verification Example
//!
//! Demonstrates using Creusot deductive verifier contracts with elicitation.
//! Creusot uses Why3 theorem prover for formal verification.
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example verification_creusot_example --features verification,verify-creusot
//! ```
//!
//! # Verifying with Creusot
//!
//! ```bash
//! # Requires Creusot toolchain installed
//! cargo creusot --features verify-creusot
//! ```

#![cfg(all(feature = "verification", feature = "verify-creusot"))]

use elicitation::verification::contracts::creusot::{
    CreusotI32Positive, CreusotOptionIsSome, CreusotResultIsOk, CreusotStringNonEmpty,
    CreusotVecNonEmpty,
};
use elicitation::verification::Contract;

fn main() {
    println!("=== Creusot Verification Example ===\n");

    // Example 1: String contracts
    println!("1. String Contracts (Creusot)");
    let hello = String::from("hello");
    println!("   Input: {:?}", hello);
    println!(
        "   CreusotStringNonEmpty precondition: {}",
        CreusotStringNonEmpty::requires(&hello)
    );
    println!(
        "   CreusotStringNonEmpty postcondition: {}",
        CreusotStringNonEmpty::ensures(&hello, &hello)
    );
    println!();

    // Example 2: Positive integer contracts
    println!("2. Positive Integer Contracts (Creusot)");
    let positive = 42i32;
    let negative = -1i32;
    println!("   Positive input: {}", positive);
    println!(
        "   CreusotI32Positive precondition: {}",
        CreusotI32Positive::requires(&positive)
    );
    println!("   Negative input: {}", negative);
    println!(
        "   CreusotI32Positive precondition: {}",
        CreusotI32Positive::requires(&negative)
    );
    println!();

    // Example 3: Option contracts
    println!("3. Option<T> Contracts (Creusot)");
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    println!(
        "   Some(42): {}",
        CreusotOptionIsSome::<i32>::requires(&some_value)
    );
    println!(
        "   None: {}",
        CreusotOptionIsSome::<i32>::requires(&none_value)
    );
    println!();

    // Example 4: Result contracts
    println!("4. Result<T, E> Contracts (Creusot)");
    let ok_value: Result<i32, String> = Ok(42);
    let err_value: Result<i32, String> = Err("error".to_string());
    println!(
        "   Ok(42): {}",
        CreusotResultIsOk::<i32, String>::requires(&ok_value)
    );
    println!(
        "   Err(\"error\"): {}",
        CreusotResultIsOk::<i32, String>::requires(&err_value)
    );
    println!();

    // Example 5: Vec contracts
    println!("5. Vec<T> Contracts (Creusot)");
    let non_empty_vec = vec![1, 2, 3];
    let empty_vec: Vec<i32> = vec![];
    println!(
        "   vec![1, 2, 3]: {}",
        CreusotVecNonEmpty::<i32>::requires(&non_empty_vec)
    );
    println!(
        "   vec![]: {}",
        CreusotVecNonEmpty::<i32>::requires(&empty_vec)
    );
    println!();

    println!("=== Why Choose Creusot? ===\n");
    println!("✓ Deductive verification: Proves complex mathematical properties");
    println!("✓ Why3 integration: Access to multiple SMT solvers");
    println!("✓ Functional correctness: Verify algorithms are correct");
    println!("✓ Specification language: Expressive contracts with logic");
    println!("✓ Sound: If verified, guarantees hold");
    println!();

    println!("=== Creusot Limitations ===\n");
    println!("✗ Requires annotations: Must add #[requires], #[ensures]");
    println!("✗ Learning curve: Need to understand separation logic");
    println!("✗ Complex setup: Requires Why3, OpCaml, SMT solvers");
    println!("✗ Slow: Proof search can be time-consuming");
    println!();

    println!("=== Creusot Verification Attributes ===\n");
    println!("In actual Creusot verification, you would annotate:");
    println!();
    println!("  #[requires(input.len() > 0)]");
    println!("  #[ensures(result.len() > 0)]");
    println!("  fn process(input: String) -> String {{ ... }}");
    println!();
    println!("Creusot compiles to WhyML and proves properties via Why3.");
}
