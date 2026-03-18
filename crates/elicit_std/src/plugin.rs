//! `StdMacrosPlugin` — MCP plugin exposing std macro emit tools.

use elicitation::emit_code::EmitCode;
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use tracing::instrument;

use crate::{AssembleParams, ConcatParams, EnvParams, FormatParams, IncludeStrParams};

/// MCP plugin that exposes Rust standard library macros as fragment tools,
/// plus the terminal `std__assemble` tool.
///
/// Fragment tools take macro parameters as JSON and return a Rust source
/// fragment.  Fragments are composable: pass a fragment string as an
/// expression argument to another tool.  The final step is `std__assemble`,
/// which wraps statement-level fragments in a `#[tokio::main]` binary.
///
/// # Registration
///
/// ```rust,no_run
/// use elicit_std::StdMacrosPlugin;
/// use elicitation::PluginRegistry;
///
/// let registry = PluginRegistry::new()
///     .register("std", StdMacrosPlugin);
/// ```
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "std")]
pub struct StdMacrosPlugin;

// ── format! ───────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "std",
    name = "format",
    description = "Emit a `format!(template, args…)` expression. \
                   The template is a format string literal; args are Rust \
                   expressions interpolated in order. \
                   Returns the Rust source fragment — no runtime execution."
)]
#[instrument(skip_all)]
async fn emit_format(p: FormatParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

// ── include_str! ──────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "std",
    name = "include_str",
    description = "Emit an `include_str!(\"path\")` expression that embeds a \
                   file's contents as a `&'static str` at compile time. \
                   The path is relative to the emitted source file. \
                   Returns the Rust source fragment — no runtime execution."
)]
#[instrument(skip_all)]
async fn emit_include_str(p: IncludeStrParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

// ── env! ──────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "std",
    name = "env",
    description = "Emit an `env!(\"VAR\")` expression that reads an environment \
                   variable at compile time of the emitted binary. \
                   Optionally include an error message for when the var is unset. \
                   Returns the Rust source fragment — no runtime execution."
)]
#[instrument(skip_all)]
async fn emit_env(p: EnvParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

// ── concat! ───────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "std",
    name = "concat",
    description = "Emit a `concat!(\"a\", \"b\", …)` expression that joins \
                   string literal parts into a single `&'static str` at compile \
                   time. \
                   Returns the Rust source fragment — no runtime execution."
)]
#[instrument(skip_all)]
async fn emit_concat(p: ConcatParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

// ── assemble (terminal) ───────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "std",
    name = "assemble",
    description = "Assemble statement-level fragment strings into a compilable \
                   Rust binary. Each string in `steps` is a Rust fragment \
                   previously returned by an emit tool. Returns a JSON object \
                   with `main_rs` (pretty-printed source) and `cargo_toml` \
                   (generated dependency manifest)."
)]
#[instrument(skip_all)]
async fn assemble_binary(p: AssembleParams) -> Result<CallToolResult, ErrorData> {
    let output = p
        .assemble()
        .map_err(|e| ErrorData::internal_error(e, None))?;
    let json = serde_json::json!({
        "main_rs": output.main_rs,
        "cargo_toml": output.cargo_toml,
    });
    Ok(CallToolResult::success(vec![Content::text(
        json.to_string(),
    )]))
}
