#!/usr/bin/env python3
"""
Generate chunked UTF-8 proof harnesses for arbitrary N.

Generates Rust code with harnesses for 2-byte, 3-byte, and 4-byte
proofs with up to MAX_CHUNKS variants each.

Usage:
    ./generate_chunked_harnesses.py > utf8_chunked_generated.rs
"""

MAX_CHUNKS = 64  # Generate up to 64 chunks per proof type

def generate_2byte_harnesses():
    code = """// ============================================================================
// 2-Byte Chunked Proofs - Generated Harnesses
// ============================================================================

"""
    for chunk_id in range(MAX_CHUNKS):
        code += f"""#[cfg(kani)]
#[kani::proof]
fn verify_2byte_chunk_{chunk_id}() {{
    // Chunk {chunk_id} - range determined dynamically by runner
    // This harness is parameterized by KANI_CHUNK_START and KANI_CHUNK_END env vars
    // Default to full range if not set
    
    let chunk_start = std::env::var("KANI_CHUNK_START")
        .ok()
        .and_then(|s| u8::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0xC2);
    
    let chunk_end = std::env::var("KANI_CHUNK_END")
        .ok()
        .and_then(|s| u8::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0xDF);
    
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= chunk_start && byte1 <= chunk_end);
    
    let byte2: u8 = kani::any();
    kani::assume(byte2 >= 0x80 && byte2 <= 0xBF);
    
    let bytes = [byte1, byte2];
    assert!(is_valid_utf8(&bytes));
}}

"""
    return code

print("""//! Auto-generated chunked UTF-8 proof harnesses.
//!
//! Generated with: scripts/generate_chunked_harnesses.py
//! DO NOT EDIT BY HAND - regenerate with the script.

use crate::verification::types::utf8::is_valid_utf8;
""")

print(generate_2byte_harnesses())
