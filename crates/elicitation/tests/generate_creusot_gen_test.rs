//! Tests for `cli::generate::creusot_gen` — validates generated Creusot companion output.

#![cfg(feature = "cli")]

use elicitation::cli::generate::{
    creusot_gen::generate_creusot_file,
    scanner::{ArgDescriptor, ArgKind, PropDescriptor, TransitionFn, VsmDescriptor},
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
            creusot_fn: Some(inv_fn.clone()),
            verus_inv_body: None,
            creusot_inv_body: body.map(|s| s.to_string()),
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
            kind: ArgKind::Proof { inner: consistent.clone() },
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
            creusot_fn: Some(inv_fn.clone()),
            verus_inv_body: None,
            creusot_inv_body: body.map(|s| s.to_string()),
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
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(out.contains("AUTO-GENERATED"), "expected AUTO-GENERATED header");
    assert!(out.contains("NavMachine"), "expected machine name in header");
}

#[test]
fn generated_file_has_cfg_creusot_imports() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(out.contains("#[cfg(creusot)]"), "expected #[cfg(creusot)]");
    assert!(out.contains("::creusot_std::prelude::*"), "expected creusot_std import");
    assert!(out.contains("elicitation::Established"), "expected Established import");
}

#[test]
fn logic_fn_emitted_with_body() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(out.contains("#[logic]"), "expected #[logic]");
    assert!(
        out.contains("pub fn nav_consistent"),
        "expected logic fn; got:\n{out}"
    );
    assert!(out.contains("true"), "expected body in logic fn");
}

#[test]
fn logic_fn_uses_todo_when_body_missing() {
    let vsm = vsm_with_body("NavMachine", None, vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("TODO: creusot_inv_body"),
        "expected TODO placeholder; got:\n{out}"
    );
}

#[test]
fn marker_proof_fn_emitted() {
    let vsm = vsm_with_body("ConnMachine", Some("true"), vec!["begin"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(
        out.contains("verify_conn_consistent_prop_creusot"),
        "expected marker fn; got:\n{out}"
    );
    assert!(out.contains("#[trusted]"), "expected #[trusted]");
    assert!(out.contains("#[requires(true)]"), "expected requires(true)");
    assert!(out.contains("#[ensures(result)]"), "expected ensures(result)");
}

#[test]
fn wrapper_emitted_per_transition() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go", "back"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(out.contains("fn go__creusot"), "expected go wrapper; got:\n{out}");
    assert!(out.contains("fn back__creusot"), "expected back wrapper; got:\n{out}");
}

#[test]
fn wrapper_has_requires_and_ensures() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(out.contains("#[requires(nav_consistent(&state))]"), "expected requires; got:\n{out}");
    assert!(out.contains("#[ensures(nav_consistent(&result.0))]"), "expected ensures; got:\n{out}");
}

#[test]
fn wrapper_body_calls_through() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(out.contains("{ go("), "expected call-through body; got:\n{out}");
}

#[test]
fn wrapper_with_extra_string_arg() {
    let vsm = vsm_with_transition_and_body(
        "ConnMachine",
        "begin",
        Some("true"),
        vec![ArgDescriptor {
            name: "profile_name".to_string(),
            ty: "String".to_string(),
            kind: ArgKind::StringArg,
        }],
    );
    let out = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(out.contains("profile_name: String"), "expected String param; got:\n{out}");
    assert!(out.contains("begin(state, proof, profile_name)"), "expected call with extra arg; got:\n{out}");
}

// ─── Snapshot tests against live archive ──────────────────────────────────────

#[test]
fn scan_and_generate_archive_connection_creusot() {
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
        inv.creusot_inv_body.as_deref(),
        Some("true"),
        "connection body should be 'true'"
    );

    let out = generate_creusot_file(conn, &vsm_dir);
    assert!(out.contains("AUTO-GENERATED"), "missing header");
    assert!(out.contains("#[logic]"), "missing #[logic]");
    assert!(out.contains("pub fn archive_connection_consistent"), "missing logic fn");
    assert!(out.contains("{ true }"), "expected trivial body");
    assert!(out.contains("verify_archive_connection_consistent_prop_creusot"), "missing marker");

    for t in &conn.transitions {
        assert!(
            out.contains(&format!("fn {t}__creusot")),
            "missing wrapper for transition {t}"
        );
        assert!(
            out.contains(&format!("{{ {t}(")),
            "missing call-through for {t}"
        );
    }
}

#[test]
fn scan_and_generate_archive_nav_creusot() {
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

    let inv = nav.invariant.as_ref().expect("should have invariant");
    assert!(
        inv.creusot_inv_body.as_ref().map(|b| b.contains("pearlite!")).unwrap_or(false),
        "nav body should use pearlite!"
    );

    let out = generate_creusot_file(nav, &vsm_dir);
    assert!(out.contains("pub fn archive_nav_consistent"), "missing logic fn");
    assert!(out.contains("pearlite!"), "expected pearlite body");
    assert!(out.contains("requires(archive_nav_consistent"), "missing requires");
    assert!(out.contains("ensures(archive_nav_consistent"), "missing ensures");
}
