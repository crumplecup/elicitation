//! Tests for the `cli::generate::scanner` module.

#![cfg(feature = "cli")]

use elicitation::cli::generate::scanner::{
    ArgKind, extract_prop_descriptor, extract_vsms_from_file,
};
use std::path::Path;
use syn::{File, Item};

// ─── Snippet tests ────────────────────────────────────────────────────────────

#[test]
fn parse_prop_descriptor_from_snippet() {
    let src = r#"
        #[derive(Prop)]
        #[prop(kani_invariant_fn = "my_kani_fn", verus_invariant_fn = "my_verus_fn")]
        pub struct MyConsistent;
    "#;
    let file: File = syn::parse_str(src).unwrap();
    let props: Vec<_> = file
        .items
        .iter()
        .filter_map(|i| {
            if let Item::Struct(s) = i {
                extract_prop_descriptor(s)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(props.len(), 1);
    assert_eq!(props[0].name, "MyConsistent");
    assert_eq!(props[0].kani_fn.as_deref(), Some("my_kani_fn"));
    assert_eq!(props[0].verus_fn.as_deref(), Some("my_verus_fn"));
    assert_eq!(props[0].creusot_fn, None);
}

#[test]
fn parse_vsm_descriptor_from_snippet() {
    let src = r#"
        #[derive(Prop)]
        #[prop(kani_invariant_fn = "nav_consistent")]
        pub struct NavConsistent;

        #[derive(VerifiedStateMachine)]
        #[vsm(transitions = [begin, advance, reset])]
        pub struct NavMachine;
    "#;
    let file: File = syn::parse_str(src).unwrap();
    let descs = extract_vsms_from_file(&file, Path::new("nav.rs"));
    assert_eq!(descs.len(), 1);
    let d = &descs[0];
    assert_eq!(d.machine, "NavMachine");
    assert_eq!(d.transitions, vec!["begin", "advance", "reset"]);
    let inv = d.invariant.as_ref().unwrap();
    assert_eq!(inv.kani_fn.as_deref(), Some("nav_consistent"));
}

#[test]
fn vsm_without_companion_is_ok() {
    let src = r#"
        #[derive(VerifiedStateMachine)]
        #[vsm(transitions = [start, stop])]
        pub struct OrphanMachine;
    "#;
    let file: File = syn::parse_str(src).unwrap();
    let descs = extract_vsms_from_file(&file, Path::new("orphan.rs"));
    assert_eq!(descs.len(), 1);
    assert!(descs[0].invariant.is_none());
}

#[test]
fn non_vsm_struct_ignored() {
    let src = r#"
        #[derive(Debug, Clone)]
        pub struct Foo { bar: u32 }
    "#;
    let file: File = syn::parse_str(src).unwrap();
    let descs = extract_vsms_from_file(&file, Path::new("foo.rs"));
    assert!(descs.is_empty());
}

#[test]
fn all_three_fn_names_parsed() {
    let src = r#"
        #[derive(Prop)]
        #[prop(
            kani_invariant_fn    = "k_fn",
            verus_invariant_fn   = "v_fn",
            creusot_invariant_fn = "c_fn",
        )]
        pub struct AllThreeConsistent;
    "#;
    let file: File = syn::parse_str(src).unwrap();
    let props: Vec<_> = file
        .items
        .iter()
        .filter_map(|i| {
            if let Item::Struct(s) = i {
                extract_prop_descriptor(s)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(props.len(), 1);
    assert_eq!(props[0].kani_fn.as_deref(), Some("k_fn"));
    assert_eq!(props[0].verus_fn.as_deref(), Some("v_fn"));
    assert_eq!(props[0].creusot_fn.as_deref(), Some("c_fn"));
}

// ─── Filesystem scan test ─────────────────────────────────────────────────────

#[test]
fn scan_archive_vsms() {
    use elicitation::cli::generate::scan_vsms;
    use std::path::PathBuf;

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("elicit_server/src/archive/vsm");

    if !root.exists() {
        // Skip in environments without the full workspace checked out.
        return;
    }

    let vsms = scan_vsms(&root);
    // Expect exactly 4 archive VSMs.
    let mut names: Vec<&str> = vsms.iter().map(|v| v.machine.as_str()).collect();
    names.sort_unstable();
    assert_eq!(vsms.len(), 4, "expected 4 archive VSMs, got: {names:?}");

    let expected = [
        "ArchiveConnectionMachine",
        "ArchiveNavMachine",
        "ArchiveOverlayMachine",
        "ArchivePanelMachine",
    ];
    for name in &expected {
        assert!(
            names.contains(name),
            "VSM '{name}' not found; got {names:?}"
        );
    }

    // Check that transition_fns are populated (signatures found in source).
    for vsm in &vsms {
        assert!(
            !vsm.transition_fns.is_empty(),
            "{} has no transition_fns — signatures not found",
            vsm.machine
        );
        // Every name in transitions should have a matching TransitionFn.
        for t_name in &vsm.transitions {
            assert!(
                vsm.transition_fns.iter().any(|tf| &tf.name == t_name),
                "transition '{t_name}' missing from transition_fns in {}",
                vsm.machine
            );
        }
    }
}

// ─── TransitionFn tests ───────────────────────────────────────────────────────

#[test]
fn transition_fn_arg_classification() {
    let src = r#"
        pub fn nav_loaded(
            _state: NavState,
            proof: Established<NavConsistent>,
            nav: NavTree,
        ) -> (NavState, Established<NavConsistent>) {
            (_state, proof)
        }

        #[derive(VerifiedStateMachine)]
        #[vsm(transitions = [nav_loaded])]
        pub struct NavMachine;
    "#;
    let file: File = syn::parse_str(src).unwrap();
    let descs = extract_vsms_from_file(&file, Path::new("nav.rs"));
    assert_eq!(descs.len(), 1);

    let tfns = &descs[0].transition_fns;
    assert_eq!(tfns.len(), 1);

    let tf = &tfns[0];
    assert_eq!(tf.name, "nav_loaded");

    let state = tf.state_arg().expect("state arg");
    assert_eq!(state.kind, ArgKind::State);
    assert_eq!(state.ty, "NavState");

    let proof = tf.proof_arg().expect("proof arg");
    assert!(matches!(&proof.kind, ArgKind::Proof { inner } if inner == "NavConsistent"));

    let extra: Vec<_> = tf.extra_args().collect();
    assert_eq!(extra.len(), 1);
    assert_eq!(extra[0].name, "nav");
    assert_eq!(extra[0].kind, ArgKind::Other);
}

#[test]
fn transition_fn_string_and_option_args() {
    let src = r#"
        pub fn connect(
            _state: ConnState,
            proof: Established<ConnConsistent>,
            name: String,
            tag: Option<String>,
        ) -> (ConnState, Established<ConnConsistent>) {
            (_state, proof)
        }

        #[derive(VerifiedStateMachine)]
        #[vsm(transitions = [connect])]
        pub struct ConnMachine;
    "#;
    let file: File = syn::parse_str(src).unwrap();
    let descs = extract_vsms_from_file(&file, Path::new("conn.rs"));
    let tf = &descs[0].transition_fns[0];

    let extra: Vec<_> = tf.extra_args().collect();
    assert_eq!(extra.len(), 2);
    assert_eq!(extra[0].kind, ArgKind::StringArg);
    assert_eq!(extra[1].kind, ArgKind::OptionArg);
}
