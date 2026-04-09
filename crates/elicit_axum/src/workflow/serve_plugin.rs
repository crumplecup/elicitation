//! `AxumServePlugin` — MCP tools for axum server setup and serving.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// Describes an axum server configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServeDescriptor {
    /// The TCP address the server binds to.
    pub bind_addr: String,
    /// Human-readable description of the serve configuration.
    pub description: String,
    /// Whether graceful shutdown is configured.
    pub graceful_shutdown: bool,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for serve_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ServeParams {
    /// The TCP bind address (e.g. "0.0.0.0:3000").
    pub bind_addr: String,
}

/// Parameters for serve_with_shutdown_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ServeShutdownParams {
    /// The TCP bind address.
    pub bind_addr: String,
    /// Description of the shutdown signal (e.g. "SIGTERM" or "ctrl-c").
    pub signal_description: String,
}

/// Parameters for make_service_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MakeServiceParams {
    /// Description of the router or app being wrapped.
    pub router_description: String,
}

/// Parameters for tcp_listener_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TcpListenerParams {
    /// The address to bind the TCP listener to.
    pub addr: String,
}

/// Parameters for serve_dir_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ServeDirParams {
    /// Filesystem path of the directory to serve.
    pub dir_path: String,
}

/// Parameters for serve_file_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ServeFileParams {
    /// Filesystem path of the file to serve.
    pub file_path: String,
}

/// Parameters for serve_fallback_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ServeFallbackParams {
    /// Filesystem path of the directory to serve.
    pub dir_path: String,
    /// Fallback file path for missing files (SPA pattern).
    pub fallback_path: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_serve",
    name = "serve_describe",
    emit = None,
    description = "Describe axum::serve() for a given bind address."
)]
#[instrument]
async fn serve_describe(p: ServeParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = ServeDescriptor {
        bind_addr: p.bind_addr.clone(),
        description: format!(
            "axum::serve(listener, app) — serves the axum app on {}. Requires a TcpListener. \
             Returns a future that must be awaited.",
            p.bind_addr
        ),
        graceful_shutdown: false,
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_serve",
    name = "serve_with_shutdown_describe",
    emit = None,
    description = "Describe axum::serve() with graceful shutdown configured."
)]
#[instrument]
async fn serve_with_shutdown_describe(p: ServeShutdownParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = ServeDescriptor {
        bind_addr: p.bind_addr.clone(),
        description: format!(
            "axum::serve with graceful shutdown on {} — waits for '{}' signal before shutdown",
            p.bind_addr, p.signal_description
        ),
        graceful_shutdown: true,
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_serve",
    name = "make_service_describe",
    emit = None,
    description = "Describe IntoMakeService wrapping a router or app."
)]
#[instrument]
async fn make_service_describe(p: MakeServiceParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "IntoMakeService wrapping: {}. Use app.into_make_service() to create a MakeService for \
         use with hyper or custom transports.",
        p.router_description
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_serve",
    name = "tcp_listener_describe",
    emit = None,
    description = "Describe binding a tokio TcpListener to an address for use with axum::serve()."
)]
#[instrument]
async fn tcp_listener_describe(p: TcpListenerParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "tokio::net::TcpListener::bind(\"{}\").await — binds a TCP listener to the address. \
         Returns TcpListener for use with axum::serve().",
        p.addr
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_serve",
    name = "serve_dir_describe",
    emit = None,
    description = "Describe tower_http::services::ServeDir for serving static files from a directory."
)]
#[instrument]
async fn serve_dir_describe(p: ServeDirParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "tower_http::services::ServeDir::new(\"{}\") — serves static files from the directory. \
         Use with Router::nest_service() or as a fallback.",
        p.dir_path
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_serve",
    name = "serve_file_describe",
    emit = None,
    description = "Describe tower_http::services::ServeFile for serving a single static file."
)]
#[instrument]
async fn serve_file_describe(p: ServeFileParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "tower_http::services::ServeFile::new(\"{}\") — serves a single static file. Useful as a \
         Router fallback for SPA applications.",
        p.file_path
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_serve",
    name = "serve_fallback_describe",
    emit = None,
    description = "Describe ServeDir with a ServeFile fallback for SPA applications."
)]
#[instrument]
async fn serve_fallback_describe(p: ServeFallbackParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "ServeDir::new(\"{}\").fallback(ServeFile::new(\"{}\")) — serves static files with a \
         fallback to '{}' for missing files (SPA pattern).",
        p.dir_path, p.fallback_path, p.fallback_path
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

/// Plugin exposing axum server setup and serving tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_serve")]
pub struct AxumServePlugin;
