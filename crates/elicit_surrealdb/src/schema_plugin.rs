//! SurrealSchemaPlugin — MCP tools for SurrealDB DDL schema definitions.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

fn ok_text(s: impl Into<String>) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(rmcp::model::CallToolResult::success(vec![
        rmcp::model::Content::text(s.into()),
    ]))
}

#[derive(Debug, Deserialize, JsonSchema)]
struct EmptyParams {}

// ── Parameters ────────────────────────────────────────────────────────────────

/// Parameters for `surreal_schema__define_namespace`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineNamespaceParams {
    /// Namespace name.
    pub name: String,
    /// Whether to include IF NOT EXISTS clause.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Parameters for `surreal_schema__define_database`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineDatabaseParams {
    /// Database name.
    pub name: String,
    /// Whether to include IF NOT EXISTS clause.
    #[serde(default)]
    pub if_not_exists: bool,
    /// Optional CHANGEFEED duration string (e.g. `"1h"`).
    #[serde(default)]
    pub changefeed: Option<String>,
}

/// Parameters for `surreal_schema__define_table`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineTableParams {
    /// Table name.
    pub name: String,
    /// Whether to include IF NOT EXISTS clause.
    #[serde(default)]
    pub if_not_exists: bool,
    /// Whether the table is SCHEMAFULL (true) or SCHEMALESS (false).
    #[serde(default)]
    pub schemafull: bool,
    /// Whether to include DROP clause.
    #[serde(default)]
    pub drop: bool,
    /// Optional AS SELECT expression.
    #[serde(default)]
    pub as_select: Option<String>,
    /// Optional CHANGEFEED duration string.
    #[serde(default)]
    pub changefeed: Option<String>,
}

/// Parameters for `surreal_schema__define_field`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineFieldParams {
    /// Field name (supports dotted paths, e.g. `address.city`).
    pub name: String,
    /// Parent table name.
    pub table: String,
    /// Whether to include FLEXIBLE modifier.
    #[serde(default)]
    pub flexible: bool,
    /// Optional TYPE clause (e.g. `"string"`, `"option<int>"`).
    #[serde(default)]
    pub kind: Option<String>,
    /// Optional ASSERT expression.
    #[serde(default)]
    pub assert_expr: Option<String>,
    /// Optional DEFAULT expression.
    #[serde(default)]
    pub default_expr: Option<String>,
    /// Optional VALUE expression.
    #[serde(default)]
    pub value_expr: Option<String>,
    /// Whether to mark the field READONLY.
    #[serde(default)]
    pub readonly: bool,
}

/// Parameters for `surreal_schema__define_index`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineIndexParams {
    /// Index name.
    pub name: String,
    /// Table name the index is on.
    pub table: String,
    /// Fields to index.
    pub fields: Vec<String>,
    /// Whether to add UNIQUE constraint.
    #[serde(default)]
    pub unique: bool,
    /// Optional SEARCH ANALYZER name.
    #[serde(default)]
    pub search_analyzer: Option<String>,
    /// Whether to include BM25 ranking (requires search_analyzer).
    #[serde(default)]
    pub bm25: bool,
    /// Optional MTREE DIMENSION value for vector search.
    #[serde(default)]
    pub mtree_dimension: Option<u32>,
    /// Optional HNSW DIMENSION value for vector search.
    #[serde(default)]
    pub hnsw_dimension: Option<u32>,
}

/// Parameters for `surreal_schema__define_event`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineEventParams {
    /// Event name.
    pub name: String,
    /// Table the event is attached to.
    pub table: String,
    /// WHEN expression (e.g. `"$event = 'CREATE'"`).
    pub when_expr: String,
    /// THEN expression body.
    pub then_expr: String,
}

/// Parameters for `surreal_schema__define_function`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineFunctionParams {
    /// Fully qualified function name (e.g. `fn::greet`).
    pub fn_name: String,
    /// Argument declarations, each like `"$name: string"`.
    #[serde(default)]
    pub args: Vec<String>,
    /// Optional return type (e.g. `"string"`).
    #[serde(default)]
    pub return_type: Option<String>,
    /// Function body statements.
    pub body: String,
}

/// Parameters for `surreal_schema__define_param`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineParamParams {
    /// Parameter name without leading `$`.
    pub name: String,
    /// VALUE expression to assign.
    pub value: String,
}

/// Parameters for `surreal_schema__define_analyzer`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineAnalyzerParams {
    /// Analyzer name.
    pub name: String,
    /// Tokenizer names (e.g. `["blank", "camel"]`).
    #[serde(default)]
    pub tokenizers: Vec<String>,
    /// Filter names (e.g. `["lowercase", "snowball(English)"]`).
    #[serde(default)]
    pub filters: Vec<String>,
}

/// Parameters for `surreal_schema__define_access_jwt`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineAccessJwtParams {
    /// Access method name.
    pub name: String,
    /// Scope target, e.g. `"DATABASE"` or `"NAMESPACE"`.
    pub on_scope: String,
    /// JWT algorithm, e.g. `"HS256"`.
    pub algorithm: String,
    /// JWT signing key.
    pub key: String,
}

/// Parameters for `surreal_schema__define_access_record`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineAccessRecordParams {
    /// Access method name.
    pub name: String,
    /// SIGNUP expression.
    pub signup_expr: String,
    /// SIGNIN expression.
    pub signin_expr: String,
    /// Optional DURATION FOR SESSION value (e.g. `"7d"`).
    #[serde(default)]
    pub session_duration: Option<String>,
}

/// Parameters for `surreal_schema__define_user`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DefineUserParams {
    /// Username.
    pub name: String,
    /// Scope: `"ROOT"`, `"NAMESPACE"`, or `"DATABASE"`.
    pub on_scope: String,
    /// Plaintext password.
    pub password: String,
    /// Role names to assign (e.g. `["owner"]`).
    #[serde(default)]
    pub roles: Vec<String>,
}

/// Parameters for `surreal_schema__remove_table`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveTableParams {
    /// Table name.
    pub name: String,
    /// Whether to include IF EXISTS clause.
    #[serde(default)]
    pub if_exists: bool,
}

/// Parameters for `surreal_schema__remove_field`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveFieldParams {
    /// Field name.
    pub name: String,
    /// Parent table name.
    pub table: String,
    /// Whether to include IF EXISTS clause.
    #[serde(default)]
    pub if_exists: bool,
}

/// Parameters for `surreal_schema__remove_index`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RemoveIndexParams {
    /// Index name.
    pub name: String,
    /// Parent table name.
    pub table: String,
    /// Whether to include IF EXISTS clause.
    #[serde(default)]
    pub if_exists: bool,
}

/// Parameters for `surreal_schema__info_for_table`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InfoForTableParams {
    /// Table name.
    pub name: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_namespace",
    description = "Generate a DEFINE NAMESPACE statement."
)]
#[instrument]
async fn define_namespace(
    p: DefineNamespaceParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let ine = if p.if_not_exists {
        " IF NOT EXISTS"
    } else {
        ""
    };
    ok_text(format!("DEFINE NAMESPACE{ine} {};", p.name))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_database",
    description = "Generate a DEFINE DATABASE statement with optional CHANGEFEED."
)]
#[instrument]
async fn define_database(
    p: DefineDatabaseParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let ine = if p.if_not_exists {
        " IF NOT EXISTS"
    } else {
        ""
    };
    let cf = p
        .changefeed
        .as_deref()
        .map(|d| format!(" CHANGEFEED {d}"))
        .unwrap_or_default();
    ok_text(format!("DEFINE DATABASE{ine} {}{cf};", p.name))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_table",
    description = "Generate a DEFINE TABLE statement with schema, DROP, AS SELECT, and CHANGEFEED options."
)]
#[instrument]
async fn define_table(p: DefineTableParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let ine = if p.if_not_exists {
        " IF NOT EXISTS"
    } else {
        ""
    };
    let drop = if p.drop { " DROP" } else { "" };
    let schema = if p.schemafull {
        " SCHEMAFULL"
    } else {
        " SCHEMALESS"
    };
    let as_sel = p
        .as_select
        .as_deref()
        .map(|s| format!(" AS {s}"))
        .unwrap_or_default();
    let cf = p
        .changefeed
        .as_deref()
        .map(|d| format!(" CHANGEFEED {d}"))
        .unwrap_or_default();
    ok_text(format!(
        "DEFINE TABLE{ine} {}{drop}{schema}{as_sel}{cf};",
        p.name
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_field",
    description = "Generate a DEFINE FIELD statement with optional TYPE, ASSERT, DEFAULT, VALUE, and READONLY."
)]
#[instrument]
async fn define_field(p: DefineFieldParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let flex = if p.flexible { " FLEXIBLE" } else { "" };
    let kind = p
        .kind
        .as_deref()
        .map(|k| format!(" TYPE {k}"))
        .unwrap_or_default();
    let assert = p
        .assert_expr
        .as_deref()
        .map(|e| format!(" ASSERT {e}"))
        .unwrap_or_default();
    let default = p
        .default_expr
        .as_deref()
        .map(|e| format!(" DEFAULT {e}"))
        .unwrap_or_default();
    let value = p
        .value_expr
        .as_deref()
        .map(|e| format!(" VALUE {e}"))
        .unwrap_or_default();
    let ro = if p.readonly { " READONLY" } else { "" };
    ok_text(format!(
        "DEFINE FIELD{flex} {} ON TABLE {}{kind}{assert}{default}{value}{ro};",
        p.name, p.table
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_index",
    description = "Generate a DEFINE INDEX statement with optional UNIQUE, SEARCH ANALYZER, MTREE, and HNSW."
)]
#[instrument]
async fn define_index(p: DefineIndexParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let fields_str = p.fields.join(", ");
    let unique = if p.unique { " UNIQUE" } else { "" };
    let search = if let Some(a) = &p.search_analyzer {
        let bm = if p.bm25 { " BM25" } else { "" };
        format!(" SEARCH ANALYZER {a}{bm}")
    } else {
        String::new()
    };
    let mtree = p
        .mtree_dimension
        .map(|d| format!(" MTREE DIMENSION {d}"))
        .unwrap_or_default();
    let hnsw = p
        .hnsw_dimension
        .map(|d| format!(" HNSW DIMENSION {d}"))
        .unwrap_or_default();
    ok_text(format!(
        "DEFINE INDEX {} ON TABLE {} FIELDS {fields_str}{unique}{search}{mtree}{hnsw};",
        p.name, p.table
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_event",
    description = "Generate a DEFINE EVENT statement with WHEN and THEN clauses."
)]
#[instrument]
async fn define_event(p: DefineEventParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "DEFINE EVENT {} ON TABLE {} WHEN {} THEN ({});",
        p.name, p.table, p.when_expr, p.then_expr
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_function",
    description = "Generate a DEFINE FUNCTION statement with optional typed arguments and return type."
)]
#[instrument]
async fn define_function(
    p: DefineFunctionParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let args_str = p.args.join(", ");
    let ret = p
        .return_type
        .as_deref()
        .map(|r| format!(" -> {r}"))
        .unwrap_or_default();
    ok_text(format!(
        "DEFINE FUNCTION {fn_name}({args_str}){ret} {{\n    {body}\n}};",
        fn_name = p.fn_name,
        body = p.body
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_param",
    description = "Generate a DEFINE PARAM statement assigning a global parameter value."
)]
#[instrument]
async fn define_param(p: DefineParamParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("DEFINE PARAM ${} VALUE {};", p.name, p.value))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_analyzer",
    description = "Generate a DEFINE ANALYZER statement with optional tokenizers and filters."
)]
#[instrument]
async fn define_analyzer(
    p: DefineAnalyzerParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let tok = if p.tokenizers.is_empty() {
        String::new()
    } else {
        format!(" TOKENIZERS {}", p.tokenizers.join(", "))
    };
    let fil = if p.filters.is_empty() {
        String::new()
    } else {
        format!(" FILTERS {}", p.filters.join(", "))
    };
    ok_text(format!("DEFINE ANALYZER {}{tok}{fil};", p.name))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_access_jwt",
    description = "Generate a DEFINE ACCESS … TYPE JWT statement."
)]
#[instrument]
async fn define_access_jwt(
    p: DefineAccessJwtParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "DEFINE ACCESS {} ON {} TYPE JWT ALGORITHM {} KEY '{}';",
        p.name, p.on_scope, p.algorithm, p.key
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_access_record",
    description = "Generate a DEFINE ACCESS … TYPE RECORD statement with SIGNUP, SIGNIN, and optional session duration."
)]
#[instrument]
async fn define_access_record(
    p: DefineAccessRecordParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let dur = p
        .session_duration
        .as_deref()
        .map(|d| format!("\n    DURATION FOR SESSION {d}"))
        .unwrap_or_default();
    ok_text(format!(
        "DEFINE ACCESS {} ON DATABASE TYPE RECORD\n    SIGNUP ({})\n    SIGNIN ({}){dur};",
        p.name, p.signup_expr, p.signin_expr
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_user",
    description = "Generate a DEFINE USER statement with password and optional roles."
)]
#[instrument]
async fn define_user(p: DefineUserParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let roles_str = if p.roles.is_empty() {
        String::new()
    } else {
        format!(" ROLES {}", p.roles.join(", "))
    };
    ok_text(format!(
        "DEFINE USER {} ON {} PASSWORD '{}'{roles_str};",
        p.name, p.on_scope, p.password
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "remove_table",
    description = "Generate a REMOVE TABLE statement with optional IF EXISTS."
)]
#[instrument]
async fn remove_table(p: RemoveTableParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let ie = if p.if_exists { " IF EXISTS" } else { "" };
    ok_text(format!("REMOVE TABLE{ie} {};", p.name))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "remove_field",
    description = "Generate a REMOVE FIELD … ON TABLE statement with optional IF EXISTS."
)]
#[instrument]
async fn remove_field(p: RemoveFieldParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let ie = if p.if_exists { " IF EXISTS" } else { "" };
    ok_text(format!("REMOVE FIELD{ie} {} ON TABLE {};", p.name, p.table))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "remove_index",
    description = "Generate a REMOVE INDEX … ON TABLE statement with optional IF EXISTS."
)]
#[instrument]
async fn remove_index(p: RemoveIndexParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let ie = if p.if_exists { " IF EXISTS" } else { "" };
    ok_text(format!("REMOVE INDEX{ie} {} ON TABLE {};", p.name, p.table))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "info_for_db",
    description = "Generate an INFO FOR DB statement to inspect the current database schema."
)]
#[instrument]
async fn info_for_db(_p: EmptyParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text("INFO FOR DB;")
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "info_for_table",
    description = "Generate an INFO FOR TABLE <name> statement to inspect a table's schema."
)]
#[instrument]
async fn info_for_table(p: InfoForTableParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!("INFO FOR TABLE {};", p.name))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing SurrealDB DDL schema definition tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_schema")]
pub struct SurrealSchemaPlugin;
