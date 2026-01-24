//! Kani proofs for UTF-8 validation

#[cfg(kani)]
mod kani_proofs {
    use crate::verification::types::{Utf8Bytes, is_valid_utf8};

    /// Verify: All ASCII bytes (0x00-0x7F) are valid UTF-8
    #[kani::proof]
    #[kani::unwind(15)] // 10 byte array + nested UTF-8 validator loop
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

    /// Verify: Valid 2-byte sequences are accepted
    #[kani::proof]
    fn verify_valid_two_byte_accepted() {
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xC2 && byte1 <= 0xDF); // Valid 2-byte leader (not overlong)

        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0x80 && byte2 <= 0xBF); // Valid continuation

        let bytes = [byte1, byte2];
        assert!(is_valid_utf8(&bytes));
    }

    /// Verify: Valid 3-byte sequences (non-surrogate) are accepted
    ///
    /// **Expensive:** This proof explores ~49K symbolic combinations (12 × 64 × 64).
    /// Expected runtime: Hours to days depending on hardware.
    #[kani::proof]
    fn verify_valid_three_byte_accepted() {
        // Test 0xE1-0xEC range (avoids 0xE0 overlong and 0xED surrogate checks)
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xE1 && byte1 <= 0xEC); // 12 values
        
        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0x80 && byte2 <= 0xBF); // 64 values
        
        let byte3: u8 = kani::any();
        kani::assume(byte3 >= 0x80 && byte3 <= 0xBF); // 64 values
        
        let bytes = [byte1, byte2, byte3];
        assert!(is_valid_utf8(&bytes));
    }

    /// Verify: Valid 4-byte sequences (code point <= 0x10FFFF) are accepted
    ///
    /// **Very Expensive:** This proof explores ~786K symbolic combinations (3 × 64³).
    /// Expected runtime: Days to weeks depending on hardware.
    #[kani::proof]
    fn verify_valid_four_byte_accepted() {
        // Test 0xF1-0xF3 range (avoids 0xF0 overlong and 0xF4 overflow)
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xF1 && byte1 <= 0xF3); // 3 values
        
        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0x80 && byte2 <= 0xBF); // 64 values
        
        let byte3: u8 = kani::any();
        kani::assume(byte3 >= 0x80 && byte3 <= 0xBF); // 64 values
        
        let byte4: u8 = kani::any();
        kani::assume(byte4 >= 0x80 && byte4 <= 0xBF); // 64 values
        
        let bytes = [byte1, byte2, byte3, byte4];
        assert!(is_valid_utf8(&bytes));
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

    /// Verify: Round-trip ASCII through Utf8Bytes preserves content
    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_utf8bytes_roundtrip_ascii() {
        let len: usize = kani::any();
        kani::assume(len > 0 && len <= 5);

        let mut bytes = [0u8; 5];
        for i in 0..len {
            let byte: u8 = kani::any();
            kani::assume(byte >= 0x20 && byte <= 0x7E); // Printable ASCII
            bytes[i] = byte;
        }

        let utf8 = Utf8Bytes::<5>::new(bytes, len).unwrap();
        let recovered = utf8.as_str();
        assert_eq!(recovered.len(), len);
        assert_eq!(recovered.as_bytes(), &bytes[..len]);
    }
}
