use verus_builtin_macros::verus;

verus! {

pub struct ShadowProjArea {
    pub west: u32,
    pub south: u32,
    pub east: u32,
    pub north: u32,
}

pub fn make_proj_area(west: u32, south: u32, east: u32, north: u32) -> (result: ShadowProjArea)
    ensures
        result.west == west,
        result.south == south,
        result.east == east,
        result.north == north,
{
    ShadowProjArea { west, south, east, north }
}

pub fn verify_proj_area_new_fields() -> (result: ShadowProjArea)
    ensures
        result.west == 1,
        result.south == 2,
        result.east == 3,
        result.north == 4,
{
    make_proj_area(1u32, 2u32, 3u32, 4u32)
}

pub fn verify_proj_area_roundtrip() -> (result: ShadowProjArea)
    ensures
        result.west == 10,
        result.south == 20,
        result.east == 30,
        result.north == 40,
{
    make_proj_area(10u32, 20u32, 30u32, 40u32)
}

pub fn verify_proj_area_antimeridian() -> (result: bool)
    ensures result == true,
{
    let west = 170u32;
    let east = 10u32;
    west > east
}

}
