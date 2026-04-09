//! `TowerCorsPlugin` — CorsLayer configuration MCP tools.

use elicitation::{ElicitPlugin, ToCodeLiteral, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn json_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

// ── Types ─────────────────────────────────────────────────────────────────────

/// Configuration for a CorsLayer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct CorsConfig {
    /// Whether to allow credentials (cookies, authorization headers).
    pub allow_credentials: bool,
    /// Allowed request headers ("*" means any).
    pub allow_headers: Vec<String>,
    /// Allowed HTTP methods ("*" means any).
    pub allow_methods: Vec<String>,
    /// Allowed origin ("*" means any, or a specific origin URL).
    pub allow_origin: String,
    /// Whether to allow private network access.
    pub allow_private_network: bool,
    /// Response headers exposed to the browser.
    pub expose_headers: Vec<String>,
    /// Max age in seconds for preflight cache (None means no header).
    pub max_age_secs: Option<u64>,
    /// Headers included in the Vary response header.
    pub vary: Vec<String>,
}

// ── Unique per-tool params ────────────────────────────────────────────────────

/// Parameters for cors_new (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsNewParams {}

/// Parameters for cors_permissive (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsPermissiveParams {}

/// Parameters for cors_very_permissive (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsVeryPermissiveParams {}

/// Parameters for cors_allow_credentials.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowCredentialsParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The boolean value to set.
    pub value: bool,
}

/// Parameters for cors_allow_headers_any.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowHeadersAnyParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
}

/// Parameters for cors_allow_headers_list.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowHeadersListParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The list of headers to allow.
    pub headers: Vec<String>,
}

/// Parameters for cors_allow_methods_any.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowMethodsAnyParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
}

/// Parameters for cors_allow_methods_list.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowMethodsListParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The list of HTTP methods to allow.
    pub methods: Vec<String>,
}

/// Parameters for cors_allow_origin_any.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowOriginAnyParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
}

/// Parameters for cors_allow_origin_exact.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowOriginExactParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The exact origin URL to allow.
    pub origin: String,
}

/// Parameters for cors_allow_origin_list.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowOriginListParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The list of origin URLs to allow.
    pub origins: Vec<String>,
}

/// Parameters for cors_allow_private_network.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsAllowPrivateNetworkParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The boolean value to set.
    pub value: bool,
}

/// Parameters for cors_expose_headers_any.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsExposeHeadersAnyParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
}

/// Parameters for cors_expose_headers_list.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsExposeHeadersListParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The list of headers to expose.
    pub headers: Vec<String>,
}

/// Parameters for cors_max_age.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsMaxAgeParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The max age in seconds.
    pub seconds: u64,
}

/// Parameters for cors_vary_list.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorsVaryListParams {
    /// The current CORS configuration.
    pub config: CorsConfig,
    /// The list of headers for the Vary directive.
    pub headers: Vec<String>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_new",
    description = "Create an empty (restrictive) CorsLayer configuration."
)]
#[instrument]
async fn cors_new(_p: CorsNewParams) -> Result<CallToolResult, ErrorData> {
    let result = CorsConfig {
        allow_credentials: false,
        allow_headers: vec![],
        allow_methods: vec![],
        allow_origin: String::new(),
        allow_private_network: false,
        expose_headers: vec![],
        max_age_secs: None,
        vary: vec![],
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_permissive",
    description = "Create a permissive CorsLayer configuration allowing common origins."
)]
#[instrument]
async fn cors_permissive(_p: CorsPermissiveParams) -> Result<CallToolResult, ErrorData> {
    let result = CorsConfig {
        allow_credentials: false,
        allow_headers: vec!["content-type".to_string(), "authorization".to_string()],
        allow_methods: vec![
            "GET".to_string(),
            "POST".to_string(),
            "PUT".to_string(),
            "DELETE".to_string(),
        ],
        allow_origin: "*".to_string(),
        allow_private_network: false,
        expose_headers: vec![],
        max_age_secs: Some(3600),
        vary: vec!["Origin".to_string()],
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_very_permissive",
    description = "Create a very permissive CorsLayer configuration allowing all origins and methods."
)]
#[instrument]
async fn cors_very_permissive(_p: CorsVeryPermissiveParams) -> Result<CallToolResult, ErrorData> {
    let result = CorsConfig {
        allow_credentials: true,
        allow_headers: vec!["*".to_string()],
        allow_methods: vec!["*".to_string()],
        allow_origin: "*".to_string(),
        allow_private_network: true,
        expose_headers: vec!["*".to_string()],
        max_age_secs: Some(86400),
        vary: vec![],
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_credentials",
    description = "Set the allow_credentials flag on a CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_credentials(
    p: CorsAllowCredentialsParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_credentials = p.value;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_headers_any",
    description = "Allow any request header in the CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_headers_any(p: CorsAllowHeadersAnyParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_headers = vec!["*".to_string()];
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_headers_list",
    description = "Set a specific list of allowed request headers in the CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_headers_list(
    p: CorsAllowHeadersListParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_headers = p.headers;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_methods_any",
    description = "Allow any HTTP method in the CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_methods_any(p: CorsAllowMethodsAnyParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_methods = vec!["*".to_string()];
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_methods_list",
    description = "Set a specific list of allowed HTTP methods in the CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_methods_list(
    p: CorsAllowMethodsListParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_methods = p.methods;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_origin_any",
    description = "Allow any origin in the CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_origin_any(p: CorsAllowOriginAnyParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_origin = "*".to_string();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_origin_exact",
    description = "Set an exact allowed origin in the CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_origin_exact(
    p: CorsAllowOriginExactParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_origin = p.origin;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_origin_list",
    description = "Set a list of allowed origins in the CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_origin_list(p: CorsAllowOriginListParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_origin = p.origins.join(",");
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_allow_private_network",
    description = "Set the allow_private_network flag on a CorsLayer configuration."
)]
#[instrument]
async fn cors_allow_private_network(
    p: CorsAllowPrivateNetworkParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.allow_private_network = p.value;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_expose_headers_any",
    description = "Expose all response headers in the CorsLayer configuration."
)]
#[instrument]
async fn cors_expose_headers_any(
    p: CorsExposeHeadersAnyParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.expose_headers = vec!["*".to_string()];
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_expose_headers_list",
    description = "Set a list of exposed response headers in the CorsLayer configuration."
)]
#[instrument]
async fn cors_expose_headers_list(
    p: CorsExposeHeadersListParams,
) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.expose_headers = p.headers;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_max_age",
    description = "Set the max age in seconds for preflight response caching."
)]
#[instrument]
async fn cors_max_age(p: CorsMaxAgeParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.max_age_secs = Some(p.seconds);
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_cors",
    name = "cors_vary_list",
    description = "Set the Vary response headers in the CorsLayer configuration."
)]
#[instrument]
async fn cors_vary_list(p: CorsVaryListParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.vary = p.headers;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

/// Plugin exposing CorsLayer configuration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_cors")]
pub struct TowerCorsPlugin;
