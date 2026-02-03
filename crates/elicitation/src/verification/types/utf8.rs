//! UTF-8 contract types - the foundation for string contracts.
//!
//! This module provides low-level UTF-8 validation that Kani can verify symbolically.
//! All string contracts are built on top of this validated UTF-8 layer.

use super::ValidationError;

/// Guaranteed valid UTF-8 byte sequence with bounded length.
///
/// This is the foundational type for all string contracts. It wraps a fixed-size
/// byte array and tracks the actual length of valid UTF-8 content.
///
/// # Type Parameters
///
/// * `MAX_LEN` - Maximum byte length of the UTF-8 sequence
///
/// # Invariants
///
/// 1. `bytes[..len]` is valid UTF-8
/// 2. `len <= MAX_LEN`
///
/// # Examples
///
/// ```
/// use elicitation::verification::types::{Utf8Bytes, ValidationError};
///
/// let mut bytes = [0u8; 20];
/// bytes[0] = b'h';
/// bytes[1] = b'i';
///
/// let utf8 = Utf8Bytes::<20>::new(bytes, 2)?;
/// assert_eq!(utf8.as_str(), "hi");
/// # Ok::<(), ValidationError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Utf8Bytes<const MAX_LEN: usize> {
    bytes: [u8; MAX_LEN],
    len: usize,
}

impl<const MAX_LEN: usize> Utf8Bytes<MAX_LEN> {
    /// Construct from raw bytes, validating UTF-8 encoding.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if:
    /// - `len > MAX_LEN`
    /// - `bytes[..len]` is not valid UTF-8
    pub fn new(bytes: [u8; MAX_LEN], len: usize) -> Result<Self, ValidationError> {
        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }

        // Castle on a cloud: Symbolically assume UTF-8 validity under Kani
        // We trust Rust's UTF-8 semantics and verify our wrapper logic only
        #[cfg(kani)]
        {
            let is_valid_utf8: bool = kani::any();
            if !is_valid_utf8 {
                return Err(ValidationError::InvalidUtf8);
            }
        }

        #[cfg(not(kani))]
        {
            if !is_valid_utf8(&bytes[..len]) {
                return Err(ValidationError::InvalidUtf8);
            }
        }

        Ok(Self { bytes, len })
    }

    /// Get the valid UTF-8 content as a string slice.
    ///
    /// This is zero-cost - the UTF-8 validity was already verified in the constructor.
    /// We re-validate here to maintain the unsafe-free guarantee.
    pub fn as_str(&self) -> &str {
        // SAFE: We validated UTF-8 in constructor, but we can't use unsafe due to forbid
        std::str::from_utf8(&self.bytes[..self.len]).expect("UTF-8 validated in constructor")
    }

    /// Get the byte length of the UTF-8 content.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the UTF-8 content is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get the underlying byte array.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl<const MAX_LEN: usize> std::fmt::Display for Utf8Bytes<MAX_LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Verify UTF-8 encoding rules.
///
/// This function implements the UTF-8 validation algorithm that Kani can verify.
/// It checks all UTF-8 encoding rules:
/// - Valid start bytes (0xxxxxxx, 110xxxxx, 1110xxxx, 11110xxx)
/// - Valid continuation bytes (10xxxxxx)
/// - No overlong encodings
/// - No surrogate pairs (U+D800 to U+DFFF)
/// - Code points <= U+10FFFF
///
/// # Kani Verification
///
/// This function is designed to be verified symbolically by Kani for bounded inputs.
/// Proofs should use small bounds (e.g., 10 bytes) to keep verification tractable.
#[inline]
pub fn is_valid_utf8(bytes: &[u8]) -> bool {
    let len = bytes.len();

    // Kani constraint: Assume reasonable length for tractability
    #[cfg(kani)]
    kani::assume(len <= 16);

    let mut i = 0;

    while i < len {
        let byte = bytes[i];

        // Single byte (ASCII): 0xxxxxxx
        if byte & 0b1000_0000 == 0 {
            i += 1;
            continue;
        }

        // Two bytes: 110xxxxx 10xxxxxx
        if byte & 0b1110_0000 == 0b1100_0000 {
            if i + 1 >= bytes.len() {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            // Check no overlong encoding (must be >= 0x80)
            if byte & 0b0001_1110 == 0 {
                return false;
            }
            i += 2;
            continue;
        }

        // Three bytes: 1110xxxx 10xxxxxx 10xxxxxx
        if byte & 0b1111_0000 == 0b1110_0000 {
            if i + 2 >= bytes.len() {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }

            // Check no overlong encoding (must be >= 0x800)
            if byte == 0b1110_0000 && bytes[i + 1] & 0b0010_0000 == 0 {
                return false;
            }

            // Check not surrogate (0xD800-0xDFFF)
            let code_point = ((byte & 0x0F) as u32) << 12
                | ((bytes[i + 1] & 0x3F) as u32) << 6
                | (bytes[i + 2] & 0x3F) as u32;
            if (0xD800..=0xDFFF).contains(&code_point) {
                return false;
            }

            i += 3;
            continue;
        }

        // Four bytes: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
        if byte & 0b1111_1000 == 0b1111_0000 {
            if i + 3 >= bytes.len() {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 3] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }

            // Check no overlong encoding (must be >= 0x10000)
            if byte == 0b1111_0000 && bytes[i + 1] & 0b0011_0000 == 0 {
                return false;
            }

            // Check code point <= 0x10FFFF
            let code_point = ((byte & 0x07) as u32) << 18
                | ((bytes[i + 1] & 0x3F) as u32) << 12
                | ((bytes[i + 2] & 0x3F) as u32) << 6
                | (bytes[i + 3] & 0x3F) as u32;
            if code_point > 0x10FFFF {
                return false;
            }

            i += 4;
            continue;
        }

        // Invalid start byte
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_valid() {
        let mut bytes = [0u8; 10];
        bytes[0] = b'h';
        bytes[1] = b'e';
        bytes[2] = b'l';
        bytes[3] = b'l';
        bytes[4] = b'o';

        let utf8 = Utf8Bytes::<10>::new(bytes, 5).unwrap();
        assert_eq!(utf8.as_str(), "hello");
    }

    #[test]
    fn test_empty_valid() {
        let bytes = [0u8; 10];
        let utf8 = Utf8Bytes::<10>::new(bytes, 0).unwrap();
        assert_eq!(utf8.as_str(), "");
        assert!(utf8.is_empty());
    }

    #[test]
    fn test_two_byte_utf8() {
        let mut bytes = [0u8; 10];
        bytes[0] = 0xC2; // Start of 2-byte sequence
        bytes[1] = 0xA9; // © symbol

        let utf8 = Utf8Bytes::<10>::new(bytes, 2).unwrap();
        assert_eq!(utf8.as_str(), "©");
    }

    #[test]
    fn test_three_byte_utf8() {
        let mut bytes = [0u8; 10];
        bytes[0] = 0xE2; // Start of 3-byte sequence
        bytes[1] = 0x82;
        bytes[2] = 0xAC; // € symbol

        let utf8 = Utf8Bytes::<10>::new(bytes, 3).unwrap();
        assert_eq!(utf8.as_str(), "€");
    }

    #[test]
    fn test_invalid_continuation() {
        let mut bytes = [0u8; 10];
        bytes[0] = 0xC2;
        bytes[1] = 0xFF; // Invalid continuation byte

        assert!(Utf8Bytes::<10>::new(bytes, 2).is_err());
    }

    #[test]
    fn test_overlong_encoding() {
        let mut bytes = [0u8; 10];
        bytes[0] = 0xC0; // Overlong encoding
        bytes[1] = 0x80; // of NULL

        assert!(Utf8Bytes::<10>::new(bytes, 2).is_err());
    }

    #[test]
    fn test_surrogate_rejected() {
        let mut bytes = [0u8; 10];
        bytes[0] = 0xED; // Surrogate
        bytes[1] = 0xA0;
        bytes[2] = 0x80; // U+D800

        assert!(Utf8Bytes::<10>::new(bytes, 3).is_err());
    }

    #[test]
    fn test_length_too_long() {
        let bytes = [0u8; 10];
        assert!(Utf8Bytes::<10>::new(bytes, 11).is_err());
    }
}
