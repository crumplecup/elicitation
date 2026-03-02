//! `MethodPlugin` — MCP tools for every `reqwest::Method` method.
//!
//! Registered under the `"method"` namespace, producing tools:
//! `method__from_str`, `method__as_str`, `method__is_safe`, `method__is_idempotent`.

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

/// Parameters for tools that operate on an HTTP method string.
#[derive(Debug, Deserialize, JsonSchema)]
struct MethodParams {
    /// HTTP method string (e.g. `"GET"`, `"POST"`). Case-insensitive.
    method: String,
}

/// MCP plugin exposing all `reqwest::Method` methods as tools.
///
/// Register under the `"method"` namespace:
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_reqwest::plugins::MethodPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("method", MethodPlugin);
/// ```
pub struct MethodPlugin;

impl ElicitPlugin for MethodPlugin {
    fn name(&self) -> &'static str {
        "method"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<MethodParams>(
                "from_str",
                "Parse and validate an HTTP method string. Returns the normalized uppercase method and its properties (is_safe, is_idempotent), or an error for invalid input.",
            ),
            typed_tool::<MethodParams>(
                "as_str",
                "Return the canonical uppercase string representation of the method (e.g. \"GET\").",
            ),
            typed_tool::<MethodParams>(
                "is_safe",
                "Return true if the method is safe (has no intended side effects): GET, HEAD, OPTIONS, TRACE.",
            ),
            typed_tool::<MethodParams>(
                "is_idempotent",
                "Return true if the method is idempotent (repeated requests have the same effect): GET, HEAD, PUT, DELETE, OPTIONS, TRACE.",
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
                "from_str" => {
                    let p: MethodParams = parse_args(&params)?;
                    match reqwest::Method::from_bytes(p.method.as_bytes()) {
                        Ok(m) => {
                            let json = serde_json::json!({
                                "method": m.as_str(),
                                "is_safe": m.is_safe(),
                                "is_idempotent": m.is_idempotent(),
                            });
                            Ok(CallToolResult::success(vec![Content::text(
                                json.to_string(),
                            )]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "as_str" => {
                    let p: MethodParams = parse_args(&params)?;
                    match reqwest::Method::from_bytes(p.method.as_bytes()) {
                        Ok(m) => Ok(CallToolResult::success(vec![Content::text(m.as_str())])),
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "is_safe" => bool_tool(&params, |m| m.is_safe()),
                "is_idempotent" => bool_tool(&params, |m| m.is_idempotent()),
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
    f: impl Fn(reqwest::Method) -> bool,
) -> Result<CallToolResult, ErrorData> {
    let p: MethodParams = parse_args(params)?;
    match reqwest::Method::from_bytes(p.method.as_bytes()) {
        Ok(m) => Ok(CallToolResult::success(vec![Content::text(
            f(m).to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}
