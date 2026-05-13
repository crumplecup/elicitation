//! SurrealControlPlugin — MCP tools for SurrealQL control-flow, variables, graph traversal,
//! and type-casting statements.

use elicitation::{ElicitPlugin, elicit_tool};
use elicitation_derive::Elicit;
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn ok_text(s: impl Into<String>) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(rmcp::model::CallToolResult::success(vec![
        rmcp::model::Content::text(s.into()),
    ]))
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CastTypesReferenceParams {}

// ── Parameters ────────────────────────────────────────────────────────────────

/// Parameters for `surreal_control__let`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LetParams {
    /// Variable name (without `$` prefix, e.g. `"user"`).
    pub name: String,
    /// SurrealQL expression to bind (e.g. `"SELECT * FROM person LIMIT 1"`).
    pub value: String,
}

/// Parameters for `surreal_control__return`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReturnParams {
    /// SurrealQL expression to return (e.g. `"$user.name"`).
    pub expression: String,
}

/// Parameters for `surreal_control__if_else`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IfElseParams {
    /// Condition expression (e.g. `"$user.age >= 18"`).
    pub condition: String,
    /// Body executed when the condition is true.
    pub then_body: String,
    /// Body executed when the condition is false (optional).
    pub else_body: Option<String>,
}

/// Parameters for `surreal_control__if_else_if`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IfElseIfParams {
    /// The if/else-if/else branches as `[(condition, body)]` pairs (last may have empty condition
    /// for the `ELSE` branch).
    pub branches: Vec<IfBranch>,
}

/// A single if/else-if branch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct IfBranch {
    /// Condition expression; empty string means this is an `ELSE` branch.
    pub condition: String,
    /// Body for this branch.
    pub body: String,
}

/// Parameters for `surreal_control__throw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ThrowParams {
    /// Error message or SurrealQL expression to throw (e.g. `"\"Not authorised\""`).
    pub expression: String,
}

/// Parameters for `surreal_control__graph_out`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GraphOutParams {
    /// Starting record (e.g. `"person:john"`).
    pub from: String,
    /// Edge table (e.g. `"likes"`).
    pub edge: String,
    /// Optional target table filter (e.g. `"post"`).
    pub to: Option<String>,
    /// Optional WHERE clause on the edge (e.g. `"since > '2024-01-01'"`).
    pub edge_where: Option<String>,
}

/// Parameters for `surreal_control__graph_in`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GraphInParams {
    /// Starting record (e.g. `"post:1"`).
    pub from: String,
    /// Edge table (e.g. `"likes"`).
    pub edge: String,
    /// Optional source table filter (e.g. `"person"`).
    pub from_table: Option<String>,
}

/// Parameters for `surreal_control__graph_both`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GraphBothParams {
    /// Starting record.
    pub from: String,
    /// Edge table.
    pub edge: String,
}

/// Parameters for `surreal_control__graph_traverse`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GraphTraverseParams {
    /// Starting record (e.g. `"person:john"`).
    pub from: String,
    /// Traversal path, e.g. `"->knows->person->likes->post"`.
    pub path: String,
}

/// Parameters for `surreal_control__cast`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CastParams {
    /// SurrealQL type name to cast to (`"int"`, `"float"`, `"string"`, `"bool"`, `"datetime"`,
    /// `"duration"`, `"decimal"`, `"bytes"`, `"uuid"`, `"array"`, `"object"`, `"number"`).
    pub target_type: String,
    /// Expression to cast (e.g. `"\"42\""`).
    pub expression: String,
}

// ── Tools ────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_control",
    name = "let",
    description = "Emit a SurrealQL LET statement to bind a value to a variable."
)]
#[instrument]
async fn surreal_let(p: LetParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "LET ${name} = {value};",
        name = p.name,
        value = p.value
    ))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "return",
    description = "Emit a SurrealQL RETURN statement to return a value from a block or function."
)]
#[instrument]
async fn surreal_return(p: ReturnParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("RETURN {};", p.expression))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "if_else",
    description = "Emit a SurrealQL IF/ELSE conditional block."
)]
#[instrument]
async fn surreal_if_else(p: IfElseParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let else_block = p
        .else_body
        .map(|b| format!(" ELSE {{\n    {b}\n}}"))
        .unwrap_or_default();
    ok_text(format!(
        "IF {cond} {{\n    {then}\n}}{else_block};",
        cond = p.condition,
        then = p.then_body
    ))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "if_else_if",
    description = "Emit a multi-branch SurrealQL IF / ELSE IF / ELSE block."
)]
#[instrument]
async fn surreal_if_else_if(p: IfElseIfParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    if p.branches.is_empty() {
        return ok_text("-- No branches provided");
    }
    let mut parts = Vec::new();
    let mut first = true;
    for branch in &p.branches {
        if branch.condition.is_empty() {
            parts.push(format!("ELSE {{\n    {}\n}}", branch.body));
        } else if first {
            parts.push(format!(
                "IF {} {{\n    {}\n}}",
                branch.condition, branch.body
            ));
            first = false;
        } else {
            parts.push(format!(
                "ELSE IF {} {{\n    {}\n}}",
                branch.condition, branch.body
            ));
        }
    }
    ok_text(format!("{};", parts.join(" ")))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "throw",
    description = "Emit a SurrealQL THROW statement to raise an error."
)]
#[instrument]
async fn surreal_throw(p: ThrowParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("THROW {};", p.expression))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "graph_out",
    description = "Emit a SurrealQL outbound graph traversal expression using the -> operator."
)]
#[instrument]
async fn graph_out(p: GraphOutParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let to = p.to.as_deref().unwrap_or("");
    let edge_clause = if let Some(w) = p.edge_where {
        format!("[WHERE {}]", w)
    } else {
        String::new()
    };
    let to_clause = if to.is_empty() {
        String::new()
    } else {
        format!("->{to}")
    };
    ok_text(format!(
        "SELECT ->{}{}{}  FROM {};",
        p.edge, edge_clause, to_clause, p.from
    ))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "graph_in",
    description = "Emit a SurrealQL inbound graph traversal expression using the <- operator."
)]
#[instrument]
async fn graph_in(p: GraphInParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let from_clause = p.from_table.map(|t| format!("<-{t}")).unwrap_or_default();
    ok_text(format!(
        "SELECT <-{}{} FROM {};",
        p.edge, from_clause, p.from
    ))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "graph_both",
    description = "Emit a SurrealQL bidirectional graph traversal expression using the <-> operator."
)]
#[instrument]
async fn graph_both(p: GraphBothParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("SELECT <->{} FROM {};", p.edge, p.from))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "graph_traverse",
    description = "Emit a multi-hop SurrealQL graph traversal path (e.g. person->knows->person->likes->post)."
)]
#[instrument]
async fn graph_traverse(p: GraphTraverseParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "SELECT {path} FROM {from};",
        path = p.path,
        from = p.from
    ))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "cast",
    description = "Emit a SurrealQL type-casting expression, e.g. <int>\"42\"."
)]
#[instrument]
async fn surreal_cast(p: CastParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("<{}>{}", p.target_type, p.expression))
}

#[elicit_tool(
    plugin = "surreal_control",
    name = "cast_types_reference",
    description = "Return a reference table of all SurrealQL built-in type-cast targets."
)]
#[instrument]
async fn cast_types_reference(
    _p: CastTypesReferenceParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(
        "SurrealQL built-in type casts:\n\
         <int>       — 64-bit integer\n\
         <float>     — 64-bit float\n\
         <string>    — string\n\
         <bool>      — boolean\n\
         <datetime>  — ISO 8601 datetime\n\
         <duration>  — SurrealDB duration (e.g. 1y2w3d)\n\
         <decimal>   — arbitrary-precision decimal\n\
         <number>    — numeric (int or float)\n\
         <bytes>     — byte string\n\
         <uuid>      — UUID\n\
         <array>     — array\n\
         <object>    — object\n\
         <record>    — record link\n\
         <geometry>  — geometry value\n\
         <future>    — deferred computation\n\
         <option<T>> — optional value of type T",
    )
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing SurrealQL control-flow, variable binding, graph traversal,
/// and type-casting tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_control")]
pub struct SurrealControlPlugin;
