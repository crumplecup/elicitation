use verus_builtin_macros::verus;

verus! {

// ============================================================================
// Primitive numeric types
// ============================================================================

/// Proof that i8 can be constructed and used.
pub fn verify_i8_construction(value: i8) -> (result: i8)
    ensures result == value,
{
    value
}

/// Proof that u8 can be constructed and used.
pub fn verify_u8_construction(value: u8) -> (result: u8)
    ensures result == value,
{
    value
}

/// Proof that i16 can be constructed and used.
pub fn verify_i16_construction(value: i16) -> (result: i16)
    ensures result == value,
{
    value
}

/// Proof that u16 can be constructed and used.
pub fn verify_u16_construction(value: u16) -> (result: u16)
    ensures result == value,
{
    value
}

/// Proof that i32 can be constructed and used.
pub fn verify_i32_construction(value: i32) -> (result: i32)
    ensures result == value,
{
    value
}

/// Proof that u32 can be constructed and used.
pub fn verify_u32_construction(value: u32) -> (result: u32)
    ensures result == value,
{
    value
}

/// Proof that i64 can be constructed and used.
pub fn verify_i64_construction(value: i64) -> (result: i64)
    ensures result == value,
{
    value
}

/// Proof that u64 can be constructed and used.
pub fn verify_u64_construction(value: u64) -> (result: u64)
    ensures result == value,
{
    value
}

/// Proof that f32 can be constructed and used.
pub fn verify_f32_construction(value: f32) -> (result: f32)
    ensures result == value,
{
    value
}

/// Proof that f64 can be constructed and used.
pub fn verify_f64_construction(value: f64) -> (result: f64)
    ensures result == value,
{
    value
}

/// Proof that bool can be constructed and used.
pub fn verify_bool_construction(value: bool) -> (result: bool)
    ensures result == value,
{
    value
}

/// Proof that char can be constructed and used.
pub fn verify_char_construction(value: char) -> (result: char)
    ensures result == value,
{
    value
}

// ============================================================================
// Unit type
// ============================================================================

/// Proof that unit type can be constructed.
pub fn verify_unit_construction() -> (result: ())
    ensures result == (),
{
    ()
}

} // verus!
