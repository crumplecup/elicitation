//! Tests for Creusot shadow-workspace sanitization.

#![cfg(feature = "cli")]

use elicitation::cli::creusot_shadow::{prepare_shadow_workspace, sanitize_rust_source};
use tempfile::tempdir;

#[test]
fn sanitize_rust_source_gates_instrument_and_hostile_derives() {
    let src = r#"
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct Demo;

#[tracing::instrument]
fn trace_init() {
    tracing::trace!("hello");
}
"#;

    let out = sanitize_rust_source(src).expect("sanitized source");
    assert!(out.contains("#[derive(Debug)]"));
    assert!(out.contains("#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]"));
    assert!(out.contains("#[cfg_attr(not(creusot), tracing::instrument)]"));
    assert!(out.contains("#[cfg(not(creusot))]"));
}

#[test]
fn prepare_shadow_workspace_sanitizes_rust_files_in_copy() {
    let temp = tempdir().expect("tempdir");
    let root = temp.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"shadow-demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("write Cargo.toml");
    std::fs::create_dir_all(root.join("src")).expect("mkdir src");
    std::fs::write(
        root.join("src/lib.rs"),
        "#[instrument]\nfn f() { tracing::debug!(\"x\"); }\n",
    )
    .expect("write lib.rs");

    let shadow = prepare_shadow_workspace(root).expect("prepare shadow");
    let lib = std::fs::read_to_string(shadow.join("src/lib.rs")).expect("read shadow lib.rs");
    assert!(lib.contains("#[cfg_attr(not(creusot), instrument)]"));
    assert!(lib.contains("#[cfg(not(creusot))]"));
    assert!(lib.contains("#[cfg(not(creusot))]"));
}

#[test]
fn sanitize_rust_source_does_not_duplicate_existing_deep_model() {
    let src = r#"
#[cfg_attr(creusot, derive(DeepModel))]
enum Demo {
    A,
    B,
}
"#;

    let out = sanitize_rust_source(src).expect("sanitized source");
    assert_eq!(out.matches("DeepModel").count(), 1);
}

#[test]
fn sanitize_rust_source_gates_partial_ord_for_float_wrappers() {
    let src = r#"
#[derive(Debug, PartialEq, PartialOrd)]
struct Demo(f64);
"#;

    let out = sanitize_rust_source(src).expect("sanitized source");
    assert!(out.contains("#[derive(Debug)]"));
    assert!(out.contains("#[cfg_attr(not(creusot), derive(PartialEq, PartialOrd))]"));
}

#[test]
fn sanitize_rust_source_strips_panic_messages() {
    let src = r#"
fn demo() -> i32 {
    panic!("bad {}", 1)
}
"#;

    let out = sanitize_rust_source(src).expect("sanitized source");
    assert!(out.contains("panic!()"));
}

#[test]
fn sanitize_rust_source_rewrites_vec_list_macro() {
    let src = r#"
fn demo() -> Vec<i32> {
    vec![1, 2, 3]
}
"#;

    let out = sanitize_rust_source(src).expect("sanitized source");
    assert!(out.contains("let mut __elicitation_shadow_vec = ::std::vec::Vec::new();"));
    assert!(out.contains("__elicitation_shadow_vec.push(1);"));
    assert!(out.contains("__elicitation_shadow_vec.push(2);"));
    assert!(out.contains("__elicitation_shadow_vec.push(3);"));
}

#[test]
fn sanitize_rust_source_adds_deep_model_and_gates_more_hostile_derives() {
    let src = r#"
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, derive_more::FromStr, strum::EnumIter)]
enum Demo {
    A,
    B,
}
"#;

    let out = sanitize_rust_source(src).expect("sanitized source");
    assert!(out.contains("#[derive(Debug, PartialEq, Eq)]"));
    assert!(out.contains(
        "#[cfg_attr(not(creusot), derive(PartialOrd, Ord, derive_more::FromStr, strum::EnumIter))]"
    ));
    assert!(out.contains("#[cfg_attr(creusot, derive(creusot_std::model::DeepModel))]"));
}

#[test]
fn sanitize_rust_source_gates_runtime_parse_helpers() {
    let src = r#"
impl Demo {
    pub fn from_value(value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }

    pub fn from_abbr(value: &str) -> Option<Self> {
        let lwr = value.to_lowercase();
        let _ = lwr.as_str();
        None
    }
}
"#;

    let out = sanitize_rust_source(src).expect("sanitized source");
    assert_eq!(out.matches("#[cfg(not(creusot))]").count(), 2);
    assert!(out.contains("pub fn from_value"));
    assert!(out.contains("pub fn from_abbr"));
}

#[test]
fn sanitize_rust_source_stubs_main_and_erases_tracing_exprs() {
    let src = r#"
fn main() {
    match "x" {
        _ => tracing::info!("hello"),
    }
}
"#;

    let out = sanitize_rust_source(src).expect("sanitized source");
    assert!(out.contains("#[cfg(creusot)]"));
    assert!(out.contains("#[cfg(not(creusot))]"));
    assert!(out.contains("match \"x\""));
    assert!(!out.contains("tracing::info!"));
}

#[test]
fn sanitize_rust_source_adds_deep_model_for_plain_structs() {
    let src = r#"
#[derive(Debug, PartialEq, Eq)]
struct Demo {
    pub value: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Proofy {
    proof: Established<Token>,
}
#[derive(Debug, PartialEq, Eq)]
struct Private {
    value: i32,
}
"#;
    let out = sanitize_rust_source(src).expect("sanitized source");
    assert!(out.contains("struct Demo"));
    assert!(out.contains("struct Proofy"));
    assert!(out.contains("struct Private"));
    assert!(out.contains("pub value: i32"));
    assert_eq!(
        out.matches("#[cfg_attr(creusot, derive(creusot_std::model::DeepModel))]")
            .count(),
        2
    );
}
