//! The [`VerifiedWorkflow`] supertrait — the compiler-enforced contract for
//! fully-proven workflow propositions.
//!
//! # Overview
//!
//! [`VerifiedWorkflow`] is the workflow analogue of [`ElicitComplete`]: a
//! **marker supertrait** that aggregates every obligation a proposition type
//! must satisfy before it can be used as a first-class building block in a
//! verified workflow chain.
//!
//! Writing `impl VerifiedWorkflow for MyProp {}` is the final registration
//! step. The compiler accepts it only when `MyProp` satisfies all supertraits;
//! if anything is missing it rejects with a precise error pointing at what is
//! still absent.
//!
//! # The Contract
//!
//! At present the single supertrait is [`Prop`]:
//!
//! | Supertrait | What it provides | How it is satisfied |
//! |---|---|---|
//! | [`Prop`] | Non-empty `kani_proof`, `verus_proof`, `creusot_proof` | `#[derive(Prop)]` or manual `impl Prop` |
//!
//! [`Prop`] no longer has default (empty) implementations — every proposition
//! must supply real proof methods, exactly as [`Elicitation`] requires proof
//! methods on elicitation types. Use `#[derive(Prop)]` for trivial zero-cost
//! markers; write a manual `impl Prop` for propositions with meaningful
//! semantic content.
//!
//! # Proof Composition Guarantee
//!
//! [`And<P, Q>`] automatically composes: when both `P` and `Q` implement
//! `VerifiedWorkflow`, so does `And<P, Q>`, and its `kani_proof()` delegates
//! to both:
//!
//! ```text
//! And<UrlParsed, HttpsRequired>::kani_proof()
//!   = UrlParsed::kani_proof() + HttpsRequired::kani_proof()
//! ```
//!
//! This is mechanical and structurally impossible to omit — the `And<P, Q>`
//! impl concatenates the two token streams directly.
//!
//! # Runtime Validation (for Tests)
//!
//! Two families of provided methods are available for integration tests:
//!
//! ## Non-emptiness (`validate_proofs_non_empty`)
//!
//! ```rust,ignore
//! assert!(UrlParsed::validate_proofs_non_empty(),
//!     "UrlParsed has an empty proof — check the impl");
//! ```
//!
//! ## Constituent delegation (`kani_proof_contains`)
//!
//! ```rust,ignore
//! type Composite = And<UrlParsed, HttpsRequired>;
//! assert!(Composite::kani_proof_contains::<UrlParsed>());
//! assert!(Composite::kani_proof_contains::<HttpsRequired>());
//! ```
//!
//! # Adding a New Proposition
//!
//! 1. Define the struct: `pub struct MyProp;`
//! 2. For a trivial marker: `#[derive(Debug, Clone, Copy, Prop)]` — done.  
//!    For a semantically meaningful prop: write `impl Prop for MyProp { ... }`.
//! 3. Write `impl VerifiedWorkflow for MyProp {}`. Fix any compile errors.
//! 4. Add `assert_verified::<MyProp>("MyProp")` to the
//!    `workflow_verified_test.rs` inside the prop's home crate
//!    (e.g. `crates/elicit_url/tests/workflow_verified_test.rs`).
//! 5. If `MyProp` participates in an `And<>` conjunction used in production,
//!    add a containment assertion to the same test file.
//! 6. Run `just test-package <crate>` — all tests must pass.
//!
//! # Usage in Generic Bounds
//!
//! Use `VerifiedWorkflow` as the single bound anywhere you need a fully-proven
//! proposition type:
//!
//! ```rust,ignore
//! use elicitation::VerifiedWorkflow;
//! use elicitation::contracts::{And, Established};
//!
//! /// Accept any proven proposition as a workflow stage marker.
//! fn register_stage<P: VerifiedWorkflow>(name: &str) { /* ... */ }
//!
//! /// A pipeline that requires both stages to be formally proven.
//! struct Pipeline<Pre: VerifiedWorkflow, Post: VerifiedWorkflow> {
//!     _pre: std::marker::PhantomData<(Pre, Post)>,
//! }
//! ```
//!
//! [`ElicitComplete`]: crate::complete::ElicitComplete
//! [`Elicitation`]: crate::Elicitation

use crate::contracts::{And, Prop};

/// Compiler-enforced supertrait for fully-proven workflow propositions.
///
/// `impl VerifiedWorkflow for MyProp {}` compiles only when `MyProp`
/// satisfies [`Prop`] with non-empty proof methods. Use it as the single
/// bound in generic code that requires a formally-proven typestate marker.
///
/// See the [module documentation][self] for the full design rationale,
/// the proof composition guarantee, and the step-by-step guide for adding
/// new propositions.
pub trait VerifiedWorkflow: Prop {
    /// Runtime check: all three proof methods return non-empty TokenStreams.
    ///
    /// Use in tests to catch any `impl Prop` that accidentally returns
    /// `TokenStream::new()`. Call this for every proposition type in your
    /// test suite.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// assert!(UrlParsed::validate_proofs_non_empty(), "UrlParsed proofs must be non-empty");
    /// ```
    #[cfg(feature = "proofs")]
    fn validate_proofs_non_empty() -> bool {
        !Self::kani_proof().is_empty()
            && !Self::verus_proof().is_empty()
            && !Self::creusot_proof().is_empty()
    }

    /// Runtime check: does this proposition's Kani proof contain `Inner`'s?
    ///
    /// Use in tests to assert delegation — e.g., that `And<P, Q>`'s proof
    /// includes both `P`'s and `Q`'s proofs. Catches regressions in manual
    /// or macro-generated proof implementations.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// type PQ = And<UrlParsed, HttpsRequired>;
    /// assert!(PQ::kani_proof_contains::<UrlParsed>());
    /// assert!(PQ::kani_proof_contains::<HttpsRequired>());
    /// ```
    #[cfg(feature = "proofs")]
    fn kani_proof_contains<Inner: Prop>() -> bool {
        let outer = Self::kani_proof().to_string();
        let inner = Inner::kani_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }

    /// Runtime check: does this proposition's Verus proof contain `Inner`'s?
    #[cfg(feature = "proofs")]
    fn verus_proof_contains<Inner: Prop>() -> bool {
        let outer = Self::verus_proof().to_string();
        let inner = Inner::verus_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }

    /// Runtime check: does this proposition's Creusot proof contain `Inner`'s?
    #[cfg(feature = "proofs")]
    fn creusot_proof_contains<Inner: Prop>() -> bool {
        let outer = Self::creusot_proof().to_string();
        let inner = Inner::creusot_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }
}

/// Composition propagates: `And<P, Q>` is verified when both `P` and `Q` are.
impl<P: VerifiedWorkflow, Q: VerifiedWorkflow> VerifiedWorkflow for And<P, Q> {}
