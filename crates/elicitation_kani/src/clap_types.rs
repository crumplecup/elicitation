//! Kani proofs for clap type elicitation.
//!
//! Available with the `clap-types` feature.
//!
//! # Proof Strategy
//!
//! For each `Select` enum type we verify:
//! 1. **Label count**: `labels().len() == options().len()`
//! 2. **Roundtrip**: every label produced by `labels()` is accepted by `from_label()`
//! 3. **Unknown rejection**: `from_label("__unknown__")` returns `None`
//!
//! These are the invariants the MCP elicitation machinery depends on at runtime.

#[cfg(feature = "clap-types")]
use elicitation::Select;

// ============================================================================
// ColorChoice
// ============================================================================

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_color_choice_label_count() {
    let labels = clap::ColorChoice::labels();
    let options = clap::ColorChoice::options();
    assert!(
        labels.len() == options.len(),
        "ColorChoice: labels and options have equal length"
    );
    assert!(labels.len() == 3, "ColorChoice has 3 variants");
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_color_choice_roundtrip_auto() {
    let labels = clap::ColorChoice::labels();
    let result = clap::ColorChoice::from_label(&labels[0]);
    assert!(result.is_some(), "ColorChoice::Auto label roundtrips");
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_color_choice_roundtrip_always() {
    let labels = clap::ColorChoice::labels();
    let result = clap::ColorChoice::from_label(&labels[1]);
    assert!(result.is_some(), "ColorChoice::Always label roundtrips");
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_color_choice_roundtrip_never() {
    let labels = clap::ColorChoice::labels();
    let result = clap::ColorChoice::from_label(&labels[2]);
    assert!(result.is_some(), "ColorChoice::Never label roundtrips");
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_color_choice_unknown_rejected() {
    let result = clap::ColorChoice::from_label("__unknown__");
    assert!(result.is_none(), "ColorChoice rejects unknown labels");
}

// ============================================================================
// ArgAction
// ============================================================================

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_arg_action_label_count() {
    let labels = clap::ArgAction::labels();
    let options = clap::ArgAction::options();
    assert!(
        labels.len() == options.len(),
        "ArgAction: labels and options have equal length"
    );
    assert!(labels.len() == 8, "ArgAction has 8 variants");
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_arg_action_all_labels_roundtrip() {
    let labels = clap::ArgAction::labels();
    for label in &labels {
        let result = clap::ArgAction::from_label(label);
        assert!(result.is_some(), "ArgAction label roundtrips");
    }
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_arg_action_unknown_rejected() {
    let result = clap::ArgAction::from_label("__unknown__");
    assert!(result.is_none(), "ArgAction rejects unknown labels");
}

// ============================================================================
// ValueSource
// ============================================================================

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_value_source_label_count() {
    let labels = clap::parser::ValueSource::labels();
    let options = clap::parser::ValueSource::options();
    assert!(
        labels.len() == options.len(),
        "ValueSource: labels and options have equal length"
    );
    assert!(labels.len() == 3, "ValueSource has 3 variants");
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_value_source_all_labels_roundtrip() {
    let labels = clap::parser::ValueSource::labels();
    for label in &labels {
        let result = clap::parser::ValueSource::from_label(label);
        assert!(result.is_some(), "ValueSource label roundtrips");
    }
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_value_source_unknown_rejected() {
    let result = clap::parser::ValueSource::from_label("__unknown__");
    assert!(result.is_none(), "ValueSource rejects unknown labels");
}

// ============================================================================
// ErrorKind
// ============================================================================

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_error_kind_label_count() {
    let labels = clap::error::ErrorKind::labels();
    let options = clap::error::ErrorKind::options();
    assert!(
        labels.len() == options.len(),
        "ErrorKind: labels and options have equal length"
    );
    assert!(labels.len() == 17, "ErrorKind has 17 variants");
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_error_kind_all_labels_roundtrip() {
    let labels = clap::error::ErrorKind::labels();
    for label in &labels {
        let result = clap::error::ErrorKind::from_label(label);
        assert!(result.is_some(), "ErrorKind label roundtrips");
    }
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_error_kind_unknown_rejected() {
    let result = clap::error::ErrorKind::from_label("__unknown__");
    assert!(result.is_none(), "ErrorKind rejects unknown labels");
}

// ============================================================================
// ValueHint
// ============================================================================

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_value_hint_label_count() {
    let labels = clap::builder::ValueHint::labels();
    let options = clap::builder::ValueHint::options();
    assert!(
        labels.len() == options.len(),
        "ValueHint: labels and options have equal length"
    );
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_value_hint_all_labels_roundtrip() {
    let labels = clap::builder::ValueHint::labels();
    for label in &labels {
        let result = clap::builder::ValueHint::from_label(label);
        assert!(result.is_some(), "ValueHint label roundtrips");
    }
}

#[cfg(feature = "clap-types")]
#[kani::proof]
fn verify_value_hint_unknown_rejected() {
    let result = clap::builder::ValueHint::from_label("__unknown__");
    assert!(result.is_none(), "ValueHint rejects unknown labels");
}
