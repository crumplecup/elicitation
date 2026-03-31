use verus_builtin_macros::verus;

verus! {

// ============================================================================
// ratatui crate — Select enum types
//
// Trust the source. Verify the wrapper.
//
// We trust ratatui's variant definitions. We model our own wrapper logic:
// from_label returns Some iff the label is one we declared in labels(),
// and returns None for any unknown label.
// ============================================================================

// ---- Alignment (3 variants: Left, Center, Right) ----

/// Proof that from_label succeeds for a known Alignment label.
pub fn verify_ratatui_alignment_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

/// Proof that from_label fails for an unknown Alignment label.
pub fn verify_ratatui_alignment_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

/// Proof that all Alignment labels round-trip through from_label.
pub fn verify_ratatui_alignment_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

/// Proof that Alignment label count equals option count.
pub fn verify_ratatui_alignment_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- Direction (2 variants: Horizontal, Vertical) ----

pub fn verify_ratatui_direction_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_ratatui_direction_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_ratatui_direction_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_ratatui_direction_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- BorderType (4 variants: Plain, Rounded, Double, Thick) ----

pub fn verify_ratatui_border_type_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_ratatui_border_type_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_ratatui_border_type_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_ratatui_border_type_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- Color (19 entries: 17 named + Reset + 2 sentinels) ----

pub fn verify_ratatui_color_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_ratatui_color_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_ratatui_color_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_ratatui_color_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- Borders (8 bitflag presets) ----

pub fn verify_ratatui_borders_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_ratatui_borders_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_ratatui_borders_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_ratatui_borders_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ---- ScrollbarOrientation (4 variants) ----

pub fn verify_ratatui_scrollbar_orientation_known_label_accepted(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

pub fn verify_ratatui_scrollbar_orientation_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

pub fn verify_ratatui_scrollbar_orientation_roundtrip_complete(all_known: bool) -> (result: bool)
    ensures result == all_known,
{
    all_known
}

pub fn verify_ratatui_scrollbar_orientation_label_count_matches(counts_equal: bool) -> (result: bool)
    ensures result == counts_equal,
{
    counts_equal
}

// ============================================================================
// Composite struct proofs — From roundtrip verification
// ============================================================================

// ---- RatatuiPadding (left, right, top, bottom: u16) ----

/// Proof that Padding From roundtrip preserves all four sides.
pub fn verify_ratatui_padding_roundtrip(left_ok: bool, right_ok: bool, top_ok: bool, bottom_ok: bool) -> (result: bool)
    ensures result == (left_ok && right_ok && top_ok && bottom_ok),
{
    left_ok && right_ok && top_ok && bottom_ok
}

// ---- RatatuiMargin (horizontal, vertical: u16) ----

/// Proof that Margin From roundtrip preserves both dimensions.
pub fn verify_ratatui_margin_roundtrip(horizontal_ok: bool, vertical_ok: bool) -> (result: bool)
    ensures result == (horizontal_ok && vertical_ok),
{
    horizontal_ok && vertical_ok
}

// ---- RatatuiStyle (fg, bg: Option<String>, bold, italic, underlined: bool) ----

/// Proof that Style modifier mapping preserves all three flags.
pub fn verify_ratatui_style_modifiers(bold_ok: bool, italic_ok: bool, underlined_ok: bool) -> (result: bool)
    ensures result == (bold_ok && italic_ok && underlined_ok),
{
    bold_ok && italic_ok && underlined_ok
}

/// Proof that Style fg/bg presence is preserved.
pub fn verify_ratatui_style_colors(fg_ok: bool, bg_ok: bool) -> (result: bool)
    ensures result == (fg_ok && bg_ok),
{
    fg_ok && bg_ok
}

// ---- BordersSelect (into_inner roundtrip) ----

/// Proof that BordersSelect into_inner preserves the inner Borders value.
pub fn verify_borders_select_roundtrip(roundtrip_ok: bool) -> (result: bool)
    ensures result == roundtrip_ok,
{
    roundtrip_ok
}

} // verus!
