//! Kani proofs for chrono datetime generators.
//!
//! These proofs verify the correctness of chrono generator wrapper logic
//! following the "castle on cloud" pattern.

#![cfg(kani)]
#![cfg(feature = "chrono")]

use crate::{DateTimeUtcGenerationMode, DateTimeUtcGenerator, Generator};
use crate::{NaiveDateTimeGenerationMode, NaiveDateTimeGenerator};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

// ============================================================================
// DateTime<Utc> Generator Proofs
// ============================================================================

/// Verify UnixEpoch mode produces UNIX_EPOCH.
///
/// Castle on cloud: We trust DateTime::UNIX_EPOCH is correct.
/// We verify our generator wrapper selects it correctly.
#[kani::proof]
fn verify_datetime_utc_generator_unix_epoch() {
    let mode = DateTimeUtcGenerationMode::UnixEpoch;
    let reference = DateTime::UNIX_EPOCH;
    let generator = DateTimeUtcGenerator::with_reference(mode, reference);

    let dt = generator.generate();

    assert_eq!(
        dt,
        DateTime::UNIX_EPOCH,
        "UnixEpoch mode produces UNIX_EPOCH"
    );
}

/// Verify positive offset calculation.
///
/// Castle on cloud: We trust chrono's Duration and addition.
/// We verify our wrapper computes offset correctly.
#[kani::proof]
fn verify_datetime_utc_generator_offset_positive() {
    let seconds: i64 = kani::any();

    // Constrain to reasonable range
    kani::assume(seconds > 0);
    kani::assume(seconds < 100_000); // ~27 hours

    let mode = DateTimeUtcGenerationMode::Offset { seconds };
    let reference = DateTime::UNIX_EPOCH;
    let generator = DateTimeUtcGenerator::with_reference(mode, reference);

    let dt = generator.generate();

    // Verify dt is after reference
    assert!(dt > reference, "Positive offset produces future time");
}

/// Verify negative offset calculation.
#[kani::proof]
fn verify_datetime_utc_generator_offset_negative() {
    let seconds: i64 = kani::any();

    // Constrain to reasonable range
    kani::assume(seconds < 0);
    kani::assume(seconds > -100_000); // ~27 hours

    let mode = DateTimeUtcGenerationMode::Offset { seconds };
    // Use future reference so we can subtract
    let reference = DateTime::UNIX_EPOCH + Duration::try_seconds(200_000).unwrap();
    let generator = DateTimeUtcGenerator::with_reference(mode, reference);

    let dt = generator.generate();

    // Verify dt is before reference
    assert!(dt < reference, "Negative offset produces past time");
}

/// Verify zero offset returns reference unchanged.
#[kani::proof]
fn verify_datetime_utc_generator_offset_zero() {
    let mode = DateTimeUtcGenerationMode::Offset { seconds: 0 };
    let reference = DateTime::UNIX_EPOCH;
    let generator = DateTimeUtcGenerator::with_reference(mode, reference);

    let dt = generator.generate();

    assert_eq!(dt, reference, "Zero offset returns reference unchanged");
}

/// Verify generator mode is preserved.
#[kani::proof]
fn verify_datetime_utc_generator_mode_preserved() {
    let mode = DateTimeUtcGenerationMode::UnixEpoch;
    let reference = DateTime::UNIX_EPOCH;
    let generator = DateTimeUtcGenerator::with_reference(mode, reference);

    assert_eq!(generator.mode(), mode, "Generator preserves mode");
}

/// Verify generator reference is preserved.
#[kani::proof]
fn verify_datetime_utc_generator_reference_preserved() {
    let mode = DateTimeUtcGenerationMode::UnixEpoch;
    let reference = DateTime::UNIX_EPOCH;
    let generator = DateTimeUtcGenerator::with_reference(mode, reference);

    assert_eq!(
        generator.reference(),
        reference,
        "Generator preserves reference"
    );
}

// ============================================================================
// NaiveDateTime Generator Proofs
// ============================================================================

/// Verify UnixEpoch mode produces UNIX_EPOCH.
#[kani::proof]
fn verify_naive_datetime_generator_unix_epoch() {
    let mode = NaiveDateTimeGenerationMode::UnixEpoch;
    let reference = NaiveDateTime::UNIX_EPOCH;
    let generator = NaiveDateTimeGenerator::with_reference(mode, reference);

    let dt = generator.generate();

    assert_eq!(
        dt,
        NaiveDateTime::UNIX_EPOCH,
        "UnixEpoch mode produces UNIX_EPOCH"
    );
}

/// Verify positive offset calculation.
#[kani::proof]
fn verify_naive_datetime_generator_offset_positive() {
    let seconds: i64 = kani::any();

    kani::assume(seconds > 0);
    kani::assume(seconds < 100_000);

    let mode = NaiveDateTimeGenerationMode::Offset { seconds };
    let reference = NaiveDateTime::UNIX_EPOCH;
    let generator = NaiveDateTimeGenerator::with_reference(mode, reference);

    let dt = generator.generate();

    assert!(dt > reference, "Positive offset produces future time");
}

/// Verify negative offset calculation.
#[kani::proof]
fn verify_naive_datetime_generator_offset_negative() {
    let seconds: i64 = kani::any();

    kani::assume(seconds < 0);
    kani::assume(seconds > -100_000);

    let mode = NaiveDateTimeGenerationMode::Offset { seconds };
    let reference = NaiveDateTime::UNIX_EPOCH + Duration::try_seconds(200_000).unwrap();
    let generator = NaiveDateTimeGenerator::with_reference(mode, reference);

    let dt = generator.generate();

    assert!(dt < reference, "Negative offset produces past time");
}

/// Verify zero offset returns reference unchanged.
#[kani::proof]
fn verify_naive_datetime_generator_offset_zero() {
    let mode = NaiveDateTimeGenerationMode::Offset { seconds: 0 };
    let reference = NaiveDateTime::UNIX_EPOCH;
    let generator = NaiveDateTimeGenerator::with_reference(mode, reference);

    let dt = generator.generate();

    assert_eq!(dt, reference, "Zero offset returns reference unchanged");
}

/// Verify generator mode is preserved.
#[kani::proof]
fn verify_naive_datetime_generator_mode_preserved() {
    let mode = NaiveDateTimeGenerationMode::UnixEpoch;
    let reference = NaiveDateTime::UNIX_EPOCH;
    let generator = NaiveDateTimeGenerator::with_reference(mode, reference);

    assert_eq!(generator.mode(), mode, "Generator preserves mode");
}

/// Verify generator reference is preserved.
#[kani::proof]
fn verify_naive_datetime_generator_reference_preserved() {
    let mode = NaiveDateTimeGenerationMode::UnixEpoch;
    let reference = NaiveDateTime::UNIX_EPOCH;
    let generator = NaiveDateTimeGenerator::with_reference(mode, reference);

    assert_eq!(
        generator.reference(),
        reference,
        "Generator preserves reference"
    );
}
