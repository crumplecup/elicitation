//! Extern spec contracts for elicitation constructor functions.
//!
//! These are trusted axioms about elicitation's own constructors, telling
//! Creusot what each constructor guarantees so that proof functions can be
//! verified without being marked `#[trusted]`.

use crate::*;
#[cfg(feature = "reqwest")]
use elicitation::StatusCodeValid;
#[cfg(feature = "uuid")]
use elicitation::verification::types::{
    UuidBytes, UuidV4Bytes, UuidV7Bytes, has_valid_variant, has_version, is_valid_v4, is_valid_v7,
};
use elicitation::{
    ArcNonNull, ArcSatisfies, ArrayAllSatisfy, BoolFalse, BoolTrue, BoxNonNull, BoxSatisfies,
    CharAlphabetic, CharAlphanumeric, CharNumeric, DurationPositive, HashMapNonEmpty,
    HashSetNonEmpty, I8NonNegative, I8NonZero, I8Positive, I8Range, I16NonNegative, I16NonZero,
    I16Positive, I32NonNegative, I32NonZero, I32Positive, I64NonNegative, I64NonZero, I64Positive,
    I128NonNegative, I128NonZero, I128Positive, IpPrivate, IpPublic, IpV4, IpV6, Ipv4Loopback,
    Ipv6Loopback, IsizeNonNegative, IsizeNonZero, IsizePositive, IsizeRange, OptionSome, RcNonNull,
    RcSatisfies, ResultOk, StringNonEmpty, Tuple2, Tuple3, Tuple4, U8NonZero, U8Positive, U8Range,
    U16NonZero, U16Positive, U16Range, U32NonZero, U32Positive, U32Range, U64NonZero, U64Positive,
    U64Range, U128NonZero, U128Positive, UsizeNonZero, UsizePositive, UsizeRange, ValidationError,
    VecAllSatisfy, VecDequeNonEmpty, VecNonEmpty,
    verification::types::{
        AuthorityBytes, BalancedDelimiters, Ipv4Bytes, Ipv4Private, Ipv4Public, Ipv6Bytes,
        Ipv6Private, Ipv6Public, MacAddr, PathAbsolute, PathBytes, PathNonEmpty, PathRelative,
        RegexBytes, SchemeBytes, SocketAddrV4Bytes, SocketAddrV6Bytes, UrlAbsoluteBytes, UrlBytes,
        UrlHttpBytes, UrlWithAuthorityBytes, Utf8Bytes, ValidCharClass, ValidEscapes,
        ValidQuantifiers, is_dynamic_port, is_ipv4_private, is_ipv6_private, is_local,
        is_multicast, is_nonzero_port, is_privileged_port, is_registered_port, is_unicast,
        is_universal, is_well_known_port,
    },
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// ============================================================================
// Bool constructors
// ============================================================================

extern_spec! {
    impl BoolTrue {
        #[ensures(value ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!value ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: bool) -> Result<BoolTrue, ValidationError>;
    }
}

extern_spec! {
    impl BoolFalse {
        #[ensures(!value ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: bool) -> Result<BoolFalse, ValidationError>;
    }
}

// ============================================================================
// I8 constructors
// ============================================================================

extern_spec! {
    impl I8Positive {
        #[ensures(value@ > 0 ==> match result { Ok(ref w) => i8pos_inner(*w) == value, Err(_) => false })]
        #[ensures(value@ <= 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i8) -> Result<I8Positive, ValidationError>;
    }
}

extern_spec! {
    impl I8NonNegative {
        #[ensures(value@ >= 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i8) -> Result<I8NonNegative, ValidationError>;
    }
}

extern_spec! {
    impl I8NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i8) -> Result<I8NonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: i8, const MAX: i8> I8Range<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i8) -> Result<I8Range<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// U8 constructors
// ============================================================================

extern_spec! {
    impl U8Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u8) -> Result<U8Positive, ValidationError>;
    }
}

extern_spec! {
    impl U8NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u8) -> Result<U8NonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: u8, const MAX: u8> U8Range<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u8) -> Result<U8Range<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// I16 constructors
// ============================================================================

extern_spec! {
    impl I16Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ <= 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i16) -> Result<I16Positive, ValidationError>;
    }
}

extern_spec! {
    impl I16NonNegative {
        #[ensures(value@ >= 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i16) -> Result<I16NonNegative, ValidationError>;
    }
}

extern_spec! {
    impl I16NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i16) -> Result<I16NonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: i16, const MAX: i16> elicitation::I16Range<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i16) -> Result<elicitation::I16Range<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// U16 constructors
// ============================================================================

extern_spec! {
    impl U16Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u16) -> Result<U16Positive, ValidationError>;
    }
}

extern_spec! {
    impl U16NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u16) -> Result<U16NonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: u16, const MAX: u16> U16Range<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u16) -> Result<U16Range<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// I32 constructors
// ============================================================================

extern_spec! {
    impl I32Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ <= 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i32) -> Result<I32Positive, ValidationError>;
    }
}

extern_spec! {
    impl I32NonNegative {
        #[ensures(value@ >= 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i32) -> Result<I32NonNegative, ValidationError>;
    }
}

extern_spec! {
    impl I32NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i32) -> Result<I32NonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: i32, const MAX: i32> elicitation::I32Range<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i32) -> Result<elicitation::I32Range<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// U32 constructors
// ============================================================================

extern_spec! {
    impl U32Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u32) -> Result<U32Positive, ValidationError>;
    }
}

extern_spec! {
    impl U32NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u32) -> Result<U32NonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: u32, const MAX: u32> U32Range<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u32) -> Result<U32Range<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// I64 constructors
// ============================================================================

extern_spec! {
    impl I64Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ <= 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i64) -> Result<I64Positive, ValidationError>;
    }
}

extern_spec! {
    impl I64NonNegative {
        #[ensures(value@ >= 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i64) -> Result<I64NonNegative, ValidationError>;
    }
}

extern_spec! {
    impl I64NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i64) -> Result<I64NonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: i64, const MAX: i64> elicitation::I64Range<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i64) -> Result<elicitation::I64Range<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// U64 constructors
// ============================================================================

extern_spec! {
    impl U64Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u64) -> Result<U64Positive, ValidationError>;
    }
}

extern_spec! {
    impl U64NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u64) -> Result<U64NonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: u64, const MAX: u64> U64Range<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u64) -> Result<U64Range<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// I128 constructors
// ============================================================================

extern_spec! {
    impl I128Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ <= 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i128) -> Result<I128Positive, ValidationError>;
    }
}

extern_spec! {
    impl I128NonNegative {
        #[ensures(value@ >= 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i128) -> Result<I128NonNegative, ValidationError>;
    }
}

extern_spec! {
    impl I128NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: i128) -> Result<I128NonZero, ValidationError>;
    }
}

// ============================================================================
// U128 constructors
// ============================================================================

extern_spec! {
    impl U128Positive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u128) -> Result<U128Positive, ValidationError>;
    }
}

extern_spec! {
    impl U128NonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u128) -> Result<U128NonZero, ValidationError>;
    }
}

// ============================================================================
// Isize constructors
// ============================================================================

extern_spec! {
    impl IsizePositive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ <= 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: isize) -> Result<IsizePositive, ValidationError>;
    }
}

extern_spec! {
    impl IsizeNonNegative {
        #[ensures(value@ >= 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: isize) -> Result<IsizeNonNegative, ValidationError>;
    }
}

extern_spec! {
    impl IsizeNonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: isize) -> Result<IsizeNonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: isize, const MAX: isize> IsizeRange<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: isize) -> Result<IsizeRange<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// Usize constructors
// ============================================================================

extern_spec! {
    impl UsizePositive {
        #[ensures(value@ > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: usize) -> Result<UsizePositive, ValidationError>;
    }
}

extern_spec! {
    impl UsizeNonZero {
        #[ensures(value@ != 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: usize) -> Result<UsizeNonZero, ValidationError>;
    }
}

extern_spec! {
    impl<const MIN: usize, const MAX: usize> UsizeRange<MIN, MAX> {
        #[ensures(MIN@ <= value@ && value@ <= MAX@ ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < MIN@ || value@ > MAX@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: usize) -> Result<UsizeRange<MIN, MAX>, ValidationError>;
    }
}

// ============================================================================
// Char constructors
// ============================================================================

extern_spec! {
    impl CharAlphabetic {
        #[ensures(char_is_alphabetic(value) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!char_is_alphabetic(value) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: char) -> Result<CharAlphabetic, ValidationError>;
    }
}

extern_spec! {
    impl CharNumeric {
        #[ensures(char_is_numeric(value) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!char_is_numeric(value) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: char) -> Result<CharNumeric, ValidationError>;
    }
}

extern_spec! {
    impl CharAlphanumeric {
        #[ensures(char_is_alphanumeric(value) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!char_is_alphanumeric(value) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: char) -> Result<CharAlphanumeric, ValidationError>;
    }
}

// ============================================================================
// Duration constructors
// ============================================================================

extern_spec! {
    impl DurationPositive {
        #[ensures(duration_is_positive(duration) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!duration_is_positive(duration) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(duration: std::time::Duration) -> Result<DurationPositive, ValidationError>;
    }
}

// ============================================================================
// Utf8Bytes constructors
// ============================================================================

extern_spec! {
    impl<const MAX_LEN: usize> Utf8Bytes<MAX_LEN> {
        #[ensures(len@ > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        #[ensures(len@ <= MAX_LEN@ ==> match result { Ok(ref u) => utf8_len(u) == len, Err(_) => true })]
        #[ensures(len@ == 0 && len@ <= MAX_LEN@ ==> match result { Ok(_) => true, Err(_) => false })]
        fn new(bytes: [u8; MAX_LEN], len: usize) -> Result<Utf8Bytes<MAX_LEN>, ValidationError>;
    }
}

// ============================================================================
// SocketAddrV4Bytes / SocketAddrV6Bytes constructors
// ============================================================================

extern_spec! {
    impl SocketAddrV4Bytes {
        #[ensures(v4_port(result) == port)]
        fn from_octets(ip: [u8; 4], port: u16) -> SocketAddrV4Bytes;
        #[ensures(true)]
        fn ip(&self) -> &Ipv4Bytes;
        #[ensures(true)]
        fn into_parts(self) -> (Ipv4Bytes, u16);
    }
}

extern_spec! {
    impl SocketAddrV6Bytes {
        #[ensures(v6_port(result) == port)]
        fn from_octets(ip: [u8; 16], port: u16) -> SocketAddrV6Bytes;
        #[ensures(true)]
        fn ip(&self) -> &Ipv6Bytes;
        #[ensures(true)]
        fn into_parts(self) -> (Ipv6Bytes, u16);
    }
}

// ============================================================================
// Port classification free functions
// ============================================================================

extern_spec! {
    #[ensures(result == (port@ <= 1023))]
    fn is_well_known_port(port: u16) -> bool;
}

extern_spec! {
    #[ensures(result == (1024 <= port@ && port@ <= 49151))]
    fn is_registered_port(port: u16) -> bool;
}

extern_spec! {
    #[ensures(result == (port@ >= 49152))]
    fn is_dynamic_port(port: u16) -> bool;
}

extern_spec! {
    #[ensures(result == (port@ < 1024))]
    fn is_privileged_port(port: u16) -> bool;
}

extern_spec! {
    #[ensures(result == (port@ != 0))]
    fn is_nonzero_port(port: u16) -> bool;
}

// ============================================================================
// PathBytes constructors
// ============================================================================

extern_spec! {
    impl<const MAX_LEN: usize> PathBytes<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        #[ensures(bytes@.len() <= MAX_LEN@ ==> match result { Ok(ref p) => path_len(p)@ == bytes@.len(), Err(_) => true })]
        #[ensures(bytes@.len() == 0 ==> match result { Ok(ref p) => path_is_empty(p), Err(_) => true })]
        #[ensures(bytes@.len() > 0 && bytes@.len() <= MAX_LEN@ ==> match result { Ok(ref p) => !path_is_empty(p), Err(_) => true })]
        fn from_slice(bytes: &[u8]) -> Result<PathBytes<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

// ============================================================================
// MacAddr constructors and free predicates
// ============================================================================

extern_spec! {
    impl MacAddr {
        #[ensures(mac_octets(result) == octets)]
        #[ensures(mac_is_unicast(result) == (octets[0]@ % 2 == 0))]
        #[ensures(mac_is_multicast(result) == (octets[0]@ % 2 != 0))]
        #[ensures(mac_is_universal(result) == (octets[0]@ % 4 < 2))]
        #[ensures(mac_is_local(result) == (octets[0]@ % 4 >= 2))]
        #[ensures(mac_is_broadcast(result) == (octets[0] == 0xFFu8 && octets[1] == 0xFFu8 && octets[2] == 0xFFu8 && octets[3] == 0xFFu8 && octets[4] == 0xFFu8 && octets[5] == 0xFFu8))]
        #[ensures(mac_is_null(result) == (octets[0] == 0u8 && octets[1] == 0u8 && octets[2] == 0u8 && octets[3] == 0u8 && octets[4] == 0u8 && octets[5] == 0u8))]
        fn new(octets: [u8; 6]) -> MacAddr;
    }
}

extern_spec! {
    #[ensures(result == (octets[0]@ % 2 == 0))]
    fn is_unicast(octets: &[u8; 6]) -> bool;
}

extern_spec! {
    #[ensures(result == (octets[0]@ % 2 != 0))]
    fn is_multicast(octets: &[u8; 6]) -> bool;
}

extern_spec! {
    #[ensures(result == (octets[0]@ % 4 < 2))]
    fn is_universal(octets: &[u8; 6]) -> bool;
}

extern_spec! {
    #[ensures(result == (octets[0]@ % 4 >= 2))]
    fn is_local(octets: &[u8; 6]) -> bool;
}

// ============================================================================
// Collection constructors (Vec, Option, Result, Box/Arc/Rc, VecDeque)
// ============================================================================

extern_spec! {
    impl<T> VecNonEmpty<T> {
        #[ensures(vec@.len() > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(vec@.len() == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(vec: Vec<T>) -> Result<VecNonEmpty<T>, ValidationError>;
    }
}

extern_spec! {
    impl<T> OptionSome<T> {
        #[ensures(match value { Some(_) => match result { Ok(_) => true, Err(_) => false }, None => match result { Err(_) => true, Ok(_) => false } })]
        fn new(value: Option<T>) -> Result<OptionSome<T>, ValidationError>;
    }
}

extern_spec! {
    impl<T> ResultOk<T> {
        #[ensures(match value { Ok(_) => match result { Ok(_) => true, Err(_) => false }, Err(_) => match result { Err(_) => true, Ok(_) => false } })]
        fn new<E>(value: Result<T, E>) -> Result<ResultOk<T>, ValidationError>;
    }
}

extern_spec! {
    impl<T> BoxNonNull<T> {
        #[ensures(match result { Ok(_) => true, Err(_) => false })]
        fn new(b: Box<T>) -> Result<BoxNonNull<T>, ValidationError>;
    }
}

extern_spec! {
    impl<T> ArcNonNull<T> {
        #[ensures(match result { Ok(_) => true, Err(_) => false })]
        fn new(a: std::sync::Arc<T>) -> Result<ArcNonNull<T>, ValidationError>;
    }
}

extern_spec! {
    impl<T> RcNonNull<T> {
        #[ensures(match result { Ok(_) => true, Err(_) => false })]
        fn new(r: std::rc::Rc<T>) -> Result<RcNonNull<T>, ValidationError>;
    }
}

extern_spec! {
    impl<T> VecDequeNonEmpty<T> {
        #[ensures(deque@.len() > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(deque@.len() == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(deque: VecDeque<T>) -> Result<VecDequeNonEmpty<T>, ValidationError>;
    }
}

extern_spec! {
    impl<K: Eq + std::hash::Hash + DeepModel, V> HashMapNonEmpty<K, V> {
        #[ensures(map@.len() > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(map@.len() == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(map: HashMap<K, V>) -> Result<HashMapNonEmpty<K, V>, ValidationError>;
    }
}

extern_spec! {
    impl<T: Eq + std::hash::Hash + DeepModel> HashSetNonEmpty<T> {
        #[ensures(set@.len() > 0 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(set@.len() == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(set: HashSet<T>) -> Result<HashSetNonEmpty<T>, ValidationError>;
    }
}

// ============================================================================
// Ipv4Bytes / Ipv6Bytes constructors
// ============================================================================

extern_spec! {
    impl Ipv4Bytes {
        #[ensures(ipv4_octets(result) == octets)]
        #[ensures(ipv4_is_loopback(result) == (octets[0] == 127u8))]
        #[ensures(ipv4_is_unspecified(result) == (octets[0] == 0u8 && octets[1] == 0u8 && octets[2] == 0u8 && octets[3] == 0u8))]
        #[ensures(ipv4_is_broadcast(result) == (octets[0] == 255u8 && octets[1] == 255u8 && octets[2] == 255u8 && octets[3] == 255u8))]
        fn new(octets: [u8; 4]) -> Ipv4Bytes;
    }
}

extern_spec! {
    impl Ipv6Bytes {
        #[ensures(ipv6_octets(result) == octets)]
        #[ensures(ipv6_is_loopback(result) == (octets[0] == 0u8 && octets[1] == 0u8 && octets[2] == 0u8 && octets[3] == 0u8 && octets[4] == 0u8 && octets[5] == 0u8 && octets[6] == 0u8 && octets[7] == 0u8 && octets[8] == 0u8 && octets[9] == 0u8 && octets[10] == 0u8 && octets[11] == 0u8 && octets[12] == 0u8 && octets[13] == 0u8 && octets[14] == 0u8 && octets[15] == 1u8))]
        #[ensures(ipv6_is_unspecified(result) == (octets[0] == 0u8 && octets[1] == 0u8 && octets[2] == 0u8 && octets[3] == 0u8 && octets[4] == 0u8 && octets[5] == 0u8 && octets[6] == 0u8 && octets[7] == 0u8 && octets[8] == 0u8 && octets[9] == 0u8 && octets[10] == 0u8 && octets[11] == 0u8 && octets[12] == 0u8 && octets[13] == 0u8 && octets[14] == 0u8 && octets[15] == 0u8))]
        fn new(octets: [u8; 16]) -> Ipv6Bytes;
    }
}

// ============================================================================
// is_ipv4_private / is_ipv6_private free functions
// ============================================================================

extern_spec! {
    #[ensures(result == (octets[0]@ == 10 || (octets[0]@ == 172 && octets[1]@ >= 16 && octets[1]@ <= 31) || (octets[0]@ == 192 && octets[1]@ == 168)))]
    fn is_ipv4_private(octets: &[u8; 4]) -> bool;
}

extern_spec! {
    #[ensures(result == (octets[0]@ / 2 == 126))]
    fn is_ipv6_private(octets: &[u8; 16]) -> bool;
}

// ============================================================================
// Ipv4Private / Ipv4Public / Ipv6Private / Ipv6Public constructors
// ============================================================================

extern_spec! {
    impl Ipv4Private {
        #[ensures((octets[0]@ == 10 || (octets[0]@ == 172 && octets[1]@ >= 16 && octets[1]@ <= 31) || (octets[0]@ == 192 && octets[1]@ == 168)) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!(octets[0]@ == 10 || (octets[0]@ == 172 && octets[1]@ >= 16 && octets[1]@ <= 31) || (octets[0]@ == 192 && octets[1]@ == 168)) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(octets: [u8; 4]) -> Result<Ipv4Private, ValidationError>;
    }
}

extern_spec! {
    impl Ipv4Public {
        #[ensures(match result { Ok(_) => true, Err(_) => true })]
        fn new(octets: [u8; 4]) -> Result<Ipv4Public, ValidationError>;
    }
}

extern_spec! {
    impl Ipv6Private {
        #[ensures((octets[0]@ / 2 == 126) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(octets[0]@ / 2 != 126 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(octets: [u8; 16]) -> Result<Ipv6Private, ValidationError>;
    }
}

extern_spec! {
    impl Ipv6Public {
        #[ensures(match result { Ok(_) => true, Err(_) => true })]
        fn new(octets: [u8; 16]) -> Result<Ipv6Public, ValidationError>;
    }
}

// ============================================================================
// std::net::Ipv4Addr / Ipv6Addr constructors (for network type proofs)
// ============================================================================

extern_spec! {
    impl Ipv4Addr {
        #[ensures(ipv4addr_first_octet(result) == a)]
        #[ensures(ipv4addr_second_octet(result) == b)]
        fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr;
    }
}

extern_spec! {
    impl Ipv6Addr {
        #[ensures(ipv6addr_is_loopback(result) == (a == 0u16 && b == 0u16 && c == 0u16 && d == 0u16 && e == 0u16 && f == 0u16 && g == 0u16 && h == 1u16))]
        #[ensures(ipv6addr_is_private(result) == (a@ / 512 == 126))]
        fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr;
    }
}

// ============================================================================
// IpV4 / IpV6 / IpPrivate / IpPublic / Ipv4Loopback / Ipv6Loopback constructors
// ============================================================================

extern_spec! {
    impl IpV4 {
        #[ensures(match ip { IpAddr::V4(_) => match result { Ok(_) => true, Err(_) => false }, IpAddr::V6(_) => match result { Err(_) => true, Ok(_) => false } })]
        fn new(ip: IpAddr) -> Result<IpV4, ValidationError>;
    }
}

extern_spec! {
    impl IpV6 {
        #[ensures(match ip { IpAddr::V6(_) => match result { Ok(_) => true, Err(_) => false }, IpAddr::V4(_) => match result { Err(_) => true, Ok(_) => false } })]
        fn new(ip: IpAddr) -> Result<IpV6, ValidationError>;
    }
}

extern_spec! {
    impl Ipv4Loopback {
        #[ensures(ipv4addr_first_octet(ip) == 127u8 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(ipv4addr_first_octet(ip) != 127u8 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(ip: Ipv4Addr) -> Result<Ipv4Loopback, ValidationError>;
    }
}

extern_spec! {
    impl Ipv6Loopback {
        #[ensures(ipv6addr_is_loopback(ip) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!ipv6addr_is_loopback(ip) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(ip: Ipv6Addr) -> Result<Ipv6Loopback, ValidationError>;
    }
}

extern_spec! {
    impl IpPrivate {
        #[ensures((match ip { IpAddr::V4(v4) => ipv4addr_first_octet(v4)@ == 10 || (ipv4addr_first_octet(v4)@ == 172 && ipv4addr_second_octet(v4)@ >= 16 && ipv4addr_second_octet(v4)@ <= 31) || (ipv4addr_first_octet(v4)@ == 192 && ipv4addr_second_octet(v4)@ == 168), IpAddr::V6(v6) => ipv6addr_is_private(v6) }) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!(match ip { IpAddr::V4(v4) => ipv4addr_first_octet(v4)@ == 10 || (ipv4addr_first_octet(v4)@ == 172 && ipv4addr_second_octet(v4)@ >= 16 && ipv4addr_second_octet(v4)@ <= 31) || (ipv4addr_first_octet(v4)@ == 192 && ipv4addr_second_octet(v4)@ == 168), IpAddr::V6(v6) => ipv6addr_is_private(v6) }) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(ip: IpAddr) -> Result<IpPrivate, ValidationError>;
    }
}

extern_spec! {
    impl IpPublic {
        #[ensures(!(match ip { IpAddr::V4(v4) => ipv4addr_first_octet(v4)@ == 10 || (ipv4addr_first_octet(v4)@ == 172 && ipv4addr_second_octet(v4)@ >= 16 && ipv4addr_second_octet(v4)@ <= 31) || (ipv4addr_first_octet(v4)@ == 192 && ipv4addr_second_octet(v4)@ == 168), IpAddr::V6(v6) => ipv6addr_is_private(v6) }) ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures((match ip { IpAddr::V4(v4) => ipv4addr_first_octet(v4)@ == 10 || (ipv4addr_first_octet(v4)@ == 172 && ipv4addr_second_octet(v4)@ >= 16 && ipv4addr_second_octet(v4)@ <= 31) || (ipv4addr_first_octet(v4)@ == 192 && ipv4addr_second_octet(v4)@ == 168), IpAddr::V6(v6) => ipv6addr_is_private(v6) }) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(ip: IpAddr) -> Result<IpPublic, ValidationError>;
    }
}

// ============================================================================
// StatusCodeValid constructor (reqwest feature)
// ============================================================================

#[cfg(feature = "reqwest")]
extern_spec! {
    impl StatusCodeValid {
        #[ensures(value@ >= 100 && value@ <= 999 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(value@ < 100 || value@ > 999 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: u16) -> Result<StatusCodeValid, ValidationError>;
    }
}

// ============================================================================
// Collection satisfy/wrapper constructors (infallible)
// ============================================================================

extern_spec! {
    impl<C> VecAllSatisfy<C> {
        #[ensures(true)]
        fn new(elements: Vec<C>) -> VecAllSatisfy<C>;
    }
}

extern_spec! {
    impl<C, const N: usize> ArrayAllSatisfy<C, N> {
        #[ensures(true)]
        fn new(elements: [C; N]) -> ArrayAllSatisfy<C, N>;
    }
}

extern_spec! {
    impl<T> BoxSatisfies<T> {
        #[ensures(true)]
        fn new(value: T) -> BoxSatisfies<T>;
    }
}

extern_spec! {
    impl<T> ArcSatisfies<T> {
        #[ensures(true)]
        fn new(value: T) -> ArcSatisfies<T>;
    }
}

extern_spec! {
    impl<T> RcSatisfies<T> {
        #[ensures(true)]
        fn new(value: T) -> RcSatisfies<T>;
    }
}

// ============================================================================
// String::new constructor
// ============================================================================

extern_spec! {
    impl String {
        #[ensures(result@ == Seq::empty())]
        fn new() -> String;
    }
}

// ============================================================================
// StringNonEmpty constructor
// ============================================================================

extern_spec! {
    impl<const MAX_LEN: usize> StringNonEmpty<MAX_LEN> {
        #[ensures(value@.len() == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(value: String) -> Result<StringNonEmpty<MAX_LEN>, ValidationError>;
    }
}

// ============================================================================
// PathAbsolute / PathRelative / PathNonEmpty constructors
// ============================================================================

extern_spec! {
    impl<const MAX_LEN: usize> PathAbsolute<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<PathAbsolute<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn get(&self) -> &PathBytes<MAX_LEN>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

extern_spec! {
    impl<const MAX_LEN: usize> PathRelative<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<PathRelative<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn get(&self) -> &PathBytes<MAX_LEN>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

extern_spec! {
    impl<const MAX_LEN: usize> PathNonEmpty<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        #[ensures(bytes@.len() == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<PathNonEmpty<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn get(&self) -> &PathBytes<MAX_LEN>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

// ============================================================================
// SchemeBytes / AuthorityBytes constructors
// ============================================================================

extern_spec! {
    impl<const MAX_LEN: usize> SchemeBytes<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        #[ensures(bytes@.len() == 0 ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<SchemeBytes<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
        #[ensures(true)]
        fn is_http(&self) -> bool;
    }
}

extern_spec! {
    impl<const MAX_LEN: usize> AuthorityBytes<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<AuthorityBytes<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

// ============================================================================
// UrlBytes / UrlWithAuthorityBytes / UrlAbsoluteBytes / UrlHttpBytes constructors
// ============================================================================

extern_spec! {
    impl<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize, const MAX_LEN: usize>
        UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
    {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

extern_spec! {
    impl<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize, const MAX_LEN: usize>
        UrlWithAuthorityBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
    {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<UrlWithAuthorityBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn url(&self) -> &UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>;
    }
}

extern_spec! {
    impl<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize, const MAX_LEN: usize>
        UrlAbsoluteBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
    {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<UrlAbsoluteBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn url(&self) -> &UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>;
    }
}

extern_spec! {
    impl<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize, const MAX_LEN: usize>
        UrlHttpBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
    {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<UrlHttpBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn url(&self) -> &UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>;
    }
}

// ============================================================================
// BalancedDelimiters / ValidEscapes / ValidQuantifiers / ValidCharClass / RegexBytes constructors
// ============================================================================

extern_spec! {
    impl<const MAX_LEN: usize> BalancedDelimiters<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<BalancedDelimiters<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

extern_spec! {
    impl<const MAX_LEN: usize> ValidEscapes<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<ValidEscapes<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

extern_spec! {
    impl<const MAX_LEN: usize> ValidQuantifiers<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<ValidQuantifiers<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

extern_spec! {
    impl<const MAX_LEN: usize> ValidCharClass<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<ValidCharClass<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

extern_spec! {
    impl<const MAX_LEN: usize> RegexBytes<MAX_LEN> {
        #[ensures(bytes@.len() > MAX_LEN@ ==> match result { Err(_) => true, Ok(_) => false })]
        fn from_slice(bytes: &[u8]) -> Result<RegexBytes<MAX_LEN>, ValidationError>;
        #[ensures(true)]
        fn as_str(&self) -> &str;
    }
}

// ============================================================================
// Tuple2 / Tuple3 / Tuple4 constructors and accessors
// ============================================================================

extern_spec! {
    impl<C1, C2> Tuple2<C1, C2> {
        #[ensures(true)]
        fn new(first: C1, second: C2) -> Tuple2<C1, C2>;
        #[ensures(true)]
        fn first(&self) -> &C1;
        #[ensures(true)]
        fn second(&self) -> &C2;
        #[ensures(true)]
        fn into_inner(self) -> (C1, C2);
    }
}

extern_spec! {
    impl<C1, C2, C3> Tuple3<C1, C2, C3> {
        #[ensures(true)]
        fn new(first: C1, second: C2, third: C3) -> Tuple3<C1, C2, C3>;
        #[ensures(true)]
        fn into_inner(self) -> (C1, C2, C3);
    }
}

extern_spec! {
    impl<C1, C2, C3, C4> Tuple4<C1, C2, C3, C4> {
        #[ensures(true)]
        fn new(first: C1, second: C2, third: C3, fourth: C4) -> Tuple4<C1, C2, C3, C4>;
        #[ensures(true)]
        fn into_inner(self) -> (C1, C2, C3, C4);
    }
}

// ============================================================================
// UuidBytes / UuidV4Bytes / UuidV7Bytes constructors and accessors
// ============================================================================
//
// Bitwise arithmetic equivalences used in specs:
//   `bytes[8]@ / 64 == 2`  ↔  `(bytes[8] & 0xC0) == 0x80`  (variant 10xx)
//   `bytes[6]@ / 16 == n`  ↔  `(bytes[6] & 0xF0) >> 4 == n` (version nibble)

#[cfg(feature = "uuid")]
extern_spec! {
    #[ensures(result == (bytes[8usize]@ / 64 == 2))]
    fn has_valid_variant(bytes: &[u8; 16]) -> bool;
}

#[cfg(feature = "uuid")]
extern_spec! {
    #[ensures(result == (bytes[6usize]@ / 16 == expected@))]
    fn has_version(bytes: &[u8; 16], expected: u8) -> bool;
}

#[cfg(feature = "uuid")]
extern_spec! {
    #[ensures(result == (bytes[8usize]@ / 64 == 2 && bytes[6usize]@ / 16 == 4))]
    fn is_valid_v4(bytes: &[u8; 16]) -> bool;
}

#[cfg(feature = "uuid")]
extern_spec! {
    #[ensures(result == (bytes[8usize]@ / 64 == 2 && bytes[6usize]@ / 16 == 7))]
    fn is_valid_v7(bytes: &[u8; 16]) -> bool;
}

#[cfg(feature = "uuid")]
extern_spec! {
    impl UuidBytes {
        #[ensures(bytes[8usize]@ / 64 == 2 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(bytes[8usize]@ / 64 != 2 ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(bytes: [u8; 16]) -> Result<UuidBytes, ValidationError>;
        #[ensures(true)]
        fn get(&self) -> &[u8; 16];
        #[ensures(true)]
        fn bytes(&self) -> [u8; 16];
        #[ensures(true)]
        fn version(&self) -> u8;
        #[ensures(true)]
        fn has_version(&self, expected: u8) -> bool;
    }
}

#[cfg(feature = "uuid")]
extern_spec! {
    impl UuidV4Bytes {
        #[ensures(bytes[8usize]@ / 64 == 2 && bytes[6usize]@ / 16 == 4 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!(bytes[8usize]@ / 64 == 2 && bytes[6usize]@ / 16 == 4) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(bytes: [u8; 16]) -> Result<UuidV4Bytes, ValidationError>;
        #[ensures(true)]
        fn get(&self) -> &UuidBytes;
        #[ensures(true)]
        fn bytes(&self) -> [u8; 16];
        #[ensures(true)]
        fn into_inner(self) -> UuidBytes;
    }
}

#[cfg(feature = "uuid")]
extern_spec! {
    impl UuidV7Bytes {
        #[ensures(bytes[8usize]@ / 64 == 2 && bytes[6usize]@ / 16 == 7 ==> match result { Ok(_) => true, Err(_) => false })]
        #[ensures(!(bytes[8usize]@ / 64 == 2 && bytes[6usize]@ / 16 == 7) ==> match result { Err(_) => true, Ok(_) => false })]
        fn new(bytes: [u8; 16]) -> Result<UuidV7Bytes, ValidationError>;
        #[ensures(true)]
        fn get(&self) -> &UuidBytes;
        #[ensures(true)]
        fn bytes(&self) -> [u8; 16];
        #[ensures(true)]
        fn timestamp_ms(&self) -> u64;
        #[ensures(true)]
        fn into_inner(self) -> UuidBytes;
    }
}
