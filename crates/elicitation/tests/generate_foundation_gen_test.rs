//! Tests for `generate foundation` — `#[derive(Elicit)]` constructibility harness generation.

#![cfg(feature = "cli")]

use elicitation::cli::generate::foundation_gen::{
    ElicitType, HarnessShape, generate_foundation_file, scan_elicit_types,
};
use std::fs;
use tempfile::tempdir;

// ─── scan_elicit_types ────────────────────────────────────────────────────────

#[test]
fn scan_finds_unit_enum_with_derive_elicit() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("types.rs"),
        r#"
#[derive(Debug, Clone, Elicit)]
pub enum Color { Red, Green, Blue }
"#,
    )
    .unwrap();

    let types = scan_elicit_types(dir.path());
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].name, "Color");
    assert!(
        matches!(&types[0].shape, HarnessShape::Constructible { first_variant } if first_variant == "Red")
    );
}

#[test]
fn scan_finds_struct_with_derive_elicit() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("types.rs"),
        r#"
#[derive(Debug, Elicit)]
pub struct Board(u8);
"#,
    )
    .unwrap();

    let types = scan_elicit_types(dir.path());
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].name, "Board");
    assert!(matches!(types[0].shape, HarnessShape::NewtypeWrapper));
}

#[test]
fn scan_treats_data_carrying_enum_as_newtype_wrapper() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("types.rs"),
        r#"
#[derive(Elicit)]
pub enum Slot { Empty, Filled(u8) }
"#,
    )
    .unwrap();

    let types = scan_elicit_types(dir.path());
    assert_eq!(types.len(), 1);
    assert!(matches!(types[0].shape, HarnessShape::NewtypeWrapper));
}

#[test]
fn scan_ignores_items_without_derive_elicit() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("types.rs"),
        r#"
#[derive(Debug, Clone)]
pub struct NotElicit;

#[derive(Debug)]
pub enum AlsoNot { A, B }
"#,
    )
    .unwrap();

    let types = scan_elicit_types(dir.path());
    assert!(types.is_empty());
}

#[test]
fn scan_deduplicates_same_type_across_files() {
    let dir = tempdir().unwrap();
    let src = r#"
#[derive(Elicit)]
pub struct Piece;
"#;
    fs::write(dir.path().join("a.rs"), src).unwrap();
    fs::write(dir.path().join("b.rs"), src).unwrap();

    let types = scan_elicit_types(dir.path());
    assert_eq!(types.len(), 1);
}

#[test]
fn scan_walks_subdirectories() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("inner");
    fs::create_dir_all(&sub).unwrap();
    fs::write(
        sub.join("deep.rs"),
        r#"
#[derive(Elicit)]
pub struct Token;
"#,
    )
    .unwrap();

    let types = scan_elicit_types(dir.path());
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].name, "Token");
}

// ─── generate_foundation_file ─────────────────────────────────────────────────

#[test]
fn generate_unit_enum_emits_constructible_harness() {
    let dir = tempdir().unwrap();
    // Minimal Cargo.toml so find_crate_name works
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"my_crate\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let types = vec![ElicitType {
        name: "Color".to_string(),
        shape: HarnessShape::Constructible {
            first_variant: "Red".to_string(),
        },
    }];
    let out = generate_foundation_file(&types, dir.path());
    assert!(
        out.contains("fn verify_color_constructible()"),
        "output:\n{out}"
    );
    assert!(out.contains("let _: Color = Color::Red;"), "output:\n{out}");
    assert!(out.contains("#[kani::proof]"), "output:\n{out}");
    assert!(out.contains("use my_crate::{Color}"), "output:\n{out}");
}

#[test]
fn generate_struct_emits_newtype_wrapper_harness() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"games\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let types = vec![ElicitType {
        name: "Board".to_string(),
        shape: HarnessShape::NewtypeWrapper,
    }];
    let out = generate_foundation_file(&types, dir.path());
    assert!(
        out.contains("fn verify_board_newtype_wrapper()"),
        "output:\n{out}"
    );
    assert!(
        out.contains("let _: Board = kani::any();"),
        "output:\n{out}"
    );
    assert!(
        out.contains("#[cfg_attr(kani, ::kani::proof)]"),
        "output:\n{out}"
    );
}

#[test]
fn generate_deduplicates_same_function_name() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"dup\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    // Two types with the same name (shouldn't normally happen, but dedup must hold)
    let types = vec![
        ElicitType {
            name: "Token".to_string(),
            shape: HarnessShape::NewtypeWrapper,
        },
        ElicitType {
            name: "Token".to_string(),
            shape: HarnessShape::NewtypeWrapper,
        },
    ];
    let out = generate_foundation_file(&types, dir.path());
    let count = out.matches("fn verify_token_newtype_wrapper()").count();
    assert_eq!(count, 1, "expected 1 harness, got:\n{out}");
}

#[test]
fn generate_empty_types_emits_header_only() {
    let dir = tempdir().unwrap();
    let out = generate_foundation_file(&[], dir.path());
    assert!(out.contains("AUTO-GENERATED"), "output:\n{out}");
    assert!(!out.contains("fn verify_"), "output:\n{out}");
}

#[test]
fn full_scan_and_generate_roundtrip() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"roundtrip\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("lib.rs"),
        r#"
#[derive(Elicit)]
pub enum Suit { Hearts, Diamonds, Clubs, Spades }

#[derive(Elicit)]
pub struct Card(u8);
"#,
    )
    .unwrap();

    let types = scan_elicit_types(dir.path());
    let out = generate_foundation_file(&types, dir.path());

    assert!(
        out.contains("fn verify_suit_constructible()"),
        "output:\n{out}"
    );
    assert!(
        out.contains("fn verify_card_newtype_wrapper()"),
        "output:\n{out}"
    );
    assert!(out.contains("use roundtrip::{"), "output:\n{out}");
}
