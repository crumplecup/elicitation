//! Verified State Machine for archive modal overlays.
//!
//! Consolidates the six overlay booleans/indices scattered across
//! [`ArchiveNavModel`](crate::archive::nav_model::ArchiveNavModel)
//! (`show_help`, `export_picker`, `save_prompt_active`, `save_prompt_text`,
//! `saved_browser_active`, `saved_browser_idx`) into a single typed state.

use elicit_ui::WcagVerified;
use elicitation::{
    Elicit, Established, KaniCompose, KaniVariantState, Prop, VerifiedStateMachine,
    contracts::ProvableFrom, formal_method,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::types::{ExportFormat, SavedQuery};

// ── ArchiveOverlayState ───────────────────────────────────────────────────────

/// State of modal overlays that float above the main panel.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
    KaniVariantState,
)]
pub enum ArchiveOverlayState {
    /// No overlay is open.
    #[default]
    OverlayNone,

    /// Key-binding help overlay.
    HelpOpen,

    /// Export format picker overlay.
    ExportPickerOpen {
        /// Currently highlighted format option (0-based).
        idx: usize,
        /// Available formats.
        formats: Vec<ExportFormat>,
    },

    /// Save-query name prompt.
    SavePromptOpen {
        /// Text being typed into the prompt.
        text: String,
    },

    /// Saved queries browser overlay.
    SavedBrowserOpen {
        /// Cached saved queries.
        entries: Vec<SavedQuery>,
        /// Currently highlighted row.
        idx: usize,
    },
}

// ── ArchiveOverlayConsistent (invariant) ─────────────────────────────────────

/// Proposition: at most one overlay is open and its state is valid.
///
/// Wired to [`WcagVerified`] from `elicit_ui`: overlays render AccessKit nodes
/// so WCAG compliance is the credential that bounds the proof state space.
#[derive(Prop)]
#[prop(credential = WcagVerified, creusot_invariant_fn = "archive_overlay_consistent", kani_invariant_fn = "archive_overlay_consistent", verus_invariant_fn = "archive_overlay_consistent", verus_inv_body = "match *state { ArchiveOverlayState::ExportPickerOpen { idx, len } => idx <= len, ArchiveOverlayState::SavedBrowserOpen { len, idx } => idx <= len, _ => true, }", verus_state_body = "ExportPickerOpen { idx: usize, len: usize }, SavedBrowserOpen { len: usize, idx: usize }, _Other,", creusot_inv_body = "pearlite! { match state { ArchiveOverlayState::ExportPickerOpen { idx, formats } => idx@ <= formats@.len(), ArchiveOverlayState::SavedBrowserOpen { entries, idx } => idx@ <= entries@.len(), _ => true, } }")]
pub struct ArchiveOverlayConsistent;

impl ProvableFrom<WcagVerified> for ArchiveOverlayConsistent {}

/// Structural invariant predicate for [`ArchiveOverlayState`].
///
/// Runtime-evaluable form of [`ArchiveOverlayConsistent`] used by Kani
/// `#[kani::requires]` / `#[kani::ensures]` in contracted wrapper functions.
///
/// Invariant: picker and browser cursor indices are within their list bounds.
#[cfg(kani)]
pub fn archive_overlay_consistent(state: &ArchiveOverlayState) -> bool {
    match state {
        ArchiveOverlayState::ExportPickerOpen { idx, formats } => *idx <= formats.len(),
        ArchiveOverlayState::SavedBrowserOpen { entries, idx } => *idx <= entries.len(),
        _ => true,
    }
}

/// Bridge `kani::Arbitrary` to `KaniCompose::kani_depth0()` so that
/// `stub_verified` can generate bounded symbolic return values.
#[cfg(kani)]
impl kani::Arbitrary for ArchiveOverlayState {
    fn any() -> Self {
        use elicitation::KaniCompose;
        ArchiveOverlayState::kani_depth0()
    }
}

// ── ArchiveOverlayMachine ─────────────────────────────────────────────────────

/// Verified state machine for archive modal overlays.
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [
    close_overlay, open_help, open_export_picker,
    picker_move_up, picker_move_down,
    open_save_prompt, prompt_push, prompt_backspace,
    open_saved_browser, saved_browser_up, saved_browser_down,
])]
pub struct ArchiveOverlayMachine;

// ── Transitions ───────────────────────────────────────────────────────────────

/// Close the active overlay (return to none).
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "trivial")]
#[instrument(skip(proof))]
pub fn close_overlay(
    _state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    (ArchiveOverlayState::OverlayNone, proof)
}

/// Open the help overlay.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "trivial")]
#[instrument(skip(proof))]
pub fn open_help(
    _state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    (ArchiveOverlayState::HelpOpen, proof)
}

/// Open the export format picker.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "special_false")]
#[instrument(skip(proof))]
pub fn open_export_picker(
    _state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
    formats: Vec<ExportFormat>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    (
        ArchiveOverlayState::ExportPickerOpen { idx: 0, formats },
        proof,
    )
}

/// Move the export picker selection up.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "conditional_special")]
#[instrument(skip(proof))]
pub fn picker_move_up(
    state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    let next = match state {
        ArchiveOverlayState::ExportPickerOpen { idx, formats } => {
            ArchiveOverlayState::ExportPickerOpen {
                idx: idx.saturating_sub(1),
                formats,
            }
        }
        other => other,
    };
    (next, proof)
}

/// Move the export picker selection down.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "conditional_special")]
#[instrument(skip(proof))]
pub fn picker_move_down(
    state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    let next = match state {
        ArchiveOverlayState::ExportPickerOpen { idx, formats } => {
            let new_idx = idx.saturating_add(1).min(formats.len().saturating_sub(1));
            ArchiveOverlayState::ExportPickerOpen {
                idx: new_idx,
                formats,
            }
        }
        other => other,
    };
    (next, proof)
}

/// Open the save-query name prompt.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "trivial")]
#[instrument(skip(proof))]
pub fn open_save_prompt(
    _state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    (
        ArchiveOverlayState::SavePromptOpen {
            text: String::new(),
        },
        proof,
    )
}

/// Append a character to the save-prompt text.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "passthrough")]
#[instrument(skip(proof))]
pub fn prompt_push(
    state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
    ch: char,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    let next = match state {
        ArchiveOverlayState::SavePromptOpen { mut text } => {
            text.push(ch);
            ArchiveOverlayState::SavePromptOpen { text }
        }
        other => other,
    };
    (next, proof)
}

/// Delete the last character from the save-prompt text.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "passthrough")]
#[instrument(skip(proof))]
pub fn prompt_backspace(
    state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    let next = match state {
        ArchiveOverlayState::SavePromptOpen { mut text } => {
            text.pop();
            ArchiveOverlayState::SavePromptOpen { text }
        }
        other => other,
    };
    (next, proof)
}

/// Open the saved queries browser.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "trivial")]
#[instrument(skip(proof))]
pub fn open_saved_browser(
    _state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
    entries: Vec<SavedQuery>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    (
        ArchiveOverlayState::SavedBrowserOpen { entries, idx: 0 },
        proof,
    )
}

/// Move the saved browser selection up.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "passthrough")]
#[instrument(skip(proof))]
pub fn saved_browser_up(
    state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    let next = match state {
        ArchiveOverlayState::SavedBrowserOpen { entries, idx } => {
            ArchiveOverlayState::SavedBrowserOpen {
                entries,
                idx: idx.saturating_sub(1),
            }
        }
        other => other,
    };
    (next, proof)
}

/// Move the saved browser selection down.
#[formal_method(contracts = [ArchiveOverlayConsistent], verus_class = "passthrough")]
#[instrument(skip(proof))]
pub fn saved_browser_down(
    state: ArchiveOverlayState,
    proof: Established<ArchiveOverlayConsistent>,
) -> (ArchiveOverlayState, Established<ArchiveOverlayConsistent>) {
    let next = match state {
        ArchiveOverlayState::SavedBrowserOpen { entries, idx } => {
            let new_idx = idx.saturating_add(1).min(entries.len().saturating_sub(1));
            ArchiveOverlayState::SavedBrowserOpen {
                entries,
                idx: new_idx,
            }
        }
        other => other,
    };
    (next, proof)
}
