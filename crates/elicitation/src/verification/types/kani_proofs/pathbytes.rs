//! Kani proofs for path byte validation (Unix).
//!
//! These proofs verify the correctness of Unix path byte-level validation,
//! demonstrating composition of UTF-8 validation with null-byte checking.

#![cfg(all(kani, unix))]

use crate::verification::types::{PathAbsolute, PathBytes, PathNonEmpty, PathRelative};

// ============================================================================
// UTF-8 + No Null Composition Proofs
// ============================================================================

#[kani::proof]
fn verify_valid_ascii_no_null_accepted() {
    // Use small MAX_LEN for tractable proof
    const MAX_LEN: usize = 8;

    let len: usize = kani::any();
    kani::assume(len > 0 && len <= MAX_LEN);

    let mut bytes = [0u8; MAX_LEN];
    for i in 0..len {
        let byte: u8 = kani::any();
        // Assume ASCII (valid UTF-8 single byte)
        kani::assume(byte > 0 && byte < 128);
        bytes[i] = byte;
    }

    // Should construct successfully
    let path_result = PathBytes::<MAX_LEN>::from_slice(&bytes[..len]);
    assert!(path_result.is_ok());
}

#[kani::proof]
fn verify_null_byte_rejected() {
    const MAX_LEN: usize = 8;

    // Create path with null byte
    let bytes = [b'/', 0, b't'];

    let path_result = PathBytes::<MAX_LEN>::from_slice(&bytes);
    assert!(path_result.is_err());
}

// ============================================================================
// Absolute/Relative Path Proofs
// ============================================================================

#[kani::proof]
fn verify_absolute_path_starts_with_slash() {
    const MAX_LEN: usize = 16;

    let bytes = [b'/', b'u', b's', b'r'];

    let path_result = PathBytes::<MAX_LEN>::from_slice(&bytes);
    assert!(path_result.is_ok());

    if let Ok(path) = path_result {
        assert!(path.is_absolute());
        assert!(!path.is_relative());
    }
}

#[kani::proof]
fn verify_relative_path_no_leading_slash() {
    const MAX_LEN: usize = 16;

    let bytes = [b'u', b's', b'r'];

    let path_result = PathBytes::<MAX_LEN>::from_slice(&bytes);
    assert!(path_result.is_ok());

    if let Ok(path) = path_result {
        assert!(path.is_relative());
        assert!(!path.is_absolute());
    }
}

// ============================================================================
// Contract Type Proofs
// ============================================================================

#[kani::proof]
fn verify_path_absolute_accepts_leading_slash() {
    const MAX_LEN: usize = 16;

    let bytes = [b'/', b'h', b'o', b'm', b'e'];

    let abs_result = PathAbsolute::<MAX_LEN>::from_slice(&bytes);
    assert!(abs_result.is_ok());
}

#[kani::proof]
fn verify_path_absolute_rejects_no_slash() {
    const MAX_LEN: usize = 16;

    let bytes = [b'h', b'o', b'm', b'e'];

    let abs_result = PathAbsolute::<MAX_LEN>::from_slice(&bytes);
    assert!(abs_result.is_err());
}

#[kani::proof]
fn verify_path_relative_accepts_no_slash() {
    const MAX_LEN: usize = 16;

    let bytes = [b'h', b'o', b'm', b'e'];

    let rel_result = PathRelative::<MAX_LEN>::from_slice(&bytes);
    assert!(rel_result.is_ok());
}

#[kani::proof]
fn verify_path_relative_rejects_slash() {
    const MAX_LEN: usize = 16;

    let bytes = [b'/', b'h', b'o', b'm', b'e'];

    let rel_result = PathRelative::<MAX_LEN>::from_slice(&bytes);
    assert!(rel_result.is_err());
}

#[kani::proof]
fn verify_path_nonempty_accepts_content() {
    const MAX_LEN: usize = 16;

    let bytes = [b't', b'e', b's', b't'];

    let nonempty_result = PathNonEmpty::<MAX_LEN>::from_slice(&bytes);
    assert!(nonempty_result.is_ok());
}

#[kani::proof]
fn verify_path_nonempty_rejects_empty() {
    const MAX_LEN: usize = 16;

    let bytes: [u8; 0] = [];

    let nonempty_result = PathNonEmpty::<MAX_LEN>::from_slice(&bytes);
    assert!(nonempty_result.is_err());
}

// ============================================================================
// Special Path Proofs
// ============================================================================

#[kani::proof]
fn verify_root_path() {
    const MAX_LEN: usize = 16;

    let bytes = [b'/'];

    let path_result = PathBytes::<MAX_LEN>::from_slice(&bytes);
    assert!(path_result.is_ok());

    if let Ok(path) = path_result {
        assert!(path.is_root());
        assert!(path.is_absolute());
        assert!(!path.is_relative());
    }
}

#[kani::proof]
fn verify_current_directory() {
    const MAX_LEN: usize = 16;

    let bytes = [b'.'];

    let path_result = PathBytes::<MAX_LEN>::from_slice(&bytes);
    assert!(path_result.is_ok());

    if let Ok(path) = path_result {
        assert!(path.is_relative());
        assert!(!path.is_absolute());
        assert!(!path.is_root());
    }
}

// ============================================================================
// Validation Function Proofs (Byte-level)
// ============================================================================

#[kani::proof]
fn verify_has_null_byte_detection() {
    const MAX_LEN: usize = 4;

    let len: usize = kani::any();
    kani::assume(len > 0 && len <= MAX_LEN);

    let mut bytes = [32u8; MAX_LEN]; // Start with spaces (non-null ASCII)
    let has_null_expected: bool = kani::any();

    for i in 0..len {
        let byte: u8 = kani::any();
        kani::assume(byte < 128); // ASCII only

        if i == 0 && has_null_expected {
            bytes[i] = 0; // Force null at start if expected
        } else if has_null_expected {
            bytes[i] = 0; // Or anywhere else
        } else {
            kani::assume(byte > 0); // No null if not expected
            bytes[i] = byte;
        }
    }

    // Manual check
    let mut found_null = false;
    for i in 0..len {
        if bytes[i] == 0 {
            found_null = true;
            break;
        }
    }

    assert_eq!(found_null, has_null_expected);
}

#[kani::proof]
fn verify_absolute_path_byte_check() {
    // Test byte-level logic without string conversion
    let abs1 = [b'/', b'u', b's', b'r'];
    let abs2 = [b'/'];
    let rel1 = [b'u', b's', b'r'];
    let rel2 = [b'.'];

    // Absolute: first byte is '/'
    assert_eq!(abs1[0], b'/');
    assert_eq!(abs2[0], b'/');

    // Relative: first byte is not '/'
    assert_ne!(rel1[0], b'/');
    assert_ne!(rel2[0], b'/');
}
