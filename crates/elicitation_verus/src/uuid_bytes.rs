//! Verus proofs for UUID byte validation types.
//!
//! Validates UUIDs (RFC 4122) with version and variant checking.
//! Simplified stubs for compositional verification.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InvalidBytes,
    InvalidVersion,
    InvalidVariant,
}

// ============================================================================
// UuidBytes - 16-byte UUID (RFC 4122)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UuidBytes {
    pub bytes: [u8; 16],
    pub validated: bool,
}

impl UuidBytes {
    /// All byte combinations are structurally valid UUIDs.
    pub fn new(bytes: [u8; 16]) -> (result: Self)
        ensures
            result.bytes == bytes,
            result.validated == true,
    {
        UuidBytes { bytes, validated: true }
    }

    pub fn version(&self) -> (result: u8)
        requires self.validated == true,
    {
        // Version is in upper 4 bits of byte 6
        (self.bytes[6] >> 4) & 0x0F
    }

    pub fn variant(&self) -> (result: u8)
        requires self.validated == true,
    {
        // Variant is in upper 2-3 bits of byte 8
        (self.bytes[8] >> 6) & 0x03
    }
}

// ============================================================================
// UuidV4 - UUID version 4 (random)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UuidV4 {
    pub bytes: [u8; 16],
    pub validated: bool,
}

impl UuidV4 {
    /// Parameters:
    /// - is_v4: Version field == 4
    pub fn new(bytes: [u8; 16], is_v4: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_v4 ==> (result matches Ok(uuid) && uuid.bytes == bytes && uuid.validated == true),
            !is_v4 ==> (result matches Err(ValidationError::InvalidVersion)),
    {
        if is_v4 {
            Ok(UuidV4 { bytes, validated: true })
        } else {
            Err(ValidationError::InvalidVersion)
        }
    }
}

// ============================================================================
// UuidNonNil - Non-nil UUID (not all zeros)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UuidNonNil {
    pub bytes: [u8; 16],
    pub validated: bool,
}

impl UuidNonNil {
    /// Parameters:
    /// - is_non_nil: At least one byte is non-zero
    pub fn new(bytes: [u8; 16], is_non_nil: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_non_nil ==> (result matches Ok(uuid) && uuid.bytes == bytes && uuid.validated == true),
            !is_non_nil ==> (result matches Err(ValidationError::InvalidBytes)),
    {
        if is_non_nil {
            Ok(UuidNonNil { bytes, validated: true })
        } else {
            Err(ValidationError::InvalidBytes)
        }
    }
}

} // verus!
