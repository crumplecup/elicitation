//! Proof-carrying composition primitives.
//!
//! This module provides a minimal type-based contract system for building
//! verified agent programs. Contracts are zero-cost proof markers that enable
//! composing elicitation steps with machine-checked guarantees.
//!
//! # Core Concepts
//!
//! - **Proposition (`Prop`)**: A type-level statement that can be true or false
//! - **Proof (`Established<P>`)**: Evidence that proposition P holds
//! - **Inhabitation (`Is<T>`)**: The proposition that a value inhabits type T
//!
//! # Example
//!
//! ```rust
//! use elicitation::contracts::{Prop, Established, Is};
//! use std::marker::PhantomData;
//!
//! // A proposition: value inhabits String
//! type StringValid = Is<String>;
//!
//! // Function that requires proof
//! fn use_validated_string(
//!     s: String,
//!     _proof: Established<StringValid>
//! ) {
//!     println!("Processing: {}", s);
//! }
//!
//! // Establish proof after validation
//! let s = String::from("hello");
//! let proof = Established::assert();
//! use_validated_string(s, proof);
//! ```
//!
//! # Design Principles
//!
//! - **Zero runtime cost**: All proofs are `PhantomData` and disappear at compile time
//! - **Minimal logic**: Just what's needed for composition (no quantifiers, no negation)
//! - **Type-safe composition**: Cannot call functions without required proofs
//! - **Monotonic refinement**: Guarantees accumulate, never weaken unexpectedly
//!
//! # When to Use
//!
//! Use contracts when:
//! - Building multi-step agent flows with dependencies between steps
//! - Enforcing preconditions that must be established by prior steps
//! - Verifying that validation happens before use (no re-validation needed)
//!
//! Don't use contracts when:
//! - Single-step elicitation (just use `.elicit()` directly)
//! - No dependencies between steps
//! - Performance is so critical you can't afford any abstraction (though cost is zero!)

use std::marker::PhantomData;

/// Marker trait: types that represent propositions.
///
/// A proposition is a type-level statement that can be true or false.
/// Propositions are combined using logical operators (`And`, `Implies`)
/// and witnessed by `Established<P>` proofs.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::{Prop, Is};
///
/// // Built-in proposition: value inhabits type T
/// type StringProp = Is<String>;
/// ```
pub trait Prop: 'static {}

/// Witness that proposition P has been established.
///
/// This is a zero-sized proof marker. Its existence proves that
/// proposition P holds in the current context.
///
/// # Zero Cost
///
/// ```rust
/// use elicitation::contracts::{Established, Is};
/// use std::mem::size_of;
///
/// let proof: Established<Is<String>> = Established::assert();
/// assert_eq!(size_of::<Established<Is<String>>>(), 0);
/// ```
///
/// # Safety Model
///
/// `Established<P>` is a semantic contract, not a memory safety guarantee.
/// Calling `Established::assert()` asserts that P is true. The type system
/// ensures you cannot call functions requiring proof without providing it,
/// but it's your responsibility to only assert when P actually holds.
pub struct Established<P: Prop> {
    _marker: PhantomData<fn() -> P>,
}

impl<P: Prop> Established<P> {
    /// Assert that proposition P holds.
    ///
    /// This is a semantic assertion - the caller asserts that P is true
    /// in the current context. Typically called by elicitation internals
    /// after successful validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use elicitation::contracts::{Established, Is};
    ///
    /// // After validating a URL
    /// let url = url::Url::parse("https://example.com").unwrap();
    /// let proof: Established<Is<url::Url>> = Established::assert();
    /// ```
    #[inline(always)]
    pub fn assert() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

// Make Established copyable (it's zero-sized)
impl<P: Prop> Copy for Established<P> {}
impl<P: Prop> Clone for Established<P> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Proposition: value inhabits type T with its invariants.
///
/// `Is<T>` represents the statement "a value of type T exists and
/// satisfies T's type invariants". For contract types (like `StringNonEmpty`),
/// this includes the contract's preconditions.
///
/// # Examples
///
/// ```rust
/// use elicitation::contracts::Is;
///
/// // Proposition: a valid String exists
/// type StringValid = Is<String>;
///
/// // Proposition: a non-empty String exists (with contract)
/// // type StringNonEmptyValid = Is<StringNonEmpty>;
/// ```
pub struct Is<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T: 'static> Prop for Is<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_established_is_zero_sized() {
        let proof: Established<Is<String>> = Established::assert();
        assert_eq!(std::mem::size_of_val(&proof), 0);
    }

    #[test]
    fn test_established_is_copy() {
        let proof: Established<Is<String>> = Established::assert();
        let proof2 = proof; // Copy
        let _proof3 = proof; // Can still use original
        let _proof4 = proof2; // Can use copy
    }

    #[test]
    fn test_can_construct_proof() {
        let _proof: Established<Is<String>> = Established::assert();
        let _proof: Established<Is<i32>> = Established::assert();
        let _proof: Established<Is<Vec<u8>>> = Established::assert();
    }

    #[test]
    fn test_proof_requires_type() {
        fn requires_string_proof(_proof: Established<Is<String>>) {}
        
        let proof: Established<Is<String>> = Established::assert();
        requires_string_proof(proof);
        
        // This would fail to compile:
        // let wrong_proof: Established<Is<i32>> = Established::assert();
        // requires_string_proof(wrong_proof);
    }
}
