//! Kani proofs for egui type elicitation.
//!
//! Available with the `egui-types` feature.
//!
//! # Proof Strategy
//!
//! ## Select enums (16 types)
//!
//! For each `Select` enum type we verify:
//! 1. **Label count**: `labels().len() == options().len()`
//! 2. **Roundtrip**: every label produced by `labels()` is accepted by `from_label()`
//! 3. **Unknown rejection**: `from_label("__unknown__")` returns `None`
//!
//! ## Composite structs (9 types)
//!
//! For each composite wrapper we verify:
//! 1. **From roundtrip**: `Wrapper::from(egui_type)` then back preserves values
//! 2. **Field preservation**: individual fields survive the conversion

#[cfg(feature = "egui-types")]
use elicitation::Select;

// ============================================================================
// Macro: select_proofs!
//
// Generates label-count, all-labels-roundtrip, and unknown-rejection proofs
// for a Select enum type with a known variant count.
// ============================================================================

macro_rules! select_proofs {
    (
        type_path    = $ty:ty,
        snake        = $snake:ident,
        variant_count = $count:literal
    ) => {
        ::paste::paste! {
            #[cfg(feature = "egui-types")]
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

            #[cfg(feature = "egui-types")]
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

            #[cfg(feature = "egui-types")]
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
// Select enum proofs — Tier 1 (small enums, 2–6 variants)
// ============================================================================

select_proofs!(type_path = egui::Align, snake = align, variant_count = 3);
select_proofs!(
    type_path = egui::Direction,
    snake = direction,
    variant_count = 4
);
select_proofs!(type_path = egui::Theme, snake = theme, variant_count = 2);
select_proofs!(
    type_path = egui::ThemePreference,
    snake = theme_preference,
    variant_count = 3
);
select_proofs!(
    type_path = egui::FontFamily,
    snake = font_family,
    variant_count = 2
);
select_proofs!(
    type_path = egui::TextWrapMode,
    snake = text_wrap_mode,
    variant_count = 3
);
select_proofs!(
    type_path = egui::TouchPhase,
    snake = touch_phase,
    variant_count = 4
);
select_proofs!(
    type_path = egui::PointerButton,
    snake = pointer_button,
    variant_count = 5
);
select_proofs!(type_path = egui::Order, snake = order, variant_count = 5);

select_proofs!(
    type_path = egui::epaint::textures::TextureFilter,
    snake = texture_filter,
    variant_count = 2
);
select_proofs!(
    type_path = egui::epaint::textures::TextureWrapMode,
    snake = texture_wrap_mode,
    variant_count = 3
);

// ============================================================================
// Select enum proofs — Tier 2 (medium enums, 7–20 variants)
// ============================================================================

select_proofs!(
    type_path = egui::TextStyle,
    snake = text_style,
    variant_count = 5
);
select_proofs!(
    type_path = egui::UiKind,
    snake = ui_kind,
    variant_count = 17
);
select_proofs!(
    type_path = egui::WidgetType,
    snake = widget_type,
    variant_count = 18
);

// ============================================================================
// Select enum proofs — Tier 3 (large enums, 35–103 variants)
// ============================================================================

select_proofs!(
    type_path = egui::CursorIcon,
    snake = cursor_icon,
    variant_count = 35
);
select_proofs!(type_path = egui::Key, snake = key, variant_count = 103);

// ============================================================================
// Composite struct proofs — From roundtrip verification
//
// For each composite wrapper (EguiColor32, EguiPos2, etc.) we verify that
// the From<egui::T> → EguiT → From<EguiT> → egui::T roundtrip preserves
// all field values.
// ============================================================================

// ── Color32 ─────────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_color32_from_roundtrip() {
    let r: u8 = kani::any();
    let g: u8 = kani::any();
    let b: u8 = kani::any();
    let a: u8 = kani::any();

    let original = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
    let wrapper = elicitation::EguiColor32::from(original);
    let restored: egui::Color32 = wrapper.into();

    assert!(restored.r() == r, "Color32 red channel preserved");
    assert!(restored.g() == g, "Color32 green channel preserved");
    assert!(restored.b() == b, "Color32 blue channel preserved");
    assert!(restored.a() == a, "Color32 alpha channel preserved");
}

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_color32_wrapper_fields() {
    let r: u8 = kani::any();
    let g: u8 = kani::any();
    let b: u8 = kani::any();
    let a: u8 = kani::any();

    let original = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
    let wrapper = elicitation::EguiColor32::from(original);

    assert!(wrapper.r == r, "Color32 wrapper.r matches input");
    assert!(wrapper.g == g, "Color32 wrapper.g matches input");
    assert!(wrapper.b == b, "Color32 wrapper.b matches input");
    assert!(wrapper.a == a, "Color32 wrapper.a matches input");
}

// ── Pos2 ────────────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_pos2_from_roundtrip() {
    let x: f32 = kani::any();
    let y: f32 = kani::any();
    kani::assume(x.is_finite());
    kani::assume(y.is_finite());

    let original = egui::Pos2::new(x, y);
    let wrapper = elicitation::EguiPos2::from(original);
    let restored: egui::Pos2 = wrapper.into();

    assert!(restored.x == x, "Pos2 x preserved");
    assert!(restored.y == y, "Pos2 y preserved");
}

// ── Vec2 ────────────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_vec2_from_roundtrip() {
    let x: f32 = kani::any();
    let y: f32 = kani::any();
    kani::assume(x.is_finite());
    kani::assume(y.is_finite());

    let original = egui::Vec2::new(x, y);
    let wrapper = elicitation::EguiVec2::from(original);
    let restored: egui::Vec2 = wrapper.into();

    assert!(restored.x == x, "Vec2 x preserved");
    assert!(restored.y == y, "Vec2 y preserved");
}

// ── Rect ────────────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_rect_from_roundtrip() {
    let min_x: f32 = kani::any();
    let min_y: f32 = kani::any();
    let max_x: f32 = kani::any();
    let max_y: f32 = kani::any();
    kani::assume(min_x.is_finite());
    kani::assume(min_y.is_finite());
    kani::assume(max_x.is_finite());
    kani::assume(max_y.is_finite());

    let original =
        egui::Rect::from_min_max(egui::Pos2::new(min_x, min_y), egui::Pos2::new(max_x, max_y));
    let wrapper = elicitation::EguiRect::from(original);
    let restored: egui::Rect = wrapper.into();

    assert!(restored.min.x == min_x, "Rect min.x preserved");
    assert!(restored.min.y == min_y, "Rect min.y preserved");
    assert!(restored.max.x == max_x, "Rect max.x preserved");
    assert!(restored.max.y == max_y, "Rect max.y preserved");
}

// ── Stroke ──────────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_stroke_from_roundtrip() {
    let width: f32 = kani::any();
    let r: u8 = kani::any();
    let g: u8 = kani::any();
    let b: u8 = kani::any();
    let a: u8 = kani::any();
    kani::assume(width.is_finite());

    let color = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
    let original = egui::Stroke::new(width, color);
    let wrapper = elicitation::EguiStroke::from(original);
    let restored: egui::Stroke = wrapper.into();

    assert!(restored.width == width, "Stroke width preserved");
    assert!(restored.color.r() == r, "Stroke color.r preserved");
    assert!(restored.color.g() == g, "Stroke color.g preserved");
    assert!(restored.color.b() == b, "Stroke color.b preserved");
    assert!(restored.color.a() == a, "Stroke color.a preserved");
}

// ── CornerRadius ────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_corner_radius_from_roundtrip() {
    let nw: u8 = kani::any();
    let ne: u8 = kani::any();
    let sw: u8 = kani::any();
    let se: u8 = kani::any();

    let original = egui::CornerRadius { nw, ne, sw, se };
    let wrapper = elicitation::EguiCornerRadius::from(original);
    let restored: egui::CornerRadius = wrapper.into();

    assert!(restored.nw == nw, "CornerRadius nw preserved");
    assert!(restored.ne == ne, "CornerRadius ne preserved");
    assert!(restored.sw == sw, "CornerRadius sw preserved");
    assert!(restored.se == se, "CornerRadius se preserved");
}

// ── Shadow ──────────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_shadow_from_roundtrip() {
    let offset_x: i8 = kani::any();
    let offset_y: i8 = kani::any();
    let blur: u8 = kani::any();
    let spread: u8 = kani::any();
    let r: u8 = kani::any();
    let g: u8 = kani::any();
    let b: u8 = kani::any();
    let a: u8 = kani::any();

    let color = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
    let original = egui::Shadow {
        offset: [offset_x, offset_y],
        blur,
        spread,
        color,
    };
    let wrapper = elicitation::EguiShadow::from(original);
    let restored: egui::Shadow = wrapper.into();

    assert!(restored.offset[0] == offset_x, "Shadow offset_x preserved");
    assert!(restored.offset[1] == offset_y, "Shadow offset_y preserved");
    assert!(restored.blur == blur, "Shadow blur preserved");
    assert!(restored.spread == spread, "Shadow spread preserved");
    assert!(restored.color.r() == r, "Shadow color.r preserved");
}

// ── Margin ──────────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_margin_from_roundtrip() {
    let left: i8 = kani::any();
    let right: i8 = kani::any();
    let top: i8 = kani::any();
    let bottom: i8 = kani::any();

    let original = egui::Margin {
        left,
        right,
        top,
        bottom,
    };
    let wrapper = elicitation::EguiMargin::from(original);
    let restored: egui::Margin = wrapper.into();

    assert!(restored.left == left, "Margin left preserved");
    assert!(restored.right == right, "Margin right preserved");
    assert!(restored.top == top, "Margin top preserved");
    assert!(restored.bottom == bottom, "Margin bottom preserved");
}

// ── FontId ──────────────────────────────────────────────────────────────

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_font_id_monospace_roundtrip() {
    let size: f32 = kani::any();
    kani::assume(size.is_finite());

    let original = egui::FontId::monospace(size);
    let wrapper = elicitation::EguiFontId::from(original);
    let restored: egui::FontId = wrapper.into();

    assert!(restored.size == size, "FontId size preserved");
    assert!(
        restored.family == egui::FontFamily::Monospace,
        "FontId Monospace family preserved"
    );
}

#[cfg(feature = "egui-types")]
#[kani::proof]
fn verify_font_id_proportional_roundtrip() {
    let size: f32 = kani::any();
    kani::assume(size.is_finite());

    let original = egui::FontId::proportional(size);
    let wrapper = elicitation::EguiFontId::from(original);
    let restored: egui::FontId = wrapper.into();

    assert!(restored.size == size, "FontId size preserved");
    assert!(
        restored.family == egui::FontFamily::Proportional,
        "FontId Proportional family preserved"
    );
}
