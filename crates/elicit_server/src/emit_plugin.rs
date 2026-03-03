//! `EmitBinaryPlugin` — recover agent tool compositions as compiled Rust binaries.
//!
//! This plugin is the capstone of the code recovery pipeline. An agent that has
//! composed a sequence of verified workflow tool calls can ask this plugin to
//! recover that composition as a standalone, compilable Rust program — with all
//! typestate ceremony, proof tokens, and contract types preserved.
//!
//! # How it works
//!
//! 1. Agent builds a workflow interactively using tools from `elicit_reqwest`
//!    and/or `elicit_serde_json`
//! 2. Agent calls `emit_binary` with the ordered list of tool names + params
//! 3. Plugin dispatches each step to the matching [`EmitCode`] impl via each
//!    crate's `dispatch_emit` function (params stay private)
//! 4. [`BinaryScaffold`] assembles the steps into `#[tokio::main] async fn main()`
//! 5. `src/main.rs` + `Cargo.toml` are written to `output_dir`
//! 6. `cargo build --release` is invoked if `compile = true`; binary path returned
//!
//! # Supported tools
//!
//! **elicit_reqwest:** `fetch`, `auth_fetch`, `post`, `api_call`, `health_check`,
//! `url_build`, `status_summary`, `build_request`, `paginated_get`
//!
//! **elicit_serde_json:** `parse_and_focus`, `validate_object`, `safe_merge`,
//! `pointer_update`, `field_chain`

use elicitation::ElicitPlugin;
use elicitation::emit_code::{BinaryScaffold, EmitCode};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::util::{parse_args, typed_tool};
use elicitation::rmcp::RoleServer;

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
pub struct EmitBinaryPlugin;

impl ElicitPlugin for EmitBinaryPlugin {
    fn name(&self) -> &'static str {
        "emit"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![typed_tool::<EmitBinaryParams>(
            "emit_binary",
            "Recover an ordered sequence of verified workflow tool calls as a standalone, \
                 compilable Rust program. Each step preserves full typestate ceremony: proof \
                 tokens, Established<P>, contract types — all intact. The emitted binary \
                 calls the same verified library APIs the agent used interactively, so the \
                 formal verification blanket extends to the generated program.\n\
                 Supported tools: fetch, auth_fetch, post, api_call, health_check, url_build, \
                 status_summary, build_request, paginated_get (elicit_reqwest); \
                 parse_and_focus, validate_object, safe_merge, pointer_update, field_chain \
                 (elicit_serde_json).",
        )]
    }

    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _cx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            match params.name.as_ref() {
                "emit_binary" => {
                    let p: EmitBinaryParams = match parse_args(&params) {
                        Ok(p) => p,
                        Err(e) => return Err(e),
                    };
                    emit_binary_impl(p).await
                }
                name => Err(ErrorData::invalid_params(
                    format!("Unknown tool: {name}"),
                    None,
                )),
            }
        })
    }
}

// ── Implementation ────────────────────────────────────────────────────────────

#[instrument(skip(p), fields(steps = p.steps.len(), output_dir = %p.output_dir))]
async fn emit_binary_impl(p: EmitBinaryParams) -> Result<CallToolResult, ErrorData> {
    let mut steps: Vec<Box<dyn EmitCode>> = Vec::new();

    for (i, step) in p.steps.iter().enumerate() {
        match dispatch_step(&step.tool, &step.params) {
            Ok(boxed) => steps.push(boxed),
            Err(e) => {
                return Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                    format!("Step {i} ('{}') failed to deserialize: {e}", step.tool),
                )]));
            }
        }
    }

    let scaffold = BinaryScaffold::new(steps, p.with_tracing);
    let output_dir = std::path::PathBuf::from(&p.output_dir);

    let main_rs = match scaffold.emit_to_disk(&output_dir, &p.package_name) {
        Ok(path) => path,
        Err(e) => {
            return Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                format!("Failed to write source: {e}"),
            )]));
        }
    };

    if !p.compile {
        return Ok(CallToolResult::success(vec![rmcp::model::Content::text(
            format!(
                "Source written to {}\nRun: cd {} && cargo run",
                main_rs.display(),
                output_dir.display(),
            ),
        )]));
    }

    match elicitation::emit_code::compile(&output_dir) {
        Ok(binary) => Ok(CallToolResult::success(vec![rmcp::model::Content::text(
            format!(
                "Binary compiled: {}\nSource: {}",
                binary.display(),
                main_rs.display(),
            ),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![rmcp::model::Content::text(
            format!("Compilation failed:\n{e}"),
        )])),
    }
}

/// Dispatch a tool name + raw JSON params to the matching `EmitCode` impl.
///
/// Tries reqwest tools first, then serde_json tools. Returns an error if
/// the tool name is not recognized by either crate.
fn dispatch_step(tool: &str, params: &serde_json::Value) -> Result<Box<dyn EmitCode>, String> {
    if let Ok(boxed) = elicit_reqwest::dispatch_reqwest_emit(tool, params.clone()) {
        return Ok(boxed);
    }
    if let Ok(boxed) = elicit_serde_json::dispatch_serde_json_emit(tool, params.clone()) {
        return Ok(boxed);
    }
    Err(format!(
        "Unknown tool: '{tool}'. Supported: fetch, auth_fetch, post, api_call, health_check, \
         url_build, status_summary, build_request, paginated_get, parse_and_focus, \
         validate_object, safe_merge, pointer_update, field_chain"
    ))
}
