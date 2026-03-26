//! Integration tests for `elicit_std` emit-only macro tools.

use elicit_std::{AssembleParams, ConcatParams, EnvParams, FormatParams, IncludeStrParams};
use elicitation::emit_code::{EmitCode, dispatch_emit_from};
use schemars::schema_for;
use serde_json::json;

// ── FormatParams ─────────────────────────────────────────────────────────────

#[test]
fn format_no_args_emits_bare_format() {
    let p = FormatParams {
        template: "Hello, world!".into(),
        args: vec![],
    };
    let src = p.emit_code().to_string();
    assert!(
        src.contains("format !"),
        "expected format! macro, got: {src}"
    );
    assert!(
        src.contains("Hello, world!"),
        "expected template in output: {src}"
    );
}

#[test]
fn format_with_args_emits_interpolated_format() {
    let p = FormatParams {
        template: "Hello, {}!".into(),
        args: vec!["name".into()],
    };
    let src = p.emit_code().to_string();
    assert!(src.contains("format !"), "expected format! macro: {src}");
    assert!(src.contains("name"), "expected arg in output: {src}");
}

#[test]
fn format_multiple_args() {
    let p = FormatParams {
        template: "{} + {} = {}".into(),
        args: vec!["a".into(), "b".into(), "c".into()],
    };
    let src = p.emit_code().to_string();
    assert!(
        src.contains("a") && src.contains("b") && src.contains("c"),
        "expected all args in output: {src}"
    );
}

#[test]
fn format_json_schema_is_valid() {
    let _schema = schema_for!(FormatParams);
}

#[test]
fn format_serde_roundtrip() {
    let p = FormatParams {
        template: "test {}".into(),
        args: vec!["x".into()],
    };
    let json = serde_json::to_string(&p).unwrap();
    let p2: FormatParams = serde_json::from_str(&json).unwrap();
    assert_eq!(p.template, p2.template);
    assert_eq!(p.args, p2.args);
}

// ── IncludeStrParams ─────────────────────────────────────────────────────────

#[test]
fn include_str_emits_correct_macro() {
    let p = IncludeStrParams {
        path: "data/config.toml".into(),
    };
    let src = p.emit_code().to_string();
    assert!(
        src.contains("include_str !"),
        "expected include_str! macro: {src}"
    );
    assert!(
        src.contains("data/config.toml"),
        "expected path in output: {src}"
    );
}

#[test]
fn include_str_json_schema_is_valid() {
    let _schema = schema_for!(IncludeStrParams);
}

#[test]
fn include_str_serde_roundtrip() {
    let p = IncludeStrParams {
        path: "../assets/schema.json".into(),
    };
    let json = serde_json::to_string(&p).unwrap();
    let p2: IncludeStrParams = serde_json::from_str(&json).unwrap();
    assert_eq!(p.path, p2.path);
}

// ── EnvParams ────────────────────────────────────────────────────────────────

#[test]
fn env_no_message_emits_bare_env() {
    let p = EnvParams {
        var: "DATABASE_URL".into(),
        error_message: None,
    };
    let src = p.emit_code().to_string();
    assert!(src.contains("env !"), "expected env! macro: {src}");
    assert!(
        src.contains("DATABASE_URL"),
        "expected var name in output: {src}"
    );
}

#[test]
fn env_with_message_emits_two_arg_env() {
    let p = EnvParams {
        var: "SECRET_KEY".into(),
        error_message: Some("SECRET_KEY must be set".into()),
    };
    let src = p.emit_code().to_string();
    assert!(src.contains("env !"), "expected env! macro: {src}");
    assert!(src.contains("SECRET_KEY"), "expected var name: {src}");
    assert!(
        src.contains("SECRET_KEY must be set"),
        "expected error message: {src}"
    );
}

#[test]
fn env_json_schema_is_valid() {
    let _schema = schema_for!(EnvParams);
}

#[test]
fn env_default_no_error_message() {
    // error_message defaults to None via #[serde(default)]
    let p: EnvParams = serde_json::from_value(json!({ "var": "FOO" })).unwrap();
    assert!(p.error_message.is_none());
}

// ── ConcatParams ─────────────────────────────────────────────────────────────

#[test]
fn concat_emits_correct_macro() {
    let p = ConcatParams {
        parts: vec!["Hello".into(), ", ".into(), "world".into(), "!".into()],
    };
    let src = p.emit_code().to_string();
    assert!(src.contains("concat !"), "expected concat! macro: {src}");
    assert!(src.contains("Hello"), "expected parts in output: {src}");
}

#[test]
fn concat_empty_parts() {
    let p = ConcatParams { parts: vec![] };
    let src = p.emit_code().to_string();
    assert!(src.contains("concat !"), "expected concat! macro: {src}");
}

#[test]
fn concat_json_schema_is_valid() {
    let _schema = schema_for!(ConcatParams);
}

#[test]
fn concat_serde_roundtrip() {
    let p = ConcatParams {
        parts: vec!["a".into(), "b".into()],
    };
    let json = serde_json::to_string(&p).unwrap();
    let p2: ConcatParams = serde_json::from_str(&json).unwrap();
    assert_eq!(p.parts, p2.parts);
}

// ── EmitEntry dispatch ───────────────────────────────────────────────────────

#[test]
fn dispatch_emit_format() {
    let params = json!({ "template": "Hello, {}!", "args": ["world"] });
    let emitter = dispatch_emit_from("format", "elicit_std", params).unwrap();
    let src = emitter.emit_code().to_string();
    assert!(src.contains("format !"), "expected format! macro: {src}");
}

#[test]
fn dispatch_emit_include_str() {
    let params = json!({ "path": "data/config.toml" });
    let emitter = dispatch_emit_from("include_str", "elicit_std", params).unwrap();
    let src = emitter.emit_code().to_string();
    assert!(
        src.contains("include_str !"),
        "expected include_str! macro: {src}"
    );
}

#[test]
fn dispatch_emit_env() {
    let params = json!({ "var": "DATABASE_URL" });
    let emitter = dispatch_emit_from("env", "elicit_std", params).unwrap();
    let src = emitter.emit_code().to_string();
    assert!(src.contains("env !"), "expected env! macro: {src}");
}

#[test]
fn dispatch_emit_concat() {
    let params = json!({ "parts": ["hello", " ", "world"] });
    let emitter = dispatch_emit_from("concat", "elicit_std", params).unwrap();
    let src = emitter.emit_code().to_string();
    assert!(src.contains("concat !"), "expected concat! macro: {src}");
}

#[test]
fn dispatch_emit_unknown_tool_errors() {
    let result = dispatch_emit_from("nonexistent", "elicit_std", json!({}));
    assert!(result.is_err(), "expected error for unknown tool");
}

#[test]
fn dispatch_emit_bad_params_errors() {
    // FormatParams requires "template" field
    let result = dispatch_emit_from("format", "elicit_std", json!({}));
    assert!(result.is_err(), "expected error for missing required field");
}

// ── Fragment composition ──────────────────────────────────────────────────────

#[test]
fn format_accepts_fragment_string_as_arg() {
    // Simulate: agent calls std__env → gets env!("USER"), passes as arg to std__format
    let env_fragment = EnvParams {
        var: "USER".into(),
        error_message: None,
    }
    .emit_code()
    .to_string();
    // env_fragment is something like: env ! ("USER")

    let p = FormatParams {
        template: "Hello, {}!".into(),
        args: vec![env_fragment],
    };
    let src = p.emit_code().to_string();
    assert!(src.contains("format !"), "expected format! macro: {src}");
    assert!(
        src.contains("env !"),
        "expected nested env! in output: {src}"
    );
}

// ── AssembleParams ────────────────────────────────────────────────────────────

#[test]
fn assemble_single_step_produces_valid_source() {
    let p = AssembleParams {
        steps: vec!["let x = 42i32 ;".into()],
        with_tracing: false,
        package_name: "test-output".into(),
    };
    let out = p.assemble().expect("assemble should succeed");
    assert!(
        out.main_rs.contains("async fn main"),
        "expected main fn: {}",
        out.main_rs
    );
    assert!(
        out.main_rs.contains("42"),
        "expected step content: {}",
        out.main_rs
    );
}

#[test]
fn assemble_multiple_steps_in_order() {
    let p = AssembleParams {
        steps: vec![
            "let a = 1i32 ;".into(),
            "let b = 2i32 ;".into(),
            "let _c = a + b ;".into(),
        ],
        with_tracing: false,
        package_name: "multi-step".into(),
    };
    let out = p.assemble().expect("assemble should succeed");
    assert!(
        out.main_rs.contains("let a"),
        "expected step a: {}",
        out.main_rs
    );
    assert!(
        out.main_rs.contains("let b"),
        "expected step b: {}",
        out.main_rs
    );
    assert!(
        out.main_rs.contains("_c"),
        "expected step c: {}",
        out.main_rs
    );
}

#[test]
fn assemble_with_tracing_includes_init() {
    let p = AssembleParams {
        steps: vec!["let _ = () ;".into()],
        with_tracing: true,
        package_name: "traced".into(),
    };
    let out = p.assemble().expect("assemble should succeed");
    assert!(
        out.main_rs.contains("tracing_subscriber"),
        "expected tracing init: {}",
        out.main_rs
    );
}

#[test]
fn assemble_cargo_toml_includes_package_name() {
    let p = AssembleParams {
        steps: vec!["let _ = () ;".into()],
        with_tracing: false,
        package_name: "my-cool-binary".into(),
    };
    let out = p.assemble().expect("assemble should succeed");
    assert!(
        out.cargo_toml.contains("my-cool-binary"),
        "expected package name: {}",
        out.cargo_toml
    );
    assert!(
        out.cargo_toml.contains("tokio"),
        "expected tokio dep: {}",
        out.cargo_toml
    );
}

#[test]
fn assemble_default_package_name() {
    let p: AssembleParams = serde_json::from_value(json!({ "steps": ["let _ = () ;"] })).unwrap();
    assert_eq!(p.package_name, "elicit-output");
    assert!(!p.with_tracing);
}

#[test]
fn assemble_empty_steps_produces_empty_main() {
    let p = AssembleParams {
        steps: vec![],
        with_tracing: false,
        package_name: "empty".into(),
    };
    let out = p.assemble().expect("assemble should succeed");
    assert!(out.main_rs.contains("async fn main"), "expected main fn");
}

#[test]
fn assemble_fragment_pipeline_end_to_end() {
    // Simulate the documented fragment pipeline:
    // 1. env! fragment
    // 2. format! fragment using env! as arg
    // 3. let binding + println as final step
    let env_frag = EnvParams {
        var: "CARGO_PKG_VERSION".into(),
        error_message: None,
    }
    .emit_code()
    .to_string();

    let fmt_frag = FormatParams {
        template: "version: {}".into(),
        args: vec![env_frag],
    }
    .emit_code()
    .to_string();

    // fmt_frag is an expression; wrap it into a statement
    let step = format!("let _msg = {fmt_frag} ;");

    let p = AssembleParams {
        steps: vec![step],
        with_tracing: false,
        package_name: "pipeline-test".into(),
    };
    let out = p.assemble().expect("assemble should succeed");
    // prettyplease formats macro calls as `format!` (no space), unlike TokenStream::to_string
    assert!(
        out.main_rs.contains("format!"),
        "expected format! in assembled source: {}",
        out.main_rs
    );
    assert!(
        out.main_rs.contains("env!"),
        "expected env! in assembled source: {}",
        out.main_rs
    );
}

#[test]
fn std_macros_have_no_crate_deps() {
    // All four macros are in std — no extra crate deps needed in emitted binary
    let format_deps = FormatParams {
        template: "x".into(),
        args: vec![],
    }
    .crate_deps();
    let include_deps = IncludeStrParams { path: "x".into() }.crate_deps();
    let env_deps = EnvParams {
        var: "X".into(),
        error_message: None,
    }
    .crate_deps();
    let concat_deps = ConcatParams { parts: vec![] }.crate_deps();
    assert!(format_deps.is_empty());
    assert!(include_deps.is_empty());
    assert!(env_deps.is_empty());
    assert!(concat_deps.is_empty());
}
