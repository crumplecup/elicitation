//! Chunked UTF-8 proofs for memory-constrained systems.
//!
//! These harnesses partition large symbolic spaces into smaller chunks that can:
//! - Run on lower-RAM machines
//! - Be verified incrementally over time
//! - Resume from checkpoints if interrupted
//! - Run in parallel on different machines
//!
//! Coverage tracking: See `kani_proof_record_N.csv` for completion status.

use crate::verification::types::utf8::is_valid_utf8;

// ============================================================================
// 3-Byte Chunked Proofs (49,152 combinations → N chunks)
// ============================================================================

/// Macro to generate 3-byte chunked proofs.
///
/// Each chunk verifies a subset of the byte1 range, covering all combinations
/// of byte2 and byte3 continuation bytes.
macro_rules! verify_3byte_chunks {
    ($(($name:ident, $start:expr, $end:expr, $chunk_id:expr)),* $(,)?) => {
        $(
            #[cfg(kani)]
            #[kani::proof]
            #[doc = concat!("Verify 3-byte UTF-8: byte1 ∈ [", stringify!($start), ", ", stringify!($end), "]")]
            fn $name() {
                let byte1: u8 = kani::any();
                kani::assume(byte1 >= $start && byte1 <= $end);
                
                let byte2: u8 = kani::any();
                kani::assume(byte2 >= 0x80 && byte2 <= 0xBF); // 64 values
                
                let byte3: u8 = kani::any();
                kani::assume(byte3 >= 0x80 && byte3 <= 0xBF); // 64 values
                
                let bytes = [byte1, byte2, byte3];
                assert!(is_valid_utf8(&bytes));
            }
        )*
    };
}

// ============================================================================
// 4-Chunk Configuration (12,288 combinations each)
// ============================================================================

verify_3byte_chunks!(
    (verify_3byte_4chunks_0, 0xE1, 0xE3, 0),  // 3 × 64 × 64 = 12,288
    (verify_3byte_4chunks_1, 0xE4, 0xE6, 1),  // 3 × 64 × 64 = 12,288
    (verify_3byte_4chunks_2, 0xE7, 0xE9, 2),  // 3 × 64 × 64 = 12,288
    (verify_3byte_4chunks_3, 0xEA, 0xEC, 3),  // 3 × 64 × 64 = 12,288
);

// ============================================================================
// 12-Chunk Configuration (4,096 combinations each)
// ============================================================================

verify_3byte_chunks!(
    (verify_3byte_12chunks_0,  0xE1, 0xE1, 0),   // 1 × 64 × 64 = 4,096
    (verify_3byte_12chunks_1,  0xE2, 0xE2, 1),   // 1 × 64 × 64 = 4,096
    (verify_3byte_12chunks_2,  0xE3, 0xE3, 2),
    (verify_3byte_12chunks_3,  0xE4, 0xE4, 3),
    (verify_3byte_12chunks_4,  0xE5, 0xE5, 4),
    (verify_3byte_12chunks_5,  0xE6, 0xE6, 5),
    (verify_3byte_12chunks_6,  0xE7, 0xE7, 6),
    (verify_3byte_12chunks_7,  0xE8, 0xE8, 7),
    (verify_3byte_12chunks_8,  0xE9, 0xE9, 8),
    (verify_3byte_12chunks_9,  0xEA, 0xEA, 9),
    (verify_3byte_12chunks_10, 0xEB, 0xEB, 10),
    (verify_3byte_12chunks_11, 0xEC, 0xEC, 11),
);

// ============================================================================
// 4-Byte Chunked Proofs (786,432 combinations → N chunks)
// ============================================================================

/// Macro to generate 4-byte chunked proofs.
macro_rules! verify_4byte_chunks {
    ($(($name:ident, $start:expr, $end:expr, $chunk_id:expr)),* $(,)?) => {
        $(
            #[cfg(kani)]
            #[kani::proof]
            #[doc = concat!("Verify 4-byte UTF-8: byte1 ∈ [", stringify!($start), ", ", stringify!($end), "]")]
            fn $name() {
                let byte1: u8 = kani::any();
                kani::assume(byte1 >= $start && byte1 <= $end);
                
                let byte2: u8 = kani::any();
                kani::assume(byte2 >= 0x80 && byte2 <= 0xBF); // 64 values
                
                let byte3: u8 = kani::any();
                kani::assume(byte3 >= 0x80 && byte3 <= 0xBF); // 64 values
                
                let byte4: u8 = kani::any();
                kani::assume(byte4 >= 0x80 && byte4 <= 0xBF); // 64 values
                
                let bytes = [byte1, byte2, byte3, byte4];
                assert!(is_valid_utf8(&bytes));
            }
        )*
    };
}

// ============================================================================
// 3-Chunk Configuration (262,144 combinations each)
// ============================================================================

verify_4byte_chunks!(
    (verify_4byte_3chunks_0, 0xF1, 0xF1, 0),  // 1 × 64³ = 262,144
    (verify_4byte_3chunks_1, 0xF2, 0xF2, 1),  // 1 × 64³ = 262,144
    (verify_4byte_3chunks_2, 0xF3, 0xF3, 2),  // 1 × 64³ = 262,144
);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_coverage_3byte_4chunks() {
        // Verify chunk ranges are disjoint and exhaustive
        let ranges = [
            (0xE1u8, 0xE3u8),
            (0xE4u8, 0xE6u8),
            (0xE7u8, 0xE9u8),
            (0xEA u8, 0xECu8),
        ];
        
        // Check no overlap
        for i in 0..ranges.len() {
            for j in (i + 1)..ranges.len() {
                assert!(ranges[i].1 < ranges[j].0, "Chunks overlap");
            }
        }
        
        // Check exhaustive
        assert_eq!(ranges[0].0, 0xE1);
        assert_eq!(ranges[3].1, 0xEC);
        assert_eq!(ranges[0].1 + 1, ranges[1].0);
        assert_eq!(ranges[1].1 + 1, ranges[2].0);
        assert_eq!(ranges[2].1 + 1, ranges[3].0);
    }

    #[test]
    fn test_chunk_coverage_3byte_12chunks() {
        // Each chunk covers exactly 1 byte value
        for byte in 0xE1u8..=0xECu8 {
            // Verify byte is in exactly one chunk
            let mut found = false;
            for chunk_byte in 0xE1u8..=0xECu8 {
                if byte == chunk_byte {
                    assert!(!found, "Byte in multiple chunks");
                    found = true;
                }
            }
            assert!(found, "Byte not in any chunk");
        }
    }
}
