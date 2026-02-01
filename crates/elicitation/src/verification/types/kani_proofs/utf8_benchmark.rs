//! Kani verification benchmarks for UTF-8 validation
//!
//! These harnesses use TINY symbolic problem spaces (2-4 combinations)
//! so Criterion can run them hundreds of times for statistical analysis.

#[cfg(kani)]
mod kani_benchmarks {
    use crate::verification::types::is_valid_utf8;

    // Baseline: Concrete single byte (0 combinations - deterministic)
    #[kani::proof]
    #[kani::unwind(1)] // 1-byte array: ceil(1/1) = 1 iteration max
    fn bench_concrete_1_byte() {
        let bytes = [b'a'];
        assert!(is_valid_utf8(&bytes));
    }

    // MICRO: 2-byte UTF-8 with 2 × 2 = 4 combinations
    #[kani::proof]
    #[kani::unwind(2)] // 2-byte array: ceil(2/1) = 2 iterations max
    fn bench_2byte_2x2() {
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xC2 && byte1 <= 0xC3); // 2 values

        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0x80 && byte2 <= 0x81); // 2 values

        let bytes = [byte1, byte2];
        assert!(is_valid_utf8(&bytes));
    }

    // MICRO: 2-byte UTF-8 with 2 × 3 = 6 combinations
    #[kani::proof]
    #[kani::unwind(2)] // 2-byte array: ceil(2/1) = 2 iterations max
    fn bench_2byte_2x3() {
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xC2 && byte1 <= 0xC3); // 2 values

        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0x80 && byte2 <= 0x82); // 3 values

        let bytes = [byte1, byte2];
        assert!(is_valid_utf8(&bytes));
    }

    // MICRO: 2-byte UTF-8 with 3 × 3 = 9 combinations
    #[kani::proof]
    #[kani::unwind(2)] // 2-byte array: ceil(2/1) = 2 iterations max
    fn bench_2byte_3x3() {
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xC2 && byte1 <= 0xC4); // 3 values

        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0x80 && byte2 <= 0x82); // 3 values

        let bytes = [byte1, byte2];
        assert!(is_valid_utf8(&bytes));
    }

    // MICRO: 2-byte UTF-8 with 4 × 4 = 16 combinations
    #[kani::proof]
    #[kani::unwind(2)] // 2-byte array: ceil(2/1) = 2 iterations max
    fn bench_2byte_4x4() {
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xC2 && byte1 <= 0xC5); // 4 values

        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0x80 && byte2 <= 0x83); // 4 values

        let bytes = [byte1, byte2];
        assert!(is_valid_utf8(&bytes));
    }

    // 4-byte UTF-8 with 2 × 2 × 2 × 2 = 16 combinations
    #[kani::proof]
    #[kani::unwind(4)] // 4-byte array: ceil(4/1) = 4 iterations max
    fn bench_4byte_2x2x2x2() {
        let byte1: u8 = kani::any();
        kani::assume(byte1 >= 0xF1 && byte1 <= 0xF2); // 2 values

        let byte2: u8 = kani::any();
        kani::assume(byte2 >= 0x80 && byte2 <= 0x81); // 2 values

        let byte3: u8 = kani::any();
        kani::assume(byte3 >= 0x80 && byte3 <= 0x81); // 2 values

        let byte4: u8 = kani::any();
        kani::assume(byte4 >= 0x80 && byte4 <= 0x81); // 2 values

        let bytes = [byte1, byte2, byte3, byte4];
        assert!(is_valid_utf8(&bytes));
    }
}
