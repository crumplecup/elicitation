//! The [`VerifiedWorkflow`] supertrait — the compiler-enforced contract for
//! fully-proven workflow propositions.
//!
//! # Overview
//!
//! Workflows in this framework are modelled as **typestate machines**: each tool
//! call advances a value through a chain of states, and each transition produces
//! an [`Established<P>`] token that carries cryptographic-strength proof that a
//! logical proposition `P` was satisfied at runtime. For that token to be
//! trustworthy as a *formal* building block, the proposition type `P` must itself
//! be accompanied by machine-checkable proofs — Kani, Verus, and Creusot — so
//! that the entire chain is verifiable end-to-end, not just at the Rust
//! type level.
//!
//! [`VerifiedWorkflow`] is the **marker supertrait** that enforces this. It is
//! the workflow analogue of [`ElicitComplete`]: writing
//! `impl VerifiedWorkflow for MyProp {}` is the final registration step that
//! proves `MyProp` has discharged every formal obligation. The compiler accepts
//! the impl only when all supertraits are satisfied; if anything is missing it
//! rejects with a precise, actionable error.
//!
//! Think of it as a living checklist: each new proposition type that appears in a
//! workflow contract is registered here, and the compiler prevents the codebase
//! from ever shipping a proposition without proof coverage.
//!
//! # The Contract
//!
//! The single supertrait is [`Prop`]:
//!
//! | Supertrait | What it provides | How it is satisfied |
//! |---|---|---|
//! | [`Prop`] | Non-empty `kani_proof`, `verus_proof`, `creusot_proof` | `#[derive(Prop)]` or manual `impl Prop` |
//!
//! [`Prop`] has **no default implementations** — every proposition must supply
//! real, non-empty proof bodies. This mirrors how [`Elicitation`] requires proof
//! methods on elicitation types. Two paths exist:
//!
//! - **`#[derive(Prop)]`** — for zero-cost typestate markers with no semantic
//!   content beyond their name. The macro generates uniquely-named trivial
//!   harnesses for all three verifiers. This is correct for the vast majority of
//!   propositions; the harness name encodes the type identity so the proof is
//!   distinguishable in composition.
//!
//! - **Manual `impl Prop`** — for propositions that encode meaningful invariants,
//!   e.g. `DbConnected` which axiomatises the observable behaviour of a connection
//!   pool. The proof body documents the assumed contract with the third-party
//!   library. Use `quote::quote!` to emit Kani/Verus/Creusot harnesses directly.
//!
//! # Proof Composition Guarantee
//!
//! The most powerful guarantee is **automatic composition through `And<P, Q>`**.
//! When both `P` and `Q` implement `VerifiedWorkflow`, the blanket impl at the
//! bottom of this module makes `And<P, Q>: VerifiedWorkflow` as well, and its
//! proof methods delegate to both constituents:
//!
//! ```text
//! And<UrlParsed, HttpsRequired>::kani_proof()
//!   ≡  UrlParsed::kani_proof()  ++  HttpsRequired::kani_proof()
//! ```
//!
//! This delegation is **structurally impossible to omit**: the `And<P, Q>`
//! implementation calls `P::kani_proof()` and `Q::kani_proof()` directly —
//! there is no way to produce a conjunctive proof that silently drops a
//! constituent. Deep conjunctions like `And<And<A, B>, C>` propagate
//! recursively through the same blanket impl, so arbitrarily complex workflow
//! contracts are always fully covered.
//!
//! Compare this with [`ElicitComplete`]'s analogous guarantee for aggregate
//! structs: `#[derive(Elicit)]` iterates struct fields mechanically. Here,
//! `And<P, Q>` provides the same mechanical provenance guarantee for logical
//! conjunctions of propositions.
//!
//! # Two Kinds of Props, One Guarantee
//!
//! **Trivial markers** — structural preconditions established by calling the
//! right API in the right order — dominate in practice:
//!
//! ```rust,ignore
//! #[derive(Prop)]
//! pub struct UrlParsed;          // URL string was well-formed
//!
//! #[derive(Prop)]
//! pub struct HttpsRequired;      // scheme is specifically "https"
//!
//! impl VerifiedWorkflow for UrlParsed {}
//! impl VerifiedWorkflow for HttpsRequired {}
//!
//! // Composition: parse AND enforce HTTPS
//! pub type SecureUrl = And<UrlParsed, HttpsRequired>;
//! // SecureUrl: VerifiedWorkflow — derived automatically by the blanket impl
//! ```
//!
//! **Semantic propositions** — propositions that axiomatise the behaviour of a
//! third-party system — appear where the Rust type system cannot verify the
//! external contract:
//!
//! ```rust,ignore
//! // The proposition asserts: if sqlx::AnyPool::connect returns Ok,
//! // the pool is ready to accept queries.
//! pub struct DbConnected;
//!
//! impl Prop for DbConnected {
//!     fn kani_proof() -> elicitation::proc_macro2::TokenStream {
//!         quote::quote! {
//!             #[kani::proof]
//!             fn verify_db_connected_axiom() {
//!                 let connect_ok: bool = kani::any();
//!                 kani::assume(connect_ok);
//!                 assert!(connect_ok);
//!             }
//!         }
//!     }
//!     // verus_proof and creusot_proof similarly
//! }
//!
//! impl VerifiedWorkflow for DbConnected {}
//! ```
//!
//! The Kani harness is marked `#[trusted]` by convention — it axiomatises the
//! external library's contract rather than verifying it. This is the same pattern
//! used throughout `elicitation_kani` for third-party type boundaries.
//!
//! # Runtime Validation (for Tests)
//!
//! Because emptiness is a value property rather than a type property, the trait
//! provides two families of runtime checks for use in integration tests:
//!
//! ## Non-emptiness (`validate_proofs_non_empty`)
//!
//! Catches any `impl Prop` that accidentally returns `TokenStream::new()`:
//!
//! ```rust,ignore
//! // In tests/workflow_verified_test.rs:
//! fn assert_verified<T: VerifiedWorkflow>(label: &str) {
//!     assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
//! }
//!
//! assert_verified::<UrlParsed>("UrlParsed");
//! assert_verified::<DbConnected>("DbConnected");
//! ```
//!
//! ## Constituent delegation (`kani_proof_contains`)
//!
//! Catches regressions in manually-written composite proofs where a constituent
//! call is accidentally dropped:
//!
//! ```rust,ignore
//! type SecureUrl = And<UrlParsed, HttpsRequired>;
//! assert!(SecureUrl::kani_proof_contains::<UrlParsed>(),
//!     "SecureUrl must contain UrlParsed proof");
//! assert!(SecureUrl::kani_proof_contains::<HttpsRequired>(),
//!     "SecureUrl must contain HttpsRequired proof");
//! ```
//!
//! For `And<P, Q>` propositions this test is redundant with the structural
//! guarantee above, but it is cheap to write and serves as a regression sentinel
//! — if the blanket impl were ever accidentally removed, the test would catch it
//! immediately.
//!
//! Every workflow crate maintains a `tests/workflow_verified_test.rs` that covers
//! both checks for all of its proposition types.
//!
//! # The Elicitation Analogy
//!
//! | Concept | Elicitation types | Workflow propositions |
//! |---|---|---|
//! | Core trait | `Elicitation` | `Prop` |
//! | Registration | `impl ElicitComplete` | `impl VerifiedWorkflow` |
//! | Trivial case | `#[derive(Elicit)]` | `#[derive(Prop)]` |
//! | Composition | `struct A { bar: Bar, baz: Baz }` | `And<P, Q>` |
//! | Composition guarantee | `#[derive(Elicit)]` iterates fields | Blanket `And<P,Q>` impl |
//! | Test file | `proof_non_empty_test.rs` | `workflow_verified_test.rs` |
//!
//! Both patterns exist for the same reason: to make it **structurally impossible**
//! to ship code that appears verified but isn't.
//!
//! # Adding a New Proposition
//!
//! 1. Define the struct: `pub struct MyProp;`
//! 2. For a trivial marker: `#[derive(Prop)]` generates all three proof methods.  
//!    For a semantic proposition: write `impl Prop for MyProp { ... }` with real
//!    harness bodies.
//! 3. Write `impl VerifiedWorkflow for MyProp {}`. Fix any compile errors — they
//!    point at exactly what is still missing from step 2.
//! 4. Add `assert_verified::<MyProp>("MyProp")` to the
//!    `workflow_verified_test.rs` inside the prop's home crate
//!    (e.g. `crates/elicit_url/tests/workflow_verified_test.rs`).
//! 5. If `MyProp` participates in a named `And<>` conjunction used in production,
//!    add containment assertions to the same test file.
//! 6. Run `just test-package <crate> --features proofs` — all tests must pass.
//!
//! # Usage in Generic Bounds
//!
//! Use `VerifiedWorkflow` as the single bound anywhere you require a formally
//! proven proposition type. This is the preferred bound over bare `Prop` in
//! public API surfaces:
//!
//! ```rust,ignore
//! use elicitation::VerifiedWorkflow;
//! use elicitation::contracts::{And, Established};
//!
//! /// Accept only propositions that have been formally registered.
//! fn require_proof<P: VerifiedWorkflow>(_evidence: &Established<P>) { /* ... */ }
//!
//! /// A two-stage pipeline where both pre- and post-conditions are proven.
//! struct Pipeline<Pre: VerifiedWorkflow, Post: VerifiedWorkflow> {
//!     _markers: std::marker::PhantomData<(Pre, Post)>,
//! }
//!
//! /// A generic connector that trusts any proven connection proposition.
//! async fn connect<C: VerifiedWorkflow>(config: &Config) -> Established<C> {
//!     // ...
//! }
//! ```
//!
//! [`ElicitComplete`]: crate::complete::ElicitComplete
//! [`Elicitation`]: crate::Elicitation
//! [`Established<P>`]: crate::contracts::Established

use crate::contracts::{And, Prop};

/// Compiler-enforced supertrait for fully-proven workflow propositions.
///
/// `impl VerifiedWorkflow for MyProp {}` compiles only when `MyProp`
/// satisfies [`Prop`] with non-empty proof methods. Use it as the single
/// bound in generic code that requires a formally-proven typestate marker.
///
/// See the module documentation for the full design rationale,
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
    fn kani_proof_contains<Inner: Prop>() -> bool {
        let outer = Self::kani_proof().to_string();
        let inner = Inner::kani_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }

    /// Runtime check: does this proposition's Verus proof contain `Inner`'s?
    fn verus_proof_contains<Inner: Prop>() -> bool {
        let outer = Self::verus_proof().to_string();
        let inner = Inner::verus_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }

    /// Runtime check: does this proposition's Creusot proof contain `Inner`'s?
    fn creusot_proof_contains<Inner: Prop>() -> bool {
        let outer = Self::creusot_proof().to_string();
        let inner = Inner::creusot_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }
}

/// Composition propagates: `And<P, Q>` is verified when both `P` and `Q` are.
impl<P: VerifiedWorkflow, Q: VerifiedWorkflow> VerifiedWorkflow for And<P, Q> {}
