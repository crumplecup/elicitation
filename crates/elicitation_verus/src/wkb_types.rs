use verus_builtin_macros::verus;
// Required by verus! macro for comparison operators (<, >, <=, >=, etc.)
// Cargo cannot detect this usage as it occurs during macro expansion
#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

verus! {

pub struct ShadowWkbEndianness {
    pub is_little_endian: bool,
}

pub fn make_wkb_endianness(is_little_endian: bool) -> (result: ShadowWkbEndianness)
    ensures result.is_little_endian == is_little_endian,
{
    ShadowWkbEndianness { is_little_endian }
}

pub fn verify_wkb_endianness_roundtrip(is_little_endian: bool) -> (result: ShadowWkbEndianness)
    ensures result.is_little_endian == is_little_endian,
{
    let original = make_wkb_endianness(is_little_endian);
    make_wkb_endianness(original.is_little_endian)
}

pub struct ShadowWkbDimension {
    pub code: u8,
}

pub fn make_wkb_dimension(code: u8) -> (result: ShadowWkbDimension)
    requires code < 4,
    ensures result.code == code,
{
    ShadowWkbDimension { code }
}

pub fn verify_wkb_dimension_roundtrip(code: u8) -> (result: ShadowWkbDimension)
    requires code < 4,
    ensures result.code == code,
{
    let original = make_wkb_dimension(code);
    make_wkb_dimension(original.code)
}

pub struct ShadowWkbGeometryType {
    pub code: u8,
}

pub fn make_wkb_geometry_type(code: u8) -> (result: ShadowWkbGeometryType)
    requires code < 7,
    ensures result.code == code,
{
    ShadowWkbGeometryType { code }
}

pub fn verify_wkb_geometry_type_roundtrip(code: u8) -> (result: ShadowWkbGeometryType)
    requires code < 7,
    ensures result.code == code,
{
    let original = make_wkb_geometry_type(code);
    make_wkb_geometry_type(original.code)
}

pub struct ShadowWkbWriteOptions {
    pub is_little_endian: bool,
}

pub fn make_wkb_write_options(is_little_endian: bool) -> (result: ShadowWkbWriteOptions)
    ensures result.is_little_endian == is_little_endian,
{
    ShadowWkbWriteOptions { is_little_endian }
}

pub fn verify_wkb_write_options_roundtrip(is_little_endian: bool) -> (result: ShadowWkbWriteOptions)
    ensures result.is_little_endian == is_little_endian,
{
    let original = make_wkb_write_options(is_little_endian);
    make_wkb_write_options(original.is_little_endian)
}

pub struct ShadowWkbBytesMeta {
    pub byte_len: usize,
    pub is_little_endian: bool,
    pub is_point: bool,
}

pub fn verify_wkb_bytes_known_point() -> (result: ShadowWkbBytesMeta)
    ensures
        result.byte_len == 21usize,
        result.is_little_endian,
        result.is_point,
{
    ShadowWkbBytesMeta {
        byte_len: 21usize,
        is_little_endian: true,
        is_point: true,
    }
}

}
