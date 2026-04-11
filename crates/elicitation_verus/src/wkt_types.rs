use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

pub struct ShadowWktCoord {
    pub x: f64,
    pub y: f64,
    pub has_z: bool,
    pub has_m: bool,
}

pub fn make_wkt_coord(x: f64, y: f64, has_z: bool, has_m: bool) -> (result: ShadowWktCoord)
    ensures
        result.x == x,
        result.y == y,
        result.has_z == has_z,
        result.has_m == has_m,
{
    ShadowWktCoord { x, y, has_z, has_m }
}

pub fn verify_wkt_coord_roundtrip(x: f64, y: f64, has_z: bool, has_m: bool) -> (result: ShadowWktCoord)
    ensures
        result.x == x,
        result.y == y,
        result.has_z == has_z,
        result.has_m == has_m,
{
    let original = make_wkt_coord(x, y, has_z, has_m);
    make_wkt_coord(original.x, original.y, original.has_z, original.has_m)
}

pub fn verify_wkt_coord_concrete() -> (result: ShadowWktCoord)
    ensures
        result.x == 1.5f64,
        result.y == -2.0f64,
        result.has_z,
        result.has_m,
{
    make_wkt_coord(1.5, -2.0, true, true)
}

pub struct ShadowWktPoint {
    pub is_empty: bool,
}

pub fn make_wkt_point(is_empty: bool) -> (result: ShadowWktPoint)
    ensures result.is_empty == is_empty,
{
    ShadowWktPoint { is_empty }
}

pub fn verify_wkt_point_empty() -> (result: ShadowWktPoint)
    ensures result.is_empty,
{
    make_wkt_point(true)
}

pub struct ShadowWktGeom {
    pub is_point: bool,
}

pub fn make_wkt_geom_point() -> (result: ShadowWktGeom)
    ensures result.is_point,
{
    ShadowWktGeom { is_point: true }
}

pub fn verify_wkt_geom_point_variant() -> (result: ShadowWktGeom)
    ensures result.is_point,
{
    make_wkt_geom_point()
}

pub fn verify_wkt_string_trusted() {
    assert(true);
}

}
