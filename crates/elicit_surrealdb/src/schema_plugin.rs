//! `SurrealSchemaPlugin` — SurrealQL DDL authoring tools.
//!
//! Every tool is a pure function that accepts parameters and emits a SurrealQL `DEFINE` or
//! `REMOVE` statement string, or a Rust SDK DDL execution snippet.

use elicitation::{ElicitPlugin, ToCodeLiteral, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn ok_text(text: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(text.into())]))
}

// ── parameter structs ─────────────────────────────────────────────────────────

/// Parameters for `DEFINE NAMESPACE`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineNamespaceParams {
    /// Namespace name.
    pub name: String,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Parameters for `DEFINE DATABASE`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineDatabaseParams {
    /// Database name.
    pub name: String,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
    /// Optional `CHANGEFEED` duration string (e.g. `"1h"`).
    #[serde(default)]
    pub changefeed: Option<String>,
}

/// Parameters for `DEFINE TABLE`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineTableParams {
    /// Table name.
    pub name: String,
    /// Whether the table is SCHEMAFULL (strict) or SCHEMALESS (default).
    #[serde(default)]
    pub schemafull: bool,
    /// Add `DROP` clause so writes are silently discarded.
    #[serde(default)]
    pub drop: bool,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
    /// Optional `CHANGEFEED` duration.
    #[serde(default)]
    pub changefeed: Option<String>,
    /// Optional `AS SELECT` projection for a view table.
    #[serde(default)]
    pub as_select: Option<String>,
    /// Optional `PERMISSIONS` clause (raw SurrealQL string).
    #[serde(default)]
    pub permissions: Option<String>,
}

/// Parameters for `DEFINE FIELD`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineFieldParams {
    /// Field name (dot-path supported, e.g. `address.city`).
    pub name: String,
    /// Table name.
    pub table: String,
    /// SurrealQL type declaration (e.g. `"string"`, `"option<int>"`, `"array<string, 10>"`).
    #[serde(default)]
    pub kind: Option<String>,
    /// `FLEXIBLE` — allow extra schema keys under an `object` type.
    #[serde(default)]
    pub flexible: bool,
    /// Optional `ASSERT` expression.
    #[serde(default)]
    pub assert: Option<String>,
    /// Optional `DEFAULT` expression.
    #[serde(default)]
    pub default_expr: Option<String>,
    /// Optional `VALUE` expression (computed/derived value).
    #[serde(default)]
    pub value_expr: Option<String>,
    /// Mark the field `READONLY`.
    #[serde(default)]
    pub readonly: bool,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Index kind for `DEFINE INDEX`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(rename_all = "snake_case")]
pub enum IndexKind {
    /// Standard (non-unique) BTree index — the default.
    Normal,
    /// Unique constraint index.
    Unique,
    /// Full-text SEARCH index with an analyzer name.
    Search {
        /// Analyzer name (must match a `DEFINE ANALYZER`).
        analyzer: String,
        /// Optional BM25 settings as `(k1, b)`.
        bm25: Option<[f64; 2]>,
        /// Highlight snippets.
        #[serde(default)]
        highlights: bool,
    },
    /// MTREE vector index.
    Mtree {
        /// Number of dimensions.
        dimension: u32,
    },
    /// HNSW vector index.
    Hnsw {
        /// Number of dimensions.
        dimension: u32,
    },
}

/// Parameters for `DEFINE INDEX`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineIndexParams {
    /// Index name.
    pub name: String,
    /// Table name.
    pub table: String,
    /// Fields to index (comma-separated or single name).
    pub fields: Vec<String>,
    /// Index kind.
    #[serde(default = "default_index_kind")]
    pub kind: IndexKind,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}
fn default_index_kind() -> IndexKind {
    IndexKind::Normal
}

/// Parameters for `DEFINE EVENT`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineEventParams {
    /// Event name.
    pub name: String,
    /// Table name.
    pub table: String,
    /// `WHEN` condition expression.
    pub when_expr: String,
    /// `THEN` action expression (SurrealQL block or statement).
    pub then_expr: String,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// A single function argument `(name, type_str)` pair.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct FnArg {
    /// Argument name without `$` sigil (e.g. `"value"`).
    pub name: String,
    /// SurrealQL type (e.g. `"string"`, `"option<int>"`).
    pub type_str: String,
}

/// Parameters for `DEFINE FUNCTION`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineFunctionParams {
    /// Fully-qualified function name (e.g. `"fn::greet"`).
    pub name: String,
    /// Function arguments.
    #[serde(default)]
    pub args: Vec<FnArg>,
    /// Return type string.
    #[serde(default)]
    pub returns: Option<String>,
    /// Function body (raw SurrealQL between `{` … `}`).
    pub body: String,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Parameters for `DEFINE PARAM`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineParamParams {
    /// Parameter name without `$` sigil.
    pub name: String,
    /// Value expression.
    pub value: String,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Parameters for `DEFINE ANALYZER`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineAnalyzerParams {
    /// Analyzer name.
    pub name: String,
    /// Tokenizers (e.g. `["blank", "camel"]`).
    #[serde(default)]
    pub tokenizers: Vec<String>,
    /// Filters (e.g. `["lowercase", "ascii"]`).
    #[serde(default)]
    pub filters: Vec<String>,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Scope for access rules: `root`, `namespace`, or `database`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(rename_all = "snake_case")]
pub enum AccessScope {
    /// Root-level access.
    Root,
    /// Namespace-level access.
    Namespace,
    /// Database-level access.
    Database,
}

/// Parameters for `DEFINE ACCESS … TYPE JWT`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineAccessJwtParams {
    /// Access name.
    pub name: String,
    /// Scope of the access rule.
    pub scope: AccessScope,
    /// JWT algorithm (e.g. `"HS512"`, `"RS256"`).
    pub algorithm: String,
    /// Secret or public-key material.
    pub key: String,
    /// Optional session duration.
    #[serde(default)]
    pub session: Option<String>,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Parameters for `DEFINE ACCESS … TYPE RECORD`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineAccessRecordParams {
    /// Access name.
    pub name: String,
    /// SIGNUP expression.
    pub signup: String,
    /// SIGNIN expression.
    pub signin: String,
    /// Optional session duration.
    #[serde(default)]
    pub session: Option<String>,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Parameters for `DEFINE USER`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineUserParams {
    /// Username.
    pub name: String,
    /// Scope.
    pub scope: AccessScope,
    /// Roles (e.g. `["owner"]`, `["editor"]`).
    pub roles: Vec<String>,
    /// Password (or `PASSHASH` value with `is_hash = true`).
    pub password: String,
    /// Whether `password` is already a bcrypt hash.
    #[serde(default)]
    pub is_hash: bool,
    /// Emit with `IF NOT EXISTS`.
    #[serde(default)]
    pub if_not_exists: bool,
}

/// Parameters for `REMOVE TABLE`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoveTableParams {
    /// Table name.
    pub name: String,
}

/// Parameters for `REMOVE FIELD`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoveFieldParams {
    /// Field name.
    pub name: String,
    /// Table name.
    pub table: String,
}

/// Parameters for `REMOVE INDEX`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoveIndexParams {
    /// Index name.
    pub name: String,
    /// Table name.
    pub table: String,
}

/// Parameters for `INFO FOR DB`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InfoForDbParams {
    /// Optionally restrict to a specific table.
    #[serde(default)]
    pub table: Option<String>,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn ine(flag: bool) -> &'static str {
    if flag { " IF NOT EXISTS" } else { "" }
}

fn scope_str(s: &AccessScope) -> &'static str {
    match s {
        AccessScope::Root => "ROOT",
        AccessScope::Namespace => "NAMESPACE",
        AccessScope::Database => "DATABASE",
    }
}

// ── plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing SurrealQL DDL authoring tools.
///
/// Each tool emits a complete SurrealQL `DEFINE` or `REMOVE` statement.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_schema")]
pub struct SurrealSchemaPlugin;

impl SurrealSchemaPlugin {
    /// Creates a new schema plugin.
    pub fn new() -> Self {
        Self
    }
}

impl Default for SurrealSchemaPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_namespace",
    description = "Emit DEFINE NAMESPACE statement."
)]
#[instrument(skip_all)]
async fn define_namespace(p: DefineNamespaceParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "DEFINE NAMESPACE{} {};",
        ine(p.if_not_exists),
        p.name
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_database",
    description = "Emit DEFINE DATABASE statement with optional CHANGEFEED duration."
)]
#[instrument(skip_all)]
async fn define_database(p: DefineDatabaseParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!("DEFINE DATABASE{} {}", ine(p.if_not_exists), p.name);
    if let Some(cf) = &p.changefeed {
        s.push_str(&format!(" CHANGEFEED {cf}"));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_table",
    description = "Emit DEFINE TABLE statement. Supports SCHEMAFULL, DROP, CHANGEFEED, AS SELECT, and PERMISSIONS."
)]
#[instrument(skip_all)]
async fn define_table(p: DefineTableParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!("DEFINE TABLE{} {}", ine(p.if_not_exists), p.name);
    if p.drop {
        s.push_str(" DROP");
    }
    if p.schemafull {
        s.push_str(" SCHEMAFULL");
    } else {
        s.push_str(" SCHEMALESS");
    }
    if let Some(cf) = &p.changefeed {
        s.push_str(&format!(" CHANGEFEED {cf}"));
    }
    if let Some(sel) = &p.as_select {
        s.push_str(&format!(" AS (\n  {sel}\n)"));
    }
    if let Some(perms) = &p.permissions {
        s.push_str(&format!("\n  PERMISSIONS {perms}"));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_field",
    description = "Emit DEFINE FIELD statement. Supports TYPE, FLEXIBLE, ASSERT, DEFAULT, VALUE, and READONLY."
)]
#[instrument(skip_all)]
async fn define_field(p: DefineFieldParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!(
        "DEFINE FIELD{} {} ON TABLE {}",
        ine(p.if_not_exists),
        p.name,
        p.table
    );
    if p.flexible {
        s.push_str(" FLEXIBLE");
    }
    if let Some(k) = &p.kind {
        s.push_str(&format!(" TYPE {k}"));
    }
    if let Some(a) = &p.assert {
        s.push_str(&format!(" ASSERT {a}"));
    }
    if let Some(d) = &p.default_expr {
        s.push_str(&format!(" DEFAULT {d}"));
    }
    if let Some(v) = &p.value_expr {
        s.push_str(&format!(" VALUE {v}"));
    }
    if p.readonly {
        s.push_str(" READONLY");
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_index",
    description = "Emit DEFINE INDEX statement. Supports UNIQUE, SEARCH ANALYZER (BM25 + HIGHLIGHTS), MTREE, and HNSW vector kinds."
)]
#[instrument(skip_all)]
async fn define_index(p: DefineIndexParams) -> Result<CallToolResult, ErrorData> {
    let fields = p.fields.join(", ");
    let mut s = format!(
        "DEFINE INDEX{} {} ON TABLE {} FIELDS {}",
        ine(p.if_not_exists),
        p.name,
        p.table,
        fields
    );
    match &p.kind {
        IndexKind::Normal => {}
        IndexKind::Unique => s.push_str(" UNIQUE"),
        IndexKind::Search {
            analyzer,
            bm25,
            highlights,
        } => {
            s.push_str(&format!(" SEARCH ANALYZER {analyzer}"));
            if let Some([k1, b]) = bm25 {
                s.push_str(&format!(" BM25({k1},{b})"));
            }
            if *highlights {
                s.push_str(" HIGHLIGHTS");
            }
        }
        IndexKind::Mtree { dimension } => {
            s.push_str(&format!(" MTREE DIMENSION {dimension}"));
        }
        IndexKind::Hnsw { dimension } => {
            s.push_str(&format!(" HNSW DIMENSION {dimension}"));
        }
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_event",
    description = "Emit DEFINE EVENT statement with WHEN condition and THEN action body."
)]
#[instrument(skip_all)]
async fn define_event(p: DefineEventParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "DEFINE EVENT{} {} ON TABLE {} WHEN {} THEN {};",
        ine(p.if_not_exists),
        p.name,
        p.table,
        p.when_expr,
        p.then_expr,
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_function",
    description = "Emit DEFINE FUNCTION statement with typed arguments and optional return type."
)]
#[instrument(skip_all)]
async fn define_function(p: DefineFunctionParams) -> Result<CallToolResult, ErrorData> {
    let args: String = p
        .args
        .iter()
        .map(|a| format!("${}: {}", a.name, a.type_str))
        .collect::<Vec<_>>()
        .join(", ");
    let ret = p
        .returns
        .as_deref()
        .map(|r| format!(" -> {r}"))
        .unwrap_or_default();
    ok_text(format!(
        "DEFINE FUNCTION{} {}({args}){ret} {{\n{}\n}};",
        ine(p.if_not_exists),
        p.name,
        p.body,
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_param",
    description = "Emit DEFINE PARAM statement setting a global SurrealDB parameter."
)]
#[instrument(skip_all)]
async fn define_param(p: DefineParamParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "DEFINE PARAM{} ${} VALUE {};",
        ine(p.if_not_exists),
        p.name,
        p.value,
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_analyzer",
    description = "Emit DEFINE ANALYZER statement with tokenizer and filter pipeline."
)]
#[instrument(skip_all)]
async fn define_analyzer(p: DefineAnalyzerParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!("DEFINE ANALYZER{} {}", ine(p.if_not_exists), p.name);
    if !p.tokenizers.is_empty() {
        s.push_str(&format!(" TOKENIZERS {}", p.tokenizers.join(", ")));
    }
    if !p.filters.is_empty() {
        s.push_str(&format!(" FILTERS {}", p.filters.join(", ")));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_access_jwt",
    description = "Emit DEFINE ACCESS … TYPE JWT statement for JWT-based authentication."
)]
#[instrument(skip_all)]
async fn define_access_jwt(p: DefineAccessJwtParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!(
        "DEFINE ACCESS{} {} ON {} TYPE JWT ALGORITHM {} KEY \"{}\"",
        ine(p.if_not_exists),
        p.name,
        scope_str(&p.scope),
        p.algorithm,
        p.key,
    );
    if let Some(sess) = &p.session {
        s.push_str(&format!(" SESSION {sess}"));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_access_record",
    description = "Emit DEFINE ACCESS … TYPE RECORD statement for record-based auth with SIGNUP and SIGNIN expressions."
)]
#[instrument(skip_all)]
async fn define_access_record(p: DefineAccessRecordParams) -> Result<CallToolResult, ErrorData> {
    let mut s = format!(
        "DEFINE ACCESS{} {} ON DATABASE TYPE RECORD\n  SIGNUP ({})\n  SIGNIN ({})",
        ine(p.if_not_exists),
        p.name,
        p.signup,
        p.signin,
    );
    if let Some(sess) = &p.session {
        s.push_str(&format!(" SESSION {sess}"));
    }
    s.push(';');
    ok_text(s)
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "define_user",
    description = "Emit DEFINE USER statement with password or passhash and role list."
)]
#[instrument(skip_all)]
async fn define_user(p: DefineUserParams) -> Result<CallToolResult, ErrorData> {
    let pw_clause = if p.is_hash {
        format!("PASSHASH \"{}\"", p.password)
    } else {
        format!("PASSWORD \"{}\"", p.password)
    };
    let roles = p.roles.join(", ");
    ok_text(format!(
        "DEFINE USER{} {} ON {} {} ROLES {};",
        ine(p.if_not_exists),
        p.name,
        scope_str(&p.scope),
        pw_clause,
        roles,
    ))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "remove_table",
    description = "Emit REMOVE TABLE statement."
)]
#[instrument(skip_all)]
async fn remove_table(p: RemoveTableParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("REMOVE TABLE {};", p.name))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "remove_field",
    description = "Emit REMOVE FIELD … ON TABLE statement."
)]
#[instrument(skip_all)]
async fn remove_field(p: RemoveFieldParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("REMOVE FIELD {} ON TABLE {};", p.name, p.table))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "remove_index",
    description = "Emit REMOVE INDEX … ON TABLE statement."
)]
#[instrument(skip_all)]
async fn remove_index(p: RemoveIndexParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("REMOVE INDEX {} ON TABLE {};", p.name, p.table))
}

#[elicit_tool(
    plugin = "surreal_schema",
    name = "info_for_db",
    description = "Emit INFO FOR DB schema introspection query, or INFO FOR TABLE for a specific table."
)]
#[instrument(skip_all)]
async fn info_for_db(p: InfoForDbParams) -> Result<CallToolResult, ErrorData> {
    ok_text(match p.table {
        Some(t) => format!("INFO FOR TABLE {t};"),
        None => "INFO FOR DB;".to_owned(),
    })
}
