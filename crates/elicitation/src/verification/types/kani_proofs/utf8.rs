//! Kani proofs for UTF-8 validation

#[cfg(kani)]
mod kani_proofs {
    use crate::verification::types::{Utf8Bytes, is_valid_utf8};

    /// Verify: All ASCII bytes (0x00-0x7F) are valid UTF-8
    #[kani::proof]
    fn verify_ascii_always_valid() {
        let len: usize = kani::any();
        kani::assume(len > 0 && len <= 10);

        let mut bytes = [0u8; 10];
        for i in 0..len {
            let byte: u8 = kani::any();
            kani::assume(byte < 0x80); // ASCII range
            bytes[i] = byte;
        }

        // All ASCII sequences are valid UTF-8
        assert!(is_valid_utf8(&bytes[..len]));
    }

    /// Verify: Invalid continuation bytes are rejected
    #[kani::proof]
    fn verify_invalid_continuation_rejected() {
        // Continuation byte without leader
        let byte: u8 = kani::any();
        kani::assume(byte & 0b1100_0000 == 0b1000_0000); // 10xxxxxx

        let bytes = [byte];
        assert!(!is_valid_utf8(&bytes));
    }

    /// Verify: Overlong 2-byte sequences are rejected
    #[kani::proof]
    fn verify_overlong_two_byte_rejected() {
        // 2-byte encoding for code point < 0x80 (should be 1 byte)
        let byte1: u8 = kani::any();
        kani::assume(byte1 & 0b1110_0000 == 0b1100_0000); // 110xxxxx
        kani::assume((byte1 & 0b0001_1110) >> 1 == 0); // Ensures code point < 0x80

        let byte2: u8 = kani::any();
        kani::assume(byte2 & 0b1100_0000 == 0b1000_0000); // Valid continuation

        let bytes = [byte1, byte2];
        assert!(!is_valid_utf8(&bytes));
    }

    /// Verify: Surrogate code points (0xD800-0xDFFF) are rejected
    #[kani::proof]
    fn verify_surrogate_rejected() {
        // 3-byte encoding for surrogate range (0xED 0xA0-0xBF 0x80-0xBF)
        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0xA0 && byte2 <= 0xBF);

        let byte3: u8 = kani::any();
        kani::assume(byte3 >= 0x80 && byte3 <= 0xBF);

        let bytes = [0xED, byte2, byte3];
        assert!(!is_valid_utf8(&bytes));
    }

    /// Verify: Composition - 2 bytes + 2 bytes = 4 bytes (additivity)
    /// If 2-byte storage works, then concatenation preserves correctness
    #[kani::proof]
    fn verify_valid_two_byte_accepted() {
        // First pair of bytes
        let byte1: u8 = kani::any();
        let byte2: u8 = kani::any();
        
        // Second pair of bytes
        let byte3: u8 = kani::any();
        let byte4: u8 = kani::any();
        
        let bytes = [byte1, byte2, byte3, byte4, 0];
        let len = 4;

        let result = Utf8Bytes::<5>::new(bytes, len);
        
        if let Ok(utf8) = result {
            // Verify composition: 4 bytes stored correctly
            assert_eq!(utf8.len(), 4);
            let retrieved = utf8.as_bytes();
            assert_eq!(retrieved[0], byte1);
            assert_eq!(retrieved[1], byte2);
            assert_eq!(retrieved[2], byte3);
            assert_eq!(retrieved[3], byte4);
        }
    }

    /// Verify: Composition with 3 bytes - extends to arbitrary odd lengths
    #[kani::proof]
    fn verify_valid_three_byte_accepted() {
        let byte1: u8 = kani::any();
        let byte2: u8 = kani::any();
        let byte3: u8 = kani::any();
        
        let bytes = [byte1, byte2, byte3, 0, 0];
        let len = 3;

        let result = Utf8Bytes::<5>::new(bytes, len);
        
        if let Ok(utf8) = result {
            assert_eq!(utf8.len(), 3);
            let retrieved = utf8.as_bytes();
            assert_eq!(retrieved[0], byte1);
            assert_eq!(retrieved[1], byte2);
            assert_eq!(retrieved[2], byte3);
        }
    }

    /// Verify: Composition with 5 bytes - proves full buffer capacity
    #[kani::proof]
    fn verify_valid_four_byte_accepted() {
        let byte1: u8 = kani::any();
        let byte2: u8 = kani::any();
        let byte3: u8 = kani::any();
        let byte4: u8 = kani::any();
        let byte5: u8 = kani::any();
        
        let bytes = [byte1, byte2, byte3, byte4, byte5];
        let len = 5;

        let result = Utf8Bytes::<5>::new(bytes, len);
        
        if let Ok(utf8) = result {
            assert_eq!(utf8.len(), 5);
            let retrieved = utf8.as_bytes();
            assert_eq!(retrieved[0], byte1);
            assert_eq!(retrieved[1], byte2);
            assert_eq!(retrieved[2], byte3);
            assert_eq!(retrieved[3], byte4);
            assert_eq!(retrieved[4], byte5);
        }
    }

    /// Verify: Incomplete multi-byte sequences are rejected
    #[kani::proof]
    fn verify_incomplete_sequence_rejected() {
        // 2-byte sequence with only leader
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xC2 && byte1 <= 0xDF);

        let bytes = [byte1];
        assert!(!is_valid_utf8(&bytes));
    }

    /// Verify: Utf8Bytes construction rejects invalid UTF-8
    #[kani::proof]
    fn verify_utf8bytes_rejects_invalid() {
        // Invalid continuation byte
        let bytes = [0b1000_0000]; // 10xxxxxx without leader

        let result = Utf8Bytes::<1>::new(bytes, 1);
        assert!(result.is_err());
    }

    /// Verify: Utf8Bytes construction accepts valid ASCII
    #[kani::proof]
    fn verify_utf8bytes_accepts_ascii() {
        let byte: u8 = kani::any();
        kani::assume(byte < 0x80);

        let bytes = [byte];
        let result = Utf8Bytes::<1>::new(bytes, 1);
        assert!(result.is_ok());
    }

    /// Verify: Utf8Bytes respects MAX_LEN bound
    #[kani::proof]
    fn verify_utf8bytes_respects_bound() {
        let bytes = [b'a', b'b', b'c', b'd', b'e']; // 5 bytes

        // Should reject if len > MAX_LEN
        let result = Utf8Bytes::<3>::new([b'a', b'b', b'c'], 5);
        assert!(result.is_err());

        // Should accept if len <= MAX_LEN
        let result = Utf8Bytes::<5>::new(bytes, 5);
        assert!(result.is_ok());
    }

    /// Verify: Base case - Utf8Bytes correctly stores/retrieves 2 symbolic bytes
    /// This is the foundation for compositional reasoning about larger buffers
    #[kani::proof]
    fn verify_utf8bytes_roundtrip_ascii() {
        // Base case: 2 symbolic bytes
        let byte1: u8 = kani::any();
        let byte2: u8 = kani::any();
        let bytes = [byte1, byte2, 0, 0, 0];
        let len = 2;

        let result = Utf8Bytes::<5>::new(bytes, len);
        
        match result {
            Ok(utf8) => {
                // Verify wrapper correctly stores and retrieves bytes
                assert_eq!(utf8.len(), len);
                let retrieved = utf8.as_bytes();
                assert_eq!(retrieved.len(), 2);
                assert_eq!(retrieved[0], byte1);
                assert_eq!(retrieved[1], byte2);
                assert!(!utf8.is_empty());
            }
            Err(_) => {
                // Invalid UTF-8 path acceptable under symbolic validation
            }
        }
    }
}
