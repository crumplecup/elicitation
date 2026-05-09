//! Tests for `cli::generate::verus_gen` — validates generated Verus V11/V12 companion output.

#![cfg(feature = "cli")]

use elicitation::cli::generate::{
    scanner::{ArgDescriptor, ArgKind, PropDescriptor, TransitionFn, VsmDescriptor},
    verus_gen::generate_verus_file,
};
use std::path::{Path, PathBuf};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn vsm_with_body(
    machine: &str,
    inv_body: Option<&str>,
    state_body: Option<&str>,
    transitions: Vec<&str>,
) -> VsmDescriptor {
    let consistent = machine.replace("Machine", "Consistent");
    let snake = to_snake(machine.trim_end_matches("Machine"));
    let inv_fn = format!("{snake}_consistent");

    VsmDescriptor {
        machine: machine.to_string(),
        transitions: transitions.iter().map(|t| t.to_string()).collect(),
        invariant: Some(PropDescriptor {
            name: consistent,
            kani_fn: Some(inv_fn.clone()),
            verus_fn: Some(inv_fn.clone()),
            creusot_fn: None,
            verus_inv_body: inv_body.map(|s| s.to_string()),
            creusot_inv_body: None,
            verus_state_body: state_body.map(|s| s.to_string()),
        }),
        transition_fns: vec![],
        source_file: PathBuf::from("src/vsm/thing.rs"),
    }
}

fn vsm_with_transition(
    machine: &str,
    transition: &str,
    inv_body: Option<&str>,
    state_body: Option<&str>,
    trans_body: Option<&str>,
) -> VsmDescriptor {
    let state = machine.replace("Machine", "State");
    let consistent = machine.replace("Machine", "Consistent");
    let snake = to_snake(machine.trim_end_matches("Machine"));
    let inv_fn = format!("{snake}_consistent");

    VsmDescriptor {
        machine: machine.to_string(),
        transitions: vec![transition.to_string()],
        invariant: Some(PropDescriptor {
            name: consistent.clone(),
            kani_fn: Some(inv_fn.clone()),
            verus_fn: Some(inv_fn.clone()),
            creusot_fn: None,
            verus_inv_body: inv_body.map(|s| s.to_string()),
            creusot_inv_body: None,
            verus_state_body: state_body.map(|s| s.to_string()),
        }),
        transition_fns: vec![TransitionFn {
            name: transition.to_string(),
            args: vec![
                ArgDescriptor {
                    name: "_state".to_string(),
                    ty: state.clone(),
                    kind: ArgKind::State,
                },
                ArgDescriptor {
                    name: "proof".to_string(),
                    ty: format!("Established<{consistent}>"),
                    kind: ArgKind::Proof {
                        inner: consistent.clone(),
                    },
                },
            ],
            body: trans_body.map(|s| s.to_string()),
            verus_class: None,
        }],
        source_file: PathBuf::from("src/vsm/thing.rs"),
    }
}

fn to_snake(s: &str) -> String {
    s.chars().fold(String::new(), |mut acc, c| {
        if c.is_uppercase() && !acc.is_empty() {
            acc.push('_');
        }
        acc.push(c.to_ascii_lowercase());
        acc
    })
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn generated_file_has_header_comment() {
    let vsm = vsm_with_body("NavMachine", Some("true"), None, vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(out.contains("AUTO-GENERATED"), "expected AUTO-GENERATED header");
    assert!(out.contains("NavMachine"), "expected machine name in header");
}

#[test]
fn generated_file_has_verus_imports() {
    let vsm = vsm_with_body("NavMachine", Some("true"), None, vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(out.contains("use vstd::prelude::*"), "expected vstd import");
    assert!(out.contains("use verus_builtin_macros::verus"), "expected verus macro import");
    // New output does NOT use #[cfg(verus)] gates — it targets elicitation_verus exclusively.
    assert!(!out.contains("#[cfg(verus)]"), "new output should not have cfg(verus) gates");
}

#[test]
fn invariant_spec_fn_emitted_with_body() {
    let vsm = vsm_with_body("NavMachine", Some("true"), None, vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("pub open spec fn nav_consistent"),
        "expected open spec fn; got:\n{out}"
    );
    assert!(out.contains("true"), "expected body in spec fn");
}

#[test]
fn invariant_spec_fn_errors_when_body_missing() {
    let vsm = vsm_with_body("NavMachine", None, None, vec!["go"]);
    let result = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(result.is_err(), "expected Err when verus_inv_body missing; got Ok");
    let msg = result.unwrap_err();
    assert!(
        msg.contains("NavMachine"),
        "error message should name the machine; got:\n{msg}"
    );
}

#[test]
fn abstract_state_enum_emitted_when_state_body_set() {
    let vsm = vsm_with_body(
        "ConnMachine",
        Some("true"),
        Some("Active, _Other,"),
        vec!["connect"],
    );
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("pub enum ConnState"),
        "expected abstract state enum; got:\n{out}"
    );
    assert!(out.contains("Active,"), "expected Active variant");
    assert!(out.contains("_Other,"), "expected _Other variant");
}

#[test]
fn abstract_state_enum_has_unspecified_placeholder_when_no_state_body() {
    let vsm = vsm_with_body("NavMachine", Some("true"), None, vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("pub enum NavState"),
        "expected state enum even without state_body; got:\n{out}"
    );
    assert!(out.contains("_Unspecified"), "expected placeholder variant");
}

#[test]
fn transition_tag_enum_emitted() {
    let vsm = vsm_with_body("NavMachine", Some("true"), None, vec!["go", "back", "reset"]);
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("pub enum NavMachineTrans"),
        "expected tag enum; got:\n{out}"
    );
    assert!(out.contains("Go,"), "expected Go tag");
    assert!(out.contains("Back,"), "expected Back tag");
    assert!(out.contains("Reset,"), "expected Reset tag");
}

#[test]
fn composition_proof_fn_emitted() {
    let vsm = vsm_with_body("NavMachine", Some("true"), None, vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("pub proof fn nav_composition"),
        "expected composition proof fn; got:\n{out}"
    );
    assert!(out.contains("ensures nav_consistent(&post)"), "expected ensures clause");
}

#[test]
fn dispatch_spec_fn_emitted() {
    let vsm = vsm_with_body("NavMachine", Some("true"), None, vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("pub open spec fn nav_post"),
        "expected nav_post dispatch fn; got:\n{out}"
    );
}

#[test]
fn leaf_trivial_emitted() {
    // No state body → no special variants → all transitions are Trivial.
    let vsm = vsm_with_body("NavMachine", Some("true"), None, vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("pub proof fn nav_leaf_trivial"),
        "expected trivial leaf lemma; got:\n{out}"
    );
}

#[test]
fn passthrough_transition_uses_passthrough_lemma() {
    // A transition body containing `other =>` is classified as Passthrough.
    let vsm = vsm_with_transition(
        "NavMachine",
        "noop",
        Some("true"),
        Some("Active, _Other,"),
        Some("match state { other => other }"), // passthrough body
    );
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("pub proof fn nav_leaf_passthrough"),
        "expected passthrough leaf lemma; got:\n{out}"
    );
    assert!(
        out.contains("NavMachineTrans::Noop => nav_leaf_passthrough"),
        "expected passthrough dispatch; got:\n{out}"
    );
}

#[test]
fn special_false_transition_uses_special_false_lemma() {
    // A transition body that returns the special variant (no passthrough) → SpecialFalse.
    let vsm = vsm_with_transition(
        "NavMachine",
        "activate",
        Some("true"),
        Some("Active { running: bool }, _Other,"),
        Some("NavState :: Active { running : false }"), // returns Active, no passthrough
    );
    let out = generate_verus_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("nav_leaf_active_false"),
        "expected SpecialFalse leaf; got:\n{out}"
    );
    assert!(
        out.contains("NavMachineTrans::Activate => nav_leaf_active_false"),
        "expected SpecialFalse dispatch; got:\n{out}"
    );
}

// ─── Snapshot-style tests against archive_nav and archive_connection ──────────

#[test]
fn scan_and_generate_archive_nav_verus() {
    use elicitation::cli::generate::scan_vsms;

    let vsm_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("elicit_server/src/archive/vsm");

    if !vsm_dir.exists() {
        return;
    }

    let vsms = scan_vsms(&vsm_dir);
    let nav = vsms
        .iter()
        .find(|v| v.machine == "ArchiveNavMachine")
        .expect("ArchiveNavMachine not found");

    assert!(
        nav.invariant
            .as_ref()
            .and_then(|i| i.verus_inv_body.as_deref())
            .is_some(),
        "verus_inv_body should be present on ArchiveNavConsistent"
    );

    let out = generate_verus_file(nav, &vsm_dir).unwrap();

    assert!(out.contains("AUTO-GENERATED"), "missing header");
    assert!(
        out.contains("pub open spec fn archive_nav_consistent"),
        "missing spec fn; got:\n{out}"
    );
    assert!(
        out.contains("pub enum ArchiveNavMachineTrans"),
        "missing tag enum; got:\n{out}"
    );
    assert!(
        out.contains("pub proof fn archive_nav_composition"),
        "missing composition proof; got:\n{out}"
    );
    assert!(
        out.contains("ensures archive_nav_consistent(&post)"),
        "missing ensures clause"
    );

    // Every transition should appear as a tag variant.
    for t in &nav.transitions {
        let pascal: String = t.split('_').map(|seg| {
            let mut c = seg.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        }).collect();
        assert!(
            out.contains(&format!("    {pascal},")),
            "missing tag {pascal} for transition {t}; got:\n{out}"
        );
    }
}

#[test]
fn scan_and_generate_archive_connection_verus() {
    use elicitation::cli::generate::scan_vsms;

    let vsm_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("elicit_server/src/archive/vsm");

    if !vsm_dir.exists() {
        return;
    }

    let vsms = scan_vsms(&vsm_dir);
    let conn = vsms
        .iter()
        .find(|v| v.machine == "ArchiveConnectionMachine")
        .expect("ArchiveConnectionMachine not found");

    let inv = conn.invariant.as_ref().expect("should have invariant");
    assert_eq!(
        inv.verus_inv_body.as_deref(),
        Some("true"),
        "connection body should be 'true'"
    );

    let out = generate_verus_file(conn, &vsm_dir).unwrap();
    assert!(
        out.contains("pub open spec fn archive_connection_consistent"),
        "missing spec fn"
    );
    assert!(out.contains("{ true }"), "expected trivial invariant body");
    assert!(
        out.contains("pub proof fn archive_connection_composition"),
        "missing composition proof"
    );
}

