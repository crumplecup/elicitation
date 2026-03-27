//! Non-empty proof validation tests.
//!
//! Calls `validate_proofs_non_empty()` on every manually-implemented
//! `Elicitation` type to ensure no method returns `TokenStream::new()`.
//! This catches regressions where a refactor accidentally drops proof
//! content from a manual impl.

#![cfg(feature = "proofs")]

use elicitation::Elicitation;

/// Assert all three proof methods return non-empty TokenStreams.
#[track_caller]
fn assert_proofs_non_empty<T: Elicitation>(label: &str) {
    assert!(!T::kani_proof().is_empty(), "{label}: kani_proof is empty");
    assert!(
        !T::verus_proof().is_empty(),
        "{label}: verus_proof is empty"
    );
    assert!(
        !T::creusot_proof().is_empty(),
        "{label}: creusot_proof is empty"
    );
}

// ============================================================================
// Primitives — bool
// ============================================================================

use elicitation::verification::types::{BoolDefault, BoolFalse, BoolTrue};

#[test]
fn bool_proofs_non_empty() {
    assert_proofs_non_empty::<bool>("bool");
    assert_proofs_non_empty::<BoolDefault>("BoolDefault");
    assert_proofs_non_empty::<BoolTrue>("BoolTrue");
    assert_proofs_non_empty::<BoolFalse>("BoolFalse");
}

// ============================================================================
// Primitives — integers
// ============================================================================

use elicitation::verification::types::{
    I8Default, I8NonNegative, I8NonZero, I8Positive, I16Default, I16NonNegative, I16NonZero,
    I16Positive, I32Default, I32NonNegative, I32NonZero, I32Positive, I64Default, I64NonNegative,
    I64NonZero, I64Positive, U8Default, U8NonZero, U8Positive, U16Default, U16NonZero, U16Positive,
    U32Default, U32NonZero, U32Positive, U64Default, U64NonZero, U64Positive,
};

#[test]
fn integer_proofs_non_empty() {
    assert_proofs_non_empty::<i8>("i8");
    assert_proofs_non_empty::<i16>("i16");
    assert_proofs_non_empty::<i32>("i32");
    assert_proofs_non_empty::<i64>("i64");
    assert_proofs_non_empty::<u8>("u8");
    assert_proofs_non_empty::<u16>("u16");
    assert_proofs_non_empty::<u32>("u32");
    assert_proofs_non_empty::<u64>("u64");
}

#[test]
fn integer_wrapper_proofs_non_empty() {
    assert_proofs_non_empty::<I8Default>("I8Default");
    assert_proofs_non_empty::<I8Positive>("I8Positive");
    assert_proofs_non_empty::<I8NonNegative>("I8NonNegative");
    assert_proofs_non_empty::<I8NonZero>("I8NonZero");
    assert_proofs_non_empty::<I16Default>("I16Default");
    assert_proofs_non_empty::<I16Positive>("I16Positive");
    assert_proofs_non_empty::<I16NonNegative>("I16NonNegative");
    assert_proofs_non_empty::<I16NonZero>("I16NonZero");
    assert_proofs_non_empty::<I32Default>("I32Default");
    assert_proofs_non_empty::<I32Positive>("I32Positive");
    assert_proofs_non_empty::<I32NonNegative>("I32NonNegative");
    assert_proofs_non_empty::<I32NonZero>("I32NonZero");
    assert_proofs_non_empty::<I64Default>("I64Default");
    assert_proofs_non_empty::<I64Positive>("I64Positive");
    assert_proofs_non_empty::<I64NonNegative>("I64NonNegative");
    assert_proofs_non_empty::<I64NonZero>("I64NonZero");
    assert_proofs_non_empty::<U8Default>("U8Default");
    assert_proofs_non_empty::<U8Positive>("U8Positive");
    assert_proofs_non_empty::<U8NonZero>("U8NonZero");
    assert_proofs_non_empty::<U16Default>("U16Default");
    assert_proofs_non_empty::<U16Positive>("U16Positive");
    assert_proofs_non_empty::<U16NonZero>("U16NonZero");
    assert_proofs_non_empty::<U32Default>("U32Default");
    assert_proofs_non_empty::<U32Positive>("U32Positive");
    assert_proofs_non_empty::<U32NonZero>("U32NonZero");
    assert_proofs_non_empty::<U64Default>("U64Default");
    assert_proofs_non_empty::<U64Positive>("U64Positive");
    assert_proofs_non_empty::<U64NonZero>("U64NonZero");
}

// ============================================================================
// Primitives — floats
// ============================================================================

use elicitation::verification::types::{
    F32Default, F32Finite, F32NonNegative, F32Positive, F64Default, F64Finite, F64NonNegative,
    F64Positive,
};

#[test]
fn float_proofs_non_empty() {
    assert_proofs_non_empty::<f32>("f32");
    assert_proofs_non_empty::<f64>("f64");
}

#[test]
fn float_wrapper_proofs_non_empty() {
    assert_proofs_non_empty::<F32Default>("F32Default");
    assert_proofs_non_empty::<F32Finite>("F32Finite");
    assert_proofs_non_empty::<F32NonNegative>("F32NonNegative");
    assert_proofs_non_empty::<F32Positive>("F32Positive");
    assert_proofs_non_empty::<F64Default>("F64Default");
    assert_proofs_non_empty::<F64Finite>("F64Finite");
    assert_proofs_non_empty::<F64NonNegative>("F64NonNegative");
    assert_proofs_non_empty::<F64Positive>("F64Positive");
}

// ============================================================================
// Primitives — char, String, PathBuf, Duration, SystemTime
// ============================================================================

use elicitation::verification::types::{
    CharAlphabetic, CharAlphanumeric, CharNumeric, DurationPositive, PathBufExists, PathBufIsDir,
    PathBufIsFile, PathBufReadable,
};

#[test]
fn char_proofs_non_empty() {
    assert_proofs_non_empty::<char>("char");
    assert_proofs_non_empty::<CharAlphabetic>("CharAlphabetic");
    assert_proofs_non_empty::<CharAlphanumeric>("CharAlphanumeric");
    assert_proofs_non_empty::<CharNumeric>("CharNumeric");
}

#[test]
fn string_proofs_non_empty() {
    assert_proofs_non_empty::<String>("String");
}

#[test]
fn pathbuf_proofs_non_empty() {
    assert_proofs_non_empty::<std::path::PathBuf>("PathBuf");
    assert_proofs_non_empty::<PathBufExists>("PathBufExists");
    assert_proofs_non_empty::<PathBufIsDir>("PathBufIsDir");
    assert_proofs_non_empty::<PathBufIsFile>("PathBufIsFile");
    assert_proofs_non_empty::<PathBufReadable>("PathBufReadable");
}

#[test]
fn duration_proofs_non_empty() {
    assert_proofs_non_empty::<std::time::Duration>("Duration");
    assert_proofs_non_empty::<DurationPositive>("DurationPositive");
}

#[test]
fn systemtime_proofs_non_empty() {
    assert_proofs_non_empty::<std::time::SystemTime>("SystemTime");
}

// ============================================================================
// Primitives — network
// ============================================================================

use elicitation::verification::types::{
    IpPrivate, IpPublic, IpV4, IpV6, Ipv4Loopback, Ipv6Loopback,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[test]
fn network_proofs_non_empty() {
    assert_proofs_non_empty::<IpAddr>("IpAddr");
    assert_proofs_non_empty::<Ipv4Addr>("Ipv4Addr");
    assert_proofs_non_empty::<Ipv6Addr>("Ipv6Addr");
    assert_proofs_non_empty::<SocketAddr>("SocketAddr");
    assert_proofs_non_empty::<SocketAddrV4>("SocketAddrV4");
    assert_proofs_non_empty::<SocketAddrV6>("SocketAddrV6");
    assert_proofs_non_empty::<IpV4>("IpV4");
    assert_proofs_non_empty::<IpV6>("IpV6");
    assert_proofs_non_empty::<IpPrivate>("IpPrivate");
    assert_proofs_non_empty::<IpPublic>("IpPublic");
    assert_proofs_non_empty::<Ipv4Loopback>("Ipv4Loopback");
    assert_proofs_non_empty::<Ipv6Loopback>("Ipv6Loopback");
}

// ============================================================================
// Generic stdlib containers
// ============================================================================

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

#[test]
fn generic_container_proofs_non_empty() {
    assert_proofs_non_empty::<Vec<bool>>("Vec<bool>");
    assert_proofs_non_empty::<Option<bool>>("Option<bool>");
    assert_proofs_non_empty::<Result<bool, String>>("Result<bool,String>");
    assert_proofs_non_empty::<Box<bool>>("Box<bool>");
    assert_proofs_non_empty::<std::sync::Arc<bool>>("Arc<bool>");
    assert_proofs_non_empty::<std::rc::Rc<bool>>("Rc<bool>");
    assert_proofs_non_empty::<HashMap<String, bool>>("HashMap<String,bool>");
    assert_proofs_non_empty::<BTreeMap<String, bool>>("BTreeMap<String,bool>");
    assert_proofs_non_empty::<HashSet<bool>>("HashSet<bool>");
    assert_proofs_non_empty::<BTreeSet<bool>>("BTreeSet<bool>");
    assert_proofs_non_empty::<VecDeque<bool>>("VecDeque<bool>");
    assert_proofs_non_empty::<LinkedList<bool>>("LinkedList<bool>");
    assert_proofs_non_empty::<[bool; 4]>("[bool; 4]");
}

// ============================================================================
// Verification wrapper types
// ============================================================================

use elicitation::verification::types::{
    ArcSatisfies, BTreeMapNonEmpty, BTreeSetNonEmpty, BoxSatisfies, HashMapNonEmpty,
    HashSetNonEmpty, LinkedListNonEmpty, OptionSome, RcSatisfies, ResultOk, VecAllSatisfy,
    VecDequeNonEmpty, VecNonEmpty,
};

#[test]
fn verification_wrapper_proofs_non_empty() {
    assert_proofs_non_empty::<VecNonEmpty<bool>>("VecNonEmpty<bool>");
    assert_proofs_non_empty::<VecAllSatisfy<bool>>("VecAllSatisfy<bool>");
    assert_proofs_non_empty::<OptionSome<bool>>("OptionSome<bool>");
    assert_proofs_non_empty::<ResultOk<bool>>("ResultOk<bool>");
    assert_proofs_non_empty::<BoxSatisfies<bool>>("BoxSatisfies<bool>");
    assert_proofs_non_empty::<ArcSatisfies<bool>>("ArcSatisfies<bool>");
    assert_proofs_non_empty::<RcSatisfies<bool>>("RcSatisfies<bool>");
    assert_proofs_non_empty::<HashMapNonEmpty<String, bool>>("HashMapNonEmpty<String,bool>");
    assert_proofs_non_empty::<BTreeMapNonEmpty<String, bool>>("BTreeMapNonEmpty<String,bool>");
    assert_proofs_non_empty::<HashSetNonEmpty<bool>>("HashSetNonEmpty<bool>");
    assert_proofs_non_empty::<BTreeSetNonEmpty<bool>>("BTreeSetNonEmpty<bool>");
    assert_proofs_non_empty::<VecDequeNonEmpty<bool>>("VecDequeNonEmpty<bool>");
    assert_proofs_non_empty::<LinkedListNonEmpty<bool>>("LinkedListNonEmpty<bool>");
}

// ============================================================================
// URL
// ============================================================================

#[cfg(feature = "url")]
mod url_tests {
    use super::assert_proofs_non_empty;
    use elicitation::verification::types::{
        UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost,
    };

    #[test]
    fn url_proofs_non_empty() {
        assert_proofs_non_empty::<url::Url>("url::Url");
        assert_proofs_non_empty::<UrlValid>("UrlValid");
        assert_proofs_non_empty::<UrlHttp>("UrlHttp");
        assert_proofs_non_empty::<UrlHttps>("UrlHttps");
        assert_proofs_non_empty::<UrlWithHost>("UrlWithHost");
        assert_proofs_non_empty::<UrlCanBeBase>("UrlCanBeBase");
    }
}

// ============================================================================
// UUID
// ============================================================================

#[cfg(feature = "uuid")]
mod uuid_tests {
    use super::assert_proofs_non_empty;
    use elicitation::verification::types::{UuidNonNil, UuidV4};

    #[test]
    fn uuid_proofs_non_empty() {
        assert_proofs_non_empty::<uuid::Uuid>("uuid::Uuid");
        assert_proofs_non_empty::<UuidNonNil>("UuidNonNil");
        assert_proofs_non_empty::<UuidV4>("UuidV4");
    }
}

// ============================================================================
// Unit / trivial types
// ============================================================================

#[test]
fn unit_proofs_non_empty() {
    assert_proofs_non_empty::<()>("()");
}

// ============================================================================
// Derived unit-variant enums (regression: previously produced empty proofs)
// ============================================================================

use elicitation::{Elicit, Prompt, Select};

/// Unit-variant enum with two states — the TicTacToe `Player` case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Elicit)]
enum TwoState {
    A,
    B,
}

/// Unit-variant enum with no `Default` derive — exercises `kani_first_variant_constructible`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Elicit)]
enum ThreeState {
    Alpha,
    Beta,
    Gamma,
}

/// Enum that wraps a unit-variant enum — exercises the cascading-emptiness case.
#[derive(Debug, Clone, PartialEq, Eq, Elicit)]
enum Wrapper {
    Empty,
    Occupied(TwoState),
}

#[test]
fn derived_unit_variant_enum_proofs_non_empty() {
    assert_proofs_non_empty::<TwoState>("TwoState (unit-variant enum regression)");
    assert_proofs_non_empty::<ThreeState>("ThreeState (no Default regression)");
}

#[test]
fn cascading_unit_variant_enum_proofs_non_empty() {
    // Wrapper delegates to TwoState; since TwoState now has a non-empty proof,
    // Wrapper's delegation loop extends by something non-empty.
    assert_proofs_non_empty::<Wrapper>("Wrapper (cascading delegation regression)");
}
