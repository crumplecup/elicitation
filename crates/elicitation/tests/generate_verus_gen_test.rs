//! Tests for `cli::generate::verus_gen` — validates generated Verus companion output.

#![cfg(feature = "cli")]

use elicitation::cli::generate::{
    scanner::{ArgDescriptor, ArgKind, PropDescriptor, TransitionFn, VsmDescriptor},
    verus_gen::generate_verus_file,
};
use std::path::{Path, PathBuf};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn vsm_with_body(machine: &str, body: Option<&str>, transitions: Vec<&str>) -> VsmDescriptor {
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
            verus_inv_body: body.map(|s| s.to_string()),
        }),
        transition_fns: vec![],
        source_file: PathBuf::from("src/vsm/thing.rs"),
    }
}

fn vsm_with_transition_and_body(
    machine: &str,
    transition: &str,
    body: Option<&str>,
    extra: Vec<ArgDescriptor>,
) -> VsmDescriptor {
    let state = machine.replace("Machine", "State");
    let consistent = machine.replace("Machine", "Consistent");
    let snake = to_snake(machine.trim_end_matches("Machine"));
    let inv_fn = format!("{snake}_consistent");

    let mut args = vec![
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
    ];
    args.extend(extra);

    VsmDescriptor {
        machine: machine.to_string(),
        transitions: vec![transition.to_string()],
        invariant: Some(PropDescriptor {
            name: consistent.clone(),
            kani_fn: Some(inv_fn.clone()),
            verus_fn: Some(inv_fn.clone()),
            creusot_fn: None,
            verus_inv_body: body.map(|s| s.to_string()),
        }),
        transition_fns: vec![TransitionFn {
            name: transition.to_string(),
            args,
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
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("AUTO-GENERATED"),
        "expected AUTO-GENERATED header"
    );
    assert!(
        out.contains("NavMachine"),
        "expected machine name in header"
    );
}

#[test]
fn generated_file_has_cfg_verus_imports() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(out.contains("#[cfg(verus)]"), "expected #[cfg(verus)]");
    assert!(out.contains("::vstd::prelude::*"), "expected vstd import");
    assert!(
        out.contains("elicitation::Established"),
        "expected Established import"
    );
}

#[test]
fn invariant_spec_fn_emitted_with_body() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("pub open spec fn nav_consistent"),
        "expected open spec fn; got:\n{out}"
    );
    // Body is verbatim
    assert!(out.contains("true"), "expected body in spec fn");
}

#[test]
fn invariant_spec_fn_uses_todo_when_body_missing() {
    let vsm = vsm_with_body("NavMachine", None, vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("TODO: verus_inv_body"),
        "expected TODO placeholder when body missing; got:\n{out}"
    );
}

#[test]
fn marker_proof_fn_emitted() {
    let vsm = vsm_with_body("ConnMachine", Some("true"), vec!["begin"]);
    let out = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("verify_conn_consistent_prop_contract"),
        "expected marker fn; got:\n{out}"
    );
    assert!(
        out.contains("ensures result == true"),
        "expected ensures clause"
    );
}

#[test]
fn assume_specification_emitted_per_transition() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go", "back"]);
    let out = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("assume_specification [go]"),
        "expected go spec"
    );
    assert!(
        out.contains("assume_specification [back]"),
        "expected back spec"
    );
}

#[test]
fn assume_specification_has_requires_and_ensures() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("requires nav_consistent("),
        "expected requires clause"
    );
    assert!(
        out.contains("ensures nav_consistent(&r.0)"),
        "expected ensures clause"
    );
}

#[test]
fn assume_specification_with_string_extra_arg() {
    let vsm = vsm_with_transition_and_body(
        "ConnMachine",
        "begin",
        Some("true"),
        vec![ArgDescriptor {
            name: "name".to_string(),
            ty: "String".to_string(),
            kind: ArgKind::StringArg,
        }],
    );
    let out = generate_verus_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("name: String"),
        "expected String arg in spec; got:\n{out}"
    );
}

// ─── Snapshot test against archive_nav reference ──────────────────────────────

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

    // verus_inv_body should now be populated from the annotation
    assert!(
        nav.invariant
            .as_ref()
            .and_then(|i| i.verus_inv_body.as_deref())
            .is_some(),
        "verus_inv_body should be present on ArchiveNavConsistent"
    );

    let out = generate_verus_file(nav, &vsm_dir);

    assert!(out.contains("AUTO-GENERATED"), "missing header");
    assert!(
        out.contains("pub open spec fn archive_nav_consistent"),
        "missing spec fn"
    );
    assert!(
        out.contains("verify_archive_nav_consistent_prop_contract"),
        "missing marker"
    );

    // Each transition should have an assume_specification
    for t in &nav.transitions {
        assert!(
            out.contains(&format!("assume_specification [{t}]")),
            "missing spec for transition {t}"
        );
    }
    assert!(
        out.contains("requires archive_nav_consistent"),
        "missing requires"
    );
    assert!(
        out.contains("ensures archive_nav_consistent"),
        "missing ensures"
    );
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

    let out = generate_verus_file(conn, &vsm_dir);
    assert!(
        out.contains("pub open spec fn archive_connection_consistent"),
        "missing spec fn"
    );
    assert!(out.contains("{ true }"), "expected trivial body");
}
