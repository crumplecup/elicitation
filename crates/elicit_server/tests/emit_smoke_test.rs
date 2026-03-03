//! Smoke test: emit a `status_summary` binary and run it.
//!
//! This test exercises the full code recovery pipeline end-to-end without an
//! agent in the loop:
//!
//! 1. Deserialize tool params via `dispatch_reqwest_emit`
//! 2. Build a `BinaryScaffold` with the `ELICIT_WORKSPACE_ROOT` env var so the
//!    generated `Cargo.toml` uses path deps (pre-publish dev builds)
//! 3. Write `src/main.rs` + `Cargo.toml` to a temp dir
//! 4. `cargo run` the generated project and assert it succeeds

#[cfg(feature = "emit")]
mod smoke {
    use elicitation::emit_code::BinaryScaffold;
    use std::path::PathBuf;

    fn workspace_root() -> PathBuf {
        // crates/elicit_server → up two levels = workspace root
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn emit_status_summary_and_run() {
        // ── 1. Dispatch params ──────────────────────────────────────────────
        let params = serde_json::json!({ "status": 200 });
        let step =
            elicit_reqwest::dispatch_reqwest_emit("status_summary", params).expect("dispatch");

        // ── 2. Build scaffold with workspace path override ──────────────────
        let scaffold = BinaryScaffold::new(vec![step], false)
            .with_workspace_root(workspace_root());

        // ── 3. Write to temp dir ────────────────────────────────────────────
        let out_dir = std::env::temp_dir().join("elicit_smoke_status_summary");
        let _ = std::fs::remove_dir_all(&out_dir); // clean slate
        let main_rs = scaffold
            .emit_to_disk(&out_dir, "smoke_status_summary")
            .expect("emit_to_disk");
        assert!(main_rs.exists(), "main.rs should exist at {}", main_rs.display());

        // Spot-check the generated Cargo.toml uses path deps, not crates.io
        let cargo_toml = std::fs::read_to_string(out_dir.join("Cargo.toml")).unwrap();
        assert!(
            cargo_toml.contains("path ="),
            "Cargo.toml should contain path deps:\n{cargo_toml}"
        );

        // ── 4. cargo run ────────────────────────────────────────────────────
        let status = std::process::Command::new("cargo")
            .args(["run", "--quiet"])
            .current_dir(&out_dir)
            .status()
            .expect("cargo run");
        assert!(status.success(), "Generated binary should run successfully");
    }
}
