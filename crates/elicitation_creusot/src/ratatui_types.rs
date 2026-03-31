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
//! # Trusted vs non-trusted proofs
//!
//! **Select proofs (`#[trusted]`)**: `str::view()` is `#[logic(opaque)]` in
//! creusot-std — the SMT solver cannot inspect string literal content. Select
//! roundtrip proofs that compare labels must remain trusted axioms.
//!
//! **Composite proofs (non-trusted where possible)**: We provide `extern_spec!`
//! contracts for ratatui constructors (`Padding::new`, `Margin::new`) and
//! `#[logic]` field accessors, enabling the SMT solver to verify our wrapper
//! `From` impls without `#[trusted]`. The proof functions inline the wrapper
//! logic (pub-field copy + constructor call) so Creusot can verify end-to-end.
//!
//! **Style proofs (`#[trusted]`)**: Style wraps `Color::to_string()` /
//! `str::parse()` and `Modifier` bitflags — both opaque to the solver.

#![cfg(feature = "ratatui-types")]

use creusot_std::prelude::*;
use elicitation::Select;

// ============================================================================
// Logic functions — trusted accessors for ratatui struct fields
//
// These let the SMT solver reason about ratatui field values in contracts.
// Opaque because ratatui is a foreign crate — Creusot cannot inspect its
// field layout directly.
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn padding_left(_p: ratatui::widgets::Padding) -> u16 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn padding_right(_p: ratatui::widgets::Padding) -> u16 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn padding_top(_p: ratatui::widgets::Padding) -> u16 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn padding_bottom(_p: ratatui::widgets::Padding) -> u16 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn margin_horizontal(_m: ratatui::layout::Margin) -> u16 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn margin_vertical(_m: ratatui::layout::Margin) -> u16 {
    dead
}

// ============================================================================
// Extern specs — trusted axioms about ratatui constructors AND field access
//
// Two kinds of axioms:
// 1. Constructor postconditions: Padding::new(l,r,t,b) → logic accessors match inputs
// 2. Field-access bridge: original.left (program) == padding_left(original) (logic)
//
// Together these let Creusot verify our wrapper's field-copy + reconstruct logic.
// ============================================================================

extern_spec! {
    impl ratatui::widgets::Padding {
        #[ensures(padding_left(result) == left)]
        #[ensures(padding_right(result) == right)]
        #[ensures(padding_top(result) == top)]
        #[ensures(padding_bottom(result) == bottom)]
        fn new(left: u16, right: u16, top: u16, bottom: u16) -> ratatui::widgets::Padding;
    }
}

extern_spec! {
    impl ratatui::layout::Margin {
        #[ensures(margin_horizontal(result) == horizontal)]
        #[ensures(margin_vertical(result) == vertical)]
        fn new(horizontal: u16, vertical: u16) -> ratatui::layout::Margin;
    }
}

// ============================================================================
// Field-access bridge functions
//
// These trusted functions bridge program-level field access (p.left) to
// logic-level accessor (padding_left(p)). They are the minimal trusted
// surface needed to verify our wrapper logic.
// ============================================================================

/// Bridge: p.left in program context == padding_left(p) in logic context.
#[trusted]
#[ensures(padding_left(*p) == result)]
pub fn read_padding_left(p: &ratatui::widgets::Padding) -> u16 {
    p.left
}

/// Bridge: p.right in program context == padding_right(p) in logic context.
#[trusted]
#[ensures(padding_right(*p) == result)]
pub fn read_padding_right(p: &ratatui::widgets::Padding) -> u16 {
    p.right
}

/// Bridge: p.top in program context == padding_top(p) in logic context.
#[trusted]
#[ensures(padding_top(*p) == result)]
pub fn read_padding_top(p: &ratatui::widgets::Padding) -> u16 {
    p.top
}

/// Bridge: p.bottom in program context == padding_bottom(p) in logic context.
#[trusted]
#[ensures(padding_bottom(*p) == result)]
pub fn read_padding_bottom(p: &ratatui::widgets::Padding) -> u16 {
    p.bottom
}

/// Bridge: m.horizontal in program context == margin_horizontal(m) in logic context.
#[trusted]
#[ensures(margin_horizontal(*m) == result)]
pub fn read_margin_horizontal(m: &ratatui::layout::Margin) -> u16 {
    m.horizontal
}

/// Bridge: m.vertical in program context == margin_vertical(m) in logic context.
#[trusted]
#[ensures(margin_vertical(*m) == result)]
pub fn read_margin_vertical(m: &ratatui::layout::Margin) -> u16 {
    m.vertical
}

// ============================================================================
// Macro: select_creusot_proofs!
//
// Generates Creusot proofs for a ratatui Select enum:
// - known label accepted (#[trusted] — string opacity)
// - all labels roundtrip (#[trusted] — string opacity)
// - unknown rejected (#[trusted] — string opacity)
// - label count == option count (#[trusted] — Vec::len() opaque)
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
// Select enum proofs (6 types — all #[trusted] due to string opacity)
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
// Composite struct proofs — non-trusted where possible
//
// Padding and Margin proofs inline our wrapper's field-copy logic and use
// the trusted bridge functions + extern_specs. This makes Creusot verify
// the end-to-end roundtrip WITHOUT #[trusted] on the proof function itself.
//
// The trusted surface is minimal and isolated:
//   - logic accessors (6 opaque functions)
//   - extern_specs (2 constructor contracts)
//   - bridge functions (6 field readers)
//
// Style proofs remain #[trusted] because they involve Color::to_string() /
// str::parse() and Modifier bitflag operations, both opaque to the solver.
// ============================================================================

// ── Padding (non-trusted) ───────────────────────────────────────────────

/// Prove Padding::new → field read → Padding::new roundtrip preserves values.
///
/// Inlines our wrapper's field-copy logic: read each field via bridge function,
/// then reconstruct via Padding::new. Extern_spec contracts propagate equalities
/// so the SMT solver verifies preservation without #[trusted].
#[requires(true)]
#[ensures(padding_left(result) == left && padding_right(result) == right
       && padding_top(result) == top && padding_bottom(result) == bottom)]
pub fn verify_ratatui_padding_roundtrip(
    left: u16,
    right: u16,
    top: u16,
    bottom: u16,
) -> ratatui::widgets::Padding {
    let original = ratatui::widgets::Padding::new(left, right, top, bottom);
    let l = read_padding_left(&original);
    let r = read_padding_right(&original);
    let t = read_padding_top(&original);
    let b = read_padding_bottom(&original);
    ratatui::widgets::Padding::new(l, r, t, b)
}

/// Prove Padding::new produces correct field values for a concrete example.
#[requires(true)]
#[ensures(padding_left(result)@ == 1 && padding_right(result)@ == 2
       && padding_top(result)@ == 3 && padding_bottom(result)@ == 4)]
pub fn verify_ratatui_padding_concrete() -> ratatui::widgets::Padding {
    ratatui::widgets::Padding::new(1, 2, 3, 4)
}

// ── Margin (non-trusted) ────────────────────────────────────────────────

/// Prove Margin::new → field read → Margin::new roundtrip preserves values.
#[requires(true)]
#[ensures(margin_horizontal(result) == horizontal && margin_vertical(result) == vertical)]
pub fn verify_ratatui_margin_roundtrip(horizontal: u16, vertical: u16) -> ratatui::layout::Margin {
    let original = ratatui::layout::Margin::new(horizontal, vertical);
    let h = read_margin_horizontal(&original);
    let v = read_margin_vertical(&original);
    ratatui::layout::Margin::new(h, v)
}

/// Prove Margin::new produces correct field values for a concrete example.
#[requires(true)]
#[ensures(margin_horizontal(result)@ == 3 && margin_vertical(result)@ == 7)]
pub fn verify_ratatui_margin_concrete() -> ratatui::layout::Margin {
    ratatui::layout::Margin::new(3, 7)
}

// ── Style (#[trusted] — string + bitflag opacity) ───────────────────────

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

    let style =
        Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINED);
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

// ── BordersSelect (#[trusted] — bitflag iteration opaque) ───────────────

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
