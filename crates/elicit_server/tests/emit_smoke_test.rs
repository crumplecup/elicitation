//! Smoke tests for the emit code recovery pipeline.
//!
//! Strategy:
//! - All generated projects share `CARGO_TARGET_DIR` pointing at the workspace
//!   `target/` — warm cache from the main build, so only the first test pays
//!   full compile cost; subsequent tests are near-instant.
//! - Output dirs live under `target/emit_tests/<tool>/` (stable paths, reused
//!   across runs).
//! - **Pure tools** (no network I/O): emit → `cargo run` — full end-to-end.
//! - **Network tools**: emit → `cargo build` — proves the generated code
//!   compiles correctly without requiring a live server.

#[cfg(feature = "emit")]
mod smoke {
    use elicit_server;
    use elicitation::emit_code::BinaryScaffold;
    use std::path::PathBuf;
    use std::process::Command;

    // ── Shared infrastructure ─────────────────────────────────────────────────

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    /// Returns `(out_dir, target_dir)` for a given tool name.
    ///
    /// All generated projects share the same `target_dir` (the workspace
    /// `target/`) so compiled artifacts are reused across tests.
    fn emit_test_paths(tool_name: &str) -> (PathBuf, PathBuf) {
        let ws = workspace_root();
        let out_dir = ws.join("target/emit_tests").join(tool_name);
        let target_dir = ws.join("target");
        (out_dir, target_dir)
    }

    /// Emit a single-step scaffold to disk and return `(out_dir, target_dir)`.
    fn emit_step(
        tool_name: &str,
        crate_name: &str,
        params: serde_json::Value,
    ) -> (PathBuf, PathBuf) {
        let step = match crate_name {
            "reqwest" => elicit_reqwest::dispatch_reqwest_emit(tool_name, params)
                .unwrap_or_else(|e| panic!("dispatch_reqwest_emit({tool_name}): {e}")),
            "serde_json" => elicit_serde_json::dispatch_serde_json_emit(tool_name, params)
                .unwrap_or_else(|e| panic!("dispatch_serde_json_emit({tool_name}): {e}")),
            "url" => elicit_url::dispatch_url_emit(tool_name, params)
                .unwrap_or_else(|e| panic!("dispatch_url_emit({tool_name}): {e}")),
            "chrono" => elicit_chrono::dispatch_chrono_emit(tool_name, params)
                .unwrap_or_else(|e| panic!("dispatch_chrono_emit({tool_name}): {e}")),
            "jiff" => elicit_jiff::dispatch_jiff_emit(tool_name, params)
                .unwrap_or_else(|e| panic!("dispatch_jiff_emit({tool_name}): {e}")),
            "time" => elicit_time::dispatch_time_emit(tool_name, params)
                .unwrap_or_else(|e| panic!("dispatch_time_emit({tool_name}): {e}")),
            "secure_fetch" => elicit_server::dispatch_secure_fetch_emit(tool_name, params)
                .unwrap_or_else(|e| panic!("dispatch_secure_fetch_emit({tool_name}): {e}")),
            "fetch_and_parse" => elicit_server::dispatch_fetch_and_parse_emit(tool_name, params)
                .unwrap_or_else(|e| panic!("dispatch_fetch_and_parse_emit({tool_name}): {e}")),
            other => panic!("Unknown crate: {other}"),
        };

        let scaffold = BinaryScaffold::new(vec![step], false).with_workspace_root(workspace_root());

        // Use "{crate_name}_{tool_name}" so tools with the same name in different crates
        // (e.g. "compute_duration" in both elicit_chrono and elicit_time) don't collide.
        let unique_name = format!("{crate_name}_{tool_name}");
        let pkg_name = unique_name.replace('-', "_");
        let (out_dir, target_dir) = emit_test_paths(&unique_name);
        scaffold
            .emit_to_disk(&out_dir, &pkg_name)
            .unwrap_or_else(|e| panic!("emit_to_disk({tool_name}): {e}"));

        // Verify Cargo.toml was generated and contains the package name.
        // (Path deps appear when workspace crates are used; tools that only depend on
        // crates.io packages won't have path deps, but still compile correctly.)
        let cargo_toml = std::fs::read_to_string(out_dir.join("Cargo.toml"))
            .unwrap_or_else(|e| panic!("Cargo.toml missing for {tool_name}: {e}"));
        assert!(
            cargo_toml.contains(&pkg_name),
            "Cargo.toml for {tool_name} is malformed:\n{cargo_toml}"
        );

        (out_dir, target_dir)
    }

    /// Emit and `cargo run` — for pure (no-network) tools.
    fn assert_runs(tool_name: &str, crate_name: &str, params: serde_json::Value) {
        let (out_dir, target_dir) = emit_step(tool_name, crate_name, params);
        let status = Command::new("cargo")
            .args(["run", "--quiet"])
            .env("CARGO_TARGET_DIR", &target_dir)
            .current_dir(&out_dir)
            .status()
            .unwrap_or_else(|e| panic!("cargo run ({tool_name}): {e}"));
        assert!(status.success(), "cargo run failed for {tool_name}");
    }

    /// Emit and `cargo build` — for network tools (compilation proof only).
    fn assert_builds(tool_name: &str, crate_name: &str, params: serde_json::Value) {
        let (out_dir, target_dir) = emit_step(tool_name, crate_name, params);
        let output = Command::new("cargo")
            .args(["build", "--quiet"])
            .env("CARGO_TARGET_DIR", &target_dir)
            .current_dir(&out_dir)
            .output()
            .unwrap_or_else(|e| panic!("cargo build ({tool_name}): {e}"));
        assert!(
            output.status.success(),
            "cargo build failed for {tool_name}:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // ── elicit_reqwest — pure tools ───────────────────────────────────────────

    #[test]
    fn emit_status_summary_and_run() {
        assert_runs(
            "status_summary",
            "reqwest",
            serde_json::json!({ "status": 200 }),
        );
    }

    #[test]
    fn emit_url_build_and_run() {
        assert_runs(
            "url_build",
            "reqwest",
            serde_json::json!({
                "base": "https://api.example.com",
                "path": "/v1/users",
                "query": { "page": "1", "limit": "20" }
            }),
        );
    }

    // ── elicit_reqwest — network tools (build only) ───────────────────────────

    #[test]
    fn emit_fetch_builds() {
        assert_builds(
            "fetch",
            "reqwest",
            serde_json::json!({
                "url": "https://httpbin.org/get",
                "timeout_secs": 30.0
            }),
        );
    }

    #[test]
    fn emit_auth_fetch_builds() {
        assert_builds(
            "auth_fetch",
            "reqwest",
            serde_json::json!({
                "url": "https://api.example.com/protected",
                "token": "test-token",
                "auth_type": "bearer"
            }),
        );
    }

    #[test]
    fn emit_post_builds() {
        assert_builds(
            "post",
            "reqwest",
            serde_json::json!({
                "url": "https://httpbin.org/post",
                "body": r#"{"key": "value"}"#,
                "content_type": "json"
            }),
        );
    }

    #[test]
    fn emit_api_call_builds() {
        assert_builds(
            "api_call",
            "reqwest",
            serde_json::json!({
                "url": "https://api.example.com/data",
                "token": "test-bearer-token",
                "body": r#"{"query": "test"}"#
            }),
        );
    }

    #[test]
    fn emit_health_check_builds() {
        assert_builds(
            "health_check",
            "reqwest",
            serde_json::json!({
                "url": "https://api.example.com/health",
                "timeout_secs": 10.0
            }),
        );
    }

    #[test]
    fn emit_build_request_builds() {
        assert_builds(
            "build_request",
            "reqwest",
            serde_json::json!({
                "method": "POST",
                "url": "https://api.example.com/submit",
                "auth_type": "bearer",
                "token": "my-token",
                "body": r#"{"data": 42}"#,
                "content_type": "json"
            }),
        );
    }

    #[test]
    fn emit_paginated_get_builds() {
        assert_builds(
            "paginated_get",
            "reqwest",
            serde_json::json!({
                "url": "https://api.example.com/items",
                "token": "my-token"
            }),
        );
    }

    // ── elicit_serde_json — all pure ──────────────────────────────────────────

    #[test]
    fn emit_parse_and_focus_and_run() {
        assert_runs(
            "parse_and_focus",
            "serde_json",
            serde_json::json!({
                "json": r#"{"user": {"name": "Alice", "age": 30}}"#,
                "pointer": "/user/name"
            }),
        );
    }

    #[test]
    fn emit_validate_object_and_run() {
        assert_runs(
            "validate_object",
            "serde_json",
            serde_json::json!({
                "json": r#"{"id": 1, "name": "Alice", "email": "alice@example.com"}"#,
                "required_keys": ["id", "name", "email"]
            }),
        );
    }

    #[test]
    fn emit_safe_merge_and_run() {
        assert_runs(
            "safe_merge",
            "serde_json",
            serde_json::json!({
                "base": {"name": "Alice", "role": "user"},
                "patch": {"role": "admin", "verified": true},
                "mode": "merge_patch"
            }),
        );
    }

    #[test]
    fn emit_pointer_update_and_run() {
        assert_runs(
            "pointer_update",
            "serde_json",
            serde_json::json!({
                "json": r#"{"user": {"status": "pending"}}"#,
                "pointer": "/user/status",
                "new_value": "active",
                "missing_key_policy": "error"
            }),
        );
    }

    #[test]
    fn emit_field_chain_and_run() {
        assert_runs(
            "field_chain",
            "serde_json",
            serde_json::json!({
                "json": r#"{"org": {"team": {"lead": "Bob"}}}"#,
                "path": ["org", "team", "lead"]
            }),
        );
    }

    // ── Multi-step composition ────────────────────────────────────────────────

    /// Compose url_build + status_summary in a single binary — the core
    /// "code recovery" use case where an agent chains two verified tools.
    #[test]
    fn emit_multi_step_composition_and_run() {
        let step1 = elicit_reqwest::dispatch_reqwest_emit(
            "url_build",
            serde_json::json!({
                "base": "https://api.example.com",
                "path": "/health"
            }),
        )
        .expect("dispatch url_build");

        let step2 = elicit_reqwest::dispatch_reqwest_emit(
            "status_summary",
            serde_json::json!({ "status": 404 }),
        )
        .expect("dispatch status_summary");

        let ws = workspace_root();
        let scaffold = BinaryScaffold::new(vec![step1, step2], false).with_workspace_root(&ws);

        let (out_dir, target_dir) = emit_test_paths("multi_step");
        scaffold
            .emit_to_disk(&out_dir, "multi_step")
            .expect("emit_to_disk");

        let status = Command::new("cargo")
            .args(["run", "--quiet"])
            .env("CARGO_TARGET_DIR", &target_dir)
            .current_dir(&out_dir)
            .status()
            .expect("cargo run");
        assert!(status.success(), "Multi-step composition failed");
    }

    // ── elicit_url ────────────────────────────────────────────────────────────

    #[test]
    fn emit_parse_url_and_run() {
        assert_runs(
            "parse_url",
            "url",
            serde_json::json!({ "url": "https://api.example.com/v1/users?page=1" }),
        );
    }

    #[test]
    fn emit_assert_https_and_run() {
        assert_runs(
            "assert_https",
            "url",
            serde_json::json!({ "url": "https://api.example.com/secure" }),
        );
    }

    #[test]
    fn emit_build_url_and_run() {
        assert_runs(
            "build_url",
            "url",
            serde_json::json!({
                "base": "https://api.example.com",
                "path": "/v2/items",
                "query": { "limit": "50", "offset": "0" }
            }),
        );
    }

    #[test]
    fn emit_join_url_and_run() {
        assert_runs(
            "join_url",
            "url",
            serde_json::json!({
                "base": "https://api.example.com/v1/",
                "relative": "users/42"
            }),
        );
    }

    // ── elicit_chrono ─────────────────────────────────────────────────────────

    #[test]
    fn emit_parse_datetime_and_run() {
        assert_runs(
            "parse_datetime",
            "chrono",
            serde_json::json!({ "datetime": "2099-06-15T12:00:00Z" }),
        );
    }

    #[test]
    fn emit_assert_future_chrono_and_run() {
        assert_runs(
            "assert_future",
            "chrono",
            serde_json::json!({ "datetime": "2099-06-15T12:00:00Z" }),
        );
    }

    #[test]
    fn emit_assert_in_range_and_run() {
        assert_runs(
            "assert_in_range",
            "chrono",
            serde_json::json!({
                "datetime": "2099-06-15T12:00:00Z",
                "start": "2099-01-01T00:00:00Z",
                "end": "2099-12-31T23:59:59Z"
            }),
        );
    }

    #[test]
    fn emit_compute_duration_chrono_and_run() {
        assert_runs(
            "compute_duration",
            "chrono",
            serde_json::json!({
                "from": "2024-01-01T00:00:00Z",
                "to": "2024-06-01T00:00:00Z"
            }),
        );
    }

    #[test]
    fn emit_add_seconds_chrono_and_run() {
        assert_runs(
            "add_seconds",
            "chrono",
            serde_json::json!({
                "datetime": "2099-06-15T12:00:00Z",
                "seconds": 3600
            }),
        );
    }

    // ── elicit_jiff ───────────────────────────────────────────────────────────

    #[test]
    fn emit_parse_timestamp_and_run() {
        assert_runs(
            "parse_timestamp",
            "jiff",
            serde_json::json!({ "timestamp": "2099-06-15T12:00:00Z" }),
        );
    }

    #[test]
    fn emit_parse_zoned_and_run() {
        assert_runs(
            "parse_zoned",
            "jiff",
            serde_json::json!({ "zoned": "2099-06-15T12:00:00[America/New_York]" }),
        );
    }

    #[test]
    fn emit_assert_future_jiff_and_run() {
        assert_runs(
            "assert_future",
            "jiff",
            serde_json::json!({ "timestamp": "2099-06-15T12:00:00Z" }),
        );
    }

    #[test]
    fn emit_convert_tz_and_run() {
        assert_runs(
            "convert_tz",
            "jiff",
            serde_json::json!({
                "zoned": "2099-06-15T12:00:00[America/New_York]",
                "timezone": "UTC"
            }),
        );
    }

    #[test]
    fn emit_compute_span_and_run() {
        assert_runs(
            "compute_span",
            "jiff",
            serde_json::json!({
                "from": "2024-01-01T00:00:00Z",
                "to": "2024-06-01T00:00:00Z"
            }),
        );
    }

    // ── elicit_time ───────────────────────────────────────────────────────────

    #[test]
    fn emit_parse_offset_datetime_and_run() {
        assert_runs(
            "parse_offset_datetime",
            "time",
            serde_json::json!({ "datetime": "2099-06-15T12:00:00Z" }),
        );
    }

    #[test]
    fn emit_parse_primitive_datetime_and_run() {
        assert_runs(
            "parse_primitive_datetime",
            "time",
            serde_json::json!({ "datetime": "2099-06-15T12:00:00" }),
        );
    }

    #[test]
    fn emit_assert_future_time_and_run() {
        assert_runs(
            "assert_future",
            "time",
            serde_json::json!({ "datetime": "2099-06-15T12:00:00Z" }),
        );
    }

    #[test]
    fn emit_compute_duration_time_and_run() {
        assert_runs(
            "compute_duration",
            "time",
            serde_json::json!({
                "from": "2024-01-01T00:00:00Z",
                "to": "2024-06-01T00:00:00Z"
            }),
        );
    }

    #[test]
    fn emit_add_seconds_time_and_run() {
        assert_runs(
            "add_seconds",
            "time",
            serde_json::json!({
                "datetime": "2099-06-15T12:00:00Z",
                "seconds": 3600
            }),
        );
    }

    // ── Cross-crate: secure_fetch (build only — network) ─────────────────────

    #[test]
    fn emit_secure_fetch_builds() {
        assert_builds(
            "secure_fetch",
            "secure_fetch",
            serde_json::json!({
                "url": "https://httpbin.org/get",
                "timeout_secs": 30.0
            }),
        );
    }

    #[test]
    fn emit_validated_api_call_builds() {
        assert_builds(
            "validated_api_call",
            "secure_fetch",
            serde_json::json!({
                "url": "https://api.example.com/data",
                "token": "test-token",
                "method": "GET"
            }),
        );
    }

    // ── Cross-crate: fetch_and_parse (build only — network) ──────────────────

    #[test]
    fn emit_fetch_and_extract_builds() {
        assert_builds(
            "fetch_and_extract",
            "fetch_and_parse",
            serde_json::json!({
                "url": "https://httpbin.org/json",
                "pointer": "/slideshow/title"
            }),
        );
    }

    #[test]
    fn emit_fetch_and_validate_builds() {
        assert_builds(
            "fetch_and_validate",
            "fetch_and_parse",
            serde_json::json!({
                "url": "https://httpbin.org/json",
                "required_keys": ["slideshow"]
            }),
        );
    }

    // ── Cross-crate multi-step: parse_url + secure_fetch ─────────────────────

    #[test]
    fn emit_parse_url_then_secure_fetch_builds() {
        let step1 = elicit_url::dispatch_url_emit(
            "parse_url",
            serde_json::json!({ "url": "https://api.example.com/health" }),
        )
        .expect("dispatch parse_url");

        let step2 = elicit_server::dispatch_secure_fetch_emit(
            "secure_fetch",
            serde_json::json!({
                "url": "https://api.example.com/health",
                "timeout_secs": 30.0
            }),
        )
        .expect("dispatch secure_fetch");

        let ws = workspace_root();
        let scaffold = BinaryScaffold::new(vec![step1, step2], false).with_workspace_root(&ws);

        let (out_dir, target_dir) = emit_test_paths("parse_url_then_secure_fetch");
        scaffold
            .emit_to_disk(&out_dir, "parse_url_then_secure_fetch")
            .expect("emit_to_disk");

        let output = Command::new("cargo")
            .args(["build", "--quiet"])
            .env("CARGO_TARGET_DIR", &target_dir)
            .current_dir(&out_dir)
            .output()
            .expect("cargo build");
        assert!(
            output.status.success(),
            "parse_url + secure_fetch cross-crate build failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
