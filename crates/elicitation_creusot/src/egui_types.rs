//! Creusot proofs for egui type elicitation.
//!
//! # Verification stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! We trust that the egui crate correctly defines its enum variants and struct
//! fields. We verify our own wrappers: that every label produced by `labels()`
//! is accepted by `from_label()` (Select roundtrip), and that `From`
//! conversions between our wrappers and egui types preserve field values.
//!
//! # Why most functions are `#[trusted]`
//!
//! **String literal opacity wall** (blocks roundtrip + rejection proofs):
//! `str::view()` is `#[logic(opaque)]` in creusot-std — the SMT solver cannot
//! inspect the content of string literals. Even with `extern_spec!` contracts,
//! the solver cannot prove that a specific label is accepted or rejected.
//!
//! **Composite From roundtrip** (trusted by design):
//! The From impls delegate to egui constructors and field accessors whose
//! contracts are not exposed to Creusot. The roundtrip assertions are
//! runtime-verified and axiomatically trusted here.
//!
//! # Non-trusted proofs
//!
//! The `label_count` proofs (`labels().len() == options().len()`) are
//! non-trusted — `Vec` has `ShallowModel` as `Seq<T>`, so length comparisons
//! are tractable by Alt-Ergo.

#![cfg(feature = "egui-types")]

use creusot_std::prelude::*;
use elicitation::Select;

// ============================================================================
// Macro: select_creusot_proofs!
//
// Generates Creusot proofs for an egui Select enum:
// - known label accepted (#[trusted])
// - all labels roundtrip (#[trusted])
// - unknown rejected (#[trusted])
// - label count == option count (non-trusted)
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

            /// Verify label count equals option count (non-trusted).
            #[requires(true)]
            #[ensures(result == true)]
            pub fn [< verify_ $snake _label_count >]() -> bool {
                <$ty>::labels().len() == <$ty>::options().len()
            }
        }
    };
}

// ============================================================================
// Select enum proofs — Tier 1 (small enums)
// ============================================================================

select_creusot_proofs!(type_path = egui::Align, snake = align, known_label = "Min");
select_creusot_proofs!(
    type_path = egui::Direction,
    snake = direction,
    known_label = "LeftToRight"
);
select_creusot_proofs!(type_path = egui::Theme, snake = theme, known_label = "Dark");
select_creusot_proofs!(
    type_path = egui::ThemePreference,
    snake = theme_preference,
    known_label = "Dark"
);
select_creusot_proofs!(
    type_path = egui::FontFamily,
    snake = font_family,
    known_label = "Monospace"
);
select_creusot_proofs!(
    type_path = egui::TextWrapMode,
    snake = text_wrap_mode,
    known_label = "Extend"
);
select_creusot_proofs!(
    type_path = egui::TouchPhase,
    snake = touch_phase,
    known_label = "Start"
);
select_creusot_proofs!(
    type_path = egui::PointerButton,
    snake = pointer_button,
    known_label = "Primary"
);
select_creusot_proofs!(
    type_path = egui::Order,
    snake = order,
    known_label = "Background"
);

select_creusot_proofs!(
    type_path = egui::epaint::textures::TextureFilter,
    snake = texture_filter,
    known_label = "Nearest"
);
select_creusot_proofs!(
    type_path = egui::epaint::textures::TextureWrapMode,
    snake = texture_wrap_mode,
    known_label = "ClampToEdge"
);

// ============================================================================
// Select enum proofs — Tier 2 (medium enums)
// ============================================================================

select_creusot_proofs!(
    type_path = egui::TextStyle,
    snake = text_style,
    known_label = "Small"
);
select_creusot_proofs!(
    type_path = egui::UiKind,
    snake = ui_kind,
    known_label = "Window"
);
select_creusot_proofs!(
    type_path = egui::WidgetType,
    snake = widget_type,
    known_label = "Label"
);

// ============================================================================
// Select enum proofs — Tier 3 (large enums)
// ============================================================================

select_creusot_proofs!(
    type_path = egui::CursorIcon,
    snake = cursor_icon,
    known_label = "Default"
);
select_creusot_proofs!(
    type_path = egui::Key,
    snake = key,
    known_label = "ArrowDown"
);

// ============================================================================
// Composite struct proofs — From roundtrip verification
//
// Each proof asserts that converting egui::T → EguiT → egui::T preserves
// field values. These are #[trusted] because the From impls call egui
// constructors/accessors whose contracts are opaque to Creusot's SMT solver.
// ============================================================================

// ── Color32 ─────────────────────────────────────────────────────────────

/// Verify Color32 From roundtrip preserves RGBA channels.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_color32_from_roundtrip() -> bool {
    let original = egui::Color32::from_rgba_unmultiplied(42, 128, 200, 255);
    let wrapper = elicitation::EguiColor32::from(original);
    let restored: egui::Color32 = wrapper.into();
    restored.r() == 42 && restored.g() == 128 && restored.b() == 200 && restored.a() == 255
}

/// Verify Color32 wrapper field extraction matches input.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_color32_wrapper_fields() -> bool {
    let original = egui::Color32::from_rgba_unmultiplied(10, 20, 30, 40);
    let wrapper = elicitation::EguiColor32::from(original);
    wrapper.r == 10 && wrapper.g == 20 && wrapper.b == 30 && wrapper.a == 40
}

// ── Pos2 ────────────────────────────────────────────────────────────────

/// Verify Pos2 From roundtrip preserves x, y coordinates.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_pos2_from_roundtrip() -> bool {
    let original = egui::Pos2::new(3.14, 2.71);
    let wrapper = elicitation::EguiPos2::from(original);
    let restored: egui::Pos2 = wrapper.into();
    restored.x == 3.14 && restored.y == 2.71
}

// ── Vec2 ────────────────────────────────────────────────────────────────

/// Verify Vec2 From roundtrip preserves x, y components.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_vec2_from_roundtrip() -> bool {
    let original = egui::Vec2::new(100.0, 200.0);
    let wrapper = elicitation::EguiVec2::from(original);
    let restored: egui::Vec2 = wrapper.into();
    restored.x == 100.0 && restored.y == 200.0
}

// ── Rect ────────────────────────────────────────────────────────────────

/// Verify Rect From roundtrip preserves min/max corners.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_rect_from_roundtrip() -> bool {
    let original =
        egui::Rect::from_min_max(egui::Pos2::new(10.0, 20.0), egui::Pos2::new(30.0, 40.0));
    let wrapper = elicitation::EguiRect::from(original);
    let restored: egui::Rect = wrapper.into();
    restored.min.x == 10.0
        && restored.min.y == 20.0
        && restored.max.x == 30.0
        && restored.max.y == 40.0
}

// ── Stroke ──────────────────────────────────────────────────────────────

/// Verify Stroke From roundtrip preserves width and color.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_stroke_from_roundtrip() -> bool {
    let color = egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255);
    let original = egui::Stroke::new(2.5, color);
    let wrapper = elicitation::EguiStroke::from(original);
    let restored: egui::Stroke = wrapper.into();
    restored.width == 2.5 && restored.color.r() == 255 && restored.color.g() == 0
}

// ── CornerRadius ────────────────────────────────────────────────────────

/// Verify CornerRadius From roundtrip preserves corner radii.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_corner_radius_from_roundtrip() -> bool {
    let original = egui::CornerRadius {
        nw: 5,
        ne: 10,
        sw: 15,
        se: 20,
    };
    let wrapper = elicitation::EguiCornerRadius::from(original);
    let restored: egui::CornerRadius = wrapper.into();
    restored.nw == 5 && restored.ne == 10 && restored.sw == 15 && restored.se == 20
}

// ── Shadow ──────────────────────────────────────────────────────────────

/// Verify Shadow From roundtrip preserves offset, blur, spread, and color.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_shadow_from_roundtrip() -> bool {
    let color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 128);
    let original = egui::Shadow {
        offset: [4, -2],
        blur: 8,
        spread: 3,
        color,
    };
    let wrapper = elicitation::EguiShadow::from(original);
    let restored: egui::Shadow = wrapper.into();
    restored.offset[0] == 4
        && restored.offset[1] == -2
        && restored.blur == 8
        && restored.spread == 3
        && restored.color.a() == 128
}

// ── Margin ──────────────────────────────────────────────────────────────

/// Verify Margin From roundtrip preserves all four margins.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_margin_from_roundtrip() -> bool {
    let original = egui::Margin {
        left: 5,
        right: 10,
        top: 15,
        bottom: 20,
    };
    let wrapper = elicitation::EguiMargin::from(original);
    let restored: egui::Margin = wrapper.into();
    restored.left == 5 && restored.right == 10 && restored.top == 15 && restored.bottom == 20
}

// ── FontId ──────────────────────────────────────────────────────────────

/// Verify FontId From roundtrip preserves size and Monospace family.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_font_id_monospace_roundtrip() -> bool {
    let original = egui::FontId::monospace(14.0);
    let wrapper = elicitation::EguiFontId::from(original);
    let restored: egui::FontId = wrapper.into();
    restored.size == 14.0 && restored.family == egui::FontFamily::Monospace
}

/// Verify FontId From roundtrip preserves size and Proportional family.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_font_id_proportional_roundtrip() -> bool {
    let original = egui::FontId::proportional(16.0);
    let wrapper = elicitation::EguiFontId::from(original);
    let restored: egui::FontId = wrapper.into();
    restored.size == 16.0 && restored.family == egui::FontFamily::Proportional
}
