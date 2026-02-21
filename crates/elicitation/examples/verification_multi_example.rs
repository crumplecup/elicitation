//! Multi-Verifier Contract Example
//!
//! Demonstrates runtime verifier swapping and contract refinement workflows.
//! Shows how to use different verifiers for different needs.
//!
//! # Running This Example
//!
//! ```bash
//! cargo run --example verification_multi_example --features verification
//! ```

#![cfg(feature = "verification")]

use elicitation::verification::contracts::{I32Positive, StringNonEmpty, VecNonEmpty};
use elicitation::verification::{Contract, DynContract, VerifierBackend, compose};

fn main() {
    println!("=== Multi-Verifier Contract Example ===\n");

    // Example 1: Runtime verifier selection
    println!("1. Runtime Verifier Selection");
    let value = 42i32;

    let kani_backend =
        VerifierBackend::Kani(Box::new(I32Positive) as Box<dyn DynContract<i32, i32>>);
    println!(
        "   Kani backend precondition: {}",
        kani_backend.check_precondition(&value)
    );

    #[cfg(feature = "verify-prusti")]
    {
        use elicitation::verification::contracts::prusti::PrustiI32Positive;
        let prusti_backend =
            VerifierBackend::Prusti(Box::new(PrustiI32Positive) as Box<dyn DynContract<i32, i32>>);
        println!(
            "   Prusti backend precondition: {}",
            prusti_backend.check_precondition(&value)
        );
    }

    #[cfg(feature = "verify-verus")]
    {
        use elicitation::verification::contracts::verus::VerusI32Positive;
        let verus_backend =
            VerifierBackend::Verus(Box::new(VerusI32Positive) as Box<dyn DynContract<i32, i32>>);
        println!(
            "   Verus backend precondition: {}",
            verus_backend.check_precondition(&value)
        );
    }
    println!();

    // Example 2: Contract refinement workflow
    println!("2. Contract Refinement Workflow");
    println!("   Step 1: Start with Kani (fast feedback)");
    let hello = String::from("hello");
    println!("      StringNonEmpty: {}", StringNonEmpty::requires(&hello));

    println!("   Step 2: Refine with composition");
    let composed = compose::and(StringNonEmpty, StringNonEmpty);
    println!(
        "      Composed contract: {}",
        composed.check_requires(&hello)
    );

    println!("   Step 3: Switch to stronger verifier for production");
    #[cfg(feature = "verify-prusti")]
    {
        );
    }
    println!();

    // Example 3: Choosing the right verifier
    println!("3. Choosing the Right Verifier");
    println!();
    println!("   🔹 Development Phase:");
    println!("      Use Kani - Fast feedback, easy setup");
    println!("      Verify basic properties quickly");
    println!();
    println!("   🔹 Testing Phase:");
    println!("      Use Prusti - Memory safety, ownership");
    println!("      Catch Rust-specific issues");
    println!();
    println!("   🔹 Production Phase:");
    println!("      Use Creusot or Verus - Formal guarantees");
    println!("      Prove correctness of critical code");
    println!();

    // Example 4: Contract composition patterns
    println!("4. Contract Composition Patterns");

    let vec = vec![1, 2, 3];

    println!("   AND: Both must hold");
    let vec_contract1 = VecNonEmpty::<i32>::new();
    let vec_contract2 = VecNonEmpty::<i32>::new();
    let and_contract = compose::and(vec_contract1, vec_contract2);
    println!("      Result: {}", and_contract.check_requires(&vec));

    println!("   OR: Either can hold");
    let vec_contract3 = VecNonEmpty::<i32>::new();
    let vec_contract4 = VecNonEmpty::<i32>::new();
    let or_contract = compose::or(vec_contract3, vec_contract4);
    println!("      Result: {}", or_contract.check_requires(&vec));

    println!("   NOT: Inverts the contract");
    let vec_contract5 = VecNonEmpty::<i32>::new();
    let not_contract = compose::not(vec_contract5);
    println!("      Result: {}", not_contract.check_requires(&vec));
    println!();

    // Example 5: Migration strategy
    println!("5. Migration Strategy");
    println!();
    println!("   Phase 1: Add default contracts (Kani)");
    println!("      ✓ Quick to add");
    println!("      ✓ Immediate value");
    println!("      ✓ No dependencies");
    println!();
    println!("   Phase 2: Identify critical paths");
    println!("      ✓ Performance-critical code");
    println!("      ✓ Security-sensitive code");
    println!("      ✓ Bug-prone algorithms");
    println!();
    println!("   Phase 3: Upgrade critical contracts");
    println!("      ✓ Switch to Creusot/Prusti/Verus");
    println!("      ✓ Add stronger properties");
    println!("      ✓ Verify edge cases");
    println!();
    println!("   Phase 4: Continuous verification");
    println!("      ✓ Run in CI/CD");
    println!("      ✓ Prevent regressions");
    println!("      ✓ Document guarantees");
    println!();

    println!("=== Verifier Comparison ===\n");
    println!("┌────────────┬────────┬──────────┬────────────┬─────────────┐");
    println!("│ Verifier   │ Speed  │ Setup    │ Coverage   │ Use Case    │");
    println!("├────────────┼────────┼──────────┼────────────┼─────────────┤");
    println!("│ Kani       │ ⚡⚡⚡  │ Easy     │ Symbolic   │ Development │");
    println!("│ Prusti     │ ⚡⚡    │ Medium   │ Ownership  │ Testing     │");
    println!("│ Creusot    │ ⚡     │ Hard     │ Functional │ Critical    │");
    println!("│ Verus      │ ⚡⚡    │ Hard     │ SMT-based  │ Research    │");
    println!("└────────────┴────────┴──────────┴────────────┴─────────────┘");
    println!();

    println!("=== Best Practices ===\n");
    println!("✓ Start simple: Use Kani defaults first");
    println!("✓ Compose contracts: Build complex from simple");
    println!("✓ Test contracts: Verify they catch bugs");
    println!("✓ Document choices: Explain why each verifier");
    println!("✓ Gradual adoption: Don't try to verify everything");
    println!("✓ Profile verification: Measure time cost");
    println!();
}
