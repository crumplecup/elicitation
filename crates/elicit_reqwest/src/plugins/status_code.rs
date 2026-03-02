//! `StatusCodePlugin` — MCP tools for every `reqwest::StatusCode` method.
//!
//! Registered under the `"status_code"` namespace, producing tools:
//! `status_code__from_u16`, `status_code__as_str`, `status_code__canonical_reason`,
//! `status_code__is_informational`, `status_code__is_success`,
//! `status_code__is_redirection`, `status_code__is_client_error`,
//! `status_code__is_server_error`.

use elicitation::ElicitPlugin;
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::plugins::util::{parse_args, typed_tool};

/// Parameters for tools that operate on a status code.
#[derive(Debug, Deserialize, JsonSchema)]
struct StatusParams {
    /// HTTP status code (e.g. `200`, `404`).
    status: u16,
}

/// Parameters for tools that construct a status code.
#[derive(Debug, Deserialize, JsonSchema)]
struct FromU16Params {
    /// Integer status code to parse.
    code: u16,
}

/// MCP plugin exposing all `reqwest::StatusCode` methods as tools.
///
/// Register under the `"status_code"` namespace:
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_reqwest::plugins::StatusCodePlugin;
///
/// let registry = PluginRegistry::new()
///     .register("status_code", StatusCodePlugin);
/// ```
pub struct StatusCodePlugin;

impl ElicitPlugin for StatusCodePlugin {
    fn name(&self) -> &'static str {
        "status_code"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<FromU16Params>(
                "from_u16",
                "Parse an integer into a status code; returns its string form, canonical reason, and class booleans.",
            ),
            typed_tool::<StatusParams>(
                "as_str",
                "Return the three-digit ASCII representation of the status code (e.g. \"404\").",
            ),
            typed_tool::<StatusParams>(
                "canonical_reason",
                "Return the canonical reason phrase for the status code (e.g. \"Not Found\"), or null if unknown.",
            ),
            typed_tool::<StatusParams>(
                "is_informational",
                "Return true if the status code is 1xx Informational.",
            ),
            typed_tool::<StatusParams>(
                "is_success",
                "Return true if the status code is 2xx Success.",
            ),
            typed_tool::<StatusParams>(
                "is_redirection",
                "Return true if the status code is 3xx Redirection.",
            ),
            typed_tool::<StatusParams>(
                "is_client_error",
                "Return true if the status code is 4xx Client Error.",
            ),
            typed_tool::<StatusParams>(
                "is_server_error",
                "Return true if the status code is 5xx Server Error.",
            ),
        ]
    }

    #[instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            match params.name.as_ref() {
                "from_u16" => {
                    let p: FromU16Params = parse_args(&params)?;
                    match reqwest::StatusCode::from_u16(p.code) {
                        Ok(sc) => {
                            let json = serde_json::json!({
                                "code": sc.as_u16(),
                                "str": sc.as_str(),
                                "canonical_reason": sc.canonical_reason(),
                                "is_informational": sc.is_informational(),
                                "is_success": sc.is_success(),
                                "is_redirection": sc.is_redirection(),
                                "is_client_error": sc.is_client_error(),
                                "is_server_error": sc.is_server_error(),
                            });
                            Ok(CallToolResult::success(vec![Content::text(
                                json.to_string(),
                            )]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "as_str" => {
                    let p: StatusParams = parse_args(&params)?;
                    match reqwest::StatusCode::from_u16(p.status) {
                        Ok(sc) => Ok(CallToolResult::success(vec![Content::text(sc.as_str())])),
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "canonical_reason" => {
                    let p: StatusParams = parse_args(&params)?;
                    match reqwest::StatusCode::from_u16(p.status) {
                        Ok(sc) => {
                            let reason = sc.canonical_reason().unwrap_or("(unknown)");
                            Ok(CallToolResult::success(vec![Content::text(reason)]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "is_informational" => bool_tool(&params, |sc| sc.is_informational()),
                "is_success" => bool_tool(&params, |sc| sc.is_success()),
                "is_redirection" => bool_tool(&params, |sc| sc.is_redirection()),
                "is_client_error" => bool_tool(&params, |sc| sc.is_client_error()),
                "is_server_error" => bool_tool(&params, |sc| sc.is_server_error()),
                other => Err(ErrorData::invalid_params(
                    format!("unknown tool: {other}"),
                    None,
                )),
            }
        })
    }
}

fn bool_tool(
    params: &CallToolRequestParams,
    f: impl Fn(reqwest::StatusCode) -> bool,
) -> Result<CallToolResult, ErrorData> {
    let p: StatusParams = parse_args(params)?;
    match reqwest::StatusCode::from_u16(p.status) {
        Ok(sc) => Ok(CallToolResult::success(vec![Content::text(
            f(sc).to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}
