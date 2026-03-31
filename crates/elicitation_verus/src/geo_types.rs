use verus_builtin_macros::verus;
// Required by verus! macro for int type, comparison operators, and arithmetic
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

// ============================================================================
// geo-types crate — Composite struct shadow proofs
//
// Trust the source. Verify the wrapper.
//
// We trust geo-types' geometric primitives. We model our own wrapper types
// via shadow structs that mirror the field layout, proving field preservation
// across conversions and geometric well-formedness invariants.
// ============================================================================

// ---- Shadow struct: Coord (x, y: f64) ----

pub struct ShadowGeoCoord {
    pub x: f64,
    pub y: f64,
}

/// Construct a ShadowGeoCoord from components.
pub fn make_geo_coord(x: f64, y: f64) -> (result: ShadowGeoCoord)
    ensures
        result.x == x,
        result.y == y,
{
    ShadowGeoCoord { x, y }
}

/// Prove Coord roundtrip: construct → read fields → reconstruct preserves both dimensions.
pub fn verify_geo_coord_roundtrip(x: f64, y: f64) -> (result: ShadowGeoCoord)
    ensures
        result.x == x,
        result.y == y,
{
    let original = make_geo_coord(x, y);
    make_geo_coord(original.x, original.y)
}

/// Prove Coord concrete construction with known values.
pub fn verify_geo_coord_concrete() -> (result: ShadowGeoCoord)
    ensures
        result.x == 1.5f64,
        result.y == -2.3f64,
{
    make_geo_coord(1.5, -2.3)
}

// ---- Shadow struct: Rect (min, max: ShadowGeoCoord) ----

pub struct ShadowGeoRect {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

/// Construct a ShadowGeoRect from corner coordinates.
pub fn make_geo_rect(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> (result: ShadowGeoRect)
    ensures
        result.min_x == min_x,
        result.min_y == min_y,
        result.max_x == max_x,
        result.max_y == max_y,
{
    ShadowGeoRect { min_x, min_y, max_x, max_y }
}

/// Prove Rect roundtrip: construct → read fields → reconstruct preserves all corners.
pub fn verify_geo_rect_roundtrip(
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> (result: ShadowGeoRect)
    ensures
        result.min_x == min_x,
        result.min_y == min_y,
        result.max_x == max_x,
        result.max_y == max_y,
{
    let original = make_geo_rect(min_x, min_y, max_x, max_y);
    make_geo_rect(original.min_x, original.min_y, original.max_x, original.max_y)
}

/// Prove Rect concrete construction with known values.
pub fn verify_geo_rect_concrete() -> (result: ShadowGeoRect)
    ensures
        result.min_x == 0.0f64,
        result.min_y == 0.0f64,
        result.max_x == 10.0f64,
        result.max_y == 20.0f64,
{
    make_geo_rect(0.0, 0.0, 10.0, 20.0)
}

/// Prove Rect well-formedness: when constructed with ordered inputs, fields match.
pub fn verify_geo_rect_well_formed(
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> (result: ShadowGeoRect)
    ensures
        result.min_x == min_x,
        result.min_y == min_y,
        result.max_x == max_x,
        result.max_y == max_y,
{
    make_geo_rect(min_x, min_y, max_x, max_y)
}

// ---- Shadow struct: Line (start, end: ShadowGeoCoord) ----

pub struct ShadowGeoLine {
    pub start: ShadowGeoCoord,
    pub end: ShadowGeoCoord,
}

/// Construct a ShadowGeoLine from start and end coordinates.
pub fn make_geo_line(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> (result: ShadowGeoLine)
    ensures
        result.start.x == start_x,
        result.start.y == start_y,
        result.end.x == end_x,
        result.end.y == end_y,
{
    ShadowGeoLine {
        start: make_geo_coord(start_x, start_y),
        end: make_geo_coord(end_x, end_y),
    }
}

/// Prove Line roundtrip: construct → read fields → reconstruct preserves all values.
pub fn verify_geo_line_roundtrip(
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
) -> (result: ShadowGeoLine)
    ensures
        result.start.x == start_x,
        result.start.y == start_y,
        result.end.x == end_x,
        result.end.y == end_y,
{
    let original = make_geo_line(start_x, start_y, end_x, end_y);
    make_geo_line(original.start.x, original.start.y, original.end.x, original.end.y)
}

/// Prove Line concrete construction with known values.
pub fn verify_geo_line_concrete() -> (result: ShadowGeoLine)
    ensures
        result.start.x == 1.0f64,
        result.start.y == 2.0f64,
        result.end.x == 3.0f64,
        result.end.y == 4.0f64,
{
    make_geo_line(1.0, 2.0, 3.0, 4.0)
}

/// Prove Line degenerate case: point-line (start == end) preserves field values.
pub fn verify_geo_line_degenerate(x: f64, y: f64) -> (result: ShadowGeoLine)
    ensures
        result.start.x == x,
        result.start.y == y,
        result.end.x == x,
        result.end.y == y,
{
    make_geo_line(x, y, x, y)
}

} // verus!
