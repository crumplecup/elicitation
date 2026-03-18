//! `assemble` — terminal tool that composes fragment strings into a binary.
//!
//! This is the final step in the fragment pipeline.  An agent collects
//! fragment strings from multiple emit tools, then calls `std__assemble` to
//! bundle them into a compilable Rust program.
//!
//! Unlike the other tools in this crate, `assemble` is **not** a fragment
//! tool — it does not return a `TokenStream` fragment.  It produces the
//! complete `main.rs` source and a matching `Cargo.toml`.

use elicitation::emit_code::{BinaryScaffold, RawFragment};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for assembling multiple fragment strings into a binary.
///
/// Each string in `steps` is a statement-level Rust fragment previously
/// returned by an emit tool (e.g. `std__format`, `std__env`).  The fragments
/// are wrapped as [`RawFragment`]s and assembled in order inside a
/// `#[tokio::main] async fn main()`.
///
/// The tool returns a JSON object:
/// ```json
/// {
///   "main_rs": "/* pretty-printed main.rs source */",
///   "cargo_toml": "/* generated Cargo.toml */"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AssembleParams {
    /// Statement-level Rust fragment strings to assemble, in order.
    ///
    /// Each element is a pre-rendered TokenStream string returned by a prior
    /// emit tool call.  Fragments run sequentially inside `main()`.
    ///
    /// Example: `["let x = 1;", "println!(\"{}\", x);"]`
    pub steps: Vec<String>,

    /// If `true`, insert `tracing_subscriber::fmt::init();` at the start of
    /// `main()` and include `tracing-subscriber` in the generated `Cargo.toml`.
    #[serde(default)]
    pub with_tracing: bool,

    /// Rust package name for the generated `Cargo.toml`.
    ///
    /// Defaults to `"elicit-output"` when not specified.
    #[serde(default = "default_package_name")]
    pub package_name: String,
}

fn default_package_name() -> String {
    "elicit-output".to_string()
}

/// The output produced by `std__assemble`.
#[derive(Debug, Serialize)]
pub struct AssembleOutput {
    /// Pretty-printed `main.rs` source ready to write to disk.
    pub main_rs: String,
    /// Generated `Cargo.toml` content ready to write to disk.
    pub cargo_toml: String,
}

impl AssembleParams {
    /// Assemble the fragments into source + Cargo.toml.
    ///
    /// Returns an error string if any fragment fails to parse as a
    /// `TokenStream` or if the assembled source fails `syn` parsing.
    pub fn assemble(&self) -> Result<AssembleOutput, String> {
        let steps: Vec<Box<dyn elicitation::emit_code::EmitCode>> = self
            .steps
            .iter()
            .map(|s| Box::new(RawFragment(s.clone())) as Box<dyn elicitation::emit_code::EmitCode>)
            .collect();

        let scaffold = BinaryScaffold::new(steps, self.with_tracing);

        let main_rs = scaffold.to_source().map_err(|e| e.to_string())?;
        let cargo_toml = scaffold.to_cargo_toml(&self.package_name);

        Ok(AssembleOutput {
            main_rs,
            cargo_toml,
        })
    }
}
