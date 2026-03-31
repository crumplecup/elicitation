//! Kani proofs for ratatui type elicitation.
//!
//! Available with the `ratatui` feature.
//!
//! # Proof Strategy
//!
//! ## Select enums (6 types)
//!
//! For each `Select` enum type we verify:
//! 1. **Label count**: `labels().len() == options().len()`
//! 2. **Roundtrip**: every label produced by `labels()` is accepted by `from_label()`
//! 3. **Unknown rejection**: `from_label("__unknown__")` returns `None`
//!
//! ## Composite structs (3 types)
//!
//! For each composite wrapper we verify:
//! 1. **From roundtrip**: `Wrapper::from(ratatui_type)` then back preserves values
//! 2. **Field preservation**: individual fields survive the conversion

#[cfg(feature = "ratatui")]
use elicitation::Select;

// ============================================================================
// Macro: select_proofs!
//
// Generates label-count, all-labels-roundtrip, and unknown-rejection proofs
// for a Select enum type with a known variant count.
// ============================================================================

/// Full select proofs: label_count + all_labels_roundtrip + unknown_rejected.
///
/// Use for enums with ≤8 variants where string comparison doesn't cause
/// unbounded unwinding in Kani.
macro_rules! select_proofs {
    (
        type_path    = $ty:ty,
        snake        = $snake:ident,
        variant_count = $count:literal
    ) => {
        select_proofs_fast!(type_path = $ty, snake = $snake, variant_count = $count);
        ::paste::paste! {
            #[cfg(feature = "ratatui")]
            #[kani::proof]
            fn [< verify_ $snake _all_labels_roundtrip >]() {
                let labels = <$ty>::labels();
                for label in &labels {
                    let result = <$ty>::from_label(label);
                    assert!(
                        result.is_some(),
                        concat!(stringify!($snake), " label roundtrips")
                    );
                }
            }
        }
    };
}

/// Lightweight select proofs: label_count + unknown_rejected only.
///
/// Use for enums with many variants (>8) where the all-labels loop causes
/// Kani to hit unbounded string-comparison unwinding.
macro_rules! select_proofs_fast {
    (
        type_path    = $ty:ty,
        snake        = $snake:ident,
        variant_count = $count:literal
    ) => {
        ::paste::paste! {
            #[cfg(feature = "ratatui")]
            #[kani::proof]
            fn [< verify_ $snake _label_count >]() {
                let labels = <$ty>::labels();
                let options = <$ty>::options();
                assert!(
                    labels.len() == options.len(),
                    concat!(stringify!($snake), ": labels and options have equal length")
                );
                assert!(
                    labels.len() == $count,
                    concat!(stringify!($snake), " has ", stringify!($count), " variants")
                );
            }

            #[cfg(feature = "ratatui")]
            #[kani::proof]
            fn [< verify_ $snake _unknown_rejected >]() {
                let result = <$ty>::from_label("__unknown__");
                assert!(
                    result.is_none(),
                    concat!(stringify!($snake), " rejects unknown labels")
                );
            }
        }
    };
}

// ============================================================================
// Select enum proofs
// ============================================================================

select_proofs!(
    type_path     = ratatui::layout::Alignment,
    snake         = ratatui_alignment,
    variant_count = 3
);

select_proofs!(
    type_path     = ratatui::layout::Direction,
    snake         = ratatui_direction,
    variant_count = 2
);

select_proofs!(
    type_path     = ratatui::widgets::BorderType,
    snake         = ratatui_border_type,
    variant_count = 4
);

// Color has 19 entries: 17 named + Reset + 2 sentinel entries (RGB, Indexed).
// Uses select_proofs_fast! for label_count/unknown_rejected, plus bucketed
// roundtrip proofs below that partition labels by byte length to avoid
// unbounded string-comparison unwinding.
select_proofs_fast!(
    type_path     = ratatui::style::Color,
    snake         = ratatui_color,
    variant_count = 19
);

// ── Color all-labels roundtrip, bucketed by byte length ─────────────────
//
// Iterating all 19 labels in one loop causes Kani's CBMC backend to unwind
// endlessly (19 labels × 19 match arms × variable-length byte comparison).
//
// Fix: partition labels by byte length. Each proof handles a small bucket
// (1–4 labels) with a known max string length, so both the loop bound and
// the byte-comparison depth are bounded.
//
// Buckets:
//   3B: "Red"
//   4B: "Blue", "Cyan", "Gray"
//   5B: "Reset", "Black", "Green", "White"
//   6B: "Yellow"
//   7B: "Magenta"
//   8B: "DarkGray", "LightRed"
//   9B: "LightBlue", "LightCyan"
//  10B: "LightGreen"
//  11B: "LightYellow"
//  12B: "LightMagenta"
//  16B: "RGB (custom)…"         (… is 3 UTF-8 bytes)
//  20B: "Indexed (0–255)…"      (… is 3 UTF-8 bytes)

macro_rules! color_bucket_proof {
    ($name:ident, [ $($label:literal),+ $(,)? ]) => {
        #[cfg(feature = "ratatui")]
        #[kani::proof]
        fn $name() {
            use ratatui::style::Color;
            $(
                let result = <Color as elicitation::Select>::from_label($label);
                assert!(
                    result.is_some(),
                    concat!("Color::from_label(\"", $label, "\") must succeed")
                );
            )+
        }
    };
}

color_bucket_proof!(verify_ratatui_color_roundtrip_3b, ["Red"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_4b, ["Blue", "Cyan", "Gray"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_5b, ["Reset", "Black", "Green", "White"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_6b, ["Yellow"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_7b, ["Magenta"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_8b, ["DarkGray", "LightRed"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_9b, ["LightBlue", "LightCyan"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_10b, ["LightGreen"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_11b, ["LightYellow"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_12b, ["LightMagenta"]);
color_bucket_proof!(verify_ratatui_color_roundtrip_sentinel, ["RGB (custom)\u{2026}", "Indexed (0\u{2013}255)\u{2026}"]);

// Borders uses bitflag presets — 8 common combinations
select_proofs!(
    type_path     = ratatui::widgets::Borders,
    snake         = ratatui_borders,
    variant_count = 8
);

select_proofs!(
    type_path     = ratatui::widgets::ScrollbarOrientation,
    snake         = ratatui_scrollbar_orientation,
    variant_count = 4
);

// ============================================================================
// Composite struct proofs — From roundtrip verification
//
// For each composite wrapper (RatatuiStyle, RatatuiPadding, RatatuiMargin) we
// verify that the From conversions preserve all field values. The third-party
// ratatui types are TRUSTED — we verify only OUR wrapper logic.
// ============================================================================

// ── Padding ─────────────────────────────────────────────────────────────

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_ratatui_padding_from_roundtrip() {
    let left: u16 = kani::any();
    let right: u16 = kani::any();
    let top: u16 = kani::any();
    let bottom: u16 = kani::any();

    let original = ratatui::widgets::Padding::new(left, right, top, bottom);
    let wrapper = elicitation::RatatuiPadding::from(original);
    let restored: ratatui::widgets::Padding = wrapper.into();

    assert!(restored.left == left, "Padding left preserved");
    assert!(restored.right == right, "Padding right preserved");
    assert!(restored.top == top, "Padding top preserved");
    assert!(restored.bottom == bottom, "Padding bottom preserved");
}

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_ratatui_padding_wrapper_fields() {
    let left: u16 = kani::any();
    let right: u16 = kani::any();
    let top: u16 = kani::any();
    let bottom: u16 = kani::any();

    let original = ratatui::widgets::Padding::new(left, right, top, bottom);
    let wrapper = elicitation::RatatuiPadding::from(original);

    assert!(wrapper.left == left, "Padding wrapper.left matches input");
    assert!(wrapper.right == right, "Padding wrapper.right matches input");
    assert!(wrapper.top == top, "Padding wrapper.top matches input");
    assert!(wrapper.bottom == bottom, "Padding wrapper.bottom matches input");
}

// ── Margin ──────────────────────────────────────────────────────────────

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_ratatui_margin_from_roundtrip() {
    let horizontal: u16 = kani::any();
    let vertical: u16 = kani::any();

    let original = ratatui::layout::Margin::new(horizontal, vertical);
    let wrapper = elicitation::RatatuiMargin::from(original);
    let restored: ratatui::layout::Margin = wrapper.into();

    assert!(restored.horizontal == horizontal, "Margin horizontal preserved");
    assert!(restored.vertical == vertical, "Margin vertical preserved");
}

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_ratatui_margin_wrapper_fields() {
    let horizontal: u16 = kani::any();
    let vertical: u16 = kani::any();

    let original = ratatui::layout::Margin::new(horizontal, vertical);
    let wrapper = elicitation::RatatuiMargin::from(original);

    assert!(wrapper.horizontal == horizontal, "Margin wrapper.horizontal matches");
    assert!(wrapper.vertical == vertical, "Margin wrapper.vertical matches");
}

// ── Style (complex — string-based color, bitflag modifiers) ─────────────
//
// Kani CANNOT verify string formatting/parsing (Color::to_string / FromStr)
// because those involve unbounded loops. We verify:
// 1. Empty style roundtrip (no strings)
// 2. Modifier boolean mapping (bool ↔ Modifier bitflag)
// 3. fg/bg Option presence preservation (Some vs None, without string content)
//
// String color roundtrip correctness is covered by regular tests in
// ratatui_types_test.rs (verify_ratatui_style_all_named_colors_roundtrip etc.)

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_ratatui_style_empty_roundtrip() {
    use ratatui::style::Style;

    let style = Style::default();
    let wrapper = elicitation::RatatuiStyle::from(style);

    assert!(wrapper.fg.is_none(), "Default style has no fg");
    assert!(wrapper.bg.is_none(), "Default style has no bg");
    assert!(!wrapper.bold, "Default style not bold");
    assert!(!wrapper.italic, "Default style not italic");
    assert!(!wrapper.underlined, "Default style not underlined");

    let restored: Style = wrapper.try_into().expect("valid style");
    assert!(restored.fg.is_none(), "Restored default has no fg");
    assert!(restored.bg.is_none(), "Restored default has no bg");
    assert!(restored.add_modifier.is_empty(), "Restored default has no modifiers");
}

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_ratatui_style_all_modifiers() {
    use ratatui::style::{Modifier, Style};

    let style = Style::default()
        .add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINED);

    let wrapper = elicitation::RatatuiStyle::from(style);

    assert!(wrapper.bold, "BOLD mapped to bold=true");
    assert!(wrapper.italic, "ITALIC mapped to italic=true");
    assert!(wrapper.underlined, "UNDERLINED mapped to underlined=true");

    let restored: Style = wrapper.try_into().expect("valid style");
    assert!(restored.add_modifier.contains(Modifier::BOLD), "BOLD roundtrips");
    assert!(restored.add_modifier.contains(Modifier::ITALIC), "ITALIC roundtrips");
    assert!(restored.add_modifier.contains(Modifier::UNDERLINED), "UNDERLINED roundtrips");
}

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_ratatui_style_modifier_combinations() {
    use ratatui::style::{Modifier, Style};

    // Verify each modifier independently maps to the correct boolean
    let bold_only = Style::default().add_modifier(Modifier::BOLD);
    let w = elicitation::RatatuiStyle::from(bold_only);
    assert!(w.bold && !w.italic && !w.underlined, "BOLD only");

    let italic_only = Style::default().add_modifier(Modifier::ITALIC);
    let w = elicitation::RatatuiStyle::from(italic_only);
    assert!(!w.bold && w.italic && !w.underlined, "ITALIC only");

    let underline_only = Style::default().add_modifier(Modifier::UNDERLINED);
    let w = elicitation::RatatuiStyle::from(underline_only);
    assert!(!w.bold && !w.italic && w.underlined, "UNDERLINED only");
}

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_ratatui_style_fg_bg_presence() {
    use ratatui::style::{Color, Style};

    // fg=Some, bg=None
    let s1 = Style::default().fg(Color::Red);
    let w1 = elicitation::RatatuiStyle::from(s1);
    assert!(w1.fg.is_some(), "fg=Some preserved");
    assert!(w1.bg.is_none(), "bg=None preserved");

    // fg=None, bg=Some
    let s2 = Style::default().bg(Color::Blue);
    let w2 = elicitation::RatatuiStyle::from(s2);
    assert!(w2.fg.is_none(), "fg=None preserved");
    assert!(w2.bg.is_some(), "bg=Some preserved");

    // fg=Some, bg=Some
    let s3 = Style::default().fg(Color::Green).bg(Color::Yellow);
    let w3 = elicitation::RatatuiStyle::from(s3);
    assert!(w3.fg.is_some(), "Both fg preserved");
    assert!(w3.bg.is_some(), "Both bg preserved");
}

// ── Borders wrapper ─────────────────────────────────────────────────────

#[cfg(feature = "ratatui")]
#[kani::proof]
fn verify_borders_select_into_inner_roundtrip() {
    use ratatui::widgets::Borders;

    let presets = [
        Borders::NONE,
        Borders::ALL,
        Borders::TOP,
        Borders::BOTTOM,
        Borders::LEFT,
        Borders::RIGHT,
        Borders::TOP | Borders::BOTTOM,
        Borders::LEFT | Borders::RIGHT,
    ];

    for borders in presets {
        let select = elicitation::BordersSelect::from(borders);
        let inner = select.into_inner();
        assert!(inner == borders, "BordersSelect preserves inner Borders value");
    }
}
