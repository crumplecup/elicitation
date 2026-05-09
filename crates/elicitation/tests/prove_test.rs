//! Tests for `elicitation prove` configuration and dry-run behaviour.

use elicitation::cli::{prove::{ProveConfig, parse_kani_list_output}, ProveOpts};
use std::path::PathBuf;

fn base_opts() -> ProveOpts {
    ProveOpts {
        kani: false,
        verus: false,
        creusot: false,
        package: None,
        kani_harness: None,
        verus_path: None,
        verus_file: None,
        csv: None,
        resume: false,
        timeout: 300,
        dry_run: false,
    }
}

#[test]
fn no_backend_selected_is_error() {
    let cfg = ProveConfig::resolve(&base_opts()).unwrap();
    let err = elicitation::cli::prove::run(&cfg).unwrap_err();
    assert!(err.to_string().contains("No backend selected"));
}

#[test]
fn kani_dry_run_with_package_succeeds() {
    let opts = ProveOpts {
        kani: true,
        package: Some("test_proofs".to_string()),
        dry_run: true,
        ..base_opts()
    };
    let cfg = ProveConfig::resolve(&opts).unwrap();
    assert!(elicitation::cli::prove::run(&cfg).is_ok());
}

#[test]
fn kani_missing_package_is_error() {
    // Ensure env vars don't accidentally provide a package.
    unsafe {
        std::env::remove_var("KANI_PACKAGE");
        std::env::remove_var("PROVE_PACKAGE");
    }
    let opts = ProveOpts {
        kani: true,
        dry_run: true,
        ..base_opts()
    };
    let cfg = ProveConfig::resolve(&opts).unwrap();
    let err = elicitation::cli::prove::run(&cfg).unwrap_err();
    assert!(err.to_string().contains("No package") || err.to_string().contains("kani"));
}

#[test]
fn creusot_dry_run_with_package_succeeds() {
    let opts = ProveOpts {
        creusot: true,
        package: Some("test_proofs".to_string()),
        dry_run: true,
        ..base_opts()
    };
    let cfg = ProveConfig::resolve(&opts).unwrap();
    assert!(elicitation::cli::prove::run(&cfg).is_ok());
}

#[test]
fn verus_missing_file_is_error() {
    unsafe {
        std::env::remove_var("VERUS_FILE");
    }
    let fake_verus = tempfile::NamedTempFile::new().unwrap();
    let opts = ProveOpts {
        verus: true,
        verus_path: Some(fake_verus.path().to_path_buf()),
        dry_run: true, // dry_run; the path-exists check still runs in run_verus
        ..base_opts()
    };
    let cfg = ProveConfig::resolve(&opts).unwrap();
    let err = elicitation::cli::prove::run(&cfg).unwrap_err();
    assert!(err.to_string().contains("Verus source file") || err.to_string().contains("VERUS_FILE") || err.to_string().contains("verus"));
}

#[test]
fn config_cli_verus_path_overrides_env() {
    let custom = PathBuf::from("/custom/verus/bin");
    let opts = ProveOpts {
        verus: true,
        verus_path: Some(custom.clone()),
        ..base_opts()
    };
    let cfg = ProveConfig::resolve(&opts).unwrap();
    assert_eq!(cfg.verus_path, custom);
}

#[test]
fn multi_backend_dry_run_all_succeed() {
    let opts = ProveOpts {
        kani: true,
        creusot: true,
        package: Some("my_proofs".to_string()),
        dry_run: true,
        ..base_opts()
    };
    let cfg = ProveConfig::resolve(&opts).unwrap();
    // verus is not selected, so no file needed
    assert!(elicitation::cli::prove::run(&cfg).is_ok());
}

#[test]
fn parse_kani_list_regular_and_contract_harnesses() {
    let output = "\
|       | Crate     | Function                          | Contract Harnesses (#[kani::proof_for_contract]) |
|-------+-----------+-----------------------------------+--------------------------------------------------|
|       | my_proofs | my_proofs::contracts::fn_a        | my_proofs::contracts::fn_a                       |
|-------+-----------+-----------------------------------+--------------------------------------------------|
|       | my_proofs | my_proofs::tests::regular_harness |
|-------+-----------+-----------------------------------+
| Total |           | 2                                 |
";
    let harnesses = parse_kani_list_output(output).unwrap();
    assert!(harnesses.contains(&"my_proofs::contracts::fn_a".to_string()));
    assert!(harnesses.contains(&"my_proofs::tests::regular_harness".to_string()));
    assert_eq!(harnesses.len(), 2);
}

#[test]
fn config_csv_and_resume_defaults() {
    let opts = base_opts();
    let cfg = ProveConfig::resolve(&opts).unwrap();
    assert_eq!(cfg.kani_csv, std::path::PathBuf::from("kani_verification_results.csv"));
    assert!(!cfg.kani_resume);
    assert_eq!(cfg.timeout, 300);
}
