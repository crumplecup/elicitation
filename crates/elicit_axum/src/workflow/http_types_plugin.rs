//! `AxumHttpTypesPlugin` — MCP tools for HTTP types (StatusCode, Method, HeaderName, Uri, Version).

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for status code tools.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StatusCodeParams {
    /// HTTP status code (100–599).
    pub code: u16,
}

/// Parameters for HTTP method tools.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodParams {
    /// HTTP method string (e.g. "GET", "POST").
    pub method: String,
}

/// Parameters for header name tools.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HeaderNameParams {
    /// HTTP header name.
    pub name: String,
}

/// Parameters for URI tools.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UriParams {
    /// The URI string to inspect.
    pub uri: String,
}

/// Parameters for HTTP version tools.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VersionParams {
    /// HTTP version string (e.g. "HTTP/1.1").
    pub version: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn canonical_reason(code: u16) -> &'static str {
    match code {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        418 => "I'm a Teapot",
        422 => "Unprocessable Entity",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        _ => "Unknown",
    }
}

fn method_description(method: &str) -> &'static str {
    match method {
        "GET" => "Retrieve a resource. Safe and idempotent.",
        "POST" => "Submit data, create a resource. Not safe or idempotent.",
        "PUT" => "Replace a resource. Idempotent.",
        "DELETE" => "Delete a resource. Idempotent.",
        "PATCH" => "Partially update a resource. Not idempotent.",
        "HEAD" => "Like GET but without response body. Safe and idempotent.",
        "OPTIONS" => "Describe communication options. Safe and idempotent.",
        "TRACE" => "Loop-back test. Safe and idempotent.",
        "CONNECT" => "Establish a tunnel.",
        _ => "Custom HTTP method.",
    }
}

fn is_standard_header(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "accept"
            | "accept-charset"
            | "accept-encoding"
            | "accept-language"
            | "authorization"
            | "cache-control"
            | "connection"
            | "content-encoding"
            | "content-length"
            | "content-type"
            | "cookie"
            | "date"
            | "etag"
            | "expect"
            | "expires"
            | "host"
            | "if-match"
            | "if-modified-since"
            | "if-none-match"
            | "if-range"
            | "if-unmodified-since"
            | "last-modified"
            | "location"
            | "origin"
            | "pragma"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "range"
            | "referer"
            | "retry-after"
            | "server"
            | "set-cookie"
            | "te"
            | "transfer-encoding"
            | "upgrade"
            | "user-agent"
            | "vary"
            | "via"
            | "www-authenticate"
            | "x-forwarded-for"
            | "x-forwarded-host"
            | "x-request-id"
    )
}

fn parse_uri(uri: &str) -> (String, String, String, String) {
    let mut rest = uri;
    let scheme = if let Some(idx) = rest.find("://") {
        let s = rest[..idx].to_string();
        rest = &rest[idx + 3..];
        s
    } else {
        String::new()
    };
    let host = if !scheme.is_empty() {
        if let Some(idx) = rest.find('/') {
            let h = rest[..idx].to_string();
            rest = &rest[idx..];
            h
        } else if let Some(idx) = rest.find('?') {
            let h = rest[..idx].to_string();
            rest = &rest[idx..];
            h
        } else {
            let h = rest.to_string();
            rest = "";
            h
        }
    } else {
        String::new()
    };
    let (path, query) = if let Some(idx) = rest.find('?') {
        (rest[..idx].to_string(), rest[idx + 1..].to_string())
    } else {
        (rest.to_string(), String::new())
    };
    (scheme, host, path, query)
}

fn header_description(name: &str) -> &'static str {
    match name.to_lowercase().as_str() {
        "content-type" => "Indicates the media type of the resource",
        "content-length" => "Indicates the size of the entity-body in bytes",
        "authorization" => "Contains credentials for authenticating the client with the server",
        "accept" => "Informs the server about the types of data the client can process",
        "cache-control" => "Directives for caching mechanisms in both requests and responses",
        "cookie" => "Contains stored HTTP cookies sent by the client",
        "set-cookie" => "Sends cookies from the server to the client",
        "location" => "Indicates the URL to redirect to",
        "user-agent" => "Contains a characteristic string identifying the client software",
        "host" => "Specifies the domain name and port of the server",
        "x-request-id" => "Custom header for request tracing/correlation",
        "x-forwarded-for" => "Identifies the originating IP address of a client through proxies",
        _ => "HTTP header field",
    }
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_http_types",
    name = "status_code_value",
    emit = None,
    description = "Return metadata for an HTTP status code including canonical reason and validity."
)]
#[instrument]
async fn status_code_value(p: StatusCodeParams) -> Result<CallToolResult, ErrorData> {
    let is_valid = (100..=599).contains(&p.code);
    let reason = canonical_reason(p.code);
    let val = serde_json::json!({
        "code": p.code,
        "canonical_reason": reason,
        "is_valid": is_valid,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "status_code_is_success",
    emit = None,
    description = "Return \"true\" if the status code is in the 2xx success range."
)]
#[instrument]
async fn status_code_is_success(p: StatusCodeParams) -> Result<CallToolResult, ErrorData> {
    let result = (200..=299).contains(&p.code);
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "status_code_is_client_error",
    emit = None,
    description = "Return \"true\" if the status code is in the 4xx client error range."
)]
#[instrument]
async fn status_code_is_client_error(p: StatusCodeParams) -> Result<CallToolResult, ErrorData> {
    let result = (400..=499).contains(&p.code);
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "status_code_is_server_error",
    emit = None,
    description = "Return \"true\" if the status code is in the 5xx server error range."
)]
#[instrument]
async fn status_code_is_server_error(p: StatusCodeParams) -> Result<CallToolResult, ErrorData> {
    let result = (500..=599).contains(&p.code);
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "status_code_canonical_reason",
    emit = None,
    description = "Return the canonical reason phrase for an HTTP status code."
)]
#[instrument]
async fn status_code_canonical_reason(p: StatusCodeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        canonical_reason(p.code),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "method_describe",
    emit = None,
    description = "Return metadata for an HTTP method including safety and idempotency."
)]
#[instrument]
async fn method_describe(p: MethodParams) -> Result<CallToolResult, ErrorData> {
    let method_upper = p.method.to_uppercase();
    let is_safe = matches!(method_upper.as_str(), "GET" | "HEAD" | "OPTIONS" | "TRACE");
    let is_idempotent = matches!(
        method_upper.as_str(),
        "GET" | "HEAD" | "PUT" | "DELETE" | "OPTIONS" | "TRACE"
    );
    let description = method_description(&method_upper);
    let val = serde_json::json!({
        "method": method_upper,
        "is_safe": is_safe,
        "is_idempotent": is_idempotent,
        "description": description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "method_is_safe",
    emit = None,
    description = "Return \"true\" if the HTTP method is considered safe (GET, HEAD, OPTIONS, TRACE)."
)]
#[instrument]
async fn method_is_safe(p: MethodParams) -> Result<CallToolResult, ErrorData> {
    let upper = p.method.to_uppercase();
    let result = matches!(upper.as_str(), "GET" | "HEAD" | "OPTIONS" | "TRACE");
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "method_is_idempotent",
    emit = None,
    description = "Return \"true\" if the HTTP method is idempotent."
)]
#[instrument]
async fn method_is_idempotent(p: MethodParams) -> Result<CallToolResult, ErrorData> {
    let upper = p.method.to_uppercase();
    let result = matches!(
        upper.as_str(),
        "GET" | "HEAD" | "PUT" | "DELETE" | "OPTIONS" | "TRACE"
    );
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "header_name_describe",
    emit = None,
    description = "Return metadata for an HTTP header name including description and standardness."
)]
#[instrument]
async fn header_name_describe(p: HeaderNameParams) -> Result<CallToolResult, ErrorData> {
    let name_lower = p.name.to_lowercase();
    let description = header_description(&name_lower);
    let is_standard = is_standard_header(&name_lower);
    let val = serde_json::json!({
        "name_lower": name_lower,
        "description": description,
        "is_standard": is_standard,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "header_name_as_str",
    emit = None,
    description = "Return the lowercase string form of an HTTP header name."
)]
#[instrument]
async fn header_name_as_str(p: HeaderNameParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.name.to_lowercase(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "uri_describe",
    emit = None,
    description = "Parse a URI string and return its components as structured JSON."
)]
#[instrument]
async fn uri_describe(p: UriParams) -> Result<CallToolResult, ErrorData> {
    let (scheme, host, path, query) = parse_uri(&p.uri);
    let description = format!(
        "URI '{}' — scheme: '{}', host: '{}', path: '{}', query: '{}'",
        p.uri, scheme, host, path, query
    );
    let val = serde_json::json!({
        "scheme": scheme,
        "host": host,
        "path": path,
        "query": query,
        "description": description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "uri_path",
    emit = None,
    description = "Extract the path component from a URI string."
)]
#[instrument]
async fn uri_path(p: UriParams) -> Result<CallToolResult, ErrorData> {
    let (_, _, path, _) = parse_uri(&p.uri);
    Ok(CallToolResult::success(vec![Content::text(path)]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "uri_query",
    emit = None,
    description = "Extract the query string component from a URI string."
)]
#[instrument]
async fn uri_query(p: UriParams) -> Result<CallToolResult, ErrorData> {
    let (_, _, _, query) = parse_uri(&p.uri);
    Ok(CallToolResult::success(vec![Content::text(query)]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "uri_host",
    emit = None,
    description = "Extract the host component from a URI string."
)]
#[instrument]
async fn uri_host(p: UriParams) -> Result<CallToolResult, ErrorData> {
    let (_, host, _, _) = parse_uri(&p.uri);
    Ok(CallToolResult::success(vec![Content::text(host)]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "uri_scheme",
    emit = None,
    description = "Extract the scheme component from a URI string."
)]
#[instrument]
async fn uri_scheme(p: UriParams) -> Result<CallToolResult, ErrorData> {
    let (scheme, _, _, _) = parse_uri(&p.uri);
    Ok(CallToolResult::success(vec![Content::text(scheme)]))
}

#[elicit_tool(
    plugin = "axum_http_types",
    name = "version_describe",
    emit = None,
    description = "Return a description of an HTTP protocol version string."
)]
#[instrument]
async fn version_describe(p: VersionParams) -> Result<CallToolResult, ErrorData> {
    let description = match p.version.as_str() {
        "HTTP/1.0" => "HTTP 1.0 — legacy protocol with connection-per-request semantics",
        "HTTP/1.1" => "HTTP 1.1 — persistent connections and chunked transfer encoding",
        "HTTP/2" => "HTTP/2 — multiplexed streams over a single connection, header compression",
        "HTTP/3" => "HTTP/3 — HTTP semantics over QUIC protocol",
        _ => "Unknown HTTP version",
    };
    let val = serde_json::json!({
        "version": p.version,
        "description": description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

/// Plugin exposing HTTP type inspection tools (StatusCode, Method, HeaderName, Uri, Version).
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_http_types")]
pub struct AxumHttpTypesPlugin;
