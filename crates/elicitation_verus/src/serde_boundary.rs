use verus_builtin_macros::verus;
// Required by verus! macro for comparison operators (<=, >, etc.)
// Cargo cannot detect this usage as it occurs during macro expansion
#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

verus! {

// ============================================================================
// Serde boundary consistency theorems.
//
// The serde bridge (Phase A) wires Deserialize impls to call Type::new(v).
// These theorems prove at the spec level:
//
//   serde succeeds  <=>  new() succeeds  <=>  the predicate holds
//
// Following the established Verus crate pattern, this module is self-contained
// with abstract type definitions (prefixed Sb*) and proof functions.
// Integers use concrete predicates; floats/URLs use abstract Boolean parameters.
// ============================================================================

// ============================================================================
// I8 serde boundary
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbI8Positive { pub value: i8 }

impl SbI8Positive {
    pub fn new(v: i8) -> (r: Result<Self, ()>)
        ensures
            v > 0  ==> (r matches Ok(p) && p.value == v),
            v <= 0 ==> (r matches Err(())),
    {
        if v > 0 { Ok(SbI8Positive { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbI8Positive::new(v) succeeds iff v > 0.
proof fn i8_positive_serde_iff(v: i8)
    ensures (v > 0) <==> (SbI8Positive::new(v) matches Ok(_)),
{}

/// Corollary: any value produced via new() has value > 0.
proof fn i8_positive_invariant(p: SbI8Positive, v: i8)
    requires SbI8Positive::new(v) matches Ok(q) && q.value == p.value,
    ensures  p.value > 0,
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbI8NonNegative { pub value: i8 }

impl SbI8NonNegative {
    pub fn new(v: i8) -> (r: Result<Self, ()>)
        ensures
            v >= 0 ==> (r matches Ok(p) && p.value == v),
            v < 0  ==> (r matches Err(())),
    {
        if v >= 0 { Ok(SbI8NonNegative { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbI8NonNegative::new(v) succeeds iff v >= 0.
proof fn i8_non_negative_serde_iff(v: i8)
    ensures (v >= 0) <==> (SbI8NonNegative::new(v) matches Ok(_)),
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbI8NonZero { pub value: i8 }

impl SbI8NonZero {
    pub fn new(v: i8) -> (r: Result<Self, ()>)
        ensures
            v != 0 ==> (r matches Ok(p) && p.value == v),
            v == 0 ==> (r matches Err(())),
    {
        if v != 0 { Ok(SbI8NonZero { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbI8NonZero::new(v) succeeds iff v != 0.
proof fn i8_non_zero_serde_iff(v: i8)
    ensures (v != 0) <==> (SbI8NonZero::new(v) matches Ok(_)),
{}

// ============================================================================
// I16 serde boundary
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbI16Positive { pub value: i16 }

impl SbI16Positive {
    pub fn new(v: i16) -> (r: Result<Self, ()>)
        ensures
            v > 0  ==> (r matches Ok(p) && p.value == v),
            v <= 0 ==> (r matches Err(())),
    {
        if v > 0 { Ok(SbI16Positive { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbI16Positive::new(v) succeeds iff v > 0.
proof fn i16_positive_serde_iff(v: i16)
    ensures (v > 0) <==> (SbI16Positive::new(v) matches Ok(_)),
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbI16NonNegative { pub value: i16 }

impl SbI16NonNegative {
    pub fn new(v: i16) -> (r: Result<Self, ()>)
        ensures
            v >= 0 ==> (r matches Ok(p) && p.value == v),
            v < 0  ==> (r matches Err(())),
    {
        if v >= 0 { Ok(SbI16NonNegative { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbI16NonNegative::new(v) succeeds iff v >= 0.
proof fn i16_non_negative_serde_iff(v: i16)
    ensures (v >= 0) <==> (SbI16NonNegative::new(v) matches Ok(_)),
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbI16NonZero { pub value: i16 }

impl SbI16NonZero {
    pub fn new(v: i16) -> (r: Result<Self, ()>)
        ensures
            v != 0 ==> (r matches Ok(p) && p.value == v),
            v == 0 ==> (r matches Err(())),
    {
        if v != 0 { Ok(SbI16NonZero { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbI16NonZero::new(v) succeeds iff v != 0.
proof fn i16_non_zero_serde_iff(v: i16)
    ensures (v != 0) <==> (SbI16NonZero::new(v) matches Ok(_)),
{}

// ============================================================================
// U8 / U16 serde boundary
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbU8Positive { pub value: u8 }

impl SbU8Positive {
    pub fn new(v: u8) -> (r: Result<Self, ()>)
        ensures
            v > 0  ==> (r matches Ok(p) && p.value == v),
            v == 0 ==> (r matches Err(())),
    {
        if v > 0 { Ok(SbU8Positive { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbU8Positive::new(v) succeeds iff v > 0.
proof fn u8_positive_serde_iff(v: u8)
    ensures (v > 0) <==> (SbU8Positive::new(v) matches Ok(_)),
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbU8NonZero { pub value: u8 }

impl SbU8NonZero {
    pub fn new(v: u8) -> (r: Result<Self, ()>)
        ensures
            v != 0 ==> (r matches Ok(p) && p.value == v),
            v == 0 ==> (r matches Err(())),
    {
        if v != 0 { Ok(SbU8NonZero { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbU8NonZero::new(v) succeeds iff v != 0.
proof fn u8_non_zero_serde_iff(v: u8)
    ensures (v != 0) <==> (SbU8NonZero::new(v) matches Ok(_)),
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbU16Positive { pub value: u16 }

impl SbU16Positive {
    pub fn new(v: u16) -> (r: Result<Self, ()>)
        ensures
            v > 0  ==> (r matches Ok(p) && p.value == v),
            v == 0 ==> (r matches Err(())),
    {
        if v > 0 { Ok(SbU16Positive { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbU16Positive::new(v) succeeds iff v > 0.
proof fn u16_positive_serde_iff(v: u16)
    ensures (v > 0) <==> (SbU16Positive::new(v) matches Ok(_)),
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbU16NonZero { pub value: u16 }

impl SbU16NonZero {
    pub fn new(v: u16) -> (r: Result<Self, ()>)
        ensures
            v != 0 ==> (r matches Ok(p) && p.value == v),
            v == 0 ==> (r matches Err(())),
    {
        if v != 0 { Ok(SbU16NonZero { value: v }) } else { Err(()) }
    }
}

/// Theorem: SbU16NonZero::new(v) succeeds iff v != 0.
proof fn u16_non_zero_serde_iff(v: u16)
    ensures (v != 0) <==> (SbU16NonZero::new(v) matches Ok(_)),
{}

// ============================================================================
// Float serde boundary theorems
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SbF64Positive { pub value: f64 }

impl SbF64Positive {
    pub fn new(value: f64, is_finite: bool, is_positive: bool) -> (r: Result<Self, ()>)
        ensures
            (is_finite && is_positive)   ==> (r matches Ok(p) && p.value == value),
            (!is_finite || !is_positive) ==> (r matches Err(())),
    {
        if is_finite && is_positive { Ok(SbF64Positive { value }) } else { Err(()) }
    }
}

/// Theorem: SbF64Positive::new succeeds iff is_finite && is_positive.
proof fn f64_positive_serde_iff(v: f64, is_finite: bool, is_positive: bool)
    ensures
        (is_finite && is_positive) <==>
            (SbF64Positive::new(v, is_finite, is_positive) matches Ok(_)),
{}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SbF64NonNegative { pub value: f64 }

impl SbF64NonNegative {
    pub fn new(value: f64, is_finite: bool, is_nn: bool) -> (r: Result<Self, ()>)
        ensures
            (is_finite && is_nn)   ==> (r matches Ok(p) && p.value == value),
            (!is_finite || !is_nn) ==> (r matches Err(())),
    {
        if is_finite && is_nn { Ok(SbF64NonNegative { value }) } else { Err(()) }
    }
}

/// Theorem: SbF64NonNegative::new succeeds iff is_finite && is_nn.
proof fn f64_non_negative_serde_iff(v: f64, is_finite: bool, is_nn: bool)
    ensures
        (is_finite && is_nn) <==>
            (SbF64NonNegative::new(v, is_finite, is_nn) matches Ok(_)),
{}

// ============================================================================
// String serde boundary theorem
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbStringNonEmpty { pub validated: bool }

impl SbStringNonEmpty {
    pub fn new(is_empty: bool) -> (r: Result<Self, ()>)
        ensures
            (!is_empty) ==> (r matches Ok(s) && s.validated == true),
            is_empty    ==> (r matches Err(())),
    {
        if !is_empty { Ok(SbStringNonEmpty { validated: true }) } else { Err(()) }
    }
}

/// Theorem: SbStringNonEmpty::new(is_empty) succeeds iff !is_empty.
proof fn string_non_empty_serde_iff(is_empty: bool)
    ensures (!is_empty) <==> (SbStringNonEmpty::new(is_empty) matches Ok(_)),
{}

/// Corollary: any SbStringNonEmpty produced via new() has validated == true.
proof fn string_non_empty_invariant(s: SbStringNonEmpty, b: bool)
    requires SbStringNonEmpty::new(b) matches Ok(q) && q.validated == s.validated,
    ensures  s.validated == true,
{}

// ============================================================================
// URL serde boundary theorems
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbUrlValid { pub validated: bool }

impl SbUrlValid {
    pub fn new(parses: bool) -> (r: Result<Self, ()>)
        ensures
            parses  ==> (r matches Ok(u) && u.validated == true),
            !parses ==> (r matches Err(())),
    {
        if parses { Ok(SbUrlValid { validated: true }) } else { Err(()) }
    }
}

/// Theorem: SbUrlValid::new(parses) succeeds iff parses.
proof fn url_valid_serde_iff(parses: bool)
    ensures parses <==> (SbUrlValid::new(parses) matches Ok(_)),
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbUrlHttps { pub validated: bool }

impl SbUrlHttps {
    pub fn new(parses: bool, is_https: bool) -> (r: Result<Self, ()>)
        ensures
            (parses && is_https)   ==> (r matches Ok(u) && u.validated == true),
            (!parses || !is_https) ==> (r matches Err(())),
    {
        if parses && is_https { Ok(SbUrlHttps { validated: true }) } else { Err(()) }
    }
}

/// Theorem: SbUrlHttps::new succeeds iff parses && is_https.
proof fn url_https_serde_iff(parses: bool, is_https: bool)
    ensures
        (parses && is_https) <==> (SbUrlHttps::new(parses, is_https) matches Ok(_)),
{}

/// Corollary: any SbUrlHttps produced via new() was an HTTPS URL.
proof fn url_https_requires_https(u: SbUrlHttps, parses: bool, is_https: bool)
    requires SbUrlHttps::new(parses, is_https) matches Ok(q) && q.validated == u.validated,
    ensures  is_https,
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbUrlHttp { pub validated: bool }

impl SbUrlHttp {
    pub fn new(parses: bool, is_http: bool) -> (r: Result<Self, ()>)
        ensures
            (parses && is_http)   ==> (r matches Ok(u) && u.validated == true),
            (!parses || !is_http) ==> (r matches Err(())),
    {
        if parses && is_http { Ok(SbUrlHttp { validated: true }) } else { Err(()) }
    }
}

/// Theorem: SbUrlHttp::new succeeds iff parses && is_http.
proof fn url_http_serde_iff(parses: bool, is_http: bool)
    ensures
        (parses && is_http) <==> (SbUrlHttp::new(parses, is_http) matches Ok(_)),
{}

} // verus!
