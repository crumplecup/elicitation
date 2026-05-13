use verus_builtin_macros::verus;

verus! {

pub struct ShadowRstarAabb {
    pub lower_x: u32,
    pub lower_y: u32,
    pub upper_x: u32,
    pub upper_y: u32,
}

pub fn make_rstar_aabb(lower_x: u32, lower_y: u32, upper_x: u32, upper_y: u32) -> (result: ShadowRstarAabb)
    ensures
        result.lower_x == lower_x,
        result.lower_y == lower_y,
        result.upper_x == upper_x,
        result.upper_y == upper_y,
{
    ShadowRstarAabb { lower_x, lower_y, upper_x, upper_y }
}

pub fn verify_rstar_aabb_roundtrip() -> (result: ShadowRstarAabb)
    ensures
        result.lower_x == 1,
        result.lower_y == 2,
        result.upper_x == 3,
        result.upper_y == 4,
{
    make_rstar_aabb(1u32, 2u32, 3u32, 4u32)
}

pub struct ShadowRstarRectangle {
    pub lower_x: u32,
    pub lower_y: u32,
    pub upper_x: u32,
    pub upper_y: u32,
}

pub fn make_rstar_rectangle(lower_x: u32, lower_y: u32, upper_x: u32, upper_y: u32) -> (result: ShadowRstarRectangle)
    ensures
        result.lower_x == lower_x,
        result.lower_y == lower_y,
        result.upper_x == upper_x,
        result.upper_y == upper_y,
{
    ShadowRstarRectangle { lower_x, lower_y, upper_x, upper_y }
}

pub fn verify_rstar_rectangle_roundtrip() -> (result: ShadowRstarRectangle)
    ensures
        result.lower_x == 0,
        result.lower_y == 1,
        result.upper_x == 2,
        result.upper_y == 3,
{
    make_rstar_rectangle(0u32, 1u32, 2u32, 3u32)
}

pub fn verify_rstar_rectangle_envelope_bounds() -> (result: ShadowRstarRectangle)
    ensures
        result.lower_x == 1,
        result.lower_y == 2,
        result.upper_x == 4,
        result.upper_y == 6,
{
    make_rstar_rectangle(1u32, 2u32, 4u32, 6u32)
}

pub struct ShadowRstarLine {
    pub from_x: u32,
    pub from_y: u32,
    pub to_x: u32,
    pub to_y: u32,
}

pub fn make_rstar_line(from_x: u32, from_y: u32, to_x: u32, to_y: u32) -> (result: ShadowRstarLine)
    ensures
        result.from_x == from_x,
        result.from_y == from_y,
        result.to_x == to_x,
        result.to_y == to_y,
{
    ShadowRstarLine { from_x, from_y, to_x, to_y }
}

pub fn verify_rstar_line_roundtrip() -> (result: ShadowRstarLine)
    ensures
        result.from_x == 5,
        result.from_y == 1,
        result.to_x == 2,
        result.to_y == 4,
{
    make_rstar_line(5u32, 1u32, 2u32, 4u32)
}

pub fn verify_rstar_line_envelope_bounds() -> (result: ShadowRstarAabb)
    ensures
        result.lower_x == 2,
        result.lower_y == 1,
        result.upper_x == 5,
        result.upper_y == 4,
{
    make_rstar_aabb(2u32, 1u32, 5u32, 4u32)
}

}
