//! The [`ElicitComplete`] supertrait for fully-implemented elicitation support.
//!
//! [`ElicitComplete`] acts as a checklist enforced by the compiler: a type may
//! only implement it once it satisfies every obligation listed in the supertraits.
//! For generic containers (e.g. `Vec<T>`) the bound `T: ElicitComplete` propagates
//! the requirement through the type hierarchy, ensuring the entire composition tree
//! is verified.
//!
//! # What "complete" means
//!
//! A type is [`ElicitComplete`] when it has:
//!
//! - [`Elicitation`] — interactive elicitation *with required proof methods*
//!   (`kani_proof`, `verus_proof`, `creusot_proof` are not defaulted, so the
//!   compiler will reject empty `TokenStream::new()` unless the proof actually
//!   delegates to its constituents).
//! - [`ElicitIntrospect`] — structural metadata (pattern, field names, etc.)
//! - [`ElicitSpec`] — agent-browsable contract spec (requires/ensures/bounds)
//! - `serde::Serialize + for<'de> Deserialize<'de>` — data interchange
//! - `schemars::JsonSchema` — JSON schema generation for tooling/agents
//!
//! # Enforcement by composition
//!
//! Because `Vec<T>` only implements `ElicitComplete` when `T: ElicitComplete`,
//! and because `Vec<T>::kani_proof()` delegates to `<T as Elicitation>::kani_proof()`,
//! an incomplete inner type blocks the entire chain from compiling as `ElicitComplete`.
//!
//! # Usage
//!
//! ```rust,ignore
//! use elicitation::ElicitComplete;
//!
//! fn register_tool<T: ElicitComplete>(name: &str) { /* ... */ }
//! ```

use crate::{ElicitIntrospect, ElicitSpec, Elicitation};

/// Supertrait that enforces complete elicitation support.
///
/// Implement this only after satisfying every supertrait. For structs derived
/// with `#[derive(Elicit)]`, all obligations are met when:
///
/// 1. Proof methods delegate to field types (not `TokenStream::new()`).
/// 2. `ElicitIntrospect` is implemented (auto-derived).
/// 3. `ElicitSpec` is implemented in `type_spec/`.
/// 4. `serde` derives are present.
/// 5. `schemars::JsonSchema` is derived.
///
/// See [`THIRD_PARTY_SUPPORT_GUIDE.md`] for the full checklist.
pub trait ElicitComplete:
    Elicitation
    + ElicitIntrospect
    + ElicitSpec
    + serde::Serialize
    + for<'de> serde::Deserialize<'de>
    + schemars::JsonSchema
{
    /// Runtime check: all three proof methods return non-empty TokenStreams.
    ///
    /// Use in tests to catch any manual `impl Elicitation` that returns
    /// `TokenStream::new()`. Call this for every type in your test suite.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// assert!(bool::validate_proofs_non_empty(), "bool proofs must be non-empty");
    /// ```
    #[cfg(feature = "proofs")]
    fn validate_proofs_non_empty() -> bool {
        !Self::kani_proof().is_empty()
            && !Self::verus_proof().is_empty()
            && !Self::creusot_proof().is_empty()
    }
    ///
    /// Use this in tests to assert delegation — i.e., that an aggregate type's
    /// proof includes its constituent types' proofs. Catches regressions in
    /// manual proof implementations.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// assert!(VecNonEmpty::<String>::kani_proof_contains::<String>(),
    ///     "VecNonEmpty proof must include String's proof");
    /// ```
    #[cfg(feature = "proofs")]
    fn kani_proof_contains<Inner: Elicitation>() -> bool {
        let outer = Self::kani_proof().to_string();
        let inner = Inner::kani_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }

    /// Runtime check: does this type's Verus proof contain `Inner`'s Verus proof?
    #[cfg(feature = "proofs")]
    fn verus_proof_contains<Inner: Elicitation>() -> bool {
        let outer = Self::verus_proof().to_string();
        let inner = Inner::verus_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }

    /// Runtime check: does this type's Creusot proof contain `Inner`'s Creusot proof?
    #[cfg(feature = "proofs")]
    fn creusot_proof_contains<Inner: Elicitation>() -> bool {
        let outer = Self::creusot_proof().to_string();
        let inner = Inner::creusot_proof().to_string();
        !inner.is_empty() && outer.contains(&inner)
    }
}
