//! UUID byte-level validation foundation.
//!
//! This module provides validated UUID byte sequences following RFC 4122.
//! It forms the foundation for version-specific UUID contract types.

#[cfg(kani)]
use super::ValidationError;

// ============================================================================
// UUID Byte Structure (RFC 4122)
// ============================================================================
//
// 16 bytes (128 bits): xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx
//
// Byte 6, bits 4-7: VERSION (M)
//   - 0001 (1) = V1 (timestamp + MAC)
//   - 0010 (2) = V2 (DCE Security)
//   - 0011 (3) = V3 (MD5 hash)
//   - 0100 (4) = V4 (random)
//   - 0101 (5) = V5 (SHA-1 hash)
//   - 0111 (7) = V7 (timestamp + random)
//
// Byte 8, bits 6-7: VARIANT (N)
//   - 10xx = RFC 4122 variant (standard)
//   - 0xxx = NCS backward compatibility
//   - 110x = Microsoft GUID
//   - 111x = Reserved for future use
//
// V4 Structure: All other bits SHOULD be random
// V7 Structure: Bytes 0-5 = unix_ts_ms (48 bits), rest random

// ============================================================================
// Core UUID Bytes Type
// ============================================================================

/// A validated UUID byte sequence.
///
/// This type guarantees:
/// - Exactly 16 bytes
/// - Valid RFC 4122 variant (10xx in byte 8 bits 6-7)
///
/// Does NOT guarantee version-specific constraints (use `UuidV4Bytes`, `UuidV7Bytes`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(kani)]
pub struct UuidBytes {
    bytes: [u8; 16],
}

#[cfg(kani)]
impl UuidBytes {
    /// Create a new UuidBytes, validating RFC 4122 variant.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::InvalidUuidVariant` if variant bits aren't 10xx.
    pub fn new(bytes: [u8; 16]) -> Result<Self, ValidationError> {
        if !has_valid_variant(&bytes) {
            return Err(ValidationError::InvalidUuidVariant);
        }
        Ok(Self { bytes })
    }

    /// Get a reference to the byte array.
    pub fn get(&self) -> &[u8; 16] {
        &self.bytes
    }

    /// Get the raw byte array.
    pub fn bytes(&self) -> [u8; 16] {
        self.bytes
    }

    /// Get the version number (bits 4-7 of byte 6).
    pub fn version(&self) -> u8 {
        (self.bytes[6] & 0xF0) >> 4
    }

    /// Check if this UUID has a specific version.
    pub fn has_version(&self, expected: u8) -> bool {
        self.version() == expected
    }
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Check if bytes have valid RFC 4122 variant (10xx pattern in byte 8 bits 6-7).
#[cfg(kani)]
pub fn has_valid_variant(bytes: &[u8; 16]) -> bool {
    // Byte 8, bits 6-7 must be 10
    (bytes[8] & 0xC0) == 0x80
}

/// Check if bytes have a specific version (bits 4-7 of byte 6).
#[cfg(kani)]
pub fn has_version(bytes: &[u8; 16], expected: u8) -> bool {
    let version = (bytes[6] & 0xF0) >> 4;
    version == expected
}

/// Check if bytes have valid V4 structure (version 4, valid variant).
#[cfg(kani)]
pub fn is_valid_v4(bytes: &[u8; 16]) -> bool {
    has_version(bytes, 4) && has_valid_variant(bytes)
}

/// Check if bytes have valid V7 structure (version 7, valid variant).
#[cfg(kani)]
pub fn is_valid_v7(bytes: &[u8; 16]) -> bool {
    has_version(bytes, 7) && has_valid_variant(bytes)
}

// ============================================================================
// Version-Specific Types
// ============================================================================

/// A UUID byte sequence guaranteed to be Version 4 (random).
///
/// Version 4 UUIDs have:
/// - Version bits = 0100 (4)
/// - Variant bits = 10xx (RFC 4122)
/// - All other bits SHOULD be random
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(kani)]
pub struct UuidV4Bytes(UuidBytes);

#[cfg(kani)]
impl UuidV4Bytes {
    /// Create a new UuidV4Bytes, validating version and variant.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::WrongUuidVersion` if version isn't 4.
    /// Returns `ValidationError::InvalidUuidVariant` if variant isn't 10xx.
    pub fn new(bytes: [u8; 16]) -> Result<Self, ValidationError> {
        // Validate variant first (base constraint)
        let uuid_bytes = UuidBytes::new(bytes)?;

        // Validate version
        if !uuid_bytes.has_version(4) {
            return Err(ValidationError::WrongUuidVersion {
                expected: 4,
                got: uuid_bytes.version(),
            });
        }

        Ok(Self(uuid_bytes))
    }

    /// Get a reference to the underlying UuidBytes.
    pub fn get(&self) -> &UuidBytes {
        &self.0
    }

    /// Get the raw byte array.
    pub fn bytes(&self) -> [u8; 16] {
        self.0.bytes()
    }

    /// Unwrap into the underlying UuidBytes.
    pub fn into_inner(self) -> UuidBytes {
        self.0
    }
}

/// A UUID byte sequence guaranteed to be Version 7 (timestamp + random).
///
/// Version 7 UUIDs have:
/// - Version bits = 0111 (7)
/// - Variant bits = 10xx (RFC 4122)
/// - Bytes 0-5: Unix timestamp in milliseconds (48 bits)
/// - Remaining bits: Random
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg(kani)]
pub struct UuidV7Bytes(UuidBytes);

#[cfg(kani)]
impl UuidV7Bytes {
    /// Create a new UuidV7Bytes, validating version and variant.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::WrongUuidVersion` if version isn't 7.
    /// Returns `ValidationError::InvalidUuidVariant` if variant isn't 10xx.
    pub fn new(bytes: [u8; 16]) -> Result<Self, ValidationError> {
        // Validate variant first (base constraint)
        let uuid_bytes = UuidBytes::new(bytes)?;

        // Validate version
        if !uuid_bytes.has_version(7) {
            return Err(ValidationError::WrongUuidVersion {
                expected: 7,
                got: uuid_bytes.version(),
            });
        }

        Ok(Self(uuid_bytes))
    }

    /// Get a reference to the underlying UuidBytes.
    pub fn get(&self) -> &UuidBytes {
        &self.0
    }

    /// Get the raw byte array.
    pub fn bytes(&self) -> [u8; 16] {
        self.0.bytes()
    }

    /// Extract the Unix timestamp in milliseconds (first 48 bits).
    pub fn timestamp_ms(&self) -> u64 {
        let bytes = self.0.bytes();
        // Read big-endian 48-bit timestamp from bytes 0-5
        ((bytes[0] as u64) << 40)
            | ((bytes[1] as u64) << 32)
            | ((bytes[2] as u64) << 24)
            | ((bytes[3] as u64) << 16)
            | ((bytes[4] as u64) << 8)
            | (bytes[5] as u64)
    }

    /// Unwrap into the underlying UuidBytes.
    pub fn into_inner(self) -> UuidBytes {
        self.0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    

    #[test]
    #[cfg(kani)]
    fn test_valid_variant() {
        // 10xx pattern in byte 8 bits 6-7
        let mut bytes = [0u8; 16];
        bytes[8] = 0b1000_0000; // 10xx
        assert!(has_valid_variant(&bytes));

        bytes[8] = 0b1011_1111; // 10xx (with other bits set)
        assert!(has_valid_variant(&bytes));
    }

    #[test]
    #[cfg(kani)]
    fn test_invalid_variant() {
        let mut bytes = [0u8; 16];

        // 00xx (NCS)
        bytes[8] = 0b0000_0000;
        assert!(!has_valid_variant(&bytes));

        // 110x (Microsoft)
        bytes[8] = 0b1100_0000;
        assert!(!has_valid_variant(&bytes));

        // 111x (Reserved)
        bytes[8] = 0b1110_0000;
        assert!(!has_valid_variant(&bytes));
    }

    #[test]
    #[cfg(kani)]
    fn test_version_extraction() {
        let mut bytes = [0u8; 16];
        bytes[8] = 0x80; // Valid variant

        // Version 4
        bytes[6] = 0b0100_0000;
        assert!(has_version(&bytes, 4));

        // Version 7
        bytes[6] = 0b0111_0000;
        assert!(has_version(&bytes, 7));
    }

    #[test]
    #[cfg(kani)]
    fn test_uuid_v4_bytes_valid() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x40; // Version 4
        bytes[8] = 0x80; // Variant 10xx

        let uuid = UuidV4Bytes::new(bytes);
        assert!(uuid.is_ok());
    }

    #[test]
    #[cfg(kani)]
    fn test_uuid_v4_bytes_wrong_version() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x70; // Version 7 (not 4)
        bytes[8] = 0x80; // Variant 10xx

        let uuid = UuidV4Bytes::new(bytes);
        assert!(uuid.is_err());
    }

    #[test]
    #[cfg(kani)]
    fn test_uuid_v7_timestamp_extraction() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x70; // Version 7
        bytes[8] = 0x80; // Variant 10xx

        // Timestamp: 0x0001_8F3B_4C5D_6E (example 48-bit value)
        bytes[0] = 0x00;
        bytes[1] = 0x01;
        bytes[2] = 0x8F;
        bytes[3] = 0x3B;
        bytes[4] = 0x4C;
        bytes[5] = 0x5D;

        let uuid = UuidV7Bytes::new(bytes).unwrap();
        let expected = 0x0001_8F3B_4C5Du64;
        assert_eq!(uuid.timestamp_ms(), expected);
    }
}
