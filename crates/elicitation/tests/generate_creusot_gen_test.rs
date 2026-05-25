//! Tests for `cli::generate::creusot_gen` — validates generated Creusot companion output.

#![cfg(feature = "cli")]

use elicitation::cli::generate::{
    ImportStyle,
    creusot_gen::{generate_creusot_file, generate_creusot_file_with_style},
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
            verus_state_body: None,
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
            creusot_fn: Some(inv_fn.clone()),
            verus_inv_body: None,
            creusot_inv_body: body.map(|s| s.to_string()),
            verus_state_body: None,
        }),
        transition_fns: vec![TransitionFn {
            name: transition.to_string(),
            args,
            body: Some("{ begin(state, proof, profile_name) }".to_string()),
            has_instrument: false,
            creusot_body: Some("{ begin_creusot_local(state, proof, profile_name) }".to_string()),
            creusot_requires: Vec::new(),
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
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
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
fn generated_file_has_cfg_creusot_imports() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
    assert!(out.contains("#[cfg(creusot)]"), "expected #[cfg(creusot)]");
    assert!(
        out.contains("::creusot_std::prelude::*"),
        "expected creusot_std import"
    );
    assert!(
        out.contains("elicitation::Established"),
        "expected Established import"
    );
}

#[test]
fn logic_fn_emitted_with_body() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
    assert!(out.contains("#[logic]"), "expected #[logic]");
    assert!(
        out.contains("pub fn nav_consistent"),
        "expected logic fn; got:\n{out}"
    );
    assert!(out.contains("true"), "expected body in logic fn");
}

#[test]
fn logic_fn_errors_when_body_missing() {
    let vsm = vsm_with_body("NavMachine", None, vec!["go"]);
    let result = generate_creusot_file(&vsm, Path::new("/repo"));
    assert!(
        result.is_err(),
        "expected Err when creusot_inv_body missing; got Ok"
    );
    let msg = result.unwrap_err();
    assert!(
        msg.contains("NavMachine"),
        "error message should name the machine; got:\n{msg}"
    );
}

#[test]
fn marker_proof_fn_emitted() {
    let vsm = vsm_with_body("ConnMachine", Some("true"), vec!["begin"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("verify_conn_consistent_prop_creusot"),
        "expected marker fn; got:\n{out}"
    );
    assert!(
        !out.contains("#[trusted]"),
        "marker proof should remain non-trusted; got:\n{out}"
    );
    assert!(out.contains("#[requires(true)]"), "expected requires(true)");
    assert!(
        out.contains("#[ensures(result)]"),
        "expected ensures(result)"
    );
}

#[test]
fn wrapper_emitted_per_transition() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go", "back"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("fn go_creusot"),
        "expected go wrapper; got:\n{out}"
    );
    assert!(
        out.contains("fn back_creusot"),
        "expected back wrapper; got:\n{out}"
    );
}

#[test]
fn wrapper_has_requires_and_ensures() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("#[requires(nav_consistent_creusot_logic(&state))]"),
        "expected requires; got:\n{out}"
    );
    assert!(
        out.contains("#[ensures(nav_consistent_creusot_logic(&result.0))]"),
        "expected ensures; got:\n{out}"
    );
}

#[test]
fn wrapper_body_calls_through() {
    let vsm = vsm_with_body("NavMachine", Some("true"), vec!["go"]);
    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("{ go("),
        "expected call-through body; got:\n{out}"
    );
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
    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("profile_name: String"),
        "expected String param; got:\n{out}"
    );
    assert!(
        out.contains("begin(state, proof, profile_name)"),
        "expected call with extra arg; got:\n{out}"
    );
}

#[test]
fn instrumented_transition_emits_local_creusot_clone() {
    let vsm = VsmDescriptor {
        machine: "ConnMachine".to_string(),
        transitions: vec!["begin".to_string()],
        invariant: Some(PropDescriptor {
            name: "ConnConsistent".to_string(),
            kani_fn: Some("conn_consistent".to_string()),
            verus_fn: Some("conn_consistent".to_string()),
            creusot_fn: Some("conn_consistent".to_string()),
            verus_inv_body: None,
            creusot_inv_body: Some("true".to_string()),
            verus_state_body: None,
        }),
        transition_fns: vec![TransitionFn {
            name: "begin".to_string(),
            args: vec![
                ArgDescriptor {
                    name: "_state".to_string(),
                    ty: "ConnState".to_string(),
                    kind: ArgKind::State,
                },
                ArgDescriptor {
                    name: "proof".to_string(),
                    ty: "Established<ConnConsistent>".to_string(),
                    kind: ArgKind::Proof {
                        inner: "ConnConsistent".to_string(),
                    },
                },
                ArgDescriptor {
                    name: "profile_name".to_string(),
                    ty: "String".to_string(),
                    kind: ArgKind::StringArg,
                },
            ],
            body: Some("begin_impl(state, proof, profile_name)".to_string()),
            has_instrument: true,
            creusot_body: Some(
                "{ begin_impl_creusot_local(state, proof, profile_name) }".to_string(),
            ),
            creusot_requires: Vec::new(),
            verus_class: None,
        }],
        source_file: PathBuf::from("src/vsm/thing.rs"),
    };

    let out = generate_creusot_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("fn begin_creusot_local(state: ConnState, proof: Established<ConnConsistent>, profile_name: String) -> (ConnState, Established<ConnConsistent>) { begin_impl_creusot_local(state, proof, profile_name) }"),
        "expected local tracing-free clone; got:\n{out}"
    );
    assert!(
        out.contains("pub fn begin_creusot(state: ConnState, proof: Established<ConnConsistent>, profile_name: String) -> (ConnState, Established<ConnConsistent>) { begin_creusot_local(state, proof, profile_name) }"),
        "expected wrapper to call local clone; got:\n{out}"
    );
}

#[test]
fn in_crate_creusot_uses_copied_body_not_extern_spec() {
    let mut vsm = vsm_with_transition_and_body(
        "ConnMachine",
        "begin",
        Some("true"),
        vec![ArgDescriptor {
            name: "profile_name".to_string(),
            ty: "String".to_string(),
            kind: ArgKind::StringArg,
        }],
    );
    vsm.transition_fns[0].has_instrument = true;
    let out =
        generate_creusot_file_with_style(&vsm, Path::new("/repo"), ImportStyle::InCrate).unwrap();
    assert!(
        out.contains("extern_spec!"),
        "current generator should still emit extern_spec; got:\n{out}"
    );
    assert!(
        out.contains("fn begin_creusot_local"),
        "expected local clone for instrumented transition; got:\n{out}"
    );
    assert!(
        out.contains("pub fn begin_creusot(state: ConnState, proof: Established<ConnConsistent>, profile_name: String) -> (ConnState, Established<ConnConsistent>) { begin_creusot_local(state, proof, profile_name) }"),
        "expected wrapper to call local clone; got:\n{out}"
    );
}

#[test]
fn in_crate_creusot_wraps_non_block_body() {
    let state = "CombatState".to_string();
    let consistent = "CombatConsistent".to_string();
    let vsm = VsmDescriptor {
        machine: "CombatMachine".to_string(),
        transitions: vec!["begin".to_string()],
        invariant: Some(PropDescriptor {
            name: consistent.clone(),
            kani_fn: Some("combat_consistent".to_string()),
            verus_fn: Some("combat_consistent".to_string()),
            creusot_fn: Some("combat_consistent".to_string()),
            verus_inv_body: None,
            creusot_inv_body: Some("true".to_string()),
            verus_state_body: None,
        }),
        transition_fns: vec![TransitionFn {
            name: "begin".to_string(),
            args: vec![
                ArgDescriptor {
                    name: "state".to_string(),
                    ty: state,
                    kind: ArgKind::State,
                },
                ArgDescriptor {
                    name: "proof".to_string(),
                    ty: format!("Established<{consistent}>"),
                    kind: ArgKind::Proof { inner: consistent },
                },
            ],
            body: Some("(state, proof)".to_string()),
            has_instrument: true,
            creusot_body: Some("(state, proof)".to_string()),
            creusot_requires: Vec::new(),
            verus_class: None,
        }],
        source_file: PathBuf::from("src/vsm/combat.rs"),
    };

    let out =
        generate_creusot_file_with_style(&vsm, Path::new("/repo"), ImportStyle::InCrate).unwrap();
    assert!(
        out.contains("fn begin_creusot_local(state: CombatState, proof: Established<CombatConsistent>) -> (CombatState, Established<CombatConsistent>) { (state, proof) }"),
        "expected local clone body to be wrapped in braces; got:\n{out}"
    );
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

    let out = generate_creusot_file(conn, &vsm_dir).unwrap();
    assert!(out.contains("AUTO-GENERATED"), "missing header");
    assert!(out.contains("#[logic]"), "missing #[logic]");
    assert!(
        out.contains("pub fn archive_connection_consistent"),
        "missing logic fn"
    );
    assert!(out.contains("{ true }"), "expected trivial body");
    assert!(
        out.contains("verify_archive_connection_consistent_prop_creusot"),
        "missing marker"
    );

    for t in &conn.transitions {
        assert!(
            out.contains(&format!("fn {t}_creusot")),
            "missing wrapper for transition {t}"
        );
        assert!(
            out.contains(&format!("{{ {t}(")) || out.contains(&format!("{{ {t}_creusot_local(")),
            "missing call-through for {t}"
        );
    }
}

#[test]
fn scan_and_generate_archive_connection_creusot_in_crate_imports() {
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

    let out = generate_creusot_file_with_style(conn, &vsm_dir, ImportStyle::InCrate).unwrap();
    assert!(
        out.contains("use crate::archive::vsm::{"),
        "expected in-crate imports; got:\n{out}"
    );
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
        inv.creusot_inv_body
            .as_ref()
            .map(|b| b.contains("pearlite!"))
            .unwrap_or(false),
        "nav body should use pearlite!"
    );

    let out = generate_creusot_file(nav, &vsm_dir).unwrap();
    assert!(
        out.contains("pub fn archive_nav_consistent"),
        "missing logic fn"
    );
    assert!(out.contains("pearlite!"), "expected pearlite body");
    assert!(
        out.contains("requires(archive_nav_consistent"),
        "missing requires"
    );
    assert!(
        out.contains("ensures(archive_nav_consistent"),
        "missing ensures"
    );
}
