//! Systematic API surface coverage for `elicit_bevy`.
//!
//! Each test imports types from `elicit_bevy` using the exact upstream Bevy
//! name (e.g. `Val`, not `BevyVal`) and verifies that construction, conversion,
//! and key methods work correctly.  Compile errors mean a shadow is missing or
//! misnamed; assertion failures mean a method exists but behaves incorrectly.
//!
//! ## Known gaps (compile-blocked; tracked separately)
//!
//! - `Node` (UI layout) — no shadow at all; `bevy::ui::Node` is the struct
//!   with `position_type`, `top`, `left`, `padding`, etc.
//! - `ComputedNode` — no shadow
//! - `Commands`, `Query`, `Res`, `ResMut` — ECS system params, no shadow yet
//! - `Startup`, `Update` — schedule labels, no shadow yet
//! - `FrameCount` — diagnostic resource, no shadow yet

use elicit_bevy::{
    AlignItems, BackgroundColor, BorderRadius, Color, Display, FlexDirection, FlexWrap,
    GlobalZIndex, IsDefaultUiCamera, JustifyContent, OverflowAxis, PositionType, Text, TextColor,
    TextFont, UiRect, Val, ZIndex,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn px(v: f32) -> Val {
    Val::from(bevy::ui::Val::Px(v))
}

fn percent(v: f32) -> Val {
    Val::from(bevy::ui::Val::Percent(v))
}

fn auto() -> Val {
    Val::from(bevy::ui::Val::Auto)
}

// ── Val ───────────────────────────────────────────────────────────────────────

#[test]
fn val_px_construction() {
    let v = px(12.0);
    let raw: bevy::ui::Val = v.into();
    assert_eq!(raw, bevy::ui::Val::Px(12.0));
}

#[test]
fn val_percent_construction() {
    let v = percent(50.0);
    let raw: bevy::ui::Val = v.into();
    assert_eq!(raw, bevy::ui::Val::Percent(50.0));
}

#[test]
fn val_auto_construction() {
    let v = auto();
    let raw: bevy::ui::Val = v.into();
    assert_eq!(raw, bevy::ui::Val::Auto);
}

#[test]
fn val_roundtrip() {
    for raw in [
        bevy::ui::Val::Px(1.0),
        bevy::ui::Val::Percent(100.0),
        bevy::ui::Val::Auto,
        bevy::ui::Val::Vw(50.0),
        bevy::ui::Val::Vh(50.0),
    ] {
        let shadow = Val::from(raw);
        assert_eq!(bevy::ui::Val::from(shadow), raw);
    }
}

// ── UiRect ────────────────────────────────────────────────────────────────────

#[test]
fn ui_rect_all_roundtrip() {
    let raw = bevy::ui::UiRect::all(bevy::ui::Val::Px(4.0));
    let shadow = UiRect::from(raw);
    let back: bevy::ui::UiRect = shadow.into();
    assert_eq!(back, raw);
}

#[test]
fn ui_rect_field_accessors() {
    let shadow = UiRect::from(bevy::ui::UiRect {
        left: bevy::ui::Val::Px(1.0),
        right: bevy::ui::Val::Px(2.0),
        top: bevy::ui::Val::Px(3.0),
        bottom: bevy::ui::Val::Px(4.0),
    });
    assert_eq!(bevy::ui::Val::from(shadow.left()), bevy::ui::Val::Px(1.0));
    assert_eq!(bevy::ui::Val::from(shadow.right()), bevy::ui::Val::Px(2.0));
    assert_eq!(bevy::ui::Val::from(shadow.top()), bevy::ui::Val::Px(3.0));
    assert_eq!(bevy::ui::Val::from(shadow.bottom()), bevy::ui::Val::Px(4.0));
}

// ── BorderRadius ──────────────────────────────────────────────────────────────

#[test]
fn border_radius_all_roundtrip() {
    let raw = bevy::ui::BorderRadius::all(bevy::ui::Val::Px(8.0));
    let shadow = BorderRadius::from(raw);
    let back: bevy::ui::BorderRadius = shadow.into();
    assert_eq!(back, raw);
}

// ── PositionType ──────────────────────────────────────────────────────────────

#[test]
fn position_type_absolute_roundtrip() {
    let shadow = PositionType::from(bevy::ui::PositionType::Absolute);
    let back: bevy::ui::PositionType = shadow.into();
    assert_eq!(back, bevy::ui::PositionType::Absolute);
}

#[test]
fn position_type_relative_roundtrip() {
    let shadow = PositionType::from(bevy::ui::PositionType::Relative);
    let back: bevy::ui::PositionType = shadow.into();
    assert_eq!(back, bevy::ui::PositionType::Relative);
}

// ── BackgroundColor ───────────────────────────────────────────────────────────

#[test]
fn background_color_roundtrip() {
    let raw = bevy::ui::BackgroundColor(bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.65));
    let shadow = BackgroundColor::from(raw);
    let back: bevy::ui::BackgroundColor = shadow.into();
    assert_eq!(back.0, raw.0);
}

// ── GlobalZIndex / ZIndex ─────────────────────────────────────────────────────

#[test]
fn global_z_index_roundtrip() {
    let raw = bevy::ui::GlobalZIndex(5);
    let shadow = GlobalZIndex::from(raw);
    let back: bevy::ui::GlobalZIndex = shadow.into();
    assert_eq!(back.0, 5);
}

#[test]
fn z_index_roundtrip() {
    let raw = bevy::ui::ZIndex(3);
    let shadow = ZIndex::from(raw);
    let back: bevy::ui::ZIndex = shadow.into();
    assert_eq!(back.0, 3);
}

// ── Display / FlexDirection / FlexWrap / JustifyContent / AlignItems ─────────

#[test]
fn display_flex_roundtrip() {
    let shadow = Display::from(bevy::ui::Display::Flex);
    let back: bevy::ui::Display = shadow.into();
    assert_eq!(back, bevy::ui::Display::Flex);
}

#[test]
fn flex_direction_column_roundtrip() {
    let shadow = FlexDirection::from(bevy::ui::FlexDirection::Column);
    let back: bevy::ui::FlexDirection = shadow.into();
    assert_eq!(back, bevy::ui::FlexDirection::Column);
}

#[test]
fn flex_wrap_wrap_roundtrip() {
    let shadow = FlexWrap::from(bevy::ui::FlexWrap::Wrap);
    let back: bevy::ui::FlexWrap = shadow.into();
    assert_eq!(back, bevy::ui::FlexWrap::Wrap);
}

#[test]
fn justify_content_center_roundtrip() {
    let shadow = JustifyContent::from(bevy::ui::JustifyContent::Center);
    let back: bevy::ui::JustifyContent = shadow.into();
    assert_eq!(back, bevy::ui::JustifyContent::Center);
}

#[test]
fn align_items_center_roundtrip() {
    let shadow = AlignItems::from(bevy::ui::AlignItems::Center);
    let back: bevy::ui::AlignItems = shadow.into();
    assert_eq!(back, bevy::ui::AlignItems::Center);
}

#[test]
fn overflow_axis_roundtrip() {
    let shadow = OverflowAxis::from(bevy::ui::OverflowAxis::Clip);
    let back: bevy::ui::OverflowAxis = shadow.into();
    assert_eq!(back, bevy::ui::OverflowAxis::Clip);
}

// ── Marker components ─────────────────────────────────────────────────────────

#[test]
fn is_default_ui_camera_converts() {
    // Unit struct shadow — construct directly, convert to bevy type.
    let shadow = IsDefaultUiCamera;
    let _: bevy::ui::IsDefaultUiCamera = shadow.into();
}

// ── Color ─────────────────────────────────────────────────────────────────────

#[test]
fn color_white_roundtrip() {
    let shadow = Color::from(bevy::color::Color::WHITE);
    let back: bevy::color::Color = shadow.into();
    assert_eq!(back, bevy::color::Color::WHITE);
}

#[test]
fn color_srgba_roundtrip() {
    let raw = bevy::color::Color::srgba(1.0, 0.0, 0.5, 0.8);
    let shadow = Color::from(raw);
    let back: bevy::color::Color = shadow.into();
    assert_eq!(back, raw);
}

#[test]
fn color_variant_name_srgba() {
    let shadow = Color::from(bevy::color::Color::srgba(1.0, 0.0, 0.0, 1.0));
    assert_eq!(shadow.variant_name(), "Srgba");
}

// ── TextFont ──────────────────────────────────────────────────────────────────

#[test]
fn text_font_default_roundtrip() {
    let raw = bevy::text::TextFont::default();
    let shadow = TextFont::from(raw.clone());
    let back: bevy::text::TextFont = shadow.into();
    assert_eq!(back, raw);
}

#[test]
fn text_font_with_size_roundtrip() {
    let raw = bevy::text::TextFont {
        font_size: bevy::text::FontSize::Px(24.0),
        ..Default::default()
    };
    let shadow = TextFont::from(raw.clone());
    assert_eq!(shadow.font_size(), 24.0);
    let back: bevy::text::TextFont = shadow.into();
    assert_eq!(back, raw);
}

// ── Text (UI widget) ──────────────────────────────────────────────────────────

#[test]
fn text_widget_construction_and_conversion() {
    let shadow = Text("Hello".to_string());
    let raw: bevy::ui::widget::Text = shadow.into();
    assert_eq!(raw.0, "Hello");
}

#[test]
fn text_widget_new_helper() {
    // bevy::ui::widget::Text::new() is the idiomatic Bevy constructor.
    // The shadow wraps the same string; verify round-trip.
    let raw = bevy::ui::widget::Text::new("World");
    let shadow = Text(raw.0.clone());
    let back: bevy::ui::widget::Text = shadow.into();
    assert_eq!(back.0, "World");
}

// ── TextColor ─────────────────────────────────────────────────────────────────

#[test]
fn text_color_white_roundtrip() {
    let raw = bevy::text::TextColor(bevy::color::Color::WHITE);
    let shadow = TextColor::from(raw);
    let back: bevy::text::TextColor = shadow.into();
    assert_eq!(back.0, bevy::color::Color::WHITE);
}

#[test]
fn text_color_srgba_roundtrip() {
    let raw = bevy::text::TextColor(bevy::color::Color::srgba(0.5, 0.5, 0.5, 1.0));
    let shadow = TextColor::from(raw);
    let back: bevy::text::TextColor = shadow.into();
    assert_eq!(back.0, raw.0);
}
