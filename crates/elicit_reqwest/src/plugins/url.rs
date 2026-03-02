//! `UrlPlugin` — MCP tools for every `url::Url` method.
//!
//! Registered under the `"url"` namespace. All tools take `{ url: String }`.
//! Mutation tools (`set_path`, `set_query`, `set_fragment`, `set_host`,
//! `set_port`, `set_scheme`, `set_username`) return the modified URL string.

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

/// Single-URL input used by most tools.
#[derive(Debug, Deserialize, JsonSchema)]
struct UrlParams {
    /// The URL to inspect (e.g. `"https://user:pass@example.com:8080/path?q=1#frag"`).
    url: String,
}

/// Input for `join` — a base URL and a relative reference.
#[derive(Debug, Deserialize, JsonSchema)]
struct JoinParams {
    /// Base URL.
    base: String,
    /// Relative URL or path to resolve against the base.
    input: String,
}

/// Input for `set_path`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SetPathParams {
    /// URL to modify.
    url: String,
    /// New path component (e.g. `"/api/v2"`).
    path: String,
}

/// Input for `set_query`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SetQueryParams {
    /// URL to modify.
    url: String,
    /// New query string, without the leading `?`; null to clear.
    query: Option<String>,
}

/// Input for `set_fragment`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SetFragmentParams {
    /// URL to modify.
    url: String,
    /// New fragment, without the leading `#`; null to clear.
    fragment: Option<String>,
}

/// Input for `set_host`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SetHostParams {
    /// URL to modify.
    url: String,
    /// New hostname; null to clear the host component.
    host: Option<String>,
}

/// Input for `set_port`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SetPortParams {
    /// URL to modify.
    url: String,
    /// New port number; null to clear.
    port: Option<u16>,
}

/// Input for `set_scheme`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SetSchemeParams {
    /// URL to modify.
    url: String,
    /// New scheme (e.g. `"https"`).
    scheme: String,
}

/// Input for `set_username`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SetUsernameParams {
    /// URL to modify.
    url: String,
    /// New username (empty string to clear).
    username: String,
}

/// Input for `set_password`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SetPasswordParams {
    /// URL to modify.
    url: String,
    /// New password; null to clear.
    password: Option<String>,
}

/// Input for `parse_with_params`.
#[derive(Debug, Deserialize, JsonSchema)]
struct ParseWithParamsEntry {
    /// Query parameter key.
    key: String,
    /// Query parameter value.
    value: String,
}

/// Input for `parse_with_params`.
#[derive(Debug, Deserialize, JsonSchema)]
struct ParseWithParamsInput {
    /// Base URL string to parse.
    url: String,
    /// Query parameters to append.
    params: Vec<ParseWithParamsEntry>,
}

/// Parse a URL string, returning an error message on failure.
fn parse_url(s: &str) -> Result<url::Url, CallToolResult> {
    url::Url::parse(s).map_err(|e| CallToolResult::error(vec![Content::text(e.to_string())]))
}

/// MCP plugin exposing all `url::Url` methods as tools.
///
/// Register under the `"url"` namespace:
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_reqwest::plugins::UrlPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("url", UrlPlugin);
/// ```
pub struct UrlPlugin;

impl ElicitPlugin for UrlPlugin {
    fn name(&self) -> &'static str {
        "url"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<UrlParams>(
                "parse",
                "Parse a URL string into its components. Returns a JSON object with scheme, username, password, host, port, path, query, fragment, origin, and validation status.",
            ),
            typed_tool::<ParseWithParamsInput>(
                "parse_with_params",
                "Parse a URL string and append query parameters. Returns the resulting URL string.",
            ),
            typed_tool::<UrlParams>("scheme", "Return the scheme of the URL (e.g. \"https\")."),
            typed_tool::<UrlParams>(
                "username",
                "Return the username from the URL authority, or empty string.",
            ),
            typed_tool::<UrlParams>(
                "password",
                "Return the password from the URL authority, or null.",
            ),
            typed_tool::<UrlParams>("has_host", "Return true if the URL has a host component."),
            typed_tool::<UrlParams>(
                "host_str",
                "Return the host as a string (e.g. \"example.com\"), or null.",
            ),
            typed_tool::<UrlParams>(
                "domain",
                "Return the domain (host without port) if the URL uses a domain name, or null.",
            ),
            typed_tool::<UrlParams>(
                "port",
                "Return the explicit port number, or null if absent.",
            ),
            typed_tool::<UrlParams>(
                "port_or_known_default",
                "Return the port or its scheme default (80 for http, 443 for https), or null.",
            ),
            typed_tool::<UrlParams>("path", "Return the path component of the URL."),
            typed_tool::<UrlParams>(
                "query",
                "Return the query string (without leading `?`), or null.",
            ),
            typed_tool::<UrlParams>(
                "fragment",
                "Return the fragment identifier (without leading `#`), or null.",
            ),
            typed_tool::<UrlParams>(
                "origin",
                "Return the origin of the URL as a string (e.g. \"https://example.com:443\").",
            ),
            typed_tool::<UrlParams>(
                "cannot_be_a_base",
                "Return true if the URL cannot be used as a base for relative URLs.",
            ),
            typed_tool::<UrlParams>(
                "has_authority",
                "Return true if the URL has an authority component.",
            ),
            typed_tool::<UrlParams>("as_str", "Return the serialized URL as a string."),
            typed_tool::<JoinParams>(
                "join",
                "Resolve a relative URL reference against a base URL; returns the resulting URL string.",
            ),
            typed_tool::<JoinParams>(
                "make_relative",
                "Return a relative URL string from base to target, or null if base and target have different origins.",
            ),
            typed_tool::<SetPathParams>(
                "set_path",
                "Return a copy of the URL with the path replaced.",
            ),
            typed_tool::<SetQueryParams>(
                "set_query",
                "Return a copy of the URL with the query string replaced (or cleared if null).",
            ),
            typed_tool::<SetFragmentParams>(
                "set_fragment",
                "Return a copy of the URL with the fragment replaced (or cleared if null).",
            ),
            typed_tool::<SetHostParams>(
                "set_host",
                "Return a copy of the URL with the host replaced (or cleared if null).",
            ),
            typed_tool::<SetPortParams>(
                "set_port",
                "Return a copy of the URL with the port replaced (or cleared if null).",
            ),
            typed_tool::<SetSchemeParams>(
                "set_scheme",
                "Return a copy of the URL with the scheme replaced.",
            ),
            typed_tool::<SetUsernameParams>(
                "set_username",
                "Return a copy of the URL with the username replaced.",
            ),
            typed_tool::<SetPasswordParams>(
                "set_password",
                "Return a copy of the URL with the password replaced (or cleared if null).",
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
                "parse" => {
                    let p: UrlParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(u) => {
                            let json = serde_json::json!({
                                "scheme": u.scheme(),
                                "username": u.username(),
                                "password": u.password(),
                                "has_host": u.has_host(),
                                "host": u.host_str(),
                                "domain": u.domain(),
                                "port": u.port(),
                                "port_or_known_default": u.port_or_known_default(),
                                "path": u.path(),
                                "query": u.query(),
                                "fragment": u.fragment(),
                                "origin": u.origin().ascii_serialization(),
                                "cannot_be_a_base": u.cannot_be_a_base(),
                                "has_authority": u.has_authority(),
                                "as_str": u.as_str(),
                            });
                            Ok(CallToolResult::success(vec![Content::text(
                                json.to_string(),
                            )]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "parse_with_params" => {
                    let p: ParseWithParamsInput = parse_args(&params)?;
                    let pairs: Vec<(&str, &str)> = p
                        .params
                        .iter()
                        .map(|e| (e.key.as_str(), e.value.as_str()))
                        .collect();
                    match url::Url::parse_with_params(&p.url, &pairs) {
                        Ok(u) => Ok(CallToolResult::success(vec![Content::text(String::from(
                            u,
                        ))])),
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "scheme" => str_tool(&params, |u| u.scheme().to_string()),
                "username" => str_tool(&params, |u| u.username().to_string()),
                "password" => opt_str_tool(&params, |u| u.password().map(|s| s.to_string())),
                "has_host" => bool_tool(&params, |u| u.has_host()),
                "host_str" => opt_str_tool(&params, |u| u.host_str().map(|s| s.to_string())),
                "domain" => opt_str_tool(&params, |u| u.domain().map(|s| s.to_string())),
                "port" => opt_u16_tool(&params, |u| u.port()),
                "port_or_known_default" => opt_u16_tool(&params, |u| u.port_or_known_default()),
                "path" => str_tool(&params, |u| u.path().to_string()),
                "query" => opt_str_tool(&params, |u| u.query().map(|s| s.to_string())),
                "fragment" => opt_str_tool(&params, |u| u.fragment().map(|s| s.to_string())),
                "origin" => str_tool(&params, |u| u.origin().ascii_serialization()),
                "cannot_be_a_base" => bool_tool(&params, |u| u.cannot_be_a_base()),
                "has_authority" => bool_tool(&params, |u| u.has_authority()),
                "as_str" => str_tool(&params, |u| u.as_str().to_string()),
                "join" => {
                    let p: JoinParams = parse_args(&params)?;
                    match url::Url::parse(&p.base) {
                        Ok(base) => match base.join(&p.input) {
                            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                                String::from(result),
                            )])),
                            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                        },
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "make_relative" => {
                    let p: JoinParams = parse_args(&params)?;
                    let base = match url::Url::parse(&p.base) {
                        Ok(u) => u,
                        Err(e) => {
                            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
                        }
                    };
                    let target = match url::Url::parse(&p.input) {
                        Ok(u) => u,
                        Err(e) => {
                            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
                        }
                    };
                    let result = base
                        .make_relative(&target)
                        .unwrap_or_else(|| "(different origins)".to_string());
                    Ok(CallToolResult::success(vec![Content::text(result)]))
                }
                "set_path" => {
                    let p: SetPathParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(mut u) => {
                            u.set_path(&p.path);
                            Ok(CallToolResult::success(vec![Content::text(String::from(
                                u,
                            ))]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "set_query" => {
                    let p: SetQueryParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(mut u) => {
                            u.set_query(p.query.as_deref());
                            Ok(CallToolResult::success(vec![Content::text(String::from(
                                u,
                            ))]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "set_fragment" => {
                    let p: SetFragmentParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(mut u) => {
                            u.set_fragment(p.fragment.as_deref());
                            Ok(CallToolResult::success(vec![Content::text(String::from(
                                u,
                            ))]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "set_host" => {
                    let p: SetHostParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(mut u) => match u.set_host(p.host.as_deref()) {
                            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                                String::from(u),
                            )])),
                            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                        },
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "set_port" => {
                    let p: SetPortParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(mut u) => match u.set_port(p.port) {
                            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                                String::from(u),
                            )])),
                            Err(()) => Ok(CallToolResult::error(vec![Content::text(
                                "cannot set port on this URL".to_string(),
                            )])),
                        },
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "set_scheme" => {
                    let p: SetSchemeParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(mut u) => match u.set_scheme(&p.scheme) {
                            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                                String::from(u),
                            )])),
                            Err(()) => Ok(CallToolResult::error(vec![Content::text(
                                "cannot set scheme on this URL".to_string(),
                            )])),
                        },
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "set_username" => {
                    let p: SetUsernameParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(mut u) => match u.set_username(&p.username) {
                            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                                String::from(u),
                            )])),
                            Err(()) => Ok(CallToolResult::error(vec![Content::text(
                                "cannot set username on this URL".to_string(),
                            )])),
                        },
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "set_password" => {
                    let p: SetPasswordParams = parse_args(&params)?;
                    match url::Url::parse(&p.url) {
                        Ok(mut u) => match u.set_password(p.password.as_deref()) {
                            Ok(()) => Ok(CallToolResult::success(vec![Content::text(
                                String::from(u),
                            )])),
                            Err(()) => Ok(CallToolResult::error(vec![Content::text(
                                "cannot set password on this URL".to_string(),
                            )])),
                        },
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                other => Err(ErrorData::invalid_params(
                    format!("unknown tool: {other}"),
                    None,
                )),
            }
        })
    }
}

fn str_tool(
    params: &CallToolRequestParams,
    f: impl Fn(url::Url) -> String,
) -> Result<CallToolResult, ErrorData> {
    let p: UrlParams = parse_args(params)?;
    match parse_url(&p.url) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(f(u))])),
        Err(r) => Ok(r),
    }
}

fn opt_str_tool(
    params: &CallToolRequestParams,
    f: impl Fn(url::Url) -> Option<String>,
) -> Result<CallToolResult, ErrorData> {
    let p: UrlParams = parse_args(params)?;
    match parse_url(&p.url) {
        Ok(u) => {
            let val = f(u).unwrap_or_else(|| "null".to_string());
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(r) => Ok(r),
    }
}

fn bool_tool(
    params: &CallToolRequestParams,
    f: impl Fn(url::Url) -> bool,
) -> Result<CallToolResult, ErrorData> {
    let p: UrlParams = parse_args(params)?;
    match parse_url(&p.url) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            f(u).to_string(),
        )])),
        Err(r) => Ok(r),
    }
}

fn opt_u16_tool(
    params: &CallToolRequestParams,
    f: impl Fn(&url::Url) -> Option<u16>,
) -> Result<CallToolResult, ErrorData> {
    let p: UrlParams = parse_args(params)?;
    match parse_url(&p.url) {
        Ok(u) => {
            let val = match f(&u) {
                Some(n) => n.to_string(),
                None => "null".to_string(),
            };
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(r) => Ok(r),
    }
}
