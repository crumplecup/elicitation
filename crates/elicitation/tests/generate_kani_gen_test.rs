//! Tests for `cli::generate::kani_gen` — validates generated Kani harness output.

#![cfg(feature = "cli")]

use elicitation::cli::generate::{
    ImportStyle, kani_gen::{generate_kani_file, generate_kani_file_with_style},
    scanner::{ArgDescriptor, ArgKind, PropDescriptor, TransitionFn, VsmDescriptor},
};
use std::path::{Path, PathBuf};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn minimal_vsm(machine: &str) -> VsmDescriptor {
    let inv_name = machine.replace("Machine", "Consistent");
    let inv_fn = to_snake(machine.trim_end_matches("Machine"));
    let inv_fn = format!("{inv_fn}_consistent");

    VsmDescriptor {
        machine: machine.to_string(),
        transitions: vec!["do_thing".to_string()],
        invariant: Some(PropDescriptor {
            name: inv_name.clone(),
            kani_fn: Some(inv_fn.clone()),
            verus_fn: Some(inv_fn.clone()),
            creusot_fn: None,
            verus_inv_body: None,
            creusot_inv_body: None,
            verus_state_body: None,
        }),
        transition_fns: vec![],
        source_file: PathBuf::from("src/vsm/thing.rs"),
    }
}

fn vsm_with_transition(
    machine: &str,
    transition: &str,
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
            name: consistent,
            kani_fn: Some(inv_fn.clone()),
            verus_fn: Some(inv_fn),
            creusot_fn: None,
            verus_inv_body: None,
            creusot_inv_body: None,
            verus_state_body: None,
        }),
        transition_fns: vec![TransitionFn {
            name: transition.to_string(),
            args,
            body: None,
            has_instrument: false,
            creusot_body: None,
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
    let vsm = minimal_vsm("NavMachine");
    let out = generate_kani_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("AUTO-GENERATED"),
        "expected AUTO-GENERATED header, got:\n{out}"
    );
    assert!(
        out.contains("NavMachine"),
        "expected machine name in header"
    );
}

#[test]
fn generated_file_has_marker_proof() {
    let vsm = minimal_vsm("NavMachine");
    let out = generate_kani_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("verify_nav_consistent_prop_marker"),
        "expected marker fn, got:\n{out}"
    );
    assert!(out.contains("#[kani::proof]"), "expected #[kani::proof]");
}

#[test]
fn generated_file_has_cfg_kani_guard() {
    let vsm = minimal_vsm("NavMachine");
    let out = generate_kani_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("#[cfg(kani)]"),
        "expected #[cfg(kani)] guard, got:\n{out}"
    );
}

#[test]
fn minimal_fallback_harness_emitted_without_transition_fns() {
    let vsm = minimal_vsm("NavMachine");
    // transition_fns is empty → should emit minimal harness
    let out = generate_kani_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("do_thing_kani_closure"),
        "expected closure fn, got:\n{out}"
    );
    assert!(
        out.contains("proof_for_contract(do_thing)"),
        "expected proof_for_contract attribute"
    );
    assert!(out.contains("kani_depth2"), "expected kani_depth2 shadow-1");
    assert!(out.contains("kani_depth0"), "expected kani_depth0 shadow-2");
    assert!(
        out.contains("kani_proof_credential"),
        "expected kani_proof_credential"
    );
}

#[test]
fn full_harness_emitted_with_known_transition_fn() {
    let vsm = vsm_with_transition("ConnMachine", "begin_connect", vec![]);
    let out = generate_kani_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("begin_connect_kani_closure"),
        "expected closure fn"
    );
    assert!(
        out.contains("proof_for_contract(begin_connect)"),
        "expected attribute"
    );
    // Both shadows
    assert!(out.contains("kani_depth2"), "expected depth2 shadow");
    assert!(out.contains("kani_depth0"), "expected depth0 shadow");
    // Credential
    assert!(out.contains("kani_proof_credential"), "expected credential");
    assert!(out.contains("ConnConsistent"), "expected consistent type");
    // Call
    assert!(out.contains("begin_connect("), "expected function call");
}

#[test]
fn extra_string_arg_emitted_correctly() {
    let vsm = vsm_with_transition(
        "ConnMachine",
        "set_name",
        vec![ArgDescriptor {
            name: "name".to_string(),
            ty: "String".to_string(),
            kind: ArgKind::StringArg,
        }],
    );
    let out = generate_kani_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("<::std::string::String as ::elicitation::KaniCompose>::kani_depth1()"),
        "expected kani_depth1() for StringArg, got:\n{out}"
    );
}

#[test]
fn extra_option_arg_emitted_correctly() {
    let vsm = vsm_with_transition(
        "NavMachine",
        "set_filter",
        vec![ArgDescriptor {
            name: "filter".to_string(),
            ty: "Option<String>".to_string(),
            kind: ArgKind::OptionArg,
        }],
    );
    let out = generate_kani_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("::core::option::Option::None"),
        "expected None for OptionArg, got:\n{out}"
    );
}

#[test]
fn extra_other_arg_emitted_with_kani_depth0() {
    let vsm = vsm_with_transition(
        "NavMachine",
        "load_tree",
        vec![ArgDescriptor {
            name: "nav".to_string(),
            ty: "NavTree".to_string(),
            kind: ArgKind::Other,
        }],
    );
    let out = generate_kani_file(&vsm, Path::new("/repo")).unwrap();
    assert!(
        out.contains("<NavTree as ::elicitation::KaniCompose>::kani_depth0()"),
        "expected kani_depth0 for Other arg, got:\n{out}"
    );
}

// ─── Snapshot test against archive_nav reference ──────────────────────────────

#[test]
fn scan_and_generate_archive_nav() {
    use elicitation::cli::generate::scan_vsms;

    let vsm_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("elicit_server/src/archive/vsm");

    if !vsm_dir.exists() {
        return; // not available in all envs
    }

    let vsms = scan_vsms(&vsm_dir);
    let nav = vsms
        .iter()
        .find(|v| v.machine == "ArchiveNavMachine")
        .expect("ArchiveNavMachine should be found");

    let out = generate_kani_file(nav, &vsm_dir).unwrap();

    // High-level structural checks — not a byte-for-byte diff.
    assert!(out.contains("AUTO-GENERATED"), "missing header");
    assert!(
        out.contains("verify_archive_nav_consistent_prop_marker"),
        "missing marker fn"
    );
    // Each transition should have a closure.
    for t in &nav.transitions {
        assert!(
            out.contains(&format!("{t}_kani_closure")),
            "missing harness for transition {t}"
        );
    }
    assert!(out.contains("kani_depth2"), "missing depth2 shadow");
    assert!(out.contains("kani_depth0"), "missing depth0 shadow");
    assert!(out.contains("kani_proof_credential"), "missing credential");
}

#[test]
fn scan_and_generate_archive_nav_in_crate_imports() {
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
        .expect("ArchiveNavMachine should be found");

    let out = generate_kani_file_with_style(nav, &vsm_dir, ImportStyle::InCrate).unwrap();
    assert!(
        out.contains("use crate::archive::vsm::{"),
        "expected in-crate imports; got:\n{out}"
    );
}
