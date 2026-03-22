//! MCP plugin exposing sqlx fragment emit tools.
//!
//! Fragment tools emit Rust source code wrapping sqlx compile-time macros.
//! They do not execute SQL — they return source fragments for the agent to
//! assemble into a binary via `std__assemble`.
//!
//! **Build-time constraint**: all emitted code requires `DATABASE_URL` set
//! in the build environment of the consuming binary.

use elicitation::emit_code::EmitCode;
use elicitation::contracts::{Established, Prop};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use tracing::instrument;

use crate::fragments::{MigrateParams, QueryAsParams, QueryParams, QueryScalarParams};

/// MCP plugin exposing sqlx fragment tools.
///
/// Registers four tools: `sqlx__query`, `sqlx__query_as`,
/// `sqlx__query_scalar`, and `sqlx__migrate`.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "sqlx_frag")]
pub struct SqlxFragPlugin;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a `sqlx::query!(sql, params…)` source fragment was emitted.
///
/// Established by [`emit_query`] after [`EmitCode::emit_code`] succeeds.
/// The fragment contains a valid macro invocation; it does NOT guarantee
/// that the emitted binary will compile (requires `DATABASE_URL` at
/// consumer build time).
pub struct QueryFragmentEmitted;
impl Prop for QueryFragmentEmitted {}

/// Proposition: a `sqlx::query_as!(Type, sql, params…)` source fragment was emitted.
///
/// Established by [`emit_query_as`] after [`EmitCode::emit_code`] succeeds.
pub struct QueryAsFragmentEmitted;
impl Prop for QueryAsFragmentEmitted {}

/// Proposition: a `sqlx::query_scalar!(sql, params…)` source fragment was emitted.
///
/// Established by [`emit_query_scalar`] after [`EmitCode::emit_code`] succeeds.
pub struct QueryScalarFragmentEmitted;
impl Prop for QueryScalarFragmentEmitted {}

/// Proposition: a `sqlx::migrate!(path).run(&pool).await?` source fragment was emitted.
///
/// Established by [`emit_migrate`] after [`EmitCode::emit_code`] succeeds.
pub struct MigrateFragmentEmitted;
impl Prop for MigrateFragmentEmitted {}

// ── query! ────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "sqlx_frag",
    name = "query",
    description = "Emit a `sqlx::query!(sql, params…)` expression. \
                   Requires DATABASE_URL at compile time of the emitted binary. \
                   Returns the Rust source fragment — no runtime SQL execution."
)]
#[instrument(skip_all)]
async fn emit_query(p: QueryParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    let _proof: Established<QueryFragmentEmitted> = Established::assert();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

// ── query_as! ─────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "sqlx_frag",
    name = "query_as",
    description = "Emit a `sqlx::query_as!(TargetType, sql, params…)` expression. \
                   The target type must implement `sqlx::FromRow`. \
                   Requires DATABASE_URL at compile time of the emitted binary. \
                   Returns the Rust source fragment — no runtime SQL execution."
)]
#[instrument(skip_all)]
async fn emit_query_as(p: QueryAsParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    let _proof: Established<QueryAsFragmentEmitted> = Established::assert();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

// ── query_scalar! ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "sqlx_frag",
    name = "query_scalar",
    description = "Emit a `sqlx::query_scalar!(sql, params…)` expression for \
                   queries returning a single scalar value (e.g. COUNT). \
                   Requires DATABASE_URL at compile time of the emitted binary. \
                   Returns the Rust source fragment — no runtime SQL execution."
)]
#[instrument(skip_all)]
async fn emit_query_scalar(p: QueryScalarParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    let _proof: Established<QueryScalarFragmentEmitted> = Established::assert();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

// ── migrate! ──────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "sqlx_frag",
    name = "migrate",
    description = "Emit a `sqlx::migrate!(path).run(&pool).await?` statement. \
                   Requires DATABASE_URL at compile time of the emitted binary. \
                   Returns the Rust source fragment — no runtime execution."
)]
#[instrument(skip_all)]
async fn emit_migrate(p: MigrateParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    let _proof: Established<MigrateFragmentEmitted> = Established::assert();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}
