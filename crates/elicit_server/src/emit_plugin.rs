//! `EmitBinaryPlugin` — recover agent tool compositions as compiled Rust binaries.
//!
//! This plugin is the capstone of the code recovery pipeline. An agent that has
//! composed a sequence of verified workflow tool calls can ask this plugin to
//! recover that composition as a standalone, compilable Rust program — with all
//! typestate ceremony, proof tokens, and contract types preserved.
//!
//! # How it works
//!
//! 1. Agent builds a workflow interactively using tools from any shadow crate
//! 2. Agent calls `emit_binary` with the ordered list of tool names + params
//! 3. Plugin dispatches each step to the matching [`EmitCode`] impl via the
//!    global `register_emit!` inventory (populated at link time)
//! 4. [`BinaryScaffold`] assembles the steps into `#[tokio::main] async fn main()`
//! 5. `src/main.rs` + `Cargo.toml` are written to `output_dir`
//! 6. `cargo build --release` is invoked if `compile = true`; binary path returned
//!
//! # Supported tools
//!
//! All tools registered via `#[elicit_tool]` in the shadow crates linked into
//! `elicit_server` are available. See the `elicit_server` crate docs for the
//! full table of plugins and their tool names.

use elicitation::emit_code::{BinaryScaffold, EmitCode};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

// ── Param types ───────────────────────────────────────────────────────────────

/// A single tool call step to recover as Rust source.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WorkflowStep {
    /// Tool name. See module docs for supported values.
    pub tool: String,
    /// Tool parameters as a JSON object matching the tool's param schema.
    pub params: serde_json::Value,
}

/// Parameters for the `emit_binary` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitBinaryParams {
    /// Ordered list of tool calls to recover as a binary.
    pub steps: Vec<WorkflowStep>,
    /// If true, inserts `tracing_subscriber::fmt::init()` in `main()`.
    #[serde(default = "default_true")]
    pub with_tracing: bool,
    /// Directory to write the generated Rust project into.
    pub output_dir: String,
    /// Package name for the generated `Cargo.toml`.
    #[serde(default = "default_package_name")]
    pub package_name: String,
    /// If true, also runs `cargo build --release` after writing source.
    #[serde(default)]
    pub compile: bool,
    /// Absolute path to the elicitation workspace root.
    ///
    /// When set, `elicit_*` / `elicitation` deps in the generated `Cargo.toml`
    /// are emitted as path deps pointing into this workspace — enabling
    /// pre-publish builds without a crates.io release. Falls back to the
    /// `ELICIT_WORKSPACE_ROOT` environment variable when omitted.
    pub workspace_root: Option<String>,
}

fn default_true() -> bool {
    true
}
fn default_package_name() -> String {
    "recovered_workflow".to_string()
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Cross-crate plugin that recovers agent tool compositions as Rust binaries.
///
/// Register with your [`elicitation::PluginRegistry`] to expose `emit_binary`
/// as an MCP tool.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "emit")]
pub struct EmitBinaryPlugin;

// ── Tool handler ──────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "emit",
    name = "emit_binary",
    emit = None,
    description = "Recover an ordered sequence of verified workflow tool calls as a standalone, \
                   compilable Rust program. Each step preserves full typestate ceremony: proof \
                   tokens, Established<P>, contract types — all intact. The emitted binary \
                   calls the same verified library APIs the agent used interactively, so the \
                   formal verification blanket extends to the generated program.\n\
                   All tools registered in the emit inventory are supported. See the \
                   elicit_server crate docs for the full plugin/tool table."
)]
#[instrument(skip(p), fields(steps = p.steps.len(), output_dir = %p.output_dir))]
async fn emit_binary(p: EmitBinaryParams) -> Result<CallToolResult, ErrorData> {
    let mut steps: Vec<Box<dyn EmitCode>> = Vec::new();

    for (i, step) in p.steps.iter().enumerate() {
        match elicitation::emit_code::dispatch_emit(&step.tool, step.params.clone()) {
            Ok(boxed) => steps.push(boxed),
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Step {i} ('{}') failed to deserialize: {e}",
                    step.tool
                ))]));
            }
        }
    }

    let mut scaffold = BinaryScaffold::new(steps, p.with_tracing);
    if let Some(root) = p.workspace_root.as_deref().map(std::path::Path::new) {
        scaffold = scaffold.with_workspace_root(root);
    }
    let output_dir = std::path::PathBuf::from(&p.output_dir);

    let main_rs = match scaffold.emit_to_disk(&output_dir, &p.package_name) {
        Ok(path) => path,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Failed to write source: {e}"
            ))]));
        }
    };

    if !p.compile {
        return Ok(CallToolResult::success(vec![Content::text(format!(
            "Source written to {}\nRun: cd {} && cargo run",
            main_rs.display(),
            output_dir.display(),
        ))]));
    }

    match elicitation::emit_code::compile(&output_dir) {
        Ok(binary) => Ok(CallToolResult::success(vec![Content::text(format!(
            "Binary compiled: {}\nSource: {}",
            binary.display(),
            main_rs.display(),
        ))])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
            "Compilation failed:\n{e}"
        ))])),
    }
}
