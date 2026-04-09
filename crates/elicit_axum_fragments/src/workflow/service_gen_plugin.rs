//! AxumServiceGenPlugin — emit complete axum service scaffold fragments.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// An environment variable binding for service config.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EnvVar {
    /// The struct field name.
    pub name: String,
    /// The environment variable key.
    pub env_key: String,
    /// Optional default value expression.
    pub default: Option<String>,
}

/// An API endpoint for documentation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EndpointDoc {
    /// HTTP method (e.g. `GET`).
    pub method: String,
    /// URL path (e.g. `/users/:id`).
    pub path: String,
    /// Human-readable description.
    pub description: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for emit_service_scaffold.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitServiceParams {
    /// Application name used in tracing and listen addresses.
    pub name: String,
    /// TCP port to listen on.
    pub port: u16,
    /// Optional application state type to inject into the router.
    pub state_type: Option<String>,
}

/// Parameters for emit_crud_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitCrudServiceParams {
    /// Resource name in singular lowercase (e.g. `user`).
    pub resource: String,
    /// Database pool type (e.g. `PgPool`).
    pub db_type: String,
    /// TCP port to listen on.
    pub port: u16,
}

/// Parameters for emit_authenticated_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitAuthServiceParams {
    /// Application name.
    pub name: String,
    /// Authentication type/scheme (e.g. `Bearer`, `ApiKey`).
    pub auth_type: String,
    /// TCP port to listen on.
    pub port: u16,
}

/// Parameters for emit_websocket_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitWsServiceParams {
    /// Application name.
    pub name: String,
    /// TCP port to listen on.
    pub port: u16,
}

/// Parameters for emit_file_server_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitFileServerParams {
    /// Directory to serve files from.
    pub dir: String,
    /// TCP port to listen on.
    pub port: u16,
    /// Whether to serve `index.html` as a fallback for unknown paths (SPA mode).
    pub spa_mode: bool,
}

/// Parameters for emit_proxy_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitProxyParams {
    /// Upstream service URL to forward requests to.
    pub upstream_url: String,
    /// TCP port to listen on.
    pub port: u16,
}

/// Parameters for emit_cargo_toml.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitCargoTomlParams {
    /// Package name.
    pub name: String,
    /// Whether to include sqlx as a dependency.
    pub use_sqlx: bool,
    /// Whether to include jsonwebtoken as a dependency.
    pub use_auth: bool,
    /// Whether to include tracing and tracing-subscriber.
    pub use_tracing: bool,
}

/// Parameters for emit_env_config.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitEnvConfigParams {
    /// Application name used in comments.
    pub app_name: String,
    /// Environment variable definitions.
    pub env_vars: Vec<EnvVar>,
}

/// Parameters for emit_docker_compose.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitDockerParams {
    /// Application name.
    pub app_name: String,
    /// TCP port the application listens on.
    pub port: u16,
    /// Whether to include a database service.
    pub use_db: bool,
    /// Database type (e.g. `postgres`, `mysql`).
    pub db_type: String,
}

/// Parameters for emit_readme.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitReadmeParams {
    /// Application name.
    pub app_name: String,
    /// Short description of the application.
    pub description: String,
    /// API endpoint documentation entries.
    pub endpoints: Vec<EndpointDoc>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_service_scaffold",
    description = "Emit a complete main.rs with tokio::main, router setup, and axum::serve."
)]
#[instrument]
async fn emit_service_scaffold(p: EmitServiceParams) -> Result<CallToolResult, ErrorData> {
    let state_line = match &p.state_type {
        Some(st) => format!(
            "\n    let state = {}::new();\n    let app = router.with_state(state);",
            st
        ),
        None => "\n    let app = router;".to_string(),
    };
    let port = p.port;
    let name = &p.name;
    let code = format!(
        r#"use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {{
    tracing_subscriber::fmt::init();
    tracing::info!("Starting {name}");

    let router = Router::new();
    // TODO: add routes
{state_line}

    let listener = TcpListener::bind("0.0.0.0:{port}").await.unwrap();
    tracing::info!("Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}}"#,
        name = name,
        port = port,
        state_line = state_line,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_crud_service",
    description = "Emit a complete CRUD service with handlers, routes, and state for a resource."
)]
#[instrument]
async fn emit_crud_service(p: EmitCrudServiceParams) -> Result<CallToolResult, ErrorData> {
    let r = &p.resource;
    let db = &p.db_type;
    let port = p.port;
    let code = format!(
        r#"// ── State ──────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AppState {{
    pub db: {db},
}}

// ── Handlers ────────────────────────────────────────────────────────────────

pub async fn list_{r}s(State(_state): State<AppState>) -> impl IntoResponse {{
    StatusCode::OK
}}

pub async fn create_{r}(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {{
    let _ = payload;
    StatusCode::CREATED
}}

pub async fn get_{r}(
    State(_state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {{
    let _ = id;
    StatusCode::OK
}}

pub async fn update_{r}(
    State(_state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {{
    let _ = (id, payload);
    StatusCode::OK
}}

pub async fn delete_{r}(
    State(_state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {{
    let _ = id;
    StatusCode::NO_CONTENT
}}

// ── Routes ───────────────────────────────────────────────────────────────────

pub fn {r}_routes() -> Router {{
    Router::new()
        .route("/{r}s", get(list_{r}s).post(create_{r}))
        .route("/{r}s/:id", get(get_{r}).put(update_{r}).delete(delete_{r}))
}}

// ── Main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {{
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = {db}::connect(&db_url).await.unwrap();
    let state = AppState {{ db }};

    let app = {r}_routes().with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:{port}").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}}"#,
        r = r,
        db = db,
        port = port,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_authenticated_service",
    description = "Emit a service scaffold with authentication middleware pre-configured."
)]
#[instrument]
async fn emit_authenticated_service(p: EmitAuthServiceParams) -> Result<CallToolResult, ErrorData> {
    let name = &p.name;
    let auth_type = &p.auth_type;
    let port = p.port;
    let code = format!(
        r#"use axum::{{Router, middleware}};
use tokio::net::TcpListener;

/// Validates {auth_type} authorization on incoming requests.
pub async fn auth_middleware(request: axum::extract::Request, next: axum::middleware::Next) -> axum::response::Response {{
    // TODO: validate {auth_type} token from Authorization header
    next.run(request).await
}}

#[tokio::main]
async fn main() {{
    tracing_subscriber::fmt::init();
    tracing::info!("Starting {name} with {auth_type} auth");

    let protected = Router::new()
        // TODO: add protected routes
        .route_layer(middleware::from_fn(auth_middleware));

    let app = Router::new().merge(protected);

    let listener = TcpListener::bind("0.0.0.0:{port}").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}}"#,
        name = name,
        auth_type = auth_type,
        port = port,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_websocket_service",
    description = "Emit a service scaffold with a WebSocket handler and upgrade endpoint."
)]
#[instrument]
async fn emit_websocket_service(p: EmitWsServiceParams) -> Result<CallToolResult, ErrorData> {
    let name = &p.name;
    let port = p.port;
    let code = format!(
        r#"use axum::{{Router, extract::ws::{{WebSocketUpgrade, WebSocket}}, response::IntoResponse}};
use tokio::net::TcpListener;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {{
    ws.on_upgrade(handle_socket)
}}

async fn handle_socket(mut socket: WebSocket) {{
    while let Some(Ok(msg)) = socket.recv().await {{
        // TODO: process message
        let _ = socket.send(msg).await;
    }}
}}

#[tokio::main]
async fn main() {{
    tracing_subscriber::fmt::init();
    tracing::info!("Starting {name} WebSocket service");

    let app = Router::new()
        .route("/ws", axum::routing::get(ws_handler));

    let listener = TcpListener::bind("0.0.0.0:{port}").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}}"#,
        name = name,
        port = port,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_file_server_service",
    description = "Emit a static file server using tower_http ServeDir, optionally in SPA mode."
)]
#[instrument]
async fn emit_file_server_service(p: EmitFileServerParams) -> Result<CallToolResult, ErrorData> {
    let dir = &p.dir;
    let port = p.port;
    let serve_dir_expr = if p.spa_mode {
        format!(
            "ServeDir::new(\"{dir}\").not_found_service(ServeFile::new(\"{dir}/index.html\"))",
            dir = dir
        )
    } else {
        format!("ServeDir::new(\"{}\")", dir)
    };
    let code = format!(
        r#"use axum::Router;
use tower_http::services::{{ServeDir, ServeFile}};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {{
    let serve_dir = {serve_dir_expr};

    let app = Router::new()
        .nest_service("/", serve_dir);

    let listener = TcpListener::bind("0.0.0.0:{port}").await.unwrap();
    tracing::info!("Serving files from \"{dir}\" on port {port}");
    axum::serve(listener, app).await.unwrap();
}}"#,
        serve_dir_expr = serve_dir_expr,
        port = port,
        dir = dir,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_proxy_service",
    description = "Emit a reverse proxy service forwarding all requests to an upstream URL."
)]
#[instrument]
async fn emit_proxy_service(p: EmitProxyParams) -> Result<CallToolResult, ErrorData> {
    let upstream = &p.upstream_url;
    let port = p.port;
    let code = format!(
        r#"use axum::{{Router, extract::Request, response::IntoResponse}};
use hyper::{{StatusCode, Uri}};
use tokio::net::TcpListener;

pub async fn proxy_handler(mut request: Request) -> impl IntoResponse {{
    let upstream = "{upstream}";
    let path = request.uri().path_and_query().map(|p| p.as_str()).unwrap_or("/");
    let new_uri = format!("{{}}{{}}", upstream, path).parse::<Uri>().unwrap();
    *request.uri_mut() = new_uri;

    // TODO: forward request using reqwest or hyper client
    (StatusCode::BAD_GATEWAY, "Proxy not fully implemented")
}}

#[tokio::main]
async fn main() {{
    tracing_subscriber::fmt::init();
    tracing::info!("Starting reverse proxy -> {upstream}");

    let app = Router::new().fallback(proxy_handler);

    let listener = TcpListener::bind("0.0.0.0:{port}").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}}"#,
        upstream = upstream,
        port = port,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_cargo_toml",
    description = "Emit a Cargo.toml with axum, tokio, serde, and optional sqlx/auth/tracing deps."
)]
#[instrument]
async fn emit_cargo_toml(p: EmitCargoTomlParams) -> Result<CallToolResult, ErrorData> {
    let mut deps = String::new();
    deps.push_str("axum = \"0.8\"\n");
    deps.push_str("tokio = { version = \"1\", features = [\"full\"] }\n");
    deps.push_str("serde = { version = \"1\", features = [\"derive\"] }\n");
    deps.push_str("serde_json = \"1\"\n");
    if p.use_sqlx {
        deps.push_str(
            "sqlx = { version = \"0.8\", features = [\"postgres\", \"runtime-tokio\", \"tls-rustls\"] }\n",
        );
    }
    if p.use_auth {
        deps.push_str("jsonwebtoken = \"9\"\n");
    }
    if p.use_tracing {
        deps.push_str("tracing = \"0.1\"\n");
        deps.push_str("tracing-subscriber = { version = \"0.3\", features = [\"env-filter\"] }\n");
    }
    let content = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n{}",
        p.name, deps
    );
    Ok(CallToolResult::success(vec![Content::text(content)]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_env_config",
    description = "Emit a .env template file with all required environment variables."
)]
#[instrument]
async fn emit_env_config(p: EmitEnvConfigParams) -> Result<CallToolResult, ErrorData> {
    let mut lines = Vec::new();
    lines.push(format!("# {} environment configuration", p.app_name));
    for v in &p.env_vars {
        if let Some(default) = &v.default {
            lines.push(format!("{}={}", v.env_key, default));
        } else {
            lines.push(format!("{}=", v.env_key));
        }
    }
    Ok(CallToolResult::success(vec![Content::text(
        lines.join("\n"),
    )]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_docker_compose",
    description = "Emit a docker-compose.yml for the application, optionally with a database service."
)]
#[instrument]
async fn emit_docker_compose(p: EmitDockerParams) -> Result<CallToolResult, ErrorData> {
    let app = &p.app_name;
    let port = p.port;
    let mut content = format!(
        "version: \"3.9\"\n\nservices:\n  {}:\n    build: .\n    ports:\n      - \"{}:{}\"\n    environment:\n      - RUST_LOG=info\n",
        app, port, port
    );
    if p.use_db {
        let db_service = match p.db_type.as_str() {
            "mysql" => "mysql:8",
            "sqlite" => "",
            _ => "postgres:16-alpine",
        };
        if !db_service.is_empty() {
            content.push_str(&format!(
                "    depends_on:\n      - db\n\n  db:\n    image: {}\n    environment:\n      POSTGRES_DB: {}\n      POSTGRES_USER: app\n      POSTGRES_PASSWORD: password\n    ports:\n      - \"5432:5432\"\n",
                db_service, app
            ));
        }
    }
    Ok(CallToolResult::success(vec![Content::text(content)]))
}

#[elicit_tool(
    plugin = "axum_service_gen",
    emit = None,
    name = "emit_readme",
    description = "Emit a README.md with description, endpoints table, and quick-start instructions."
)]
#[instrument]
async fn emit_readme(p: EmitReadmeParams) -> Result<CallToolResult, ErrorData> {
    let header = format!("# {}\n\n{}\n\n", p.app_name, p.description);
    let mut table = String::from(
        "## API Endpoints\n\n| Method | Path | Description |\n|--------|------|-------------|\n",
    );
    for ep in &p.endpoints {
        table.push_str(&format!(
            "| {} | `{}` | {} |\n",
            ep.method, ep.path, ep.description
        ));
    }
    let quickstart = "\n## Quick Start\n\n```bash\ncargo run\n```\n";
    let content = format!("{}{}{}", header, table, quickstart);
    Ok(CallToolResult::success(vec![Content::text(content)]))
}

/// Plugin exposing complete axum service scaffold generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_service_gen")]
pub struct AxumServiceGenPlugin;
