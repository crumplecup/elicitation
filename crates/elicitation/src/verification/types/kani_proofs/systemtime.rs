//! Kani proofs for SystemTime and SystemTimeGenerator.

use crate::{Generator, SystemTimeGenerationMode, SystemTimeGenerator};
use std::time::{Duration, SystemTime};

// SystemTime Generator Proofs
// ============================================================================

#[kani::proof]
fn verify_systemtime_unix_epoch() {
    let mode = SystemTimeGenerationMode::UnixEpoch;
    let reference = SystemTime::UNIX_EPOCH; // Use known reference
    let generator = SystemTimeGenerator::with_reference(mode, reference);
    
    let time = generator.generate();
    
    assert!(
        time == SystemTime::UNIX_EPOCH,
        "UnixEpoch mode generates UNIX_EPOCH"
    );
}

#[kani::proof]
fn verify_systemtime_offset_positive() {
    let seconds: i64 = kani::any();
    let nanos: u32 = kani::any();
    
    // Assume valid range for nanos
    kani::assume(nanos < 1_000_000_000);
    // Assume positive offset
    kani::assume(seconds >= 0);
    // Bound seconds to prevent overflow
    kani::assume(seconds < 1_000_000);
    
    let mode = SystemTimeGenerationMode::Offset { seconds, nanos };
    let reference = SystemTime::UNIX_EPOCH;
    let generator = SystemTimeGenerator::with_reference(mode, reference);
    
    let time = generator.generate();
    
    // Time should be reference + offset
    let expected_duration = Duration::new(seconds as u64, nanos);
    let expected = reference + expected_duration;
    
    assert!(
        time == expected,
        "Positive offset adds duration to reference"
    );
}

#[kani::proof]
fn verify_systemtime_offset_negative() {
    let seconds: i64 = kani::any();
    let nanos: u32 = kani::any();
    
    // Assume valid range for nanos
    kani::assume(nanos < 1_000_000_000);
    // Assume negative offset
    kani::assume(seconds < 0);
    // Bound seconds to prevent overflow
    kani::assume(seconds > -1_000_000);
    
    let mode = SystemTimeGenerationMode::Offset { seconds, nanos };
    let reference = SystemTime::UNIX_EPOCH + Duration::from_secs(100_000); // Reference in future
    let generator = SystemTimeGenerator::with_reference(mode, reference);
    
    let time = generator.generate();
    
    // Time should be reference - offset
    let expected_duration = Duration::new(seconds.unsigned_abs(), nanos);
    let expected = reference - expected_duration;
    
    assert!(
        time == expected,
        "Negative offset subtracts duration from reference"
    );
}

#[kani::proof]
fn verify_systemtime_offset_zero() {
    let mode = SystemTimeGenerationMode::Offset {
        seconds: 0,
        nanos: 0,
    };
    let reference = SystemTime::UNIX_EPOCH;
    let generator = SystemTimeGenerator::with_reference(mode, reference);
    
    let time = generator.generate();
    
    assert!(
        time == reference,
        "Zero offset returns reference time unchanged"
    );
}

#[kani::proof]
fn verify_systemtime_generator_mode_preserved() {
    let mode = SystemTimeGenerationMode::UnixEpoch;
    let reference = SystemTime::UNIX_EPOCH;
    let generator = SystemTimeGenerator::with_reference(mode, reference);
    
    assert!(
        generator.mode() == mode,
        "Generator preserves mode"
    );
}

#[kani::proof]
fn verify_systemtime_consistent_generation() {
    let seconds: i64 = kani::any();
    let nanos: u32 = kani::any();
    
    kani::assume(nanos < 1_000_000_000);
    kani::assume(seconds >= 0);
    kani::assume(seconds < 1000); // Small bound for faster verification
    
    let mode = SystemTimeGenerationMode::Offset { seconds, nanos };
    let reference = SystemTime::UNIX_EPOCH;
    let generator = SystemTimeGenerator::with_reference(mode, reference);
    
    // Generate twice with same generator
    let time1 = generator.generate();
    let time2 = generator.generate();
    
    // Both should be identical for deterministic modes (not Now)
    assert!(
        time1 == time2,
        "Generator produces consistent results for Offset mode"
    );
}

#[kani::proof]
fn verify_systemtime_reference_preserved() {
    let reference = SystemTime::UNIX_EPOCH + Duration::from_secs(42);
    let mode = SystemTimeGenerationMode::UnixEpoch;
    let generator = SystemTimeGenerator::with_reference(mode, reference);
    
    assert!(
        generator.reference() == reference,
        "Generator preserves reference time"
    );
}

// SystemTimeGenerationMode::Now cannot be verified with Kani
// ============================================================================
// 
// SystemTimeGenerationMode::Now calls SystemTime::now(), which in turn calls
// the system's clock_gettime() function. This is a foreign C function that
// Kani cannot verify directly.
//
// However, this is part of our "cloud of assumptions" (castle-on-cloud methodology):
// - We assume the OS provides valid SystemTime values via now()
// - We assume SystemTime arithmetic (add/sub Duration) works correctly
// - We verify that OUR code handles these values correctly
//
// The proofs above verify all deterministic modes (UnixEpoch, Offset).
// For Now mode, we rely on:
// 1. SystemTime::now() returns a valid SystemTime (stdlib guarantee)
// 2. Our Generator correctly stores and retrieves the mode
// 3. Our generate() implementation correctly calls now() when mode is Now
//
// These are tested at the unit/integration test level, not formal verification.
