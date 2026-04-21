//! SurrealConnectionPlugin — MCP tools for SurrealDB Rust SDK connection boilerplate.

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

/// Parameters for `surreal_connection__ws_client`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WsClientParams {
    /// WebSocket host (e.g. `"localhost"`).
    pub host: String,
    /// WebSocket port (e.g. `8000`).
    pub port: u16,
}

/// Parameters for `surreal_connection__http_client`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HttpClientParams {
    /// Full HTTP URL (e.g. `"http://localhost:8000"`).
    pub url: String,
}

/// Parameters for `surreal_connection__surrealkv_client`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SurrealkvClientParams {
    /// Filesystem path for the embedded SurrealKV store.
    pub path: String,
}

/// Parameters for `surreal_connection__signin_root`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SigninRootParams {
    /// Root username.
    pub username: String,
}

/// Parameters for `surreal_connection__signin_ns`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SigninNsParams {
    /// Target namespace.
    pub namespace: String,
    /// Namespace-level username.
    pub username: String,
}

/// Parameters for `surreal_connection__signin_db`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SigninDbParams {
    /// Target namespace.
    pub namespace: String,
    /// Target database.
    pub database: String,
    /// Database-level username.
    pub username: String,
}

/// Parameters for `surreal_connection__signin_record`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SigninRecordParams {
    /// Target namespace.
    pub namespace: String,
    /// Target database.
    pub database: String,
    /// Record access method name.
    pub access: String,
}

/// Parameters for `surreal_connection__use_ns_db`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UseNsDbParams {
    /// Namespace to select.
    pub namespace: String,
    /// Database to select.
    pub database: String,
}

/// Parameters for `surreal_connection__full_setup`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FullSetupParams {
    /// WebSocket host.
    pub host: String,
    /// WebSocket port.
    pub port: u16,
    /// Namespace to select after connecting.
    pub namespace: String,
    /// Database to select after connecting.
    pub database: String,
    /// Root username for sign-in.
    pub username: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_connection",
    name = "ws_client",
    description = "Generate a Rust snippet to connect to SurrealDB via WebSocket."
)]
#[instrument]
async fn ws_client(p: WsClientParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::Surreal;\nuse surrealdb::engine::remote::ws::Ws;\n\nlet db = Surreal::new::<Ws>(\"{}:{}\").await?;",
        p.host, p.port
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "http_client",
    description = "Generate a Rust snippet to connect to SurrealDB via HTTP."
)]
#[instrument]
async fn http_client(p: HttpClientParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::Surreal;\nuse surrealdb::engine::remote::http::Http;\n\nlet db = Surreal::new::<Http>(\"{}\").await?;",
        p.url
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "memory_client",
    description = "Generate a Rust snippet to create an in-memory SurrealDB instance."
)]
#[instrument]
async fn memory_client(_p: EmptyParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(
        "use surrealdb::Surreal;\nuse surrealdb::engine::local::Mem;\n\nlet db = Surreal::new::<Mem>(()).await?;",
    )
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "surrealkv_client",
    description = "Generate a Rust snippet to open an embedded SurrealKV store."
)]
#[instrument]
async fn surrealkv_client(
    p: SurrealkvClientParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::Surreal;\nuse surrealdb::engine::local::SurrealKV;\n\nlet db = Surreal::new::<SurrealKV>(\"{}\").await?;",
        p.path
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "signin_root",
    description = "Generate a Rust snippet for root-level sign-in. Password is a placeholder."
)]
#[instrument]
async fn signin_root(p: SigninRootParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::opt::auth::Root;\n\ndb.signin(Root {{\n    username: \"{}\",\n    password: \"<password>\",\n}}).await?;",
        p.username
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "signin_ns",
    description = "Generate a Rust snippet for namespace-level sign-in. Password is a placeholder."
)]
#[instrument]
async fn signin_ns(p: SigninNsParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::opt::auth::Namespace;\n\ndb.signin(Namespace {{\n    namespace: \"{ns}\",\n    username: \"{user}\",\n    password: \"<password>\",\n}}).await?;",
        ns = p.namespace,
        user = p.username
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "signin_db",
    description = "Generate a Rust snippet for database-level sign-in. Password is a placeholder."
)]
#[instrument]
async fn signin_db(p: SigninDbParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::opt::auth::Database;\n\ndb.signin(Database {{\n    namespace: \"{ns}\",\n    database: \"{db}\",\n    username: \"{user}\",\n    password: \"<password>\",\n}}).await?;",
        ns = p.namespace,
        db = p.database,
        user = p.username
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "signin_record",
    description = "Generate a Rust snippet for record-access sign-in with a custom credentials struct."
)]
#[instrument]
async fn signin_record(p: SigninRecordParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use serde::Serialize;\nuse surrealdb::opt::auth::Record;\n\n#[derive(Serialize)]\nstruct Credentials {{\n    // Add your credential fields here\n}}\n\ndb.signin(Record {{\n    namespace: \"{ns}\",\n    database: \"{db}\",\n    access: \"{access}\",\n    params: Credentials {{ /* ... */ }},\n}}).await?;",
        ns = p.namespace,
        db = p.database,
        access = p.access
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "use_ns_db",
    description = "Generate a Rust snippet to select a namespace and database."
)]
#[instrument]
async fn use_ns_db(p: UseNsDbParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "db.use_ns(\"{ns}\").use_db(\"{db}\").await?;",
        ns = p.namespace,
        db = p.database
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "full_setup",
    description = "Generate a complete Rust SurrealDB setup: connect via WebSocket, sign in as root, and select namespace/database."
)]
#[instrument]
async fn full_setup(p: FullSetupParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::Surreal;\nuse surrealdb::engine::remote::ws::Ws;\nuse surrealdb::opt::auth::Root;\n\nlet db = Surreal::new::<Ws>(\"{}:{}\").await?;\n\ndb.signin(Root {{\n    username: \"{}\",\n    password: \"<password>\",\n}}).await?;\n\ndb.use_ns(\"{}\").use_db(\"{}\").await?;",
        p.host, p.port, p.username, p.namespace, p.database
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing SurrealDB Rust SDK connection boilerplate tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_connection")]
pub struct SurrealConnectionPlugin;
