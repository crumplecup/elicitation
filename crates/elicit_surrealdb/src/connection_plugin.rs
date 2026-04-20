//! `SurrealConnectionPlugin` — Rust SDK connection/auth code generation tools.

use crate::config::{Config, ExperimentalFeature, PlannerStrategy};
use elicitation::{ElicitPlugin, elicit_tool};
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

/// Parameters for WebSocket client snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WsClientParams {
    /// Host and port (e.g. `"localhost:8000"`).
    pub host: String,
    /// Optional connection config.
    #[serde(default)]
    pub config: Option<Config>,
}

/// Parameters for HTTP client snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HttpClientParams {
    /// HTTP/HTTPS URL (e.g. `"http://localhost:8000"`).
    pub url: String,
    /// Optional connection config.
    #[serde(default)]
    pub config: Option<Config>,
}

/// Parameters for in-memory client snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemoryClientParams {
    /// Optional connection config.
    #[serde(default)]
    pub config: Option<Config>,
}

/// Parameters for SurrealKV embedded client snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SurrealKvClientParams {
    /// File system path for the SurrealKV store.
    pub path: String,
    /// Optional connection config.
    #[serde(default)]
    pub config: Option<Config>,
}

/// Parameters for root sign-in snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SigninRootParams {
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
}

/// Parameters for namespace sign-in snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SigninNsParams {
    /// Namespace.
    pub namespace: String,
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
}

/// Parameters for database sign-in snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SigninDbParams {
    /// Namespace.
    pub namespace: String,
    /// Database.
    pub database: String,
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
}

/// Parameters for record (scoped) sign-in snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SigninRecordParams {
    /// Access name (must match a `DEFINE ACCESS … TYPE RECORD`).
    pub access: String,
    /// Variable name for the params struct.
    pub params_var: String,
}

/// Parameters for `use_ns`/`use_db` snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UseNsDbParams {
    /// Namespace.
    pub namespace: String,
    /// Database.
    pub database: String,
}

/// Parameters for the full boilerplate snippet.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FullSetupParams {
    /// Transport: `"ws"`, `"wss"`, `"http"`, `"https"`, `"mem"`, or `"surrealkv"`.
    pub transport: String,
    /// Host/URL/path depending on transport.
    pub address: String,
    /// Namespace.
    pub namespace: String,
    /// Database.
    pub database: String,
    /// Username (root).
    pub username: String,
    /// Password (root).
    pub password: String,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn config_snippet(cfg: &Config) -> String {
    let mut s = String::from("let config = Config::new()\n");
    if let Some(secs) = cfg.query_timeout_secs {
        s.push_str(&format!(
            "    .query_timeout(Duration::from_secs({secs}))\n"
        ));
    }
    if let Some(secs) = cfg.transaction_timeout_secs {
        s.push_str(&format!(
            "    .transaction_timeout(Duration::from_secs({secs}))\n"
        ));
    }
    for feat in &cfg.experimental {
        let feat_str = match feat {
            ExperimentalFeature::Files => "ExperimentalFeature::Files",
            ExperimentalFeature::Surrealism => "ExperimentalFeature::Surrealism",
        };
        s.push_str(&format!("    .with_experimental({feat_str})\n"));
    }
    if let Some(ps) = &cfg.planner_strategy {
        let ps_str = match ps {
            PlannerStrategy::Default => "PlannerStrategy::Default",
            PlannerStrategy::FullTableScan => "PlannerStrategy::FullTableScan",
            PlannerStrategy::IndexedScan => "PlannerStrategy::IndexedScan",
        };
        s.push_str(&format!("    .planner_strategy({ps_str})\n"));
    }
    s.push(';');
    s
}

// ── plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing Rust SDK connection and authentication code generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_connection")]
pub struct SurrealConnectionPlugin;

impl SurrealConnectionPlugin {
    /// Creates a new connection plugin.
    pub fn new() -> Self {
        Self
    }
}

impl Default for SurrealConnectionPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "surreal_connection",
    name = "ws_client",
    description = "Emit a Surreal::new::<Ws>() WebSocket client connection code snippet."
)]
#[instrument(skip_all)]
async fn ws_client(p: WsClientParams) -> Result<CallToolResult, ErrorData> {
    let cfg = p.config.as_ref().map(config_snippet).unwrap_or_default();
    let cfg_arg = if p.config.is_some() { "config" } else { "" };
    ok_text(format!(
        "use surrealdb::{{Surreal, engine::remote::ws::Ws}};\n\
{cfg}\
let db = Surreal::new::<Ws>((\"{host}\", {cfg_arg})).await?;\n",
        host = p.host,
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "http_client",
    description = "Emit a Surreal::new::<Http>() HTTP client connection code snippet."
)]
#[instrument(skip_all)]
async fn http_client(p: HttpClientParams) -> Result<CallToolResult, ErrorData> {
    let cfg = p.config.as_ref().map(config_snippet).unwrap_or_default();
    let cfg_arg = if p.config.is_some() { "config" } else { "" };
    ok_text(format!(
        "use surrealdb::{{Surreal, engine::remote::http::Http}};\n\
{cfg}\
let db = Surreal::new::<Http>((\"{url}\", {cfg_arg})).await?;\n",
        url = p.url,
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "memory_client",
    description = "Emit a Surreal::new::<Mem>() in-memory embedded database code snippet."
)]
#[instrument(skip_all)]
async fn memory_client(p: MemoryClientParams) -> Result<CallToolResult, ErrorData> {
    let cfg = p.config.as_ref().map(config_snippet).unwrap_or_default();
    let cfg_arg = if p.config.is_some() { "config" } else { "()" };
    ok_text(format!(
        "use surrealdb::{{Surreal, engine::local::Mem}};\n\
{cfg}\
let db = Surreal::new::<Mem>({cfg_arg}).await?;\n"
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "surrealkv_client",
    description = "Emit a Surreal::new::<SurrealKv>() embedded persistent database code snippet."
)]
#[instrument(skip_all)]
async fn surrealkv_client(p: SurrealKvClientParams) -> Result<CallToolResult, ErrorData> {
    let cfg = p.config.as_ref().map(config_snippet).unwrap_or_default();
    let cfg_arg = if p.config.is_some() {
        format!("(\"{}\", config)", p.path)
    } else {
        format!("\"{}\"", p.path)
    };
    ok_text(format!(
        "use surrealdb::{{Surreal, engine::local::SurrealKv}};\n\
{cfg}\
let db = Surreal::new::<SurrealKv>({cfg_arg}).await?;\n"
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "signin_root",
    description = "Emit a db.signin(Root { … }).await? root-credential code snippet."
)]
#[instrument(skip_all)]
async fn signin_root(p: SigninRootParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::opt::auth::Root;\n\
db.signin(Root {{ username: \"{u}\", password: \"{pw}\" }}).await?;\n",
        u = p.username,
        pw = p.password,
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "signin_ns",
    description = "Emit a db.signin(Namespace { … }).await? namespace-scoped sign-in code snippet."
)]
#[instrument(skip_all)]
async fn signin_ns(p: SigninNsParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::opt::auth::Namespace;\n\
db.signin(Namespace {{ namespace: \"{ns}\", username: \"{u}\", password: \"{pw}\" }}).await?;\n",
        ns = p.namespace,
        u = p.username,
        pw = p.password,
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "signin_db",
    description = "Emit a db.signin(Database { … }).await? database-scoped sign-in code snippet."
)]
#[instrument(skip_all)]
async fn signin_db(p: SigninDbParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::opt::auth::Database;\n\
db.signin(Database {{ namespace: \"{ns}\", database: \"{db}\", username: \"{u}\", password: \"{pw}\" }}).await?;\n",
        ns = p.namespace,
        db = p.database,
        u = p.username,
        pw = p.password,
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "signin_record",
    description = "Emit a db.signin(Record { … }).await? record-access sign-in code snippet."
)]
#[instrument(skip_all)]
async fn signin_record(p: SigninRecordParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "use surrealdb::opt::auth::Record;\n\
let credentials = Record {{ access: \"{access}\", namespace: &ns, database: &db, params: {var} }};\n\
db.signin(credentials).await?;\n",
        access = p.access,
        var = p.params_var,
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "use_ns_db",
    description = "Emit a db.use_ns(…).use_db(…).await? namespace/database selection code snippet."
)]
#[instrument(skip_all)]
async fn use_ns_db(p: UseNsDbParams) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "db.use_ns(\"{ns}\").use_db(\"{db}\").await?;\n",
        ns = p.namespace,
        db = p.database,
    ))
}

#[elicit_tool(
    plugin = "surreal_connection",
    name = "full_setup",
    description = "Emit a complete SurrealDB connection + sign-in + namespace/database setup boilerplate."
)]
#[instrument(skip_all)]
async fn full_setup(p: FullSetupParams) -> Result<CallToolResult, ErrorData> {
    let (import, new_call) = match p.transport.as_str() {
        "ws" | "wss" => (
            "use surrealdb::{Surreal, engine::remote::ws::Ws};",
            format!("Surreal::new::<Ws>(\"{}\").await?", p.address),
        ),
        "http" | "https" => (
            "use surrealdb::{Surreal, engine::remote::http::Http};",
            format!("Surreal::new::<Http>(\"{}\").await?", p.address),
        ),
        "mem" => (
            "use surrealdb::{Surreal, engine::local::Mem};",
            "Surreal::new::<Mem>(()).await?".to_owned(),
        ),
        "surrealkv" => (
            "use surrealdb::{Surreal, engine::local::SurrealKv};",
            format!("Surreal::new::<SurrealKv>(\"{}\").await?", p.address),
        ),
        other => (
            "use surrealdb::Surreal;",
            format!("/* unknown transport: {other} */"),
        ),
    };
    ok_text(format!(
        "{import}\n\
use surrealdb::opt::auth::Root;\n\n\
let db = {new_call};\n\
db.signin(Root {{ username: \"{u}\", password: \"{pw}\" }}).await?;\n\
db.use_ns(\"{ns}\").use_db(\"{db}\").await?;\n",
        u = p.username,
        pw = p.password,
        ns = p.namespace,
        db = p.database,
    ))
}
