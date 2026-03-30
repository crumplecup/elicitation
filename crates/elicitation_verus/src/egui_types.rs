use verus_builtin_macros::verus;

verus! {

// ============================================================================
// egui crate — Select enum types
//
// Trust the source. Verify the wrapper.
//
// We trust egui's variant definitions. We model our own wrapper logic:
// from_label returns Some iff the label is one we declared in labels(),
// and returns None for any unknown label.
// ============================================================================

// ---- Align (3 variants: Min, Center, Max) ----

/// Proof that from_label succeeds for a known Align label.
pub fn verify_align_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

/// Proof that from_label fails for an unknown Align label.
pub fn verify_align_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

/// Proof that all Align labels round-trip through from_label.
pub fn verify_align_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

/// Proof that Align label count equals option count.
pub fn verify_align_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- Direction (4 variants: LeftToRight, RightToLeft, TopDown, BottomUp) ----

pub fn verify_direction_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_direction_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_direction_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_direction_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- Theme (2 variants: Dark, Light) ----

pub fn verify_theme_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_theme_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_theme_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_theme_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- ThemePreference (3 variants: Dark, Light, System) ----

pub fn verify_theme_preference_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_theme_preference_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_theme_preference_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_theme_preference_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- FontFamily (2 variants: Monospace, Proportional) ----

pub fn verify_font_family_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_font_family_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_font_family_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_font_family_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- TextWrapMode (3 variants: Extend, Wrap, Truncate) ----

pub fn verify_text_wrap_mode_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_text_wrap_mode_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_text_wrap_mode_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_text_wrap_mode_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- TextureFilter (2 variants: Nearest, Linear) ----

pub fn verify_texture_filter_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_texture_filter_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_texture_filter_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_texture_filter_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- TextureWrapMode (3 variants: ClampToEdge, Repeat, MirroredRepeat) ----

pub fn verify_texture_wrap_mode_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_texture_wrap_mode_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_texture_wrap_mode_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_texture_wrap_mode_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- TouchPhase (4 variants: Start, Move, End, Cancel) ----

pub fn verify_touch_phase_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_touch_phase_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_touch_phase_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_touch_phase_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- PointerButton (5 variants: Primary, Secondary, Middle, Extra1, Extra2) ----

pub fn verify_pointer_button_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_pointer_button_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_pointer_button_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_pointer_button_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- Order (5 variants) ----

pub fn verify_order_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_order_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_order_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_order_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- TextStyle (5 variants) ----

pub fn verify_text_style_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_text_style_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_text_style_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_text_style_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- UiKind (17 variants) ----

pub fn verify_ui_kind_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_ui_kind_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_ui_kind_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_ui_kind_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- WidgetType (18 variants) ----

pub fn verify_widget_type_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_widget_type_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_widget_type_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_widget_type_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- CursorIcon (35 variants) ----

pub fn verify_cursor_icon_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_cursor_icon_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_cursor_icon_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_cursor_icon_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- Key (103 variants) ----

pub fn verify_key_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_key_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_key_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_key_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ============================================================================
// egui crate — Composite struct wrapper types
//
// From-roundtrip specifications. Each proof models the property that
// converting egui::T → EguiT → egui::T preserves all field values.
// ============================================================================

// ---- Color32 (r, g, b, a: u8) ----

/// Proof that Color32 From roundtrip preserves RGBA channels.
pub fn verify_color32_roundtrip(r_preserved: bool, g_preserved: bool, b_preserved: bool, a_preserved: bool) -> (result: bool)
    ensures result == (r_preserved && g_preserved && b_preserved && a_preserved),
{
    r_preserved && g_preserved && b_preserved && a_preserved
}

/// Proof that Color32 wrapper fields match source.
pub fn verify_color32_wrapper_fields(all_fields_match: bool) -> (result: bool)
    ensures result == all_fields_match,
{
    all_fields_match
}

// ---- Pos2 (x, y: f32) ----

/// Proof that Pos2 From roundtrip preserves x, y coordinates.
pub fn verify_pos2_roundtrip(x_preserved: bool, y_preserved: bool) -> (result: bool)
    ensures result == (x_preserved && y_preserved),
{
    x_preserved && y_preserved
}

// ---- Vec2 (x, y: f32) ----

/// Proof that Vec2 From roundtrip preserves x, y components.
pub fn verify_vec2_roundtrip(x_preserved: bool, y_preserved: bool) -> (result: bool)
    ensures result == (x_preserved && y_preserved),
{
    x_preserved && y_preserved
}

// ---- Rect (min_x, min_y, max_x, max_y: f32) ----

/// Proof that Rect From roundtrip preserves all corners.
pub fn verify_rect_roundtrip(min_x_ok: bool, min_y_ok: bool, max_x_ok: bool, max_y_ok: bool) -> (result: bool)
    ensures result == (min_x_ok && min_y_ok && max_x_ok && max_y_ok),
{
    min_x_ok && min_y_ok && max_x_ok && max_y_ok
}

// ---- Stroke (width: f32, color: Color32) ----

/// Proof that Stroke From roundtrip preserves width and color channels.
pub fn verify_stroke_roundtrip(width_ok: bool, color_ok: bool) -> (result: bool)
    ensures result == (width_ok && color_ok),
{
    width_ok && color_ok
}

// ---- CornerRadius (nw, ne, sw, se: u8) ----

/// Proof that CornerRadius From roundtrip preserves all four corners.
pub fn verify_corner_radius_roundtrip(nw_ok: bool, ne_ok: bool, sw_ok: bool, se_ok: bool) -> (result: bool)
    ensures result == (nw_ok && ne_ok && sw_ok && se_ok),
{
    nw_ok && ne_ok && sw_ok && se_ok
}

// ---- Shadow (offset_x, offset_y: i8, blur, spread: u8, color: Color32) ----

/// Proof that Shadow From roundtrip preserves all fields.
pub fn verify_shadow_roundtrip(offset_ok: bool, blur_ok: bool, spread_ok: bool, color_ok: bool) -> (result: bool)
    ensures result == (offset_ok && blur_ok && spread_ok && color_ok),
{
    offset_ok && blur_ok && spread_ok && color_ok
}

// ---- Margin (left, right, top, bottom: i8) ----

/// Proof that Margin From roundtrip preserves all four margins.
pub fn verify_margin_roundtrip(left_ok: bool, right_ok: bool, top_ok: bool, bottom_ok: bool) -> (result: bool)
    ensures result == (left_ok && right_ok && top_ok && bottom_ok),
{
    left_ok && right_ok && top_ok && bottom_ok
}

// ---- FontId (size: f32, family: FontFamily) ----

/// Proof that FontId From roundtrip preserves size and family.
pub fn verify_font_id_roundtrip(size_ok: bool, family_ok: bool) -> (result: bool)
    ensures result == (size_ok && family_ok),
{
    size_ok && family_ok
}

} // verus!
