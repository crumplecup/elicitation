//! Marginal cost benchmarks for Kani verification.
//!
//! These harnesses measure verification time as buffer size increases,
//! allowing us to fit a growth curve (linear, quadratic, exponential).

use crate::verification::types::utf8::is_valid_utf8;

// ============================================================================
// UTF-8 Validation Marginal Cost
// ============================================================================

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(3)]
fn bench_utf8_2byte() {
    const SIZE: usize = 2;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(5)]
fn bench_utf8_4byte() {
    const SIZE: usize = 4;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(9)]
fn bench_utf8_8byte() {
    const SIZE: usize = 8;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(17)]
fn bench_utf8_16byte() {
    const SIZE: usize = 16;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(33)]
fn bench_utf8_32byte() {
    const SIZE: usize = 32;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(65)]
fn bench_utf8_64byte() {
    const SIZE: usize = 64;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(129)]
fn bench_utf8_128byte() {
    const SIZE: usize = 128;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(257)]
fn bench_utf8_256byte() {
    const SIZE: usize = 256;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(513)]
fn bench_utf8_512byte() {
    const SIZE: usize = 512;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(1025)]
fn bench_utf8_1024byte() {
    const SIZE: usize = 1024;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(2049)]
fn bench_utf8_2048byte() {
    const SIZE: usize = 2048;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(4097)]
fn bench_utf8_4096byte() {
    const SIZE: usize = 4096;
    let bytes: [u8; SIZE] = kani::any();
    let _ = is_valid_utf8(&bytes);
}

// ============================================================================
// UUID Format Marginal Cost
// ============================================================================

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(1)]  // No loops, deterministic bit checks
fn bench_uuid_variant_2byte() {
    let bytes: [u8; 2] = kani::any();
    // Check variant bits (simplified)
    let variant_byte = bytes[0];
    let _ = (variant_byte & 0xC0) == 0x80;
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(1)]  // No loops, deterministic bit checks
fn bench_uuid_variant_4byte() {
    let bytes: [u8; 4] = kani::any();
    let variant_byte = bytes[0];
    let version_byte = bytes[1];
    let _ = (variant_byte & 0xC0) == 0x80 && (version_byte & 0xF0) == 0x40;
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(1)]  // No loops, deterministic bit checks
fn bench_uuid_full_16byte() {
    let bytes: [u8; 16] = kani::any();
    // Full UUID validation (variant + version)
    let variant_byte = bytes[8];
    let version_byte = bytes[6];
    let _ = (variant_byte & 0xC0) == 0x80 && (version_byte & 0xF0) == 0x40;
}

// ============================================================================
// MAC Address Format Marginal Cost  
// ============================================================================

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(1)]  // No loops, deterministic bit checks
fn bench_mac_multicast_1byte() {
    let byte: u8 = kani::any();
    let _ = (byte & 0x01) != 0;
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(1)]  // No loops, deterministic bit checks
fn bench_mac_local_1byte() {
    let byte: u8 = kani::any();
    let _ = (byte & 0x02) != 0;
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(1)]  // No loops, deterministic bit checks
fn bench_mac_full_6byte() {
    let bytes: [u8; 6] = kani::any();
    let is_multicast = (bytes[0] & 0x01) != 0;
    let is_local = (bytes[0] & 0x02) != 0;
    let _ = is_multicast || is_local;
}
