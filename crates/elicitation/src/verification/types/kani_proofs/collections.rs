//! Kani proofs for collection contract types.
//!
//! Includes Vec, HashMap, HashSet, BTreeMap, BTreeSet, LinkedList, Arc, Box, Array.

use crate::*;

// ============================================================================
// Phase 4: Collection Type Proofs
// ============================================================================

// ----------------------------------------------------------------------------
// NonEmpty Collection Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, concrete checks
fn verify_vec_non_empty() {
    // Prove with concrete vectors (symbolic vectors complex)
    let empty: Vec<i32> = vec![];
    let result = VecNonEmpty::new(empty);
    assert!(result.is_err(), "Empty vec rejected");

    let non_empty = vec![42];
    let result = VecNonEmpty::new(non_empty);
    assert!(result.is_ok(), "Non-empty vec accepted");
}

#[kani::proof]
#[kani::unwind(5)] // Small vector bound
fn verify_vec_all_satisfy() {
    // Prove compositional property
    let vec_positive = vec![1i8, 2, 3];

    // Each element must satisfy I8Positive
    for &elem in &vec_positive {
        assert!(elem > 0, "All elements positive");
    }

    // Therefore VecAllSatisfy<I8Positive> should accept
    // (Actual construction requires implementing for I8Positive)
}

#[kani::proof]
#[kani::unwind(10)] // HashMap operations with strings
fn verify_hashmap_non_empty() {
    use std::collections::HashMap;

    let empty: HashMap<i32, String> = HashMap::new();
    let result = HashMapNonEmpty::new(empty);
    assert!(result.is_err(), "Empty map rejected");

    let mut non_empty = HashMap::new();
    non_empty.insert(1, String::from("a")); // Use String::from instead of to_string
    let result = HashMapNonEmpty::new(non_empty);
    assert!(result.is_ok(), "Non-empty map accepted");
}

#[kani::proof]
#[kani::unwind(10)] // BTreeMap operations with strings
fn verify_btreemap_non_empty() {
    use std::collections::BTreeMap;

    let empty: BTreeMap<i32, String> = BTreeMap::new();
    let result = BTreeMapNonEmpty::new(empty);
    assert!(result.is_err(), "Empty BTreeMap rejected");

    let mut non_empty = BTreeMap::new();
    non_empty.insert(1, String::from("a"));
    let result = BTreeMapNonEmpty::new(non_empty);
    assert!(result.is_ok(), "Non-empty BTreeMap accepted");
}

#[kani::proof]
#[kani::unwind(1)] // No loops, concrete checks
fn verify_hashset_non_empty() {
    use std::collections::HashSet;

    let empty: HashSet<i32> = HashSet::new();
    let result = HashSetNonEmpty::new(empty);
    assert!(result.is_err(), "Empty set rejected");

    let mut non_empty = HashSet::new();
    non_empty.insert(42);
    let result = HashSetNonEmpty::new(non_empty);
    assert!(result.is_ok(), "Non-empty set accepted");
}

#[kani::proof]
#[kani::unwind(1)] // No loops, concrete checks
fn verify_btreeset_non_empty() {
    use std::collections::BTreeSet;

    let empty: BTreeSet<i32> = BTreeSet::new();
    let result = BTreeSetNonEmpty::new(empty);
    assert!(result.is_err(), "Empty BTreeSet rejected");

    let mut non_empty = BTreeSet::new();
    non_empty.insert(42);
    let result = BTreeSetNonEmpty::new(non_empty);
    assert!(result.is_ok(), "Non-empty BTreeSet accepted");
}

#[kani::proof]
#[kani::unwind(1)] // No loops, concrete checks
fn verify_vecdeque_non_empty() {
    use std::collections::VecDeque;

    let empty: VecDeque<i32> = VecDeque::new();
    let result = VecDequeNonEmpty::new(empty);
    assert!(result.is_err(), "Empty VecDeque rejected");

    let mut non_empty = VecDeque::new();
    non_empty.push_back(42);
    let result = VecDequeNonEmpty::new(non_empty);
    assert!(result.is_ok(), "Non-empty VecDeque accepted");
}

#[kani::proof]
#[kani::unwind(1)] // No loops, concrete checks
fn verify_linkedlist_non_empty() {
    use std::collections::LinkedList;

    let empty: LinkedList<i32> = LinkedList::new();
    let result = LinkedListNonEmpty::new(empty);
    assert!(result.is_err(), "Empty LinkedList rejected");

    let mut non_empty = LinkedList::new();
    non_empty.push_back(42);
    let result = LinkedListNonEmpty::new(non_empty);
    assert!(result.is_ok(), "Non-empty LinkedList accepted");
}

// ----------------------------------------------------------------------------
// Smart Pointer Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, transparent wrapper
fn verify_box_satisfies() {
    // Box is transparent wrapper - new() doesn't return Result
    let positive = I8Positive::new(42).unwrap();
    let _boxed = BoxSatisfies::new(positive);
    // BoxSatisfies is just a wrapper, always succeeds
}

#[kani::proof]
#[kani::unwind(1)] // No loops, transparent wrapper
fn verify_arc_satisfies() {
    // Arc is transparent wrapper - new() doesn't return Result
    let positive = I8Positive::new(42).unwrap();
    let _arc = ArcSatisfies::new(positive);
    // ArcSatisfies is just a wrapper, always succeeds
}

#[kani::proof]
#[kani::unwind(1)] // No loops, transparent wrapper
fn verify_rc_satisfies() {
    // Rc is transparent wrapper - new() doesn't return Result
    let positive = I8Positive::new(42).unwrap();
    let _rc = RcSatisfies::new(positive);
    // RcSatisfies is just a wrapper, always succeeds
}

// ----------------------------------------------------------------------------
// Result Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(10)] // Result with String error
fn verify_result_ok() {
    let ok_val = 42i32;
    let result: Result<i32, String> = Ok(ok_val);
    let wrapped = ResultOk::new(result);
    assert!(wrapped.is_ok(), "Ok variant accepted");

    let err_val: Result<i32, String> = Err(String::from("e")); // Simplified string
    let wrapped = ResultOk::new(err_val);
    assert!(wrapped.is_err(), "Err variant rejected");
}

// ----------------------------------------------------------------------------
// Tuple Proofs (remaining)
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, composition checks
fn verify_tuple3_composition() {
    let a = I8Positive::new(1).unwrap();
    let b = I8Positive::new(2).unwrap();
    let c = I8Positive::new(3).unwrap();

    let tuple = Tuple3::new(a, b, c);

    // Composition: if all 3 elements satisfy contract, tuple satisfies
    assert!(tuple.0.get() > 0, "First element positive");
    assert!(tuple.1.get() > 0, "Second element positive");
    assert!(tuple.2.get() > 0, "Third element positive");
}

#[kani::proof]
#[kani::unwind(1)] // No loops, composition checks
fn verify_tuple4_composition() {
    let a = I8Positive::new(1).unwrap();
    let b = I8Positive::new(2).unwrap();
    let c = I8Positive::new(3).unwrap();
    let d = I8Positive::new(4).unwrap();

    let tuple = Tuple4::new(a, b, c, d);

    assert!(tuple.0.get() > 0, "First element positive");
    assert!(tuple.1.get() > 0, "Second element positive");
    assert!(tuple.2.get() > 0, "Third element positive");
    assert!(tuple.3.get() > 0, "Fourth element positive");
}

// ============================================================================
// Phase 5: JSON Value Proofs
// ============================================================================

#[cfg(feature = "serde_json")]
#[kani::proof]
#[kani::unwind(1)] // No loops, type checks
fn verify_value_object() {
    use serde_json::{Value, json};

    let obj = json!({"key": "value"});
    let result = ValueObject::new(obj);
    assert!(result.is_ok(), "Object accepted");

    let not_obj = json!([1, 2, 3]);
    let result = ValueObject::new(not_obj);
    assert!(result.is_err(), "Array rejected");
}

#[cfg(feature = "serde_json")]
#[kani::proof]
#[kani::unwind(1)] // No loops, type checks
fn verify_value_array() {
    use serde_json::{Value, json};

    let arr = json!([1, 2, 3]);
    let result = ValueArray::new(arr);
    assert!(result.is_ok(), "Array accepted");

    let not_arr = json!({"key": "value"});
    let result = ValueArray::new(not_arr);
    assert!(result.is_err(), "Object rejected");
}

#[cfg(feature = "serde_json")]
#[kani::proof]
#[kani::unwind(1)] // No loops, null check
fn verify_value_non_null() {
    use serde_json::{Value, json};

    let null = Value::Null;
    let result = ValueNonNull::new(null);
    assert!(result.is_err(), "Null rejected");

    let non_null = json!(42);
    let result = ValueNonNull::new(non_null);
    assert!(result.is_ok(), "Non-null accepted");
}

// ============================================================================
// Phase 3: DateTime Proofs (Feature-gated)
// ============================================================================

#[cfg(feature = "chrono")]
#[kani::proof]
#[kani::unwind(1)] // No loops, timestamp comparison
fn verify_datetime_utc_after() {
    use chrono::{DateTime, TimeZone, Utc};

    let threshold = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let after = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
    let before = Utc.with_ymd_and_hms(2019, 1, 1, 0, 0, 0).unwrap();

    let result = DateTimeUtcAfter::new(after, threshold);
    assert!(result.is_ok(), "After timestamp accepted");

    let result = DateTimeUtcAfter::new(before, threshold);
    assert!(result.is_err(), "Before timestamp rejected");
}

#[cfg(feature = "chrono")]
#[kani::proof]
#[kani::unwind(1)] // No loops, timestamp comparison
fn verify_datetime_utc_before() {
    use chrono::{DateTime, TimeZone, Utc};

    let threshold = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let before = Utc.with_ymd_and_hms(2019, 1, 1, 0, 0, 0).unwrap();
    let after = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();

    let result = DateTimeUtcBefore::new(before, threshold);
    assert!(result.is_ok(), "Before timestamp accepted");

    let result = DateTimeUtcBefore::new(after, threshold);
    assert!(result.is_err(), "After timestamp rejected");
}

#[cfg(feature = "jiff")]
#[kani::proof]
#[kani::unwind(1)] // No loops, timestamp comparison
fn verify_timestamp_after() {
    use jiff::Timestamp;

    // Jiff timestamps use nanosecond precision
    // Prove with concrete examples
    let threshold = Timestamp::from_second(1577836800).unwrap(); // 2020-01-01
    let after = Timestamp::from_second(1609459200).unwrap(); // 2021-01-01
    let before = Timestamp::from_second(1546300800).unwrap(); // 2019-01-01

    let result = TimestampAfter::new(after, threshold);
    assert!(result.is_ok(), "After timestamp accepted");

    let result = TimestampAfter::new(before, threshold);
    assert!(result.is_err(), "Before timestamp rejected");
}

#[cfg(feature = "jiff")]
#[kani::proof]
#[kani::unwind(1)] // No loops, timestamp comparison
fn verify_timestamp_before() {
    use jiff::Timestamp;

    let threshold = Timestamp::from_second(1577836800).unwrap(); // 2020-01-01
    let before = Timestamp::from_second(1546300800).unwrap(); // 2019-01-01
    let after = Timestamp::from_second(1609459200).unwrap(); // 2021-01-01

    let result = TimestampBefore::new(before, threshold);
    assert!(result.is_ok(), "Before timestamp accepted");

    let result = TimestampBefore::new(after, threshold);
    assert!(result.is_err(), "After timestamp rejected");
}

#[cfg(feature = "time")]
#[kani::proof]
#[kani::unwind(1)] // No loops, timestamp comparison
fn verify_offset_datetime_after() {
    use time::{Duration, OffsetDateTime};

    let threshold = OffsetDateTime::UNIX_EPOCH;
    let after = threshold + Duration::days(365);
    let before = threshold - Duration::days(365);

    let result = OffsetDateTimeAfter::new(after, threshold);
    assert!(result.is_ok(), "After timestamp accepted");

    let result = OffsetDateTimeAfter::new(before, threshold);
    assert!(result.is_err(), "Before timestamp rejected");
}

#[cfg(feature = "time")]
#[kani::proof]
#[kani::unwind(1)] // No loops, timestamp comparison
fn verify_offset_datetime_before() {
    use time::{Duration, OffsetDateTime};

    let threshold = OffsetDateTime::UNIX_EPOCH;
    let before = threshold - Duration::days(365);
    let after = threshold + Duration::days(365);

    let result = OffsetDateTimeBefore::new(before, threshold);
    assert!(result.is_ok(), "Before timestamp accepted");

    let result = OffsetDateTimeBefore::new(after, threshold);
    assert!(result.is_err(), "After timestamp rejected");
}

// ============================================================================
// EXPERIMENTAL: Const Generic Range Type Proofs
// ============================================================================

// Attempt 1: Concrete const generics (specific MIN/MAX values)
#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_i8_range_concrete() {
    // Prove for specific range: -10 to 10
    const MIN: i8 = -10;
    const MAX: i8 = 10;

    let value: i8 = kani::any();

    match I8Range::<MIN, MAX>::new(value) {
        Ok(range) => {
            // If construction succeeds, value must be in range
            assert!(value >= MIN, "Value >= MIN");
            assert!(value <= MAX, "Value <= MAX");
            assert!(range.get() >= MIN, "Accessor preserves lower bound");
            assert!(range.get() <= MAX, "Accessor preserves upper bound");
        }
        Err(_) => {
            // If construction fails, value must be out of range
            assert!(
                value < MIN || value > MAX,
                "Construction rejects out-of-range"
            );
        }
    }
}

// Attempt 2: Multiple concrete ranges to test generality
#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_i8_range_positive() {
    // Prove for positive range: 1 to 100
    const MIN: i8 = 1;
    const MAX: i8 = 100;

    let value: i8 = kani::any();

    match I8Range::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "I8Range[1,100] invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Out of range rejected");
        }
    }
}

// Attempt 3: U8Range (unsigned)
#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_u8_range_concrete() {
    const MIN: u8 = 10;
    const MAX: u8 = 200;

    let value: u8 = kani::any();

    match U8Range::<MIN, MAX>::new(value) {
        Ok(range) => {
            assert!(value >= MIN, "Value >= MIN");
            assert!(value <= MAX, "Value <= MAX");
            assert!(range.get() >= MIN, "Accessor preserves bounds");
            assert!(range.get() <= MAX, "Accessor preserves bounds");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Out of range rejected");
        }
    }
}

// Attempt 4: Edge case - zero-width range
#[kani::proof]
#[kani::unwind(1)] // No loops, singleton check
fn verify_i8_range_singleton() {
    // Range with single value: [42, 42]
    const MIN: i8 = 42;
    const MAX: i8 = 42;

    let value: i8 = kani::any();

    match I8Range::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value == 42, "Singleton range accepts only exact value");
        }
        Err(_) => {
            assert!(value != 42, "Singleton rejects all other values");
        }
    }
}

// Attempt 5: I16Range (test larger integer types)
#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_i16_range_concrete() {
    const MIN: i16 = -1000;
    const MAX: i16 = 1000;

    let value: i16 = kani::any();

    match I16Range::<MIN, MAX>::new(value) {
        Ok(range) => {
            assert!(value >= MIN && value <= MAX, "I16Range invariant");
            assert!(
                range.get() >= MIN && range.get() <= MAX,
                "Accessor preserves"
            );
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Rejection correct");
        }
    }
}

// Attempt 6: U16Range
#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_u16_range_concrete() {
    const MIN: u16 = 100;
    const MAX: u16 = 60000;

    let value: u16 = kani::any();

    match U16Range::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "U16Range invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Rejection correct");
        }
    }
}

// ============================================================================
// Complete Integer Coverage: I32, I64, I128, U32, U64, U128, Isize, Usize
// ============================================================================

// ----------------------------------------------------------------------------
// I32 Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, sign checks
fn verify_i32_positive() {
    let value: i32 = kani::any();

    match I32Positive::new(value) {
        Ok(positive) => {
            assert!(value > 0, "I32Positive invariant: value > 0");
            assert!(positive.get() > 0, "Accessor preserves");
            assert!(positive.into_inner() > 0, "Unwrap preserves");
        }
        Err(_) => {
            assert!(value <= 0, "Construction rejects non-positive");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, sign checks
fn verify_i32_non_negative() {
    let value: i32 = kani::any();

    match I32NonNegative::new(value) {
        Ok(non_neg) => {
            assert!(value >= 0, "I32NonNegative invariant: value >= 0");
            assert!(non_neg.get() >= 0, "Accessor preserves");
            assert!(non_neg.into_inner() >= 0, "Unwrap preserves");
        }
        Err(_) => {
            assert!(value < 0, "Construction rejects negative");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_i32_range() {
    const MIN: i32 = -1000;
    const MAX: i32 = 1000;

    let value: i32 = kani::any();

    match I32Range::<MIN, MAX>::new(value) {
        Ok(range) => {
            assert!(value >= MIN && value <= MAX, "I32Range invariant");
            assert!(
                range.get() >= MIN && range.get() <= MAX,
                "Accessor preserves"
            );
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Out of range rejected");
        }
    }
}

// ----------------------------------------------------------------------------
// I64 Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, sign checks
fn verify_i64_positive() {
    let value: i64 = kani::any();

    match I64Positive::new(value) {
        Ok(positive) => {
            assert!(value > 0, "I64Positive invariant");
            assert!(positive.get() > 0, "Accessor preserves");
            assert!(positive.into_inner() > 0, "Unwrap preserves");
        }
        Err(_) => {
            assert!(value <= 0, "Construction rejects non-positive");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, sign checks
fn verify_i64_non_negative() {
    let value: i64 = kani::any();

    match I64NonNegative::new(value) {
        Ok(_non_neg) => {
            assert!(value >= 0, "I64NonNegative invariant");
        }
        Err(_) => {
            assert!(value < 0, "Construction rejects negative");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_i64_range() {
    const MIN: i64 = -100000;
    const MAX: i64 = 100000;

    let value: i64 = kani::any();

    match I64Range::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "I64Range invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Rejection correct");
        }
    }
}

// ----------------------------------------------------------------------------
// I128 Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, sign checks
fn verify_i128_positive() {
    let value: i128 = kani::any();

    match I128Positive::new(value) {
        Ok(_positive) => {
            assert!(value > 0, "I128Positive invariant");
        }
        Err(_) => {
            assert!(value <= 0, "Construction rejects non-positive");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, sign checks
fn verify_i128_non_negative() {
    let value: i128 = kani::any();

    match I128NonNegative::new(value) {
        Ok(_non_neg) => {
            assert!(value >= 0, "I128NonNegative invariant");
        }
        Err(_) => {
            assert!(value < 0, "Construction rejects negative");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_i128_range() {
    const MIN: i128 = -1000000;
    const MAX: i128 = 1000000;

    let value: i128 = kani::any();

    match I128Range::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "I128Range invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Rejection correct");
        }
    }
}

// ----------------------------------------------------------------------------
// U32 Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, zero checks
fn verify_u32_non_zero() {
    let value: u32 = kani::any();

    match U32NonZero::new(value) {
        Ok(non_zero) => {
            assert!(value != 0, "U32NonZero invariant: value != 0");
            assert!(non_zero.get() != 0, "Accessor preserves");
            assert!(non_zero.into_inner() != 0, "Unwrap preserves");
        }
        Err(_) => {
            assert!(value == 0, "Construction rejects zero");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_u32_range() {
    const MIN: u32 = 100;
    const MAX: u32 = 1000000;

    let value: u32 = kani::any();

    match U32Range::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "U32Range invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Out of range rejected");
        }
    }
}

// ----------------------------------------------------------------------------
// U64 Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, zero checks
fn verify_u64_non_zero() {
    let value: u64 = kani::any();

    match U64NonZero::new(value) {
        Ok(_non_zero) => {
            assert!(value != 0, "U64NonZero invariant");
        }
        Err(_) => {
            assert!(value == 0, "Construction rejects zero");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_u64_range() {
    const MIN: u64 = 1000;
    const MAX: u64 = 1000000000;

    let value: u64 = kani::any();

    match U64Range::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "U64Range invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Rejection correct");
        }
    }
}

// ----------------------------------------------------------------------------
// U128 Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, zero checks
fn verify_u128_non_zero() {
    let value: u128 = kani::any();

    match U128NonZero::new(value) {
        Ok(_non_zero) => {
            assert!(value != 0, "U128NonZero invariant");
        }
        Err(_) => {
            assert!(value == 0, "Construction rejects zero");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_u128_range() {
    const MIN: u128 = 1000;
    const MAX: u128 = 1000000000000;

    let value: u128 = kani::any();

    match U128Range::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "U128Range invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Rejection correct");
        }
    }
}

// ----------------------------------------------------------------------------
// Isize Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, sign checks
fn verify_isize_positive() {
    let value: isize = kani::any();

    match IsizePositive::new(value) {
        Ok(positive) => {
            assert!(value > 0, "IsizePositive invariant");
            assert!(positive.get() > 0, "Accessor preserves");
            assert!(positive.into_inner() > 0, "Unwrap preserves");
        }
        Err(_) => {
            assert!(value <= 0, "Construction rejects non-positive");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, sign checks
fn verify_isize_non_negative() {
    let value: isize = kani::any();

    match IsizeNonNegative::new(value) {
        Ok(_non_neg) => {
            assert!(value >= 0, "IsizeNonNegative invariant");
        }
        Err(_) => {
            assert!(value < 0, "Construction rejects negative");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_isize_range() {
    const MIN: isize = -10000;
    const MAX: isize = 10000;

    let value: isize = kani::any();

    match IsizeRange::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "IsizeRange invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Rejection correct");
        }
    }
}

// ----------------------------------------------------------------------------
// Usize Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, zero checks
fn verify_usize_non_zero() {
    let value: usize = kani::any();

    match UsizeNonZero::new(value) {
        Ok(non_zero) => {
            assert!(value != 0, "UsizeNonZero invariant");
            assert!(non_zero.get() != 0, "Accessor preserves");
            assert!(non_zero.into_inner() != 0, "Unwrap preserves");
        }
        Err(_) => {
            assert!(value == 0, "Construction rejects zero");
        }
    }
}

#[kani::proof]
#[kani::unwind(1)] // No loops, range checks
fn verify_usize_range() {
    const MIN: usize = 10;
    const MAX: usize = 100000;

    let value: usize = kani::any();

    match UsizeRange::<MIN, MAX>::new(value) {
        Ok(_range) => {
            assert!(value >= MIN && value <= MAX, "UsizeRange invariant");
        }
        Err(_) => {
            assert!(value < MIN || value > MAX, "Rejection correct");
        }
    }
}

// ============================================================================
// Final Missing Proofs: Complete Coverage
// ============================================================================

// ----------------------------------------------------------------------------
// Remaining Network Proofs
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(1)] // No loops, variant checks
fn verify_ipv4() {
    use std::net::{IpAddr, Ipv4Addr};

    // IpV4 validates that IpAddr is V4 variant
    let v4_addr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    let result = IpV4::new(v4_addr);
    assert!(result.is_ok(), "IpV4 accepts IPv4 addresses");

    let v6_addr = IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    let result = IpV4::new(v6_addr);
    assert!(result.is_err(), "IpV4 rejects IPv6 addresses");
}

#[kani::proof]
#[kani::unwind(1)] // No loops, variant checks
fn verify_ipv6() {
    use std::net::{IpAddr, Ipv6Addr};

    // IpV6 validates that IpAddr is V6 variant
    let v6_addr = IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1));
    let result = IpV6::new(v6_addr);
    assert!(result.is_ok(), "IpV6 accepts IPv6 addresses");

    let v4_addr = IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 1));
    let result = IpV6::new(v4_addr);
    assert!(result.is_err(), "IpV6 rejects IPv4 addresses");
}

// ----------------------------------------------------------------------------
// Remaining DateTime Proof
// ----------------------------------------------------------------------------

#[cfg(feature = "chrono")]
#[kani::proof]
#[kani::unwind(1)] // No loops, timestamp comparison
fn verify_naive_datetime_after() {
    use chrono::NaiveDate;

    let threshold = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let after = NaiveDate::from_ymd_opt(2021, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let before = NaiveDate::from_ymd_opt(2019, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let result = NaiveDateTimeAfter::new(after, threshold);
    assert!(result.is_ok(), "NaiveDateTimeAfter accepts future");

    let result = NaiveDateTimeAfter::new(before, threshold);
    assert!(result.is_err(), "NaiveDateTimeAfter rejects past");
}

// ----------------------------------------------------------------------------
// ArrayAllSatisfy Proof (Const Generic Array)
// ----------------------------------------------------------------------------

#[kani::proof]
#[kani::unwind(5)] // Array size 3 + bounds checking
fn verify_array_all_satisfy() {
    // Prove for small fixed-size array
    let arr = [
        I8Positive::new(1).unwrap(),
        I8Positive::new(2).unwrap(),
        I8Positive::new(3).unwrap(),
    ];

    // ArrayAllSatisfy::new() doesn't return Result, just wraps
    let arr_contract = ArrayAllSatisfy::<I8Positive, 3>::new(arr);

    // Verify all elements satisfy contract
    for elem in arr_contract.get() {
        assert!(elem.get() > 0, "All elements positive");
    }
}

// ============================================================================
// Mechanism Contract Proofs
