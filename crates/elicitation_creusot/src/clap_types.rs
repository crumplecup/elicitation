//! Creusot proofs for clap type elicitation.
//!
//! # Verification stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! We trust that the clap crate correctly defines its enum variants and builder
//! invariants. We verify only our own business logic: that every label produced
//! by `labels()` is accepted by `from_label()` (roundtrip completeness), and
//! that unknown labels are rejected.
//!
//! # Why all functions are `#[trusted]`
//!
//! Two distinct barriers prevent de-trusting:
//!
//! **String literal opacity wall** (blocks roundtrip + rejection proofs):
//! `str::view()` is `#[logic(opaque)]` in creusot-std — the SMT solver cannot
//! inspect the content of string literals like `"Auto (detect terminal)"`.
//! Even with `extern_spec!` contracts on `from_label`, the solver cannot prove
//! that a specific string literal is accepted or rejected.
//!
//! **Third-party builder types** (blocks builder trust axioms by design):
//! The 6 `verify_clap_*_trusted()` functions are architectural axioms declaring
//! clap's builder types trusted. They should never be de-trusted.
//!
//! # Partial de-trusting opportunity
//!
//! The `label_count` proofs (`labels().len() == options().len()`) are candidates
//! for de-trusting. `Vec` has a `ShallowModel` as `Seq<T>` in creusot-std, so
//! length comparisons are tractable. An `extern_spec!` block in `extern_specs.rs`
//! specifying `#[ensures(result@.len() == N)]` for `labels()` and `options()`
//! would let Alt-Ergo discharge `N == N` without `#[trusted]`. See
//! `CREUSOT_GUIDE.md § clap Type Proofs` for the full de-trusting plan.

#![cfg(feature = "clap-types")]

use creusot_std::prelude::*;
use elicitation::Select;

// ============================================================================
// ColorChoice — 3 variants
// ============================================================================

/// Verify that a known ColorChoice label is accepted by from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_color_choice_known_label_accepted() -> bool {
    clap::ColorChoice::from_label("Auto (detect terminal)").is_some()
}

/// Verify that all ColorChoice labels round-trip through from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_color_choice_all_labels_roundtrip() -> bool {
    clap::ColorChoice::labels()
        .iter()
        .all(|label| clap::ColorChoice::from_label(label).is_some())
}

/// Verify that an unknown label is rejected by ColorChoice::from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_color_choice_unknown_rejected() -> bool {
    clap::ColorChoice::from_label("__unknown__").is_none()
}

/// Verify ColorChoice label count equals option count.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_color_choice_label_count() -> bool {
    clap::ColorChoice::labels().len() == clap::ColorChoice::options().len()
}

// ============================================================================
// ArgAction — 8 variants
// ============================================================================

/// Verify that a known ArgAction label is accepted by from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_arg_action_known_label_accepted() -> bool {
    clap::ArgAction::from_label("Set (store single value)").is_some()
}

/// Verify that all ArgAction labels round-trip through from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_arg_action_all_labels_roundtrip() -> bool {
    clap::ArgAction::labels()
        .iter()
        .all(|label| clap::ArgAction::from_label(label).is_some())
}

/// Verify that an unknown label is rejected by ArgAction::from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_arg_action_unknown_rejected() -> bool {
    clap::ArgAction::from_label("__unknown__").is_none()
}

/// Verify ArgAction label count equals option count.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_arg_action_label_count() -> bool {
    clap::ArgAction::labels().len() == clap::ArgAction::options().len()
}

// ============================================================================
// ValueSource — 3 variants
// ============================================================================

/// Verify that a known ValueSource label is accepted by from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_value_source_known_label_accepted() -> bool {
    clap::parser::ValueSource::from_label("DefaultValue").is_some()
}

/// Verify that all ValueSource labels round-trip through from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_value_source_all_labels_roundtrip() -> bool {
    clap::parser::ValueSource::labels()
        .iter()
        .all(|label| clap::parser::ValueSource::from_label(label).is_some())
}

/// Verify that an unknown label is rejected by ValueSource::from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_value_source_unknown_rejected() -> bool {
    clap::parser::ValueSource::from_label("__unknown__").is_none()
}

/// Verify ValueSource label count equals option count.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_value_source_label_count() -> bool {
    clap::parser::ValueSource::labels().len() == clap::parser::ValueSource::options().len()
}

// ============================================================================
// ErrorKind — 17 variants
// ============================================================================

/// Verify that a known ErrorKind label is accepted by from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_error_kind_known_label_accepted() -> bool {
    clap::error::ErrorKind::from_label("InvalidValue").is_some()
}

/// Verify that all ErrorKind labels round-trip through from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_error_kind_all_labels_roundtrip() -> bool {
    clap::error::ErrorKind::labels()
        .iter()
        .all(|label| clap::error::ErrorKind::from_label(label).is_some())
}

/// Verify that an unknown label is rejected by ErrorKind::from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_error_kind_unknown_rejected() -> bool {
    clap::error::ErrorKind::from_label("__unknown__").is_none()
}

/// Verify ErrorKind label count equals option count.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_error_kind_label_count() -> bool {
    clap::error::ErrorKind::labels().len() == clap::error::ErrorKind::options().len()
}

// ============================================================================
// ValueHint
// ============================================================================

/// Verify that a known ValueHint label is accepted by from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_value_hint_known_label_accepted() -> bool {
    clap::builder::ValueHint::labels()
        .first()
        .and_then(|label| clap::builder::ValueHint::from_label(label))
        .is_some()
}

/// Verify that all ValueHint labels round-trip through from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_value_hint_all_labels_roundtrip() -> bool {
    clap::builder::ValueHint::labels()
        .iter()
        .all(|label| clap::builder::ValueHint::from_label(label).is_some())
}

/// Verify that an unknown label is rejected by ValueHint::from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_value_hint_unknown_rejected() -> bool {
    clap::builder::ValueHint::from_label("__unknown__").is_none()
}

/// Verify ValueHint label count equals option count.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_value_hint_label_count() -> bool {
    clap::builder::ValueHint::labels().len() == clap::builder::ValueHint::options().len()
}

// ============================================================================
// Trusted third-party types: Arg, ArgGroup, Command, Id, PossibleValue, ValueRange
//
// These are builder/struct types. We trust clap's invariants entirely and only
// record the axiom explicitly so the verification stance is visible.
// ============================================================================

/// Trust axiom: clap::Arg invariants are maintained by the clap crate.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_clap_arg_trusted() -> bool {
    true
}

/// Trust axiom: clap::ArgGroup invariants are maintained by the clap crate.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_clap_arg_group_trusted() -> bool {
    true
}

/// Trust axiom: clap::Command invariants are maintained by the clap crate.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_clap_command_trusted() -> bool {
    true
}

/// Trust axiom: clap::Id invariants are maintained by the clap crate.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_clap_id_trusted() -> bool {
    true
}

/// Trust axiom: clap::builder::PossibleValue invariants are maintained by the clap crate.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_clap_possible_value_trusted() -> bool {
    true
}

/// Trust axiom: clap::builder::ValueRange invariants are maintained by the clap crate.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_clap_value_range_trusted() -> bool {
    true
}
