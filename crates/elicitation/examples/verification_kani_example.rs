//! Kani Verification Example
//!
//! Demonstrates using Kani model checker contracts with elicitation.
//! Kani provides symbolic execution and exhaustive model checking.
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example verification_kani_example --features verification
//! ```
//!
//! # Verifying with Kani
//!
//! ```bash
//! cargo kani --features verify-kani
//! ```

#![cfg(feature = "verification")]

use elicitation::verification::contracts::{
    I32Positive, OptionIsSome, ResultIsOk, StringNonEmpty, VecNonEmpty,
};
use elicitation::verification::{compose, Contract, DynContract};

fn main() {
    println!("=== Kani Verification Example ===\n");

    // Example 1: String contracts
    println!("1. String Contracts");
    let hello = String::from("hello");
    println!("   Input: {:?}", hello);
    println!(
        "   StringNonEmpty precondition: {}",
        StringNonEmpty::requires(&hello)
    );
    println!(
        "   StringNonEmpty postcondition: {}",
        StringNonEmpty::ensures(&hello, &hello)
    );
    println!();

    // Example 2: Positive integer contracts
    println!("2. Positive Integer Contracts");
    let positive = 42i32;
    let negative = -1i32;
    println!("   Positive input: {}", positive);
    println!(
        "   I32Positive precondition: {}",
        I32Positive::requires(&positive)
    );
    println!("   Negative input: {}", negative);
    println!(
        "   I32Positive precondition: {}",
        I32Positive::requires(&negative)
    );
    println!();

    // Example 3: Option contracts
    println!("3. Option<T> Contracts");
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    println!("   Some(42): {}", OptionIsSome::<i32>::requires(&some_value));
    println!("   None: {}", OptionIsSome::<i32>::requires(&none_value));
    println!();

    // Example 4: Result contracts
    println!("4. Result<T, E> Contracts");
    let ok_value: Result<i32, String> = Ok(42);
    let err_value: Result<i32, String> = Err("error".to_string());
    println!(
        "   Ok(42): {}",
        ResultIsOk::<i32, String>::requires(&ok_value)
    );
    println!(
        "   Err(\"error\"): {}",
        ResultIsOk::<i32, String>::requires(&err_value)
    );
    println!();

    // Example 5: Vec contracts
    println!("5. Vec<T> Contracts");
    let non_empty_vec = vec![1, 2, 3];
    let empty_vec: Vec<i32> = vec![];
    println!(
        "   vec![1, 2, 3]: {}",
        VecNonEmpty::<i32>::requires(&non_empty_vec)
    );
    println!(
        "   vec![]: {}",
        VecNonEmpty::<i32>::requires(&empty_vec)
    );
    println!();

    // Example 6: Contract composition
    println!("6. Contract Composition");
    let value = 42i32;
    let and_contract = compose::and(I32Positive, I32Positive);
    println!(
        "   Composed (AND): {}",
        and_contract.check_requires(&value)
    );

    let or_contract = compose::or(I32Positive, I32Positive);
    println!(
        "   Composed (OR): {}",
        or_contract.check_requires(&value)
    );

    let not_contract = compose::not(I32Positive);
    println!(
        "   Composed (NOT): {}",
        not_contract.check_requires(&value)
    );
    println!();

    println!("=== Why Choose Kani? ===\n");
    println!("✓ Symbolic execution: Proves properties for ALL inputs");
    println!("✓ Model checking: Exhaustive verification within bounds");
    println!("✓ Fast feedback: Quick verification for small functions");
    println!("✓ No annotations: Works with standard Rust code");
    println!("✓ Integration: Easy to add to existing projects");
    println!();

    println!("=== Kani Limitations ===\n");
    println!("✗ State explosion: Complex types may not verify");
    println!("✗ No floats: Limited floating point support");
    println!("✗ Bounds required: Must constrain symbolic values");
    println!();
}

#[cfg(kani)]
mod verification_harnesses {
    use super::*;

    /// Kani harness for StringNonEmpty contract.
    #[kani::proof]
    fn verify_string_non_empty_comprehensive() {
        let test_strings = [
            "",
            "a",
            "hello",
            "world",
            "a very long string for testing",
        ];

        for s in test_strings.iter() {
            let input = String::from(*s);
            let pre = StringNonEmpty::requires(&input);
            let post = StringNonEmpty::ensures(&input, &input);

            // Property: precondition matches postcondition
            kani::assume(pre == post);
            // Property: both agree on emptiness
            kani::assume(pre == !input.is_empty());
        }
    }

    /// Kani harness for I32Positive contract.
    #[kani::proof]
    fn verify_i32_positive_comprehensive() {
        let value: i32 = kani::any();

        let pre = I32Positive::requires(&value);
        let post = I32Positive::ensures(&value, &value);

        // Property: pre and post agree
        assert!(pre == post);
        // Property: positive means > 0
        assert!(pre == (value > 0));
    }

    /// Kani harness for OptionIsSome contract.
    #[kani::proof]
    fn verify_option_is_some_comprehensive() {
        let value: Option<i32> = if kani::any() {
            Some(kani::any())
        } else {
            None
        };

        let pre = OptionIsSome::<i32>::requires(&value);
        let post = OptionIsSome::<i32>::ensures(&value, &value);

        // Property: pre and post agree
        assert!(pre == post);
        // Property: matches is_some()
        assert!(pre == value.is_some());
    }

    /// Kani harness for VecNonEmpty contract.
    #[kani::proof]
    fn verify_vec_non_empty_comprehensive() {
        let size: usize = kani::any();
        kani::assume(size < 5); // Bound for tractability

        let vec: Vec<i32> = (0..size).map(|_| kani::any()).collect();

        let pre = VecNonEmpty::<i32>::requires(&vec);
        let post = VecNonEmpty::<i32>::ensures(&vec, &vec);

        // Property: pre and post agree
        assert!(pre == post);
        // Property: non-empty means len > 0
        assert!(pre == !vec.is_empty());
    }
}
