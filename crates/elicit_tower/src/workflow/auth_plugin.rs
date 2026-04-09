//! `TowerAuthPlugin` — Authentication validation layer MCP tools.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn json_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

// ── Types ─────────────────────────────────────────────────────────────────────

/// Configuration for an authentication validation layer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuthConfig {
    /// Authentication kind ("basic", "bearer", "custom").
    pub kind: String,
    /// Username for basic authentication.
    pub username: Option<String>,
    /// Description of the token or credential.
    pub token_description: Option<String>,
    /// Header name used for authentication.
    pub header_name: Option<String>,
    /// Expected header value.
    pub header_value: Option<String>,
}

/// Parameters for basic authentication.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BasicAuthParams {
    /// The username.
    pub username: String,
    /// Description of the password (not the actual password).
    pub password_description: String,
}

/// Parameters for bearer token authentication.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BearerAuthParams {
    /// Description of the bearer token.
    pub token_description: String,
}

/// Parameters for custom authentication.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CustomAuthParams {
    /// Description of the custom authentication scheme.
    pub description: String,
}

/// Parameters for header-value-based authentication.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AcceptHeaderParams {
    /// The header name to match.
    pub header_name: String,
    /// The expected header value.
    pub header_value: String,
}

/// Parameters for reject-if-missing header authentication.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RejectMissingParams {
    /// The header name that must be present.
    pub header_name: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_auth",
    name = "auth_basic",
    description = "Create a basic authentication layer configuration."
)]
#[instrument]
async fn auth_basic(p: BasicAuthParams) -> Result<CallToolResult, ErrorData> {
    let result = AuthConfig {
        kind: "basic".to_string(),
        username: Some(p.username),
        token_description: Some(p.password_description),
        header_name: Some("Authorization".to_string()),
        header_value: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_auth",
    name = "auth_bearer",
    description = "Create a bearer token authentication layer configuration."
)]
#[instrument]
async fn auth_bearer(p: BearerAuthParams) -> Result<CallToolResult, ErrorData> {
    let result = AuthConfig {
        kind: "bearer".to_string(),
        username: None,
        token_description: Some(p.token_description),
        header_name: Some("Authorization".to_string()),
        header_value: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_auth",
    name = "auth_custom",
    description = "Create a custom authentication layer configuration."
)]
#[instrument]
async fn auth_custom(p: CustomAuthParams) -> Result<CallToolResult, ErrorData> {
    let result = AuthConfig {
        kind: "custom".to_string(),
        username: None,
        token_description: Some(p.description),
        header_name: None,
        header_value: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_auth",
    name = "auth_accept_header_value",
    description = "Create an authentication layer that accepts requests with a specific header value."
)]
#[instrument]
async fn auth_accept_header_value(p: AcceptHeaderParams) -> Result<CallToolResult, ErrorData> {
    let result = AuthConfig {
        kind: "custom".to_string(),
        username: None,
        token_description: None,
        header_name: Some(p.header_name),
        header_value: Some(p.header_value),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_auth",
    name = "auth_reject_header_missing",
    description = "Create an authentication layer that rejects requests missing a required header."
)]
#[instrument]
async fn auth_reject_header_missing(p: RejectMissingParams) -> Result<CallToolResult, ErrorData> {
    let result = AuthConfig {
        kind: "custom".to_string(),
        username: None,
        token_description: Some(format!("Rejects requests missing '{}'", p.header_name)),
        header_name: Some(p.header_name),
        header_value: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

/// Plugin exposing authentication validation layer tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_auth")]
pub struct TowerAuthPlugin;
