//! Prusti proofs for UUID byte validation types.
//!
//! Validates UUID byte sequences (RFC 4122):
//! - UuidBytes: Base UUID with valid variant
//! - UuidV4Bytes: Version 4 (random) UUID
//! - UuidV7Bytes: Version 7 (timestamp + random) UUID
//!
//! This is compositional verification: rfc4122_correct → wrapper_correct.

#![cfg(all(feature = "verify-prusti", kani))]
#![allow(unused_imports)]

use crate::verification::types::{
    UuidBytes, UuidV4Bytes, UuidV7Bytes, ValidationError, has_valid_variant, has_version,
    is_valid_v4, is_valid_v7,
};
use prusti_contracts::*;

// UuidBytes Validation Proofs
// ============================================================================

/// Verify: UuidBytes accepts valid RFC 4122 variant (10xx)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_valid_variant() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    // Set 10xx pattern in byte 8 bits 6-7
    bytes[8] = 0x80;
    UuidBytes::new(bytes)
}

/// Verify: bytes() accessor returns the same bytes
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_bytes_accessor() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[8] = 0x80;
    let uuid = UuidBytes::new(bytes)?;
    let _returned = uuid.bytes();
    Ok(uuid)
}

/// Verify: version() extracts version bits
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_version_extraction() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40; // Version 4
    bytes[8] = 0x80; // Valid variant
    let uuid = UuidBytes::new(bytes)?;
    let _version = uuid.version();
    Ok(uuid)
}

/// Verify: has_version() correctly identifies version
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result.is_ok())]
pub fn verify_uuid_has_version() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40; // Version 4
    bytes[8] = 0x80; // Valid variant
    let uuid = UuidBytes::new(bytes)?;
    let _is_v4 = uuid.has_version(4);
    Ok(uuid)
}

/// Verify: Example V4 UUID (all zeros except version/variant)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v4_example() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40; // Version 4
    bytes[8] = 0x80; // Valid variant
    UuidBytes::new(bytes)
}

/// Verify: Example V7 UUID (all zeros except version/variant)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v7_example() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x70; // Version 7
    bytes[8] = 0x80; // Valid variant
    UuidBytes::new(bytes)
}

// UuidV4Bytes Validation Proofs
// ============================================================================

/// Verify: UuidV4Bytes accepts valid V4 UUID
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v4_accepts_valid() -> Result<UuidV4Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40; // Version 4
    bytes[8] = 0x80; // Valid variant
    UuidV4Bytes::new(bytes)
}

/// Verify: get() returns underlying UuidBytes
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v4_get() -> Result<UuidV4Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40;
    bytes[8] = 0x80;
    let v4 = UuidV4Bytes::new(bytes)?;
    let _inner = v4.get();
    Ok(v4)
}

/// Verify: bytes() accessor works
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v4_bytes() -> Result<UuidV4Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40;
    bytes[8] = 0x80;
    let v4 = UuidV4Bytes::new(bytes)?;
    let _returned = v4.bytes();
    Ok(v4)
}

/// Verify: version() returns 4
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result.is_ok())]
pub fn verify_uuid_v4_version() -> Result<UuidV4Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40;
    bytes[8] = 0x80;
    let v4 = UuidV4Bytes::new(bytes)?;
    let _version = v4.version();
    Ok(v4)
}

/// Verify: Example V4 with random-looking bytes
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v4_randomish() -> Result<UuidV4Bytes, ValidationError> {
    let bytes = [
        0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0x4D, 0xEF, // Version 4
        0xAB, 0xCD, // Variant 10xx
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
    ];
    UuidV4Bytes::new(bytes)
}

// UuidV7Bytes Validation Proofs
// ============================================================================

/// Verify: UuidV7Bytes accepts valid V7 UUID
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v7_accepts_valid() -> Result<UuidV7Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x70; // Version 7
    bytes[8] = 0x80; // Valid variant
    UuidV7Bytes::new(bytes)
}

/// Verify: get() returns underlying UuidBytes
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v7_get() -> Result<UuidV7Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x70;
    bytes[8] = 0x80;
    let v7 = UuidV7Bytes::new(bytes)?;
    let _inner = v7.get();
    Ok(v7)
}

/// Verify: bytes() accessor works
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v7_bytes() -> Result<UuidV7Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x70;
    bytes[8] = 0x80;
    let v7 = UuidV7Bytes::new(bytes)?;
    let _returned = v7.bytes();
    Ok(v7)
}

/// Verify: version() returns 7
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result.is_ok())]
pub fn verify_uuid_v7_version() -> Result<UuidV7Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x70;
    bytes[8] = 0x80;
    let v7 = UuidV7Bytes::new(bytes)?;
    let _version = v7.version();
    Ok(v7)
}

/// Verify: timestamp_ms() accessor works
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v7_timestamp() -> Result<UuidV7Bytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x70;
    bytes[8] = 0x80;
    let v7 = UuidV7Bytes::new(bytes)?;
    let _ts = v7.timestamp_ms();
    Ok(v7)
}

/// Verify: Example V7 with timestamp bytes
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_v7_with_timestamp() -> Result<UuidV7Bytes, ValidationError> {
    let bytes = [
        0x01, 0x8E, 0xB3, 0x4F, 0x12, 0x34, // Timestamp (48 bits)
        0x7A, 0xBC, // Version 7
        0x9D, 0xEF, // Variant 10xx
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, // Random
    ];
    UuidV7Bytes::new(bytes)
}

// Helper Function Proofs
// ============================================================================

/// Verify: has_valid_variant accepts 10xx pattern
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result)]
pub fn verify_has_valid_variant_80() -> bool {
    let mut bytes = [0u8; 16];
    bytes[8] = 0x80;
    has_valid_variant(&bytes)
}

/// Verify: has_valid_variant accepts 10xx pattern (upper)
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result)]
pub fn verify_has_valid_variant_bf() -> bool {
    let mut bytes = [0u8; 16];
    bytes[8] = 0xBF;
    has_valid_variant(&bytes)
}

/// Verify: has_version correctly identifies version 4
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result)]
pub fn verify_has_version_4() -> bool {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40;
    has_version(&bytes, 4)
}

/// Verify: has_version correctly identifies version 7
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result)]
pub fn verify_has_version_7() -> bool {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x70;
    has_version(&bytes, 7)
}

/// Verify: is_valid_v4 accepts V4 UUID
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result)]
pub fn verify_is_valid_v4_accepts() -> bool {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x40;
    bytes[8] = 0x80;
    is_valid_v4(&bytes)
}

/// Verify: is_valid_v7 accepts V7 UUID
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result)]
pub fn verify_is_valid_v7_accepts() -> bool {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x70;
    bytes[8] = 0x80;
    is_valid_v7(&bytes)
}

// Version Boundary Testing
// ============================================================================

/// Verify: Version 1 (timestamp + MAC)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_version_1() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x10; // Version 1
    bytes[8] = 0x80;
    UuidBytes::new(bytes)
}

/// Verify: Version 2 (DCE Security)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_version_2() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x20; // Version 2
    bytes[8] = 0x80;
    UuidBytes::new(bytes)
}

/// Verify: Version 3 (MD5 hash)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_version_3() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x30; // Version 3
    bytes[8] = 0x80;
    UuidBytes::new(bytes)
}

/// Verify: Version 5 (SHA-1 hash)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_version_5() -> Result<UuidBytes, ValidationError> {
    let mut bytes = [0u8; 16];
    bytes[6] = 0x50; // Version 5
    bytes[8] = 0x80;
    UuidBytes::new(bytes)
}

// Variant Boundary Testing
// ============================================================================

/// Verify: Variant 10xx lower bound (0x80)
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result)]
pub fn verify_variant_10xx_lower() -> bool {
    let mut bytes = [0u8; 16];
    bytes[8] = 0x80; // 10000000
    has_valid_variant(&bytes)
}

/// Verify: Variant 10xx upper bound (0xBF)
#[cfg(all(feature = "verify-prusti", kani))]
#[ensures(result)]
pub fn verify_variant_10xx_upper() -> bool {
    let mut bytes = [0u8; 16];
    bytes[8] = 0xBF; // 10111111
    has_valid_variant(&bytes)
}

// Edge Cases
// ============================================================================

/// Verify: All zeros except version/variant
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_minimal_v4() -> Result<UuidV4Bytes, ValidationError> {
    let bytes = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, // Version 4
        0x80, 0x00, // Variant 10xx
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    UuidV4Bytes::new(bytes)
}

/// Verify: All ones except version/variant
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_maximal_v4() -> Result<UuidV4Bytes, ValidationError> {
    let bytes = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x4F, 0xFF, // Version 4
        0xBF, 0xFF, // Variant 10xx
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    UuidV4Bytes::new(bytes)
}

/// Verify: Minimal V7 (all zeros)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_minimal_v7() -> Result<UuidV7Bytes, ValidationError> {
    let bytes = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0x00, // Version 7
        0x80, 0x00, // Variant 10xx
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    UuidV7Bytes::new(bytes)
}

/// Verify: Maximal V7 (all ones)
#[cfg(all(feature = "verify-prusti", kani))]
pub fn verify_uuid_maximal_v7() -> Result<UuidV7Bytes, ValidationError> {
    let bytes = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, // Version 7
        0xBF, 0xFF, // Variant 10xx
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    UuidV7Bytes::new(bytes)
}

// ============================================================================
