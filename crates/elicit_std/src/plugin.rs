//! `StdMacrosPlugin` — MCP plugin exposing std macro emit tools.

use elicitation::emit_code::EmitCode;
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use tracing::instrument;

use crate::{ConcatParams, EnvParams, FormatParams, IncludeStrParams};

/// MCP plugin that exposes Rust standard library macros as emit-only tools.
///
/// Each tool takes macro parameters as JSON and returns the equivalent Rust
/// source fragment.  The returned string is ready to embed in a
/// [`BinaryScaffold`](elicitation::emit_code::BinaryScaffold) or inspect
/// directly.
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
