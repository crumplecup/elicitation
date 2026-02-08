//! Kani proofs for UUID byte validation.
//!
//! These proofs verify the correctness of UUID byte-level validation
//! following RFC 4122 specification.

#![cfg(kani)]

use crate::verification::types::{
    UuidBytes, UuidV4Bytes, UuidV7Bytes, has_valid_variant, has_version, is_valid_v4, is_valid_v7,
};

// ============================================================================
// Variant Validation Proofs
// ============================================================================

#[kani::proof]
fn verify_valid_variant_accepted() {
    let mut bytes: [u8; 16] = kani::any();

    // Force 10xx pattern in byte 8 bits 6-7
    bytes[8] = (bytes[8] & 0x3F) | 0x80;

    // Should be valid
    assert!(has_valid_variant(&bytes));

    // Should construct UuidBytes successfully
    let uuid_bytes = UuidBytes::new(bytes);
    assert!(uuid_bytes.is_ok());
}

#[kani::proof]
fn verify_ncs_variant_rejected() {
    let mut bytes: [u8; 16] = kani::any();

    // Force 0xxx pattern (NCS backward compatibility)
    bytes[8] = bytes[8] & 0x7F; // Clear bit 7

    // Should be invalid
    assert!(!has_valid_variant(&bytes));

    // Should fail construction
    let uuid_bytes = UuidBytes::new(bytes);
    assert!(uuid_bytes.is_err());
}

#[kani::proof]
fn verify_microsoft_variant_rejected() {
    let mut bytes: [u8; 16] = kani::any();

    // Force 110x pattern (Microsoft GUID)
    bytes[8] = (bytes[8] & 0x1F) | 0xC0;

    // Should be invalid
    assert!(!has_valid_variant(&bytes));

    // Should fail construction
    let uuid_bytes = UuidBytes::new(bytes);
    assert!(uuid_bytes.is_err());
}

#[kani::proof]
fn verify_reserved_variant_rejected() {
    let mut bytes: [u8; 16] = kani::any();

    // Force 111x pattern (Reserved)
    bytes[8] = bytes[8] | 0xE0;

    // Should be invalid
    assert!(!has_valid_variant(&bytes));

    // Should fail construction
    let uuid_bytes = UuidBytes::new(bytes);
    assert!(uuid_bytes.is_err());
}

// ============================================================================
// Version Detection Proofs
// ============================================================================

#[kani::proof]
fn verify_version_extraction() {
    let mut bytes: [u8; 16] = kani::any();
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Valid variant

    // Test each version (1-15, plus 0)
    let version: u8 = kani::any();
    kani::assume(version < 16);

    // Set version bits
    bytes[6] = (bytes[6] & 0x0F) | (version << 4);

    // Version extraction should match
    assert!(has_version(&bytes, version));

    // Construct and verify
    let uuid_bytes = UuidBytes::new(bytes).unwrap();
    assert_eq!(uuid_bytes.version(), version);
}

// ============================================================================
// UUID V4 Proofs
// ============================================================================

#[kani::proof]
fn verify_v4_valid_construction() {
    let mut bytes: [u8; 16] = kani::any();

    // Force V4 structure
    bytes[6] = (bytes[6] & 0x0F) | 0x40; // Version 4
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant 10xx

    // Should pass validation
    assert!(is_valid_v4(&bytes));

    // Should construct successfully
    let v4 = UuidV4Bytes::new(bytes);
    assert!(v4.is_ok());

    // Should have correct version
    let v4_unwrapped = v4.unwrap();
    assert_eq!(v4_unwrapped.get().version(), 4);
}

#[kani::proof]
fn verify_v4_wrong_version_rejected() {
    let mut bytes: [u8; 16] = kani::any();

    // Force wrong version (not 4)
    let version: u8 = kani::any();
    kani::assume(version != 4 && version < 16);

    bytes[6] = (bytes[6] & 0x0F) | (version << 4);
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant OK

    // Should fail V4 validation
    assert!(!is_valid_v4(&bytes));

    // Should fail V4 construction
    let v4 = UuidV4Bytes::new(bytes);
    assert!(v4.is_err());
}

#[kani::proof]
fn verify_v4_invalid_variant_rejected() {
    let mut bytes: [u8; 16] = kani::any();

    // Version 4 OK
    bytes[6] = (bytes[6] & 0x0F) | 0x40;

    // Force invalid variant (not 10xx)
    let variant_bits: u8 = kani::any();
    kani::assume((variant_bits & 0xC0) != 0x80); // Not 10xx

    bytes[8] = variant_bits;

    // Should fail validation
    assert!(!is_valid_v4(&bytes));

    // Should fail construction
    let v4 = UuidV4Bytes::new(bytes);
    assert!(v4.is_err());
}

// ============================================================================
// UUID V7 Proofs
// ============================================================================

#[kani::proof]
fn verify_v7_valid_construction() {
    let mut bytes: [u8; 16] = kani::any();

    // Force V7 structure
    bytes[6] = (bytes[6] & 0x0F) | 0x70; // Version 7
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant 10xx

    // Should pass validation
    assert!(is_valid_v7(&bytes));

    // Should construct successfully
    let v7 = UuidV7Bytes::new(bytes);
    assert!(v7.is_ok());

    // Should have correct version
    let v7_unwrapped = v7.unwrap();
    assert_eq!(v7_unwrapped.get().version(), 7);
}

#[kani::proof]
fn verify_v7_wrong_version_rejected() {
    let mut bytes: [u8; 16] = kani::any();

    // Force wrong version (not 7)
    let version: u8 = kani::any();
    kani::assume(version != 7 && version < 16);

    bytes[6] = (bytes[6] & 0x0F) | (version << 4);
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant OK

    // Should fail V7 validation
    assert!(!is_valid_v7(&bytes));

    // Should fail V7 construction
    let v7 = UuidV7Bytes::new(bytes);
    assert!(v7.is_err());
}

#[kani::proof]
fn verify_v7_timestamp_extraction() {
    let mut bytes: [u8; 16] = kani::any();

    // Force V7 structure
    bytes[6] = (bytes[6] & 0x0F) | 0x70; // Version 7
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant 10xx

    // Construct V7
    let v7 = UuidV7Bytes::new(bytes).unwrap();

    // Extract timestamp
    let timestamp = v7.timestamp_ms();

    // Verify it matches first 48 bits (big-endian)
    let expected = ((bytes[0] as u64) << 40)
        | ((bytes[1] as u64) << 32)
        | ((bytes[2] as u64) << 24)
        | ((bytes[3] as u64) << 16)
        | ((bytes[4] as u64) << 8)
        | (bytes[5] as u64);

    assert_eq!(timestamp, expected);
}

// ============================================================================
// Round-Trip Proofs
// ============================================================================

#[kani::proof]
fn verify_uuid_bytes_roundtrip() {
    let mut bytes: [u8; 16] = kani::any();
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Valid variant

    // Construct
    let uuid_bytes = UuidBytes::new(bytes).unwrap();

    // Extract
    let extracted = uuid_bytes.bytes();

    // Should match original
    assert_eq!(bytes, extracted);
}

#[kani::proof]
fn verify_v4_bytes_roundtrip() {
    let mut bytes: [u8; 16] = kani::any();
    bytes[6] = (bytes[6] & 0x0F) | 0x40; // Version 4
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant 10xx

    // Construct
    let v4 = UuidV4Bytes::new(bytes).unwrap();

    // Extract
    let extracted = v4.bytes();

    // Should match original
    assert_eq!(bytes, extracted);
}

#[kani::proof]
fn verify_v7_bytes_roundtrip() {
    let mut bytes: [u8; 16] = kani::any();
    bytes[6] = (bytes[6] & 0x0F) | 0x70; // Version 7
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant 10xx

    // Construct
    let v7 = UuidV7Bytes::new(bytes).unwrap();

    // Extract
    let extracted = v7.bytes();

    // Should match original
    assert_eq!(bytes, extracted);
}

// ============================================================================
// UUID Generator Proofs
// ============================================================================

use crate::{Generator, UuidGenerationMode, UuidGenerator};
use uuid::Uuid;

/// Verify Nil mode produces the nil UUID.
///
/// Castle on cloud: We trust uuid::Uuid::nil() is correct.
/// We verify our generator wrapper calls it correctly.
#[kani::proof]
fn verify_uuid_generator_nil() {
    let mode = UuidGenerationMode::Nil;
    let generator = UuidGenerator::new(mode);

    let uuid = generator.generate();

    // Should produce nil UUID (all zeros)
    assert_eq!(uuid, Uuid::nil(), "Nil mode produces nil UUID");
    assert_eq!(uuid.as_bytes(), &[0u8; 16], "Nil UUID is all zeros");
}

/// Verify Max mode produces the max UUID.
///
/// Castle on cloud: We trust uuid::Uuid::max() is correct.
/// We verify our generator wrapper calls it correctly.
#[kani::proof]
fn verify_uuid_generator_max() {
    let mode = UuidGenerationMode::Max;
    let generator = UuidGenerator::new(mode);

    let uuid = generator.generate();

    // Should produce max UUID (all ones)
    assert_eq!(uuid, Uuid::max(), "Max mode produces max UUID");
    assert_eq!(uuid.as_bytes(), &[0xFFu8; 16], "Max UUID is all ones");
}

/// Verify V4 mode produces a UUID with correct version and variant bits.
///
/// Castle on cloud: We trust uuid::Uuid::new_v4() produces valid random V4 UUIDs.
/// We verify our generator wrapper produces UUIDs with correct format.
#[kani::proof]
fn verify_uuid_generator_v4_format() {
    let mode = UuidGenerationMode::V4;
    let generator = UuidGenerator::new(mode);

    let uuid = generator.generate();
    let bytes = uuid.as_bytes();

    // Verify version 4 (bits 12-15 of time_hi_and_version)
    let version = (bytes[6] & 0xF0) >> 4;
    assert_eq!(version, 4, "V4 mode produces version 4 UUID");

    // Verify RFC 4122 variant (bits 6-7 of clock_seq_hi_and_reserved)
    let variant_bits = bytes[8] & 0xC0;
    assert_eq!(
        variant_bits, 0x80,
        "V4 mode produces RFC 4122 variant (10xx)"
    );
}

/// Verify generator mode is preserved.
///
/// Verifies our struct correctly stores and returns the mode.
#[kani::proof]
fn verify_uuid_generator_mode_preserved() {
    let mode = UuidGenerationMode::Nil;
    let generator = UuidGenerator::new(mode);

    assert_eq!(generator.mode(), mode, "Generator preserves mode");
}

/// Verify Nil generator produces consistent results.
///
/// Verifies deterministic modes (Nil, Max) produce same UUID on repeated calls.
#[kani::proof]
fn verify_uuid_generator_nil_consistent() {
    let mode = UuidGenerationMode::Nil;
    let generator = UuidGenerator::new(mode);

    let uuid1 = generator.generate();
    let uuid2 = generator.generate();

    assert_eq!(uuid1, uuid2, "Nil mode is deterministic");
}

/// Verify Max generator produces consistent results.
///
/// Verifies deterministic modes (Nil, Max) produce same UUID on repeated calls.
#[kani::proof]
fn verify_uuid_generator_max_consistent() {
    let mode = UuidGenerationMode::Max;
    let generator = UuidGenerator::new(mode);

    let uuid1 = generator.generate();
    let uuid2 = generator.generate();

    assert_eq!(uuid1, uuid2, "Max mode is deterministic");
}
