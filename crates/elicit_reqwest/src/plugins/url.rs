//! `UrlPlugin` — MCP tools for every `url::Url` method.
//!
//! Registered under the `"url"` namespace. All tools take `{ url: String }`.
//! Mutation tools (`set_path`, `set_query`, `set_fragment`, `set_host`,
//! `set_port`, `set_scheme`, `set_username`) return the modified URL string.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

/// Single-URL input used by most tools.
#[derive(Debug, Deserialize, JsonSchema)]
struct UrlParams {
    /// The URL to inspect (e.g. `"https://user:pass@example.com:8080/path?q=1#frag"`).
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlSchemeParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlUsernameParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlPasswordParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlHasHostParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlHostStrParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlDomainParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlPortParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlPortOrKnownDefaultParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlPathParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlQueryParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlFragmentParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlOriginParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlCannotBeABaseParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlHasAuthorityParams {
    url: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlAsStrParams {
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

/// Input for `make_relative` — two absolute URLs.
#[derive(Debug, Deserialize, JsonSchema)]
struct UrlMakeRelativeParams {
    base: String,
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
#[derive(ElicitPlugin)]
#[plugin(name = "url")]
pub struct UrlPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "url",
    name = "parse",
    description = "Parse a URL string into its components. Returns a JSON object with scheme, username, password, host, port, path, query, fragment, origin, and validation status."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_parse(p: UrlParams) -> Result<CallToolResult, ErrorData> {
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

#[elicit_tool(
    plugin = "url",
    name = "parse_with_params",
    description = "Parse a URL string and append query parameters. Returns the resulting URL string."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_parse_with_params(p: ParseWithParamsInput) -> Result<CallToolResult, ErrorData> {
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

#[elicit_tool(
    plugin = "url",
    name = "scheme",
    description = "Return the scheme of the URL (e.g. \"https\")."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_scheme(p: UrlSchemeParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            u.scheme().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "username",
    description = "Return the username from the URL authority, or empty string."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_username(p: UrlUsernameParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            u.username().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "password",
    description = "Return the password from the URL authority, or null."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_password(p: UrlPasswordParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => {
            let val = u
                .password()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "null".to_string());
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "has_host",
    description = "Return true if the URL has a host component."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_has_host(p: UrlHasHostParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            u.has_host().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "host_str",
    description = "Return the host as a string (e.g. \"example.com\"), or null."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_host_str(p: UrlHostStrParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => {
            let val = u
                .host_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "null".to_string());
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "domain",
    description = "Return the domain (host without port) if the URL uses a domain name, or null."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_domain(p: UrlDomainParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => {
            let val = u
                .domain()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "null".to_string());
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "port",
    description = "Return the explicit port number, or null if absent."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_port(p: UrlPortParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => {
            let val = match u.port() {
                Some(n) => n.to_string(),
                None => "null".to_string(),
            };
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "port_or_known_default",
    description = "Return the port or its scheme default (80 for http, 443 for https), or null."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_port_or_known_default(
    p: UrlPortOrKnownDefaultParams,
) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => {
            let val = match u.port_or_known_default() {
                Some(n) => n.to_string(),
                None => "null".to_string(),
            };
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "path",
    description = "Return the path component of the URL."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_path(p: UrlPathParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            u.path().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "query",
    description = "Return the query string (without leading `?`), or null."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_query(p: UrlQueryParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => {
            let val = u
                .query()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "null".to_string());
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "fragment",
    description = "Return the fragment identifier (without leading `#`), or null."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_fragment(p: UrlFragmentParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => {
            let val = u
                .fragment()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "null".to_string());
            Ok(CallToolResult::success(vec![Content::text(val)]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "origin",
    description = "Return the origin of the URL as a string (e.g. \"https://example.com:443\")."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_origin(p: UrlOriginParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            u.origin().ascii_serialization(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "cannot_be_a_base",
    description = "Return true if the URL cannot be used as a base for relative URLs."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_cannot_be_a_base(p: UrlCannotBeABaseParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            u.cannot_be_a_base().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "has_authority",
    description = "Return true if the URL has an authority component."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_has_authority(p: UrlHasAuthorityParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            u.has_authority().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "as_str",
    description = "Return the serialized URL as a string."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_as_str(p: UrlAsStrParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(p.url.as_str()) {
        Ok(u) => Ok(CallToolResult::success(vec![Content::text(
            u.as_str().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "join",
    description = "Resolve a relative URL reference against a base URL; returns the resulting URL string."
)]
#[instrument(skip_all, fields(base = %p.base, input = %p.input))]
async fn url_join(p: JoinParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(&p.base) {
        Ok(base) => match base.join(&p.input) {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(String::from(
                result,
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        },
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "make_relative",
    description = "Return a relative URL string from base to target, or null if base and target have different origins."
)]
#[instrument(skip_all, fields(base = %p.base, input = %p.input))]
async fn url_make_relative(p: UrlMakeRelativeParams) -> Result<CallToolResult, ErrorData> {
    let base = match url::Url::parse(&p.base) {
        Ok(u) => u,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    };
    let target = match url::Url::parse(&p.input) {
        Ok(u) => u,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    };
    let result = base
        .make_relative(&target)
        .unwrap_or_else(|| "(different origins)".to_string());
    Ok(CallToolResult::success(vec![Content::text(result)]))
}

#[elicit_tool(
    plugin = "url",
    name = "set_path",
    description = "Return a copy of the URL with the path replaced."
)]
#[instrument(skip_all, fields(url = %p.url, path = %p.path))]
async fn url_set_path(p: SetPathParams) -> Result<CallToolResult, ErrorData> {
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

#[elicit_tool(
    plugin = "url",
    name = "set_query",
    description = "Return a copy of the URL with the query string replaced (or cleared if null)."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_set_query(p: SetQueryParams) -> Result<CallToolResult, ErrorData> {
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

#[elicit_tool(
    plugin = "url",
    name = "set_fragment",
    description = "Return a copy of the URL with the fragment replaced (or cleared if null)."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_set_fragment(p: SetFragmentParams) -> Result<CallToolResult, ErrorData> {
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

#[elicit_tool(
    plugin = "url",
    name = "set_host",
    description = "Return a copy of the URL with the host replaced (or cleared if null)."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_set_host(p: SetHostParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(&p.url) {
        Ok(mut u) => match u.set_host(p.host.as_deref()) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(String::from(
                u,
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        },
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "set_port",
    description = "Return a copy of the URL with the port replaced (or cleared if null)."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_set_port(p: SetPortParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(&p.url) {
        Ok(mut u) => match u.set_port(p.port) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(String::from(
                u,
            ))])),
            Err(()) => Ok(CallToolResult::error(vec![Content::text(
                "cannot set port on this URL".to_string(),
            )])),
        },
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "set_scheme",
    description = "Return a copy of the URL with the scheme replaced."
)]
#[instrument(skip_all, fields(url = %p.url, scheme = %p.scheme))]
async fn url_set_scheme(p: SetSchemeParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(&p.url) {
        Ok(mut u) => match u.set_scheme(&p.scheme) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(String::from(
                u,
            ))])),
            Err(()) => Ok(CallToolResult::error(vec![Content::text(
                "cannot set scheme on this URL".to_string(),
            )])),
        },
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "set_username",
    description = "Return a copy of the URL with the username replaced."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_set_username(p: SetUsernameParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(&p.url) {
        Ok(mut u) => match u.set_username(&p.username) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(String::from(
                u,
            ))])),
            Err(()) => Ok(CallToolResult::error(vec![Content::text(
                "cannot set username on this URL".to_string(),
            )])),
        },
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "url",
    name = "set_password",
    description = "Return a copy of the URL with the password replaced (or cleared if null)."
)]
#[instrument(skip_all, fields(url = %p.url))]
async fn url_set_password(p: SetPasswordParams) -> Result<CallToolResult, ErrorData> {
    match url::Url::parse(&p.url) {
        Ok(mut u) => match u.set_password(p.password.as_deref()) {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(String::from(
                u,
            ))])),
            Err(()) => Ok(CallToolResult::error(vec![Content::text(
                "cannot set password on this URL".to_string(),
            )])),
        },
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

// ── ToCodeLiteral impls ───────────────────────────────────────────────────────

#[cfg(feature = "emit")]
impl elicitation::emit_code::ToCodeLiteral for ParseWithParamsEntry {
    fn to_code_literal(&self) -> elicitation::proc_macro2::TokenStream {
        let key = &self.key;
        let value = &self.value;
        ::quote::quote! {
            ParseWithParamsEntry {
                key: #key.to_string(),
                value: #value.to_string(),
            }
        }
    }
}
