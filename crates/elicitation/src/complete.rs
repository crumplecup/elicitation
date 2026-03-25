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
}
