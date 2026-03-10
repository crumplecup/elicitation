//! Tests for the `crate_deps()` pipeline in `#[elicit_tool]`-annotated handlers.
//!
//! These cover each stage of the dep-inference path independently:
//!
//! 1. `crate_deps()` on a dispatched `EmitCode` step — verifies the
//!    macro read `Cargo.toml` at build time and embedded the dep list.
//! 2. `BinaryScaffold::all_deps()` — verifies dedup + scaffold defaults merged.
//! 3. `BinaryScaffold::to_cargo_toml()` — verifies the rendered TOML string
//!    contains the required dep entries.
//!
//! `elicit_url` is the canary: it's a simple single-crate tool, so its
//! `crate_deps()` should always contain at least `elicit_url` itself and
//! `elicitation`.  If these fail, the macro-level `all_crate_deps()` is broken.

#[cfg(feature = "emit")]
mod emit_deps {
    use elicitation::emit_code::BinaryScaffold;

    fn dispatch_parse_url() -> Box<dyn elicitation::emit_code::EmitCode> {
        let params = elicit_url::ParseUrlParams {
            url: "https://example.com".to_string(),
        };
        Box::new(params)
    }

    /// Stage 1: `crate_deps()` must be non-empty and include the crates the
    /// emitted code references.
    #[test]
    fn parse_url_crate_deps_non_empty() {
        let step = dispatch_parse_url();
        let deps = step.crate_deps();
        let names: Vec<&str> = deps.iter().map(|d| d.name).collect();
        println!("parse_url crate_deps: {names:?}");

        assert!(
            !deps.is_empty(),
            "crate_deps() is empty — `all_crate_deps()` in the macro failed to \
             read Cargo.toml at build time"
        );
        assert!(
            names.contains(&"elicit_url"),
            "missing own crate `elicit_url` in crate_deps; got: {names:?}"
        );
        assert!(
            names.contains(&"elicitation"),
            "missing `elicitation` in crate_deps; got: {names:?}"
        );
    }

    /// Stage 2: `BinaryScaffold::all_deps()` merges scaffold defaults with step
    /// deps; result must include `tokio` (scaffold) and `elicit_url`.
    #[test]
    fn parse_url_scaffold_all_deps() {
        let step = dispatch_parse_url();
        let scaffold = BinaryScaffold::new(vec![step], false);
        let all = scaffold.all_deps();
        let names: Vec<&str> = all.iter().map(|d| d.name).collect();
        println!("scaffold all_deps: {names:?}");

        assert!(
            names.contains(&"tokio"),
            "missing scaffold dep `tokio` in all_deps; got: {names:?}"
        );
        assert!(
            names.contains(&"elicit_url"),
            "missing `elicit_url` in all_deps; got: {names:?}"
        );
        assert!(
            names.contains(&"elicitation"),
            "missing `elicitation` in all_deps; got: {names:?}"
        );
    }

    /// Stage 3: the rendered `Cargo.toml` string must contain dep entries for
    /// everything the generated `main.rs` references.
    #[test]
    fn parse_url_cargo_toml_contains_deps() {
        let step = dispatch_parse_url();
        let scaffold = BinaryScaffold::new(vec![step], false);
        let toml = scaffold.to_cargo_toml("test_parse_url");
        println!("generated Cargo.toml:\n{toml}");

        assert!(
            toml.contains("elicit_url"),
            "Cargo.toml missing `elicit_url`:\n{toml}"
        );
        assert!(
            toml.contains("elicitation"),
            "Cargo.toml missing `elicitation`:\n{toml}"
        );
    }
}
