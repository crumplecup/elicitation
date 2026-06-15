//! Regression tests for `elicitation generate proof-crate` dependency resolution.
//!
//! Covers the case where a downstream crate uses a registry `elicitation`
//! dependency (no `path` key) and passes a subdirectory as `--crate-path`.
//! Previously the generator emitted a commented TODO placeholder instead of a
//! real dep.

#![cfg(feature = "cli")]

use elicitation::cli::generate::proof_crate_gen::generate_proof_crate;
use std::fs;
use tempfile::tempdir;

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn read_elicitation_dep_line(cargo_toml: &str) -> Option<String> {
    let mut in_deps = false;
    for line in cargo_toml.lines() {
        let trimmed = line.trim();
        if trimmed == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_deps = false;
            continue;
        }
        if in_deps && trimmed.starts_with("elicitation") {
            return Some(trimmed.to_string());
        }
    }
    None
}

// ─── Version dep (Valinoreth-shaped case) ────────────────────────────────────

/// Source crate uses `elicitation = { version = "x.y.z", features = [...] }`.
/// Generator must emit `elicitation = { version = "x.y.z" }`, not a TODO.
#[test]
fn registry_version_dep_emits_real_dep() {
    let source = tempdir().unwrap();
    // Source crate root: Cargo.toml with a registry elicitation dep.
    fs::write(
        source.path().join("Cargo.toml"),
        r#"[package]
name = "my_game"
version = "0.3.0"
edition = "2024"

[dependencies]
elicitation = { version = "0.11.1", features = ["serde"] }
"#,
    )
    .unwrap();

    // VSM files live in a subdirectory (simulates valinoreth/src/vsm).
    let vsm_dir = source.path().join("src").join("vsm");
    fs::create_dir_all(&vsm_dir).unwrap();

    let out = tempdir().unwrap();
    generate_proof_crate(
        &[],
        &[vsm_dir.as_path()],
        out.path(),
        Some("my_game_proofs"),
    )
    .unwrap();

    let cargo_toml = fs::read_to_string(out.path().join("Cargo.toml")).unwrap();
    let dep_line = read_elicitation_dep_line(&cargo_toml)
        .expect("elicitation dep missing from generated Cargo.toml");

    assert!(
        !dep_line.starts_with('#'),
        "elicitation dep must not be a comment/placeholder, got: {dep_line}"
    );
    assert!(
        dep_line.contains("0.11.1"),
        "elicitation dep must preserve the source version, got: {dep_line}"
    );
}

/// Same as above but with the bare-string short form: `elicitation = "x.y.z"`.
#[test]
fn registry_bare_version_dep_emits_real_dep() {
    let source = tempdir().unwrap();
    fs::write(
        source.path().join("Cargo.toml"),
        r#"[package]
name = "tiny_crate"
version = "1.0.0"
edition = "2024"

[dependencies]
elicitation = "0.10.5"
"#,
    )
    .unwrap();

    let vsm_dir = source.path().join("src");
    fs::create_dir_all(&vsm_dir).unwrap();

    let out = tempdir().unwrap();
    generate_proof_crate(
        &[],
        &[vsm_dir.as_path()],
        out.path(),
        Some("tiny_crate_proofs"),
    )
    .unwrap();

    let cargo_toml = fs::read_to_string(out.path().join("Cargo.toml")).unwrap();
    let dep_line = read_elicitation_dep_line(&cargo_toml)
        .expect("elicitation dep missing from generated Cargo.toml");

    assert!(
        !dep_line.starts_with('#'),
        "elicitation dep must not be a comment/placeholder, got: {dep_line}"
    );
    assert!(
        dep_line.contains("0.10.5"),
        "elicitation dep must preserve the source version, got: {dep_line}"
    );
}

/// Second run must produce the same elicitation dep (idempotency).
#[test]
fn regeneration_is_idempotent_for_registry_dep() {
    let source = tempdir().unwrap();
    fs::write(
        source.path().join("Cargo.toml"),
        r#"[package]
name = "stable_crate"
version = "2.0.0"
edition = "2024"

[dependencies]
elicitation = { version = "0.11.1" }
"#,
    )
    .unwrap();

    let vsm_dir = source.path().join("src").join("vsm");
    fs::create_dir_all(&vsm_dir).unwrap();

    let out = tempdir().unwrap();

    // First run.
    generate_proof_crate(
        &[],
        &[vsm_dir.as_path()],
        out.path(),
        Some("stable_crate_proofs"),
    )
    .unwrap();
    let first = fs::read_to_string(out.path().join("Cargo.toml")).unwrap();
    let first_dep = read_elicitation_dep_line(&first).unwrap();

    // Second run — must not degrade the dep.
    generate_proof_crate(
        &[],
        &[vsm_dir.as_path()],
        out.path(),
        Some("stable_crate_proofs"),
    )
    .unwrap();
    let second = fs::read_to_string(out.path().join("Cargo.toml")).unwrap();
    let second_dep = read_elicitation_dep_line(&second).unwrap();

    assert_eq!(
        first_dep, second_dep,
        "elicitation dep changed between regeneration runs"
    );
    assert!(
        !second_dep.starts_with('#'),
        "second run must not introduce a TODO placeholder, got: {second_dep}"
    );
}

/// When the source manifest has no elicitation dep at all but the output
/// already has one, regeneration must preserve it.
#[test]
fn regeneration_preserves_manually_set_dep() {
    let source = tempdir().unwrap();
    fs::write(
        source.path().join("Cargo.toml"),
        r#"[package]
name = "no_elicitation_source"
version = "0.1.0"
edition = "2024"
"#,
    )
    .unwrap();

    let vsm_dir = source.path().join("src");
    fs::create_dir_all(&vsm_dir).unwrap();

    let out = tempdir().unwrap();

    // Simulate: first generation leaves a TODO, user manually fixes it.
    generate_proof_crate(
        &[],
        &[vsm_dir.as_path()],
        out.path(),
        Some("no_elicitation_proofs"),
    )
    .unwrap();

    // Overwrite Cargo.toml with a manually corrected version.
    let manual = fs::read_to_string(out.path().join("Cargo.toml"))
        .unwrap()
        .replace(
            "# elicitation = { path = \"<path-to-elicitation>\" }  # TODO: set this",
            "elicitation = { version = \"0.11.1\" }",
        );
    fs::write(out.path().join("Cargo.toml"), &manual).unwrap();

    // Second run must preserve the manually set dep.
    generate_proof_crate(
        &[],
        &[vsm_dir.as_path()],
        out.path(),
        Some("no_elicitation_proofs"),
    )
    .unwrap();
    let after = fs::read_to_string(out.path().join("Cargo.toml")).unwrap();
    let dep =
        read_elicitation_dep_line(&after).expect("elicitation dep missing after second generation");

    assert!(
        !dep.starts_with('#'),
        "manually set dep must not be replaced with TODO, got: {dep}"
    );
    assert!(
        dep.contains("0.11.1"),
        "manually set dep must be preserved, got: {dep}"
    );
}
