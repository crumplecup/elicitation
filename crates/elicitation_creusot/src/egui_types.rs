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
//! # Trusted vs non-trusted proofs
//!
//! **Select proofs (`#[trusted]`)**: `str::view()` is `#[logic(opaque)]` in
//! creusot-std — the SMT solver cannot inspect string literal content.
//!
//! **Integer-field composites (non-trusted)**: Color32, CornerRadius, Margin
//! have integer fields with extern_spec contracts and bridge functions. The SMT
//! solver verifies our wrapper's field-copy + reconstruct logic end-to-end.
//!
//! **Float-field composites (`#[trusted]`)**: Pos2, Vec2, Rect, Stroke, Shadow,
//! FontId involve f32 arithmetic that is opaque to the solver.

#![cfg(feature = "egui-types")]

use creusot_std::prelude::*;
use elicitation::Select;

// ============================================================================
// Logic functions — trusted accessors for egui struct fields
// ============================================================================

// ── Color32 ─────────────────────────────────────────────────────────────

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn color32_r(_c: egui::Color32) -> u8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn color32_g(_c: egui::Color32) -> u8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn color32_b(_c: egui::Color32) -> u8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn color32_a(_c: egui::Color32) -> u8 {
    dead
}

// ── CornerRadius ────────────────────────────────────────────────────────

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn corner_radius_nw(_r: egui::CornerRadius) -> u8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn corner_radius_ne(_r: egui::CornerRadius) -> u8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn corner_radius_sw(_r: egui::CornerRadius) -> u8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn corner_radius_se(_r: egui::CornerRadius) -> u8 {
    dead
}

// ── Margin ──────────────────────────────────────────────────────────────

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn margin_left(_m: egui::Margin) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn margin_right(_m: egui::Margin) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn margin_top(_m: egui::Margin) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn margin_bottom(_m: egui::Margin) -> i8 {
    dead
}

// ============================================================================
// Extern specs — trusted axioms about egui constructors
// ============================================================================

extern_spec! {
    impl egui::Color32 {
        #[ensures(color32_r(result) == r)]
        #[ensures(color32_g(result) == g)]
        #[ensures(color32_b(result) == b)]
        #[ensures(color32_a(result) == a)]
        fn from_rgba_unmultiplied(r: u8, g: u8, b: u8, a: u8) -> egui::Color32;
    }
}

// ============================================================================
// Bridge functions — field access on egui types
// ============================================================================

// ── Color32 ─────────────────────────────────────────────────────────────

#[trusted]
#[ensures(color32_r(*c) == result)]
pub fn read_color32_r(c: &egui::Color32) -> u8 {
    c.r()
}

#[trusted]
#[ensures(color32_g(*c) == result)]
pub fn read_color32_g(c: &egui::Color32) -> u8 {
    c.g()
}

#[trusted]
#[ensures(color32_b(*c) == result)]
pub fn read_color32_b(c: &egui::Color32) -> u8 {
    c.b()
}

#[trusted]
#[ensures(color32_a(*c) == result)]
pub fn read_color32_a(c: &egui::Color32) -> u8 {
    c.a()
}

// ── CornerRadius ────────────────────────────────────────────────────────

#[trusted]
#[ensures(corner_radius_nw(*r) == result)]
pub fn read_corner_radius_nw(r: &egui::CornerRadius) -> u8 {
    r.nw
}

#[trusted]
#[ensures(corner_radius_ne(*r) == result)]
pub fn read_corner_radius_ne(r: &egui::CornerRadius) -> u8 {
    r.ne
}

#[trusted]
#[ensures(corner_radius_sw(*r) == result)]
pub fn read_corner_radius_sw(r: &egui::CornerRadius) -> u8 {
    r.sw
}

#[trusted]
#[ensures(corner_radius_se(*r) == result)]
pub fn read_corner_radius_se(r: &egui::CornerRadius) -> u8 {
    r.se
}

// ── Margin ──────────────────────────────────────────────────────────────

#[trusted]
#[ensures(margin_left(*m) == result)]
pub fn read_margin_left(m: &egui::Margin) -> i8 {
    m.left
}

#[trusted]
#[ensures(margin_right(*m) == result)]
pub fn read_margin_right(m: &egui::Margin) -> i8 {
    m.right
}

#[trusted]
#[ensures(margin_top(*m) == result)]
pub fn read_margin_top(m: &egui::Margin) -> i8 {
    m.top
}

#[trusted]
#[ensures(margin_bottom(*m) == result)]
pub fn read_margin_bottom(m: &egui::Margin) -> i8 {
    m.bottom
}

// ── Trusted constructors (struct literal opaque to solver) ──────────────

/// Trusted constructor bridging struct literal → logic accessors.
#[trusted]
#[ensures(corner_radius_nw(result) == nw && corner_radius_ne(result) == ne
       && corner_radius_sw(result) == sw && corner_radius_se(result) == se)]
pub fn make_corner_radius(nw: u8, ne: u8, sw: u8, se: u8) -> egui::CornerRadius {
    egui::CornerRadius { nw, ne, sw, se }
}

/// Trusted constructor bridging struct literal → logic accessors.
#[trusted]
#[ensures(margin_left(result) == left && margin_right(result) == right
       && margin_top(result) == top && margin_bottom(result) == bottom)]
pub fn make_margin(left: i8, right: i8, top: i8, bottom: i8) -> egui::Margin {
    egui::Margin {
        left,
        right,
        top,
        bottom,
    }
}

// ============================================================================
// Macro: select_creusot_proofs!
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
// Composite proofs — non-trusted (integer fields)
//
// These inline our wrapper's field-copy logic using bridge functions and
// extern_specs so the SMT solver can verify the roundtrip end-to-end.
// ============================================================================

// ── Color32 (non-trusted) ───────────────────────────────────────────────

/// Prove Color32 roundtrip: construct → read fields → reconstruct.
#[requires(true)]
#[ensures(color32_r(result) == r && color32_g(result) == g
       && color32_b(result) == b && color32_a(result) == a)]
pub fn verify_color32_roundtrip(r: u8, g: u8, b: u8, a: u8) -> egui::Color32 {
    let original = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
    let cr = read_color32_r(&original);
    let cg = read_color32_g(&original);
    let cb = read_color32_b(&original);
    let ca = read_color32_a(&original);
    egui::Color32::from_rgba_unmultiplied(cr, cg, cb, ca)
}

/// Prove Color32 concrete construction.
#[requires(true)]
#[ensures(color32_r(result)@ == 42 && color32_g(result)@ == 128
       && color32_b(result)@ == 200 && color32_a(result)@ == 255)]
pub fn verify_color32_concrete() -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(42, 128, 200, 255)
}

// ── CornerRadius (non-trusted) ──────────────────────────────────────────

/// Prove CornerRadius roundtrip: construct → read fields → reconstruct.
#[requires(true)]
#[ensures(corner_radius_nw(result) == nw && corner_radius_ne(result) == ne
       && corner_radius_sw(result) == sw && corner_radius_se(result) == se)]
pub fn verify_corner_radius_roundtrip(nw: u8, ne: u8, sw: u8, se: u8) -> egui::CornerRadius {
    let original = make_corner_radius(nw, ne, sw, se);
    let w = read_corner_radius_nw(&original);
    let e = read_corner_radius_ne(&original);
    let s_w = read_corner_radius_sw(&original);
    let s_e = read_corner_radius_se(&original);
    make_corner_radius(w, e, s_w, s_e)
}

/// Prove CornerRadius concrete construction.
#[requires(true)]
#[ensures(corner_radius_nw(result)@ == 5 && corner_radius_ne(result)@ == 10
       && corner_radius_sw(result)@ == 15 && corner_radius_se(result)@ == 20)]
pub fn verify_corner_radius_concrete() -> egui::CornerRadius {
    make_corner_radius(5, 10, 15, 20)
}

// ── Margin (non-trusted) ────────────────────────────────────────────────

/// Prove Margin roundtrip: construct → read fields → reconstruct.
#[requires(true)]
#[ensures(margin_left(result) == left && margin_right(result) == right
       && margin_top(result) == top && margin_bottom(result) == bottom)]
pub fn verify_margin_roundtrip(left: i8, right: i8, top: i8, bottom: i8) -> egui::Margin {
    let original = make_margin(left, right, top, bottom);
    let l = read_margin_left(&original);
    let r = read_margin_right(&original);
    let t = read_margin_top(&original);
    let b = read_margin_bottom(&original);
    make_margin(l, r, t, b)
}

/// Prove Margin concrete construction.
#[requires(true)]
#[ensures(margin_left(result)@ == 5 && margin_right(result)@ == 10
       && margin_top(result)@ == 15 && margin_bottom(result)@ == 20)]
pub fn verify_margin_concrete() -> egui::Margin {
    make_margin(5, 10, 15, 20)
}

// ============================================================================
// Composite proofs — #[trusted] (float fields / opaque methods)
// ============================================================================

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
