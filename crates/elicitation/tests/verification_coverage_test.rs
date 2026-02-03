//! Test verification coverage across ALL supported types.

use elicitation::Elicit;
use elicitation::verification::types::*;

/// Comprehensive struct testing ALL type categories.
#[derive(Elicit)]
#[allow(dead_code)] // Test struct
struct ComprehensiveTest {
    // Primitives
    age: I8Positive,
    count: U32NonZero,
    ratio: F32Positive,
    flag: BoolTrue,
    initial: CharAlphanumeric,

    // Strings
    name: StringNonEmpty,

    // Collections
    tags: VecNonEmpty<StringNonEmpty>,
    metadata: OptionSome<I32Positive>,
    pair: Tuple2<StringNonEmpty, I8Positive>,

    // Durations
    timeout: DurationPositive,

    // Network types
    #[cfg(feature = "uuid")]
    id: UuidNonNil,

    #[cfg(feature = "url")]
    endpoint: UrlHttps,

    // Filesystem
    config_path: PathBufExists,

    // DateTime (pick one)
    #[cfg(feature = "chrono")]
    created: DateTimeUtcAfter,

    // Value
    #[cfg(feature = "serde_json")]
    data: ValueObject,
}

#[test]
fn test_comprehensive_struct_compiles() {
    // If this compiles, verification code was generated for ALL field types
    assert!(true);
}

// To verify: cargo expand --test verification_coverage_test --features verify-kani,uuid,url,chrono,serde_json
// Should generate __make_ComprehensiveTest with stub_verified for EVERY field type
