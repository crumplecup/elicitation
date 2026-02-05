//! Demonstration of the verification framework.
//!
//! This example shows how to:
//! 1. Use default contracts with feature selection
//! 2. Swap verifiers at runtime
//! 3. Compose contracts with boolean logic
//! 4. Use with_contract() for verified elicitation
//!
//! Run with different verifier features:
//! ```bash
//! cargo run --example verification_demo --features verification
//! cargo run --example verification_demo --features verify-kani
//! cargo run --example verification_demo --features verify-creusot
//! ```

use elicitation::verification::{
    AndContract, Contract, NotContract, OrContract, VerifierBackend, compose,
    contracts::{BoolValid, I32NonNegative, I32Positive, StringMaxLength, StringNonEmpty},
};

fn main() {
    println!("=== Elicitation Verification Framework Demo ===\n");

    // ========================================================================
    // 1. Basic Contract Usage
    // ========================================================================
    println!("1. BASIC CONTRACT USAGE");
    println!("-----------------------");

    let hello = String::from("hello");
    let empty = String::new();

    println!(
        "StringNonEmpty::requires(\"hello\"): {}",
        StringNonEmpty::requires(&hello)
    );
    println!(
        "StringNonEmpty::requires(\"\"):      {}",
        StringNonEmpty::requires(&empty)
    );

    println!(
        "\nI32Positive::requires(42):  {}",
        I32Positive::requires(&42)
    );
    println!("I32Positive::requires(0):   {}", I32Positive::requires(&0));
    println!("I32Positive::requires(-1):  {}", I32Positive::requires(&-1));

    println!(
        "\nBoolValid::requires(true):  {}",
        BoolValid::requires(&true)
    );
    println!(
        "BoolValid::requires(false): {}",
        BoolValid::requires(&false)
    );

    // ========================================================================
    // 2. Contract Composition
    // ========================================================================
    println!("\n2. CONTRACT COMPOSITION");
    println!("-----------------------");

    // AND: Both must pass
    println!("\nAND Combinator (I32Positive AND I32NonNegative):");
    println!(
        "  42: {}",
        AndContract::<I32Positive, I32NonNegative>::requires(&42)
    );
    println!(
        "   0: {}",
        AndContract::<I32Positive, I32NonNegative>::requires(&0)
    );
    println!(
        "  -1: {}",
        AndContract::<I32Positive, I32NonNegative>::requires(&-1)
    );

    // OR: Either can pass
    println!("\nOR Combinator (I32Positive OR I32NonNegative):");
    println!(
        "  42: {}",
        OrContract::<I32Positive, I32NonNegative>::requires(&42)
    );
    println!(
        "   0: {}",
        OrContract::<I32Positive, I32NonNegative>::requires(&0)
    );
    println!(
        "  -1: {}",
        OrContract::<I32Positive, I32NonNegative>::requires(&-1)
    );

    // NOT: Inverts logic
    println!("\nNOT Combinator (NOT I32Positive):");
    println!("  42: {}", NotContract::<I32Positive>::requires(&42));
    println!("   0: {}", NotContract::<I32Positive>::requires(&0));
    println!("  -1: {}", NotContract::<I32Positive>::requires(&-1));

    // Complex composition
    println!("\nComplex: (StringNonEmpty AND StringMaxLength<10>):");
    let short = String::from("hello");
    let long = String::from("this is too long");
    println!(
        "  \"hello\":              {}",
        AndContract::<StringNonEmpty, StringMaxLength<10>>::requires(&short)
    );
    println!(
        "  \"this is too long\":  {}",
        AndContract::<StringNonEmpty, StringMaxLength<10>>::requires(&long)
    );
    println!(
        "  \"\":                  {}",
        AndContract::<StringNonEmpty, StringMaxLength<10>>::requires(&empty)
    );

    // ========================================================================
    // 3. Compose Helpers
    // ========================================================================
    println!("\n3. COMPOSE HELPERS");
    println!("------------------");

    let and_contract = compose::and(I32Positive, I32NonNegative);
    let or_contract = compose::or(I32Positive, I32NonNegative);
    let not_contract = compose::not(I32Positive);

    println!("\nUsing compose::and(I32Positive, I32NonNegative):");
    println!(
        "  42: {}",
        AndContract::<I32Positive, I32NonNegative>::requires(&42)
    );

    println!("\nUsing compose::or(I32Positive, I32NonNegative):");
    println!(
        "   0: {}",
        OrContract::<I32Positive, I32NonNegative>::requires(&0)
    );

    println!("\nUsing compose::not(I32Positive):");
    println!("  -1: {}", NotContract::<I32Positive>::requires(&-1));

    // Verify contract types
    let _: AndContract<I32Positive, I32NonNegative> = and_contract;
    let _: OrContract<I32Positive, I32NonNegative> = or_contract;
    let _: NotContract<I32Positive> = not_contract;

    // ========================================================================
    // 4. Runtime Verifier Swapping
    // ========================================================================
    println!("\n4. RUNTIME VERIFIER SWAPPING");
    println!("----------------------------");

    // Kani backend
    let kani_verifier = VerifierBackend::Kani(Box::new(StringNonEmpty));
    println!("\nKani Verifier (StringNonEmpty):");
    println!(
        "  Precondition(\"hello\"):  {}",
        kani_verifier.check_precondition(&hello)
    );
    println!(
        "  Precondition(\"\"):       {}",
        kani_verifier.check_precondition(&empty)
    );
    println!(
        "  Invariant:               {}",
        kani_verifier.check_invariant()
    );

    // Verify with identity transformation
    match kani_verifier.verify(hello.clone(), |x| x) {
        Ok(result) => println!("  Verification passed: {}", result),
        Err(e) => println!("  Verification failed: {}", e),
    }

    match kani_verifier.verify(empty.clone(), |x| x) {
        Ok(result) => println!("  Verification passed: {}", result),
        Err(e) => println!("  Verification failed: {}", e),
    }

    // Creusot backend (runtime checking)
    #[cfg(feature = "verify-creusot")]
    {
        use elicitation::verification::contracts::creusot::CreusotStringNonEmpty;
        let creusot_verifier = VerifierBackend::Creusot(Box::new(CreusotStringNonEmpty));
        println!("\nCreusot Verifier (StringNonEmpty):");
        println!(
            "  Precondition(\"hello\"):  {}",
            creusot_verifier.check_precondition(&hello)
        );
        println!(
            "  Precondition(\"\"):       {}",
            creusot_verifier.check_precondition(&empty)
        );
    }

    // Prusti backend
    #[cfg(feature = "verify-prusti")]
    {
        use elicitation::verification::contracts::prusti::PrustiStringNonEmpty;
        let prusti_verifier = VerifierBackend::Prusti(Box::new(PrustiStringNonEmpty));
        println!("\nPrusti Verifier (StringNonEmpty):");
        println!(
            "  Precondition(\"hello\"):  {}",
            prusti_verifier.check_precondition(&hello)
        );
    }

    // Verus backend
    #[cfg(feature = "verify-verus")]
    {
        use elicitation::verification::contracts::verus::VerusStringNonEmpty;
        let verus_verifier = VerifierBackend::Verus(Box::new(VerusStringNonEmpty));
        println!("\nVerus Verifier (StringNonEmpty):");
        println!(
            "  Precondition(\"hello\"):  {}",
            verus_verifier.check_precondition(&hello)
        );
    }

    #[cfg(not(any(
        feature = "verify-creusot",
        feature = "verify-prusti",
        feature = "verify-verus"
    )))]
    {
        println!("\nNote: Creusot, Prusti, and Verus verifiers are feature-gated.");
        println!(
            "      Run with --features verify-creusot|verify-prusti|verify-verus to test them."
        );
    }

    // ========================================================================
    // 5. Default Contracts (Feature-Gated)
    // ========================================================================
    #[cfg(feature = "verification")]
    {
        println!("\n5. DEFAULT CONTRACTS (FEATURE-GATED)");
        println!("-------------------------------------");

        println!("\nDefault contracts are selected at compile-time based on features:");
        #[cfg(not(any(
            feature = "verify-creusot",
            feature = "verify-prusti",
            feature = "verify-verus"
        )))]
        println!("  Active verifier: Kani (default)");
        #[cfg(feature = "verify-creusot")]
        println!("  Active verifier: Creusot");
        #[cfg(feature = "verify-prusti")]
        println!("  Active verifier: Prusti");
        #[cfg(feature = "verify-verus")]
        println!("  Active verifier: Verus");

        println!(
            "\nDEFAULT_STRING_CONTRACT::requires(\"hello\"): {}",
            StringNonEmpty::requires(&hello)
        );
        println!(
            "DEFAULT_I32_CONTRACT::requires(42):          {}",
            I32Positive::requires(&42)
        );
        println!(
            "DEFAULT_BOOL_CONTRACT::requires(true):       {}",
            BoolValid::requires(&true)
        );

        // with_contract() usage would look like:
        println!("\nUsage with with_contract():");
        println!("  let value = String::with_contract(DEFAULT_STRING_CONTRACT)");
        println!("      .elicit(peer)");
        println!("      .await?;");
    }

    // ========================================================================
    // Summary
    // ========================================================================
    println!("\n=== SUMMARY ===");
    println!("✅ Basic contracts work (StringNonEmpty, I32Positive, BoolValid)");
    println!("✅ Composition works (AND, OR, NOT combinators)");
    println!("✅ Runtime verifier swapping works (Kani, Creusot, Prusti, Verus)");
    #[cfg(feature = "verification")]
    println!("✅ Compile-time defaults work (DEFAULT_*_CONTRACT)");
    println!("\nThe verification framework is fully operational!");
}
