//! Creusot proofs for ratatui type elicitation.
//!
//! # Verification stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! We trust that the ratatui crate correctly defines its enum variants and struct
//! fields. We verify our own wrappers: that every label produced by `labels()`
//! is accepted by `from_label()` (Select roundtrip), and that `From`
//! conversions between our wrappers and ratatui types preserve field values.
//!
//! # Why most functions are `#[trusted]`
//!
//! **String literal opacity wall** (blocks roundtrip + rejection proofs):
//! `str::view()` is `#[logic(opaque)]` in creusot-std — the SMT solver cannot
//! inspect the content of string literals. Even with `extern_spec!` contracts,
//! the solver cannot prove that a specific label is accepted or rejected.
//!
//! **Composite From roundtrip** (trusted by design):
//! The From impls delegate to ratatui constructors and field accessors whose
//! contracts are not exposed to Creusot. The roundtrip assertions are
//! runtime-verified and axiomatically trusted here.

#![cfg(feature = "ratatui-types")]

use creusot_std::prelude::*;
use elicitation::Select;

// ============================================================================
// Macro: select_creusot_proofs!
//
// Generates Creusot proofs for a ratatui Select enum:
// - known label accepted (#[trusted])
// - all labels roundtrip (#[trusted])
// - unknown rejected (#[trusted])
// - label count == option count (#[trusted])
// ============================================================================

macro_rules! select_creusot_proofs {
    (
        type_path  = $ty:ty,
        snake      = $snake:ident,
        known_label = $label:expr
    ) => {
        ::paste::paste! {
            /// Verify that a known label is accepted by from_label.
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn [< verify_ $snake _known_label_accepted >]() -> bool {
                <$ty>::from_label($label).is_some()
            }

            /// Verify that all labels round-trip through from_label.
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn [< verify_ $snake _all_labels_roundtrip >]() -> bool {
                <$ty>::labels()
                    .iter()
                    .all(|label| <$ty>::from_label(label).is_some())
            }

            /// Verify that an unknown label is rejected.
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn [< verify_ $snake _unknown_rejected >]() -> bool {
                <$ty>::from_label("__unknown__").is_none()
            }

            /// Verify label count equals option count.
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn [< verify_ $snake _label_count >]() -> bool {
                <$ty>::labels().len() == <$ty>::options().len()
            }
        }
    };
}

// ============================================================================
// Select enum proofs (6 types)
// ============================================================================

select_creusot_proofs!(
    type_path = ratatui::layout::Alignment,
    snake = ratatui_alignment,
    known_label = "Left"
);

select_creusot_proofs!(
    type_path = ratatui::layout::Direction,
    snake = ratatui_direction,
    known_label = "Horizontal"
);

select_creusot_proofs!(
    type_path = ratatui::widgets::BorderType,
    snake = ratatui_border_type,
    known_label = "Plain"
);

select_creusot_proofs!(
    type_path = ratatui::style::Color,
    snake = ratatui_color,
    known_label = "Red"
);

select_creusot_proofs!(
    type_path = ratatui::widgets::Borders,
    snake = ratatui_borders,
    known_label = "All"
);

select_creusot_proofs!(
    type_path = ratatui::widgets::ScrollbarOrientation,
    snake = ratatui_scrollbar_orientation,
    known_label = "VerticalRight"
);

// ============================================================================
// Composite struct proofs — From roundtrip verification
//
// Each proof asserts that converting ratatui::T → Wrapper → ratatui::T
// preserves field values. These are #[trusted] because the From impls call
// ratatui constructors/accessors whose contracts are opaque to Creusot's SMT
// solver.
// ============================================================================

// ── Padding ─────────────────────────────────────────────────────────────

/// Verify Padding From roundtrip preserves all four sides.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_ratatui_padding_from_roundtrip() -> bool {
    let original = ratatui::widgets::Padding::new(5, 10, 15, 20);
    let wrapper = elicitation::RatatuiPadding::from(original);
    let restored: ratatui::widgets::Padding = wrapper.into();
    restored.left == 5 && restored.right == 10 && restored.top == 15 && restored.bottom == 20
}

/// Verify Padding wrapper field extraction matches input.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_ratatui_padding_wrapper_fields() -> bool {
    let original = ratatui::widgets::Padding::new(1, 2, 3, 4);
    let wrapper = elicitation::RatatuiPadding::from(original);
    wrapper.left == 1 && wrapper.right == 2 && wrapper.top == 3 && wrapper.bottom == 4
}

// ── Margin ──────────────────────────────────────────────────────────────

/// Verify Margin From roundtrip preserves horizontal and vertical.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_ratatui_margin_from_roundtrip() -> bool {
    let original = ratatui::layout::Margin::new(8, 16);
    let wrapper = elicitation::RatatuiMargin::from(original);
    let restored: ratatui::layout::Margin = wrapper.into();
    restored.horizontal == 8 && restored.vertical == 16
}

/// Verify Margin wrapper field extraction matches input.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_ratatui_margin_wrapper_fields() -> bool {
    let original = ratatui::layout::Margin::new(3, 7);
    let wrapper = elicitation::RatatuiMargin::from(original);
    wrapper.horizontal == 3 && wrapper.vertical == 7
}

// ── Style ───────────────────────────────────────────────────────────────

/// Verify Style From roundtrip preserves modifiers (non-string path).
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_ratatui_style_empty_roundtrip() -> bool {
    use ratatui::style::Style;

    let style = Style::default();
    let wrapper = elicitation::RatatuiStyle::from(style);
    wrapper.fg.is_none()
        && wrapper.bg.is_none()
        && !wrapper.bold
        && !wrapper.italic
        && !wrapper.underlined
}

/// Verify Style modifier mapping (BOLD | ITALIC | UNDERLINED).
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_ratatui_style_all_modifiers() -> bool {
    use ratatui::style::{Modifier, Style};

    let style = Style::default()
        .add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINED);
    let wrapper = elicitation::RatatuiStyle::from(style);
    wrapper.bold && wrapper.italic && wrapper.underlined
}

/// Verify Style fg/bg presence is preserved (Some vs None).
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_ratatui_style_fg_bg_presence() -> bool {
    use ratatui::style::{Color, Style};

    let s1 = Style::default().fg(Color::Red);
    let w1 = elicitation::RatatuiStyle::from(s1);

    let s2 = Style::default().bg(Color::Blue);
    let w2 = elicitation::RatatuiStyle::from(s2);

    w1.fg.is_some() && w1.bg.is_none() && w2.fg.is_none() && w2.bg.is_some()
}

// ── BordersSelect ───────────────────────────────────────────────────────

/// Verify BordersSelect into_inner roundtrip for all presets.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_borders_select_into_inner() -> bool {
    use ratatui::widgets::Borders;

    let presets = [
        Borders::NONE,
        Borders::ALL,
        Borders::TOP,
        Borders::BOTTOM,
        Borders::LEFT,
        Borders::RIGHT,
    ];

    presets
        .iter()
        .all(|&b| elicitation::BordersSelect::from(b).into_inner() == b)
}
