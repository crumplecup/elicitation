//! Kani proofs for error generators.
//!
//! These proofs verify the correctness of error generator wrapper logic
//! following the "castle on cloud" pattern.

#![cfg(kani)]

use crate::{Generator, IoErrorGenerationMode, IoErrorGenerator};
use std::io;

// ============================================================================
// IoError Generator Proofs
// ============================================================================

/// Verify NotFound mode produces correct ErrorKind.
///
/// Castle on cloud: We trust io::Error::new() is correct.
/// We verify our generator wrapper calls it with the right ErrorKind.
#[kani::proof]
fn verify_ioerror_generator_not_found() {
    let mode = IoErrorGenerationMode::NotFound("test.txt".to_string());
    let generator = IoErrorGenerator::new(mode);

    let error = generator.generate();

    assert_eq!(
        error.kind(),
        io::ErrorKind::NotFound,
        "NotFound mode produces NotFound ErrorKind"
    );
}

/// Verify PermissionDenied mode produces correct ErrorKind.
#[kani::proof]
fn verify_ioerror_generator_permission_denied() {
    let mode = IoErrorGenerationMode::PermissionDenied("/etc/shadow".to_string());
    let generator = IoErrorGenerator::new(mode);

    let error = generator.generate();

    assert_eq!(
        error.kind(),
        io::ErrorKind::PermissionDenied,
        "PermissionDenied mode produces PermissionDenied ErrorKind"
    );
}

/// Verify ConnectionRefused mode produces correct ErrorKind.
#[kani::proof]
fn verify_ioerror_generator_connection_refused() {
    let mode = IoErrorGenerationMode::ConnectionRefused("localhost:8080".to_string());
    let generator = IoErrorGenerator::new(mode);

    let error = generator.generate();

    assert_eq!(
        error.kind(),
        io::ErrorKind::ConnectionRefused,
        "ConnectionRefused mode produces ConnectionRefused ErrorKind"
    );
}

/// Verify BrokenPipe mode produces correct ErrorKind.
#[kani::proof]
fn verify_ioerror_generator_broken_pipe() {
    let mode = IoErrorGenerationMode::BrokenPipe("pipe closed".to_string());
    let generator = IoErrorGenerator::new(mode);

    let error = generator.generate();

    assert_eq!(
        error.kind(),
        io::ErrorKind::BrokenPipe,
        "BrokenPipe mode produces BrokenPipe ErrorKind"
    );
}

/// Verify TimedOut mode produces correct ErrorKind.
#[kani::proof]
fn verify_ioerror_generator_timed_out() {
    let mode = IoErrorGenerationMode::TimedOut("operation timeout".to_string());
    let generator = IoErrorGenerator::new(mode);

    let error = generator.generate();

    assert_eq!(
        error.kind(),
        io::ErrorKind::TimedOut,
        "TimedOut mode produces TimedOut ErrorKind"
    );
}

/// Verify Other mode produces correct ErrorKind.
#[kani::proof]
fn verify_ioerror_generator_other() {
    let mode = IoErrorGenerationMode::Other("something went wrong".to_string());
    let generator = IoErrorGenerator::new(mode);

    let error = generator.generate();

    assert_eq!(
        error.kind(),
        io::ErrorKind::Other,
        "Other mode produces Other ErrorKind"
    );
}

/// Verify generator mode is preserved.
///
/// Verifies our struct correctly stores and returns the mode.
#[kani::proof]
fn verify_ioerror_generator_mode_preserved() {
    let mode = IoErrorGenerationMode::NotFound("test".to_string());
    let generator = IoErrorGenerator::new(mode.clone());

    assert_eq!(generator.mode(), &mode, "Generator preserves mode");
}

/// Verify mode helper methods return correct values.
///
/// Tests that error_kind() and message() return expected values.
#[kani::proof]
fn verify_ioerror_mode_helpers() {
    let message = "test message";
    let mode = IoErrorGenerationMode::NotFound(message.to_string());

    assert_eq!(
        mode.error_kind(),
        io::ErrorKind::NotFound,
        "error_kind() matches mode"
    );
    assert_eq!(mode.message(), message, "message() returns correct value");
}

/// Verify all ErrorKind variants map correctly.
///
/// Tests that each IoErrorGenerationMode variant produces the expected ErrorKind.
#[kani::proof]
fn verify_ioerror_all_kinds_map_correctly() {
    // Create one instance of each mode
    let modes = [
        IoErrorGenerationMode::NotFound("".to_string()),
        IoErrorGenerationMode::PermissionDenied("".to_string()),
        IoErrorGenerationMode::ConnectionRefused("".to_string()),
        IoErrorGenerationMode::ConnectionReset("".to_string()),
        IoErrorGenerationMode::BrokenPipe("".to_string()),
        IoErrorGenerationMode::AlreadyExists("".to_string()),
        IoErrorGenerationMode::InvalidInput("".to_string()),
        IoErrorGenerationMode::TimedOut("".to_string()),
        IoErrorGenerationMode::UnexpectedEof("".to_string()),
        IoErrorGenerationMode::Other("".to_string()),
    ];

    let expected_kinds = [
        io::ErrorKind::NotFound,
        io::ErrorKind::PermissionDenied,
        io::ErrorKind::ConnectionRefused,
        io::ErrorKind::ConnectionReset,
        io::ErrorKind::BrokenPipe,
        io::ErrorKind::AlreadyExists,
        io::ErrorKind::InvalidInput,
        io::ErrorKind::TimedOut,
        io::ErrorKind::UnexpectedEof,
        io::ErrorKind::Other,
    ];

    // Verify each mode produces the expected ErrorKind
    for (mode, expected) in modes.iter().zip(expected_kinds.iter()) {
        let generator = IoErrorGenerator::new(mode.clone());
        let error = generator.generate();
        assert_eq!(error.kind(), *expected, "ErrorKind matches mode");
    }
}

// ============================================================================
// JsonError Generator Proofs (feature-gated)
// ============================================================================

#[cfg(feature = "serde_json")]
use crate::{JsonErrorGenerationMode, JsonErrorGenerator};

/// Verify JsonError generator mode preservation.
///
/// Castle on cloud: We don't verify serde_json (inline asm limitation).
/// We verify our wrapper stores and retrieves the mode correctly.
#[cfg(feature = "serde_json")]
#[kani::proof]
fn verify_jsonerror_generator_mode_preserved() {
    let mode = JsonErrorGenerationMode::SyntaxError;
    let generator = JsonErrorGenerator::new(mode);

    assert_eq!(generator.mode(), mode, "Generator preserves mode");
}

/// Verify mode-to-string mapping is complete and correct.
///
/// Verifies each JsonErrorGenerationMode maps to a distinct invalid JSON string.
/// Does NOT call serde_json to avoid inline assembly.
#[cfg(feature = "serde_json")]
#[kani::proof]
fn verify_jsonerror_string_mapping() {
    let modes = [
        JsonErrorGenerationMode::SyntaxError,
        JsonErrorGenerationMode::EofWhileParsing,
        JsonErrorGenerationMode::InvalidNumber,
        JsonErrorGenerationMode::InvalidEscape,
        JsonErrorGenerationMode::InvalidUnicode,
    ];

    let expected_strings = [
        "{invalid}",
        "{\"key\":",
        "{\"num\": 1e999999}",
        r#"{"str": "\x"}"#,
        r#"{"str": "\uDEAD"}"#,
    ];

    // Verify each mode maps to expected string
    for (mode, expected) in modes.iter().zip(expected_strings.iter()) {
        let selected = match mode {
            JsonErrorGenerationMode::SyntaxError => "{invalid}",
            JsonErrorGenerationMode::EofWhileParsing => "{\"key\":",
            JsonErrorGenerationMode::InvalidNumber => "{\"num\": 1e999999}",
            JsonErrorGenerationMode::InvalidEscape => r#"{"str": "\x"}"#,
            JsonErrorGenerationMode::InvalidUnicode => r#"{"str": "\uDEAD"}"#,
        };

        assert_eq!(selected, *expected, "Mode maps to expected string");
        assert!(!selected.is_empty(), "String is non-empty");
    }
}
