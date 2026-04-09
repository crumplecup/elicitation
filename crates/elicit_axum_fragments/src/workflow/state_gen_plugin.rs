//! AxumStateGenPlugin — emit axum application state type fragments.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// A field in the state struct.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StateField {
    /// Field name.
    pub name: String,
    /// Field type (e.g. `Arc<Mutex<Vec<User>>>`).
    pub field_type: String,
    /// Doc comment for the field.
    pub doc: String,
}

/// An environment variable binding.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EnvVar {
    /// The struct field name.
    pub name: String,
    /// The environment variable key.
    pub env_key: String,
    /// Optional default value as a Rust expression string.
    pub default: Option<String>,
}

/// A field name and its default value expression.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FieldDefault {
    /// Field name.
    pub field: String,
    /// Default value expression.
    pub value: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for emit_state_struct.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitStateStructParams {
    /// Name of the state struct.
    pub name: String,
    /// Fields to include in the struct.
    pub fields: Vec<StateField>,
}

/// Parameters for emit_state_impl.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitStateImplParams {
    /// Name of the state struct.
    pub name: String,
    /// Fields to include in the constructor.
    pub fields: Vec<StateField>,
}

/// Parameters for emit_state_from_env.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitStateFromEnvParams {
    /// Name of the state struct.
    pub name: String,
    /// Environment variable bindings.
    pub env_vars: Vec<EnvVar>,
}

/// Parameters for emit_app_state_with_db.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitDbStateParams {
    /// Name of the application state struct.
    pub state_name: String,
    /// Database pool type (e.g. `PgPool`).
    pub db_type: String,
    /// Environment variable key for the database URL.
    pub db_url_env: String,
}

/// Parameters for emit_shared_state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitSharedStateParams {
    /// Type alias name.
    pub name: String,
    /// Inner type to wrap in `Arc`.
    pub inner_type: String,
}

/// Parameters for emit_state_extractor_impl.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitStateExtractorParams {
    /// The outer application state type.
    pub state_type: String,
    /// The sub-state type to extract.
    pub extracted_type: String,
    /// Field name on the state struct that holds the extracted type.
    pub field_name: String,
}

/// Parameters for emit_nested_state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitNestedStateParams {
    /// Name of the outer state struct.
    pub outer: String,
    /// Fields for the inner state struct.
    pub inner_fields: Vec<StateField>,
}

/// Parameters for emit_state_test_fixture.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitStateFixtureParams {
    /// State struct type name.
    pub state_type: String,
    /// Field names and their default value expressions.
    pub field_defaults: Vec<FieldDefault>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_state_gen",
    emit = None,
    name = "emit_state_struct",
    description = "Emit a `#[derive(Clone)] pub struct` for application state with documented fields."
)]
#[instrument]
async fn emit_state_struct(p: EmitStateStructParams) -> Result<CallToolResult, ErrorData> {
    let fields = p
        .fields
        .iter()
        .map(|f| format!("    /// {}\n    pub {}: {},", f.doc, f.name, f.field_type))
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!("#[derive(Clone)]\npub struct {} {{\n{}\n}}", p.name, fields);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_state_gen",
    emit = None,
    name = "emit_state_impl",
    description = "Emit an `impl` block with a `new()` constructor for the state struct."
)]
#[instrument]
async fn emit_state_impl(p: EmitStateImplParams) -> Result<CallToolResult, ErrorData> {
    let params = p
        .fields
        .iter()
        .map(|f| format!("{}: {}", f.name, f.field_type))
        .collect::<Vec<_>>()
        .join(", ");
    let body = p
        .fields
        .iter()
        .map(|f| format!("            {},", f.name))
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        "impl {} {{\n    pub fn new({}) -> Self {{\n        Self {{\n{}\n        }}\n    }}\n}}",
        p.name, params, body
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_state_gen",
    emit = None,
    name = "emit_state_from_env",
    description = "Emit a state constructor that reads fields from environment variables."
)]
#[instrument]
async fn emit_state_from_env(p: EmitStateFromEnvParams) -> Result<CallToolResult, ErrorData> {
    let bindings = p
        .env_vars
        .iter()
        .map(|v| {
            if let Some(default) = &v.default {
                format!(
                    "    let {} = std::env::var(\"{}\").unwrap_or_else(|_| {}.to_string());",
                    v.name, v.env_key, default
                )
            } else {
                format!(
                    "    let {} = std::env::var(\"{}\").expect(\"{} must be set\");",
                    v.name, v.env_key, v.env_key
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let fields = p
        .env_vars
        .iter()
        .map(|v| format!("        {},", v.name))
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        "impl {} {{\n    pub fn from_env() -> Self {{\n{}\n        Self {{\n{}\n        }}\n    }}\n}}",
        p.name, bindings, fields
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_state_gen",
    emit = None,
    name = "emit_app_state_with_db",
    description = "Emit a full app state struct with a database pool and async constructor."
)]
#[instrument]
async fn emit_app_state_with_db(p: EmitDbStateParams) -> Result<CallToolResult, ErrorData> {
    let name = &p.state_name;
    let db = &p.db_type;
    let env_key = &p.db_url_env;
    let code = format!(
        r#"#[derive(Clone)]
pub struct {name} {{
    /// Database connection pool.
    pub db: {db},
}}

impl {name} {{
    /// Create a new `{name}` by connecting to the database URL in `{env_key}`.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {{
        let db_url = std::env::var("{env_key}").expect("{env_key} must be set");
        let db = {db}::connect(&db_url).await?;
        Ok(Self {{ db }})
    }}
}}"#
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_state_gen",
    emit = None,
    name = "emit_shared_state",
    description = "Emit a `pub type` alias wrapping an inner type in `Arc`."
)]
#[instrument]
async fn emit_shared_state(p: EmitSharedStateParams) -> Result<CallToolResult, ErrorData> {
    let code = format!("pub type {} = Arc<{}>;", p.name, p.inner_type);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_state_gen",
    emit = None,
    name = "emit_state_extractor_impl",
    description = "Emit a `FromRef` implementation to extract a sub-state from the app state."
)]
#[instrument]
async fn emit_state_extractor_impl(
    p: EmitStateExtractorParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        "impl FromRef<{}> for {} {{\n    fn from_ref(state: &{}) -> Self {{\n        state.{}.clone()\n    }}\n}}",
        p.state_type, p.extracted_type, p.state_type, p.field_name
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_state_gen",
    emit = None,
    name = "emit_nested_state",
    description = "Emit a nested state pattern with an inner struct and `FromRef` impl."
)]
#[instrument]
async fn emit_nested_state(p: EmitNestedStateParams) -> Result<CallToolResult, ErrorData> {
    let inner_name = format!("{}Inner", p.outer);
    let fields_def = p
        .inner_fields
        .iter()
        .map(|f| format!("    /// {}\n    pub {}: {},", f.doc, f.name, f.field_type))
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        r#"#[derive(Clone)]
pub struct {inner_name} {{
{fields_def}
}}

#[derive(Clone)]
pub struct {outer} {{
    /// Inner shared state.
    pub inner: {inner_name},
}}

impl FromRef<{outer}> for {inner_name} {{
    fn from_ref(state: &{outer}) -> Self {{
        state.inner.clone()
    }}
}}"#,
        inner_name = inner_name,
        outer = p.outer,
        fields_def = fields_def,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_state_gen",
    emit = None,
    name = "emit_state_test_fixture",
    description = "Emit a test fixture function that constructs a state value with default fields."
)]
#[instrument]
async fn emit_state_test_fixture(p: EmitStateFixtureParams) -> Result<CallToolResult, ErrorData> {
    let state_lower = p.state_type.to_lowercase();
    let fields = p
        .field_defaults
        .iter()
        .map(|fd| format!("        {}: {},", fd.field, fd.value))
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        "fn test_{state_lower}() -> {state_type} {{\n    {state_type} {{\n{fields}\n    }}\n}}",
        state_lower = state_lower,
        state_type = p.state_type,
        fields = fields,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

/// Plugin exposing axum application state generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_state_gen")]
pub struct AxumStateGenPlugin;
