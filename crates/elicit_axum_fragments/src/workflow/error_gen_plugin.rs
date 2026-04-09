//! AxumErrorGenPlugin — emit axum error type and IntoResponse fragments.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// An error variant with HTTP status code.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ErrorVariant {
    /// Variant name (e.g. `NotFound`).
    pub name: String,
    /// Human-readable error message.
    pub message: String,
    /// HTTP status code (e.g. `404`).
    pub status: u16,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for emit_app_error.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitAppErrorParams {
    /// Error enum name.
    pub name: String,
    /// Variants to generate with their status codes and messages.
    pub variants: Vec<ErrorVariant>,
}

/// Parameters for emit_error_response_impl.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitErrorResponseParams {
    /// Existing error type name.
    pub error_type: String,
    /// Variants of the error type to map to HTTP responses.
    pub variants: Vec<ErrorVariant>,
}

/// Parameters for emit_error_kind_enum.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitErrorKindParams {
    /// Error kind enum name.
    pub name: String,
    /// Variants with their messages.
    pub variants: Vec<ErrorVariant>,
}

/// Parameters for emit_http_error.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitHttpErrorParams {
    /// Error struct name.
    pub name: String,
}

/// Parameters for emit_validation_error.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitValidationErrorParams {
    /// Field names that can have validation errors.
    pub fields: Vec<String>,
}

/// Parameters for emit_database_error.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitDatabaseErrorParams {
    /// Database crate type to wrap (e.g. `sqlx`).
    pub db_type: String,
}

/// Parameters for emit_auth_error.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitAuthErrorParams {}

/// Parameters for emit_from_impls.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitFromImplsParams {
    /// Target error type name.
    pub error_type: String,
    /// Source error types to convert from.
    pub source_types: Vec<String>,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn status_code_expr(status: u16) -> &'static str {
    match status {
        200 => "StatusCode::OK",
        201 => "StatusCode::CREATED",
        204 => "StatusCode::NO_CONTENT",
        400 => "StatusCode::BAD_REQUEST",
        401 => "StatusCode::UNAUTHORIZED",
        403 => "StatusCode::FORBIDDEN",
        404 => "StatusCode::NOT_FOUND",
        405 => "StatusCode::METHOD_NOT_ALLOWED",
        409 => "StatusCode::CONFLICT",
        410 => "StatusCode::GONE",
        422 => "StatusCode::UNPROCESSABLE_ENTITY",
        429 => "StatusCode::TOO_MANY_REQUESTS",
        500 => "StatusCode::INTERNAL_SERVER_ERROR",
        502 => "StatusCode::BAD_GATEWAY",
        503 => "StatusCode::SERVICE_UNAVAILABLE",
        504 => "StatusCode::GATEWAY_TIMEOUT",
        _ => "StatusCode::INTERNAL_SERVER_ERROR",
    }
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_error_gen",
    emit = None,
    name = "emit_app_error",
    description = "Emit an AppError enum with all variants and an IntoResponse impl returning JSON bodies."
)]
#[instrument]
async fn emit_app_error(p: EmitAppErrorParams) -> Result<CallToolResult, ErrorData> {
    let variants_def = p
        .variants
        .iter()
        .map(|v| format!("    /// {}.\n    {},", v.message, v.name))
        .collect::<Vec<_>>()
        .join("\n");

    let match_arms = p
        .variants
        .iter()
        .map(|v| {
            let status = status_code_expr(v.status);
            format!(
                "            {}::{} => ({}, Json(serde_json::json!({{\"error\": \"{}\"}})))\n                .into_response(),",
                p.name, v.name, status, v.message
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let code = format!(
        r#"#[derive(Debug)]
pub enum {name} {{
{variants_def}
}}

impl IntoResponse for {name} {{
    fn into_response(self) -> Response {{
        match self {{
{match_arms}
        }}
    }}
}}"#,
        name = p.name,
        variants_def = variants_def,
        match_arms = match_arms,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_error_gen",
    emit = None,
    name = "emit_error_response_impl",
    description = "Emit only the IntoResponse impl for an existing error type."
)]
#[instrument]
async fn emit_error_response_impl(p: EmitErrorResponseParams) -> Result<CallToolResult, ErrorData> {
    let match_arms = p
        .variants
        .iter()
        .map(|v| {
            let status = status_code_expr(v.status);
            format!(
                "            {}::{} => ({}, \"{}\").into_response(),",
                p.error_type, v.name, status, v.message
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let code = format!(
        "impl IntoResponse for {} {{\n    fn into_response(self) -> Response {{\n        match self {{\n{}\n        }}\n    }}\n}}",
        p.error_type, match_arms
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_error_gen",
    emit = None,
    name = "emit_error_kind_enum",
    description = "Emit an ErrorKind enum with a Display impl using the variant messages."
)]
#[instrument]
async fn emit_error_kind_enum(p: EmitErrorKindParams) -> Result<CallToolResult, ErrorData> {
    let variants_def = p
        .variants
        .iter()
        .map(|v| format!("    /// {}.\n    {},", v.message, v.name))
        .collect::<Vec<_>>()
        .join("\n");

    let display_arms = p
        .variants
        .iter()
        .map(|v| {
            format!(
                "            {}::{} => write!(f, \"{}\"),",
                p.name, v.name, v.message
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let code = format!(
        "#[derive(Debug, Clone, PartialEq, Eq)]\npub enum {name} {{\n{variants_def}\n}}\n\nimpl std::fmt::Display for {name} {{\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n        match self {{\n{display_arms}\n        }}\n    }}\n}}",
        name = p.name,
        variants_def = variants_def,
        display_arms = display_arms,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_error_gen",
    emit = None,
    name = "emit_http_error",
    description = "Emit a simple HTTP error struct with status and message fields plus IntoResponse."
)]
#[instrument]
async fn emit_http_error(p: EmitHttpErrorParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"#[derive(Debug)]
pub struct {name} {{
    /// HTTP status code for this error.
    pub status: StatusCode,
    /// Human-readable error message.
    pub message: String,
}}

impl {name} {{
    /// Create a new `{name}` with the given status and message.
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {{
        Self {{ status, message: message.into() }}
    }}
}}

impl IntoResponse for {name} {{
    fn into_response(self) -> Response {{
        (self.status, self.message).into_response()
    }}
}}"#,
        name = p.name,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_error_gen",
    emit = None,
    name = "emit_validation_error",
    description = "Emit a ValidationError with per-field error messages and an IntoResponse returning 422."
)]
#[instrument]
async fn emit_validation_error(p: EmitValidationErrorParams) -> Result<CallToolResult, ErrorData> {
    let field_list = p.fields.join(", ");
    let code = format!(
        r#"/// Validation error with per-field messages.
/// Fields: {field_list}
#[derive(Debug, Default)]
pub struct ValidationError {{
    /// Per-field error messages.
    pub errors: std::collections::HashMap<String, Vec<String>>,
}}

impl ValidationError {{
    /// Add an error message for a field.
    pub fn add(&mut self, field: impl Into<String>, message: impl Into<String>) {{
        self.errors.entry(field.into()).or_default().push(message.into());
    }}

    /// Return `true` if there are no errors.
    pub fn is_empty(&self) -> bool {{
        self.errors.is_empty()
    }}
}}

impl IntoResponse for ValidationError {{
    fn into_response(self) -> Response {{
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({{ "errors": self.errors }})),
        )
            .into_response()
    }}
}}"#,
        field_list = field_list,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_error_gen",
    emit = None,
    name = "emit_database_error",
    description = "Emit a DatabaseError enum wrapping database-specific errors with IntoResponse returning 500."
)]
#[instrument]
async fn emit_database_error(p: EmitDatabaseErrorParams) -> Result<CallToolResult, ErrorData> {
    let db = &p.db_type;
    let code = format!(
        r#"#[derive(Debug)]
pub enum DatabaseError {{
    /// A query failed.
    Query(Box<dyn std::error::Error + Send + Sync>),
    /// A connection could not be established.
    Connection(Box<dyn std::error::Error + Send + Sync>),
    /// A pool timeout occurred.
    PoolTimeout,
}}

impl From<{db}::Error> for DatabaseError {{
    fn from(err: {db}::Error) -> Self {{
        Self::Query(Box::new(err))
    }}
}}

impl IntoResponse for DatabaseError {{
    fn into_response(self) -> Response {{
        tracing::error!(error = ?self, "Database error");
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
    }}
}}"#,
        db = db,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_error_gen",
    emit = None,
    name = "emit_auth_error",
    description = "Emit an AuthError enum with Unauthorized, Forbidden, InvalidToken, and ExpiredToken variants."
)]
#[instrument]
async fn emit_auth_error(_p: EmitAuthErrorParams) -> Result<CallToolResult, ErrorData> {
    let code = r#"#[derive(Debug)]
pub enum AuthError {
    /// The request is missing valid authentication credentials.
    Unauthorized,
    /// The authenticated user lacks permission for this resource.
    Forbidden,
    /// The provided token is malformed or cannot be decoded.
    InvalidToken,
    /// The provided token has expired.
    ExpiredToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token expired"),
        };
        (status, Json(serde_json::json!({"error": message}))).into_response()
    }
}"#;
    Ok(CallToolResult::success(vec![Content::text(
        code.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_error_gen",
    emit = None,
    name = "emit_from_impls",
    description = "Emit `From<SourceError>` implementations for each provided source type."
)]
#[instrument]
async fn emit_from_impls(p: EmitFromImplsParams) -> Result<CallToolResult, ErrorData> {
    let impls = p
        .source_types
        .iter()
        .map(|src| {
            format!(
                "impl From<{}> for {} {{\n    fn from(err: {}) -> Self {{\n        // TODO: map {} to {}\n        let _ = err;\n        todo!()\n    }}\n}}",
                src, p.error_type, src, src, p.error_type
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    Ok(CallToolResult::success(vec![Content::text(impls)]))
}

/// Plugin exposing axum error type generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_error_gen")]
pub struct AxumErrorGenPlugin;
