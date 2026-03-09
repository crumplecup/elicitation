//! `WorkflowPlugin` — phrase-level HTTP tool compositions.
//!
//! While the atomic plugins (`http`, `url`, `status_code`, …) are the **letters** of the
//! alphabet, this plugin provides **words**: each tool composes 2-4 primitives into a
//! meaningful verb with explicit contract documentation.
//!
//! # Contracts and Propositions
//!
//! Every tool documents its **assumptions** (what the caller must provide) and the
//! **propositions it establishes** on success, using the `elicitation` contract
//! vocabulary.  The Rust implementation carries those proofs internally via
//! [`elicitation::contracts::Established`] — they are zero-cost `PhantomData` markers
//! that disappear at compile time.
//!
//! Example contract chain for `fetch`:
//!
//! ```text
//! UrlValid → RequestCompleted → StatusSuccess
//!         ↓          ↓               ↓
//!    url::Url    resp = .send()   resp.status().is_success()
//!    parses OK   returns Ok       returns true
//! ```
//!
//! # Select Pattern
//!
//! Several tools accept **enum parameters** whose JSON schema restricts the caller
//! to valid variants.  This is the [`Select`][elicitation::Select] pattern applied
//! to tool composition: the schema enforces that only documented, type-safe choices
//! may be passed.
//!
//! Registered under the `"workflow"` namespace.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use elicitation::contracts::{And, Established, Prop, both};
use elicitation::{
    ElicitPlugin, F64Positive, PluginContext, UrlValid as UrlValidType, elicit_tool,
};
use reqwest::header::{HeaderMap, HeaderValue};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: the URL string is syntactically valid and was successfully parsed.
pub struct UrlValid;
impl Prop for UrlValid {}

/// Proposition: the HTTP request was dispatched and a response was received.
pub struct RequestCompleted;
impl Prop for RequestCompleted {}

/// Proposition: the response status code is in the 2xx (success) range.
pub struct StatusSuccess;
impl Prop for StatusSuccess {}

/// Proposition: the request carried a non-empty authorization credential.
pub struct Authorized;
impl Prop for Authorized {}

/// Composite: a complete successful fetch (URL valid, request sent, 2xx status).
pub type FetchSucceeded = And<UrlValid, And<RequestCompleted, StatusSuccess>>;

/// Composite: an authenticated successful fetch.
pub type AuthFetchSucceeded = And<Authorized, FetchSucceeded>;

// ── Select-pattern enums ──────────────────────────────────────────────────────

/// Authorization strategy for an HTTP request.
///
/// Implements the [`Select`][elicitation::Select] pattern: the JSON schema
/// restricts the caller to exactly these variants, preventing ad-hoc strings.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuthType {
    /// No authorization header.
    None,
    /// `Authorization: Bearer <token>`.
    Bearer,
    /// `Authorization: Basic <base64(user:password)>`.
    Basic,
    /// Custom header `X-Api-Key: <token>`.
    ApiKey,
}

/// Content-Type for an outgoing request body.
///
/// Implements the [`Select`][elicitation::Select] pattern.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    /// `application/json` — the body is a JSON string.
    Json,
    /// `application/x-www-form-urlencoded` — the body is URL-encoded.
    FormUrlEncoded,
    /// `text/plain` — the body is plain text.
    PlainText,
    /// `application/octet-stream` — the body is binary (base64-encoded string).
    OctetStream,
}

impl ContentType {
    fn as_mime(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::FormUrlEncoded => "application/x-www-form-urlencoded",
            ContentType::PlainText => "text/plain",
            ContentType::OctetStream => "application/octet-stream",
        }
    }
}

// ── Parameter types ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
struct FetchParams {
    /// Destination URL. Assumes: syntactically valid, host is reachable.
    url: UrlValidType,
    /// Optional timeout in seconds (must be > 0; default: 30).
    timeout_secs: Option<F64Positive>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AuthFetchParams {
    /// Destination URL.
    url: UrlValidType,
    /// Authorization credential (token, base64-encoded "user:pass", or API key).
    /// Assumes: non-empty.
    token: String,
    /// Auth strategy. Constrains the credential format.
    auth_type: AuthType,
    /// Optional timeout in seconds (must be > 0; default: 30).
    timeout_secs: Option<F64Positive>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PostParams {
    /// Destination URL.
    url: UrlValidType,
    /// Request body string.
    body: String,
    /// Content-Type for the body.
    content_type: ContentType,
    /// Optional timeout in seconds (must be > 0; default: 30).
    timeout_secs: Option<F64Positive>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ApiCallParams {
    /// Destination URL.
    url: UrlValidType,
    /// Bearer token. Assumes: non-empty.
    token: String,
    /// JSON body string. Assumes: valid JSON.
    body: String,
    /// Optional timeout in seconds (must be > 0; default: 30).
    timeout_secs: Option<F64Positive>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct HealthCheckParams {
    /// URL to probe. Assumes: syntactically valid.
    url: UrlValidType,
    /// Optional timeout in seconds (must be > 0; default: 10).
    timeout_secs: Option<F64Positive>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UrlBuildParams {
    /// Base URL. Assumes: syntactically valid URL string.
    base: UrlValidType,
    /// Optional path to append (e.g. `"/v1/users"`).
    path: Option<String>,
    /// Optional query parameters to append.
    query: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct StatusSummaryParams {
    /// HTTP status code (100–599). Assumes: value in valid range.
    status: u16,
}

/// Parameters for [`WorkflowPlugin::build_request`].
///
/// Use [`BuildRequestParamsBuilder`] to construct this type.
#[derive(Debug, Deserialize, JsonSchema, derive_builder::Builder)]
#[builder(setter(into))]
pub struct BuildRequestParams {
    /// HTTP method (e.g. `"GET"`, `"POST"`).
    pub method: String,
    /// Destination URL.
    pub url: UrlValidType,
    /// Authorization type.
    pub auth_type: AuthType,
    /// Credential for the chosen auth type. Required unless auth_type is `none`.
    #[builder(setter(into, strip_option), default)]
    pub token: Option<String>,
    /// Optional body for methods that carry one.
    #[builder(setter(into, strip_option), default)]
    pub body: Option<String>,
    /// Content-Type for the body (required when body is present).
    #[builder(setter(into, strip_option), default)]
    pub content_type: Option<ContentType>,
    /// Optional timeout in seconds.
    #[builder(setter(into, strip_option), default)]
    pub timeout_secs: Option<F64Positive>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PaginatedGetParams {
    /// URL of the first (or current) page.
    url: UrlValidType,
    /// Optional bearer token.
    token: Option<String>,
    /// Optional timeout in seconds (must be > 0; default: 30).
    timeout_secs: Option<F64Positive>,
}

// ── Result types ─────────────────────────────────────────────────────────────

/// Result of a successful fetch, carrying the established propositions in the
/// `contract` field for downstream tools to inspect.
#[derive(Debug, Serialize)]
pub struct FetchResult {
    /// HTTP status code of the response.
    pub status: u16,
    /// Final URL after any redirects.
    pub url: String,
    /// Response body as a UTF-8 string.
    pub body: String,
    /// Human-readable summary of the contract propositions established.
    pub contract: String,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin for phrase-level HTTP workflow compositions.
///
/// Registers under the `"workflow"` namespace and exposes ten tools that compose
/// 2-4 primitives each, with documented contract invariants and enum-constrained
/// parameters (the [`Select`][elicitation::Select] pattern).
///
/// | Tool | Word analogy | Establishes |
/// |---|---|---|
/// | `url_build` | "construct" | `UrlValid` |
/// | `fetch` | "get-and-check" | `FetchSucceeded` |
/// | `fetch_json` | "get-as-json" | `FetchSucceeded` |
/// | `fetch_auth` | "authenticated-get" | `AuthFetchSucceeded` |
/// | `post_json` | "post-and-check" | `FetchSucceeded` |
/// | `api_call` | "bearer-post" | `AuthFetchSucceeded` |
/// | `health_check` | "probe" | _(none — returns bool)_ |
/// | `build_request` | "compose-spec" | _(pure, no side-effects)_ |
/// | `status_summary` | "classify" | _(none — pure)_ |
/// | `paginated_get` | "page-and-link" | `FetchSucceeded` + next URL |
#[derive(ElicitPlugin)]
#[plugin(name = "workflow")]
pub struct WorkflowPlugin(pub Arc<PluginContext>);

impl WorkflowPlugin {
    /// Create a new `WorkflowPlugin` backed by a shared reqwest client.
    pub fn new(client: reqwest::Client) -> Self {
        Self(Arc::new(PluginContext { http: client }))
    }

    /// Create a `WorkflowPlugin` with a default client.
    pub fn default_client() -> Self {
        Self(PluginContext::new())
    }

    /// Construct and validate a URL from base, optional path, and query pairs.
    ///
    /// Returns the validated URL string. Errors if `base` is not a valid URL.
    pub fn build_url(
        base: &str,
        path: Option<&str>,
        query: &[(&str, &str)],
    ) -> Result<String, String> {
        let mut url = url::Url::parse(base)
            .map_err(|e| format!("UrlValid not established: '{base}' — {e}"))?;
        if let Some(p) = path {
            url.set_path(p);
        }
        if !query.is_empty() {
            let qs: String = query
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            url.set_query(Some(&qs));
        }
        Ok(url.to_string())
    }

    /// GET `url`, verify the response is successful, and return the result.
    pub async fn fetch(
        &self,
        url: &str,
        timeout: Duration,
    ) -> Result<(FetchResult, Established<FetchSucceeded>), String> {
        do_fetch(&self.0.http, url, HeaderMap::new(), timeout)
            .await
            .map_err(|r| {
                r.content
                    .first()
                    .and_then(|c| c.as_text().map(|t| t.text.to_string()))
                    .unwrap_or_else(|| "fetch failed".to_string())
            })
    }

    /// GET `url` with an Authorization header, verify success.
    pub async fn auth_fetch(
        &self,
        url: &str,
        token: &str,
        auth_type: AuthType,
        timeout: Duration,
    ) -> Result<(FetchResult, Established<FetchSucceeded>), String> {
        let parsed_url =
            url::Url::parse(url).map_err(|e| format!("UrlValid not established: '{url}' — {e}"))?;
        let url_proof: Established<UrlValid> = Established::assert();

        let rb = self.0.http.get(parsed_url.as_str()).timeout(timeout);
        let (rb, _auth_proof) = apply_auth(rb, &auth_type, Some(token));

        let resp = rb
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;
        let req_proof: Established<RequestCompleted> = Established::assert();

        if !resp.status().is_success() {
            return Err(format!(
                "StatusSuccess not established: got {}",
                resp.status().as_u16()
            ));
        }
        let status_proof: Established<StatusSuccess> = Established::assert();
        let combined = both(url_proof, both(req_proof, status_proof));

        let status = resp.status().as_u16();
        let final_url = resp.url().to_string();
        let body = resp.text().await.unwrap_or_default();
        Ok((
            FetchResult {
                status,
                url: final_url,
                body,
                contract: "UrlValid ∧ RequestCompleted ∧ StatusSuccess".to_string(),
            },
            combined,
        ))
    }

    /// POST `url` with `body` and `content_type`, verify success.
    pub async fn post(
        &self,
        url: &str,
        body: &str,
        content_type: ContentType,
        timeout: Duration,
    ) -> Result<(FetchResult, Established<FetchSucceeded>), String> {
        do_post(
            &self.0.http,
            url,
            body.to_string(),
            content_type.as_mime(),
            HeaderMap::new(),
            timeout,
        )
        .await
        .map_err(|r| {
            r.content
                .first()
                .and_then(|c| c.as_text().map(|t| t.text.to_string()))
                .unwrap_or_else(|| "post failed".to_string())
        })
    }

    /// Authenticated JSON POST with Bearer token.
    pub async fn api_call(
        &self,
        url: &str,
        token: &str,
        body: &str,
        timeout: Duration,
    ) -> Result<(FetchResult, Established<FetchSucceeded>), String> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {token}").parse().unwrap());
        do_post(
            &self.0.http,
            url,
            body.to_string(),
            ContentType::Json.as_mime(),
            headers,
            timeout,
        )
        .await
        .map_err(|r| {
            r.content
                .first()
                .and_then(|c| c.as_text().map(|t| t.text.to_string()))
                .unwrap_or_else(|| "api_call failed".to_string())
        })
    }

    /// Probe `url` and return `true` if it responds with 2xx within `timeout`.
    pub async fn health_check(&self, url: &str, timeout: Duration) -> bool {
        do_fetch(&self.0.http, url, HeaderMap::new(), timeout)
            .await
            .is_ok()
    }

    /// Build and send an HTTP request with full control over method, auth, body.
    pub async fn build_request(
        &self,
        params: BuildRequestParams,
    ) -> Result<(FetchResult, Established<FetchSucceeded>), String> {
        let method = params.method.as_str();
        let timeout = Duration::from_secs_f64(params.timeout_secs.map(|t| t.get()).unwrap_or(30.0));

        let parsed_url = params.url.get().clone();
        let _url_proof: Established<UrlValid> = Established::assert();

        let method_val = reqwest::Method::from_bytes(method.as_bytes())
            .map_err(|e| format!("Invalid HTTP method '{method}': {e}"))?;

        let rb = self
            .0
            .http
            .request(method_val, parsed_url.as_str())
            .timeout(timeout);
        let (rb, _auth_proof) = apply_auth(rb, &params.auth_type, params.token.as_deref());

        let rb = if let (Some(b), Some(ct)) = (params.body.as_deref(), &params.content_type) {
            rb.header("Content-Type", ct.as_mime()).body(b.to_string())
        } else {
            rb
        };

        let resp = rb
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;
        let _req_proof: Established<RequestCompleted> = Established::assert();

        if !resp.status().is_success() {
            return Err(format!(
                "StatusSuccess not established: got {}",
                resp.status().as_u16()
            ));
        }
        let _status_proof: Established<StatusSuccess> = Established::assert();
        let combined = both(_url_proof, both(_req_proof, _status_proof));

        let status = resp.status().as_u16();
        let final_url = resp.url().to_string();
        let resp_body = resp.text().await.unwrap_or_default();
        Ok((
            FetchResult {
                status,
                url: final_url,
                body: resp_body,
                contract: "UrlValid ∧ RequestCompleted ∧ StatusSuccess".to_string(),
            },
            combined,
        ))
    }

    /// GET paginated resources, following `Link: rel="next"` headers.
    ///
    /// Returns response bodies for all pages (stops when no next-page link).
    pub async fn paginated_get(
        &self,
        url: &str,
        token: Option<&str>,
        timeout: Duration,
    ) -> Result<Vec<String>, String> {
        let mut pages = Vec::new();
        let mut next = Some(url.to_string());
        while let Some(current_url) = next {
            let mut headers = HeaderMap::new();
            if let Some(t) = token {
                headers.insert("Authorization", format!("Bearer {t}").parse().unwrap());
            }
            let resp = self
                .0
                .http
                .get(&current_url)
                .timeout(timeout)
                .headers(headers)
                .send()
                .await
                .map_err(|e| format!("Paginated GET failed: {e}"))?;
            next = extract_link_next(resp.headers());
            pages.push(resp.text().await.unwrap_or_default());
        }
        Ok(pages)
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn timeout(secs: Option<F64Positive>) -> Duration {
    Duration::from_secs_f64(secs.map(|t| t.get()).unwrap_or(30.0))
}

fn parse_url_inner(s: &str) -> Result<(url::Url, Established<UrlValid>), CallToolResult> {
    match url::Url::parse(s) {
        Ok(u) => Ok((u, Established::assert())),
        Err(e) => Err(CallToolResult::error(vec![Content::text(format!(
            "UrlValid not established: '{s}' — {e}"
        ))])),
    }
}

/// Parse a `Link: <url>; rel="next"` header and extract the next-page URL.
fn extract_link_next(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get("link")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(',')
                .find(|part| part.contains(r#"rel="next""#))
                .and_then(|part| {
                    let url = part.split(';').next()?.trim();
                    url.strip_prefix('<').and_then(|u| u.strip_suffix('>'))
                })
                .map(|u| u.to_string())
        })
}

async fn do_fetch(
    client: &reqwest::Client,
    url_str: &str,
    extra_headers: HeaderMap,
    timeout_dur: Duration,
) -> Result<(FetchResult, Established<FetchSucceeded>), CallToolResult> {
    let (parsed_url, url_proof) = parse_url_inner(url_str)?;

    let resp = client
        .get(parsed_url.as_str())
        .timeout(timeout_dur)
        .headers(extra_headers)
        .send()
        .await
        .map_err(|e| {
            CallToolResult::error(vec![Content::text(format!(
                "RequestCompleted not established: {e}"
            ))])
        })?;
    let req_proof: Established<RequestCompleted> = Established::assert();

    if !resp.status().is_success() {
        let status = resp.status().as_u16();
        return Err(CallToolResult::error(vec![Content::text(format!(
            "StatusSuccess not established: got status {status}"
        ))]));
    }
    let status_proof: Established<StatusSuccess> = Established::assert();

    let combined: Established<FetchSucceeded> = both(url_proof, both(req_proof, status_proof));

    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();
    let body = resp.text().await.unwrap_or_default();

    Ok((
        FetchResult {
            status,
            url: final_url,
            body,
            contract: "UrlValid ∧ RequestCompleted ∧ StatusSuccess".to_string(),
        },
        combined,
    ))
}

async fn do_post(
    client: &reqwest::Client,
    url_str: &str,
    body: String,
    content_type_str: &str,
    extra_headers: HeaderMap,
    timeout_dur: Duration,
) -> Result<(FetchResult, Established<FetchSucceeded>), CallToolResult> {
    let (parsed_url, url_proof) = parse_url_inner(url_str)?;

    let resp = client
        .post(parsed_url.as_str())
        .timeout(timeout_dur)
        .header("Content-Type", content_type_str)
        .headers(extra_headers)
        .body(body)
        .send()
        .await
        .map_err(|e| {
            CallToolResult::error(vec![Content::text(format!(
                "RequestCompleted not established: {e}"
            ))])
        })?;
    let req_proof: Established<RequestCompleted> = Established::assert();

    if !resp.status().is_success() {
        let status = resp.status().as_u16();
        return Err(CallToolResult::error(vec![Content::text(format!(
            "StatusSuccess not established: got status {status}"
        ))]));
    }
    let status_proof: Established<StatusSuccess> = Established::assert();

    let combined: Established<FetchSucceeded> = both(url_proof, both(req_proof, status_proof));

    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();
    let body_text = resp.text().await.unwrap_or_default();

    Ok((
        FetchResult {
            status,
            url: final_url,
            body: body_text,
            contract: "UrlValid ∧ RequestCompleted ∧ StatusSuccess".to_string(),
        },
        combined,
    ))
}

fn apply_auth(
    rb: reqwest::RequestBuilder,
    auth: &AuthType,
    token: Option<&str>,
) -> (reqwest::RequestBuilder, Option<Established<Authorized>>) {
    match auth {
        AuthType::None => (rb, None),
        AuthType::Bearer => {
            let t = token.unwrap_or("");
            if t.is_empty() {
                (rb, None)
            } else {
                (rb.bearer_auth(t), Some(Established::assert()))
            }
        }
        AuthType::Basic => {
            let t = token.unwrap_or("");
            if t.is_empty() {
                (rb, None)
            } else {
                let (user, pass) = t.split_once(':').unwrap_or((t, ""));
                (rb.basic_auth(user, Some(pass)), Some(Established::assert()))
            }
        }
        AuthType::ApiKey => {
            let t = token.unwrap_or("");
            if t.is_empty() {
                (rb, None)
            } else {
                (rb.header("X-Api-Key", t), Some(Established::assert()))
            }
        }
    }
}

/// Minimal percent-encoding for query parameter keys and values.
fn urlencoding_simple(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "workflow",
    name = "url_build",
    description = "Build a validated URL from base, optional path, and query parameters. \
                   Assumes: base is a well-formed URL string. \
                   Establishes: UrlValid — the result parses without error."
)]
#[instrument(skip_all, fields(base = %p.base.get()))]
async fn wf_url_build(
    ctx: Arc<PluginContext>,
    p: UrlBuildParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = &ctx; // stateless — no HTTP call
    let mut url = p.base.into_inner();
    if let Some(path) = &p.path {
        url.set_path(path);
    }
    if let Some(query) = &p.query {
        let qs: String = query
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding_simple(k), urlencoding_simple(v)))
            .collect::<Vec<_>>()
            .join("&");
        url.set_query(if qs.is_empty() { None } else { Some(&qs) });
    }
    let result = serde_json::json!({
        "url": url.to_string(),
        "contract": "UrlValid",
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "workflow",
    name = "fetch",
    description = "GET a URL and return the response body. \
                   Assumes: url is a valid URL; host is reachable; response is 2xx. \
                   Establishes: UrlValid ∧ RequestCompleted ∧ StatusSuccess (FetchSucceeded)."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_fetch(ctx: Arc<PluginContext>, p: FetchParams) -> Result<CallToolResult, ErrorData> {
    match do_fetch(
        &ctx.http,
        p.url.get().as_str(),
        HeaderMap::new(),
        timeout(p.timeout_secs),
    )
    .await
    {
        Ok((r, _proof)) => {
            let json = serde_json::to_string(&r).unwrap_or_default();
            Ok(CallToolResult::success(vec![Content::text(json)]))
        }
        Err(err_result) => Ok(err_result),
    }
}

#[elicit_tool(
    plugin = "workflow",
    name = "fetch_json",
    description = "GET a URL with Accept: application/json and return the body. \
                   Assumes: url is valid; server returns a 2xx JSON response. \
                   Establishes: FetchSucceeded."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_fetch_json(
    ctx: Arc<PluginContext>,
    p: FetchParams,
) -> Result<CallToolResult, ErrorData> {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    match do_fetch(
        &ctx.http,
        p.url.get().as_str(),
        headers,
        timeout(p.timeout_secs),
    )
    .await
    {
        Ok((r, _proof)) => {
            let json = serde_json::to_string(&r).unwrap_or_default();
            Ok(CallToolResult::success(vec![Content::text(json)]))
        }
        Err(err_result) => Ok(err_result),
    }
}

#[elicit_tool(
    plugin = "workflow",
    name = "fetch_auth",
    description = "GET a URL with authorization (Bearer/Basic/ApiKey) and return the body. \
                   Assumes: url is valid; token is non-empty; response is 2xx. \
                   Establishes: Authorized ∧ FetchSucceeded (AuthFetchSucceeded)."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_fetch_auth(
    ctx: Arc<PluginContext>,
    p: AuthFetchParams,
) -> Result<CallToolResult, ErrorData> {
    let url_proof: Established<UrlValid> = Established::assert();

    let rb = ctx
        .http
        .get(p.url.get().as_str())
        .timeout(timeout(p.timeout_secs));
    let (rb, auth_proof_opt) = apply_auth(rb, &p.auth_type, Some(&p.token));

    let resp = match rb.send().await {
        Ok(r) => r,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "RequestCompleted not established: {e}"
            ))]));
        }
    };
    let req_proof: Established<RequestCompleted> = Established::assert();

    if !resp.status().is_success() {
        let s = resp.status().as_u16();
        return Ok(CallToolResult::error(vec![Content::text(format!(
            "StatusSuccess not established: got {s}"
        ))]));
    }
    let status_proof: Established<StatusSuccess> = Established::assert();
    let fetch_proof: Established<FetchSucceeded> = both(url_proof, both(req_proof, status_proof));

    let contract = if let Some(auth_proof) = auth_proof_opt {
        let _: Established<AuthFetchSucceeded> = both(auth_proof, fetch_proof);
        "Authorized ∧ UrlValid ∧ RequestCompleted ∧ StatusSuccess"
    } else {
        "UrlValid ∧ RequestCompleted ∧ StatusSuccess (no auth credential provided)"
    };

    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();
    let body = resp.text().await.unwrap_or_default();
    let result = serde_json::json!({
        "status": status,
        "url": final_url,
        "body": body,
        "contract": contract,
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "workflow",
    name = "post_json",
    description = "POST a body to a URL and return the response body. \
                   Content-Type is set from the content_type enum (Select pattern). \
                   Assumes: url is valid; response is 2xx. \
                   Establishes: FetchSucceeded."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_post_json(ctx: Arc<PluginContext>, p: PostParams) -> Result<CallToolResult, ErrorData> {
    match do_post(
        &ctx.http,
        p.url.get().as_str(),
        p.body,
        p.content_type.as_mime(),
        HeaderMap::new(),
        timeout(p.timeout_secs),
    )
    .await
    {
        Ok((r, _proof)) => {
            let json = serde_json::to_string(&r).unwrap_or_default();
            Ok(CallToolResult::success(vec![Content::text(json)]))
        }
        Err(err_result) => Ok(err_result),
    }
}

#[elicit_tool(
    plugin = "workflow",
    name = "api_call",
    description = "POST JSON with a Bearer token and return the response body. \
                   Convenience composition of fetch_auth + post_json for REST APIs. \
                   Assumes: url is valid; token is non-empty; body is valid JSON; response is 2xx. \
                   Establishes: Authorized ∧ FetchSucceeded."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_api_call(
    ctx: Arc<PluginContext>,
    p: ApiCallParams,
) -> Result<CallToolResult, ErrorData> {
    let url_proof: Established<UrlValid> = Established::assert();

    let auth_proof: Established<Authorized> = if p.token.is_empty() {
        return Ok(CallToolResult::error(vec![Content::text(
            "Authorized not established: token is empty",
        )]));
    } else {
        Established::assert()
    };

    let resp = match ctx
        .http
        .post(p.url.get().as_str())
        .bearer_auth(&p.token)
        .header("Content-Type", "application/json")
        .timeout(timeout(p.timeout_secs))
        .body(p.body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "RequestCompleted not established: {e}"
            ))]));
        }
    };
    let req_proof: Established<RequestCompleted> = Established::assert();

    if !resp.status().is_success() {
        let s = resp.status().as_u16();
        return Ok(CallToolResult::error(vec![Content::text(format!(
            "StatusSuccess not established: got {s}"
        ))]));
    }
    let status_proof: Established<StatusSuccess> = Established::assert();
    let fetch_proof: Established<FetchSucceeded> = both(url_proof, both(req_proof, status_proof));
    let _combined: Established<AuthFetchSucceeded> = both(auth_proof, fetch_proof);

    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();
    let body = resp.text().await.unwrap_or_default();
    let result = serde_json::json!({
        "status": status,
        "url": final_url,
        "body": body,
        "contract": "Authorized ∧ UrlValid ∧ RequestCompleted ∧ StatusSuccess",
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "workflow",
    name = "health_check",
    description = "Probe a URL with HEAD and report whether it is healthy. \
                   Returns { healthy, status, url }. Does not require 2xx — \
                   reports actual status so callers can branch on result. \
                   Assumes: url is syntactically valid."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_health_check(
    ctx: Arc<PluginContext>,
    p: HealthCheckParams,
) -> Result<CallToolResult, ErrorData> {
    let url_str = p.url.get().as_str();

    let resp = ctx
        .http
        .head(url_str)
        .timeout(timeout(p.timeout_secs))
        .send()
        .await;

    let result = match resp {
        Ok(r) => {
            let status = r.status().as_u16();
            serde_json::json!({
                "healthy": r.status().is_success(),
                "status": status,
                "url": url_str,
            })
        }
        Err(e) => serde_json::json!({
            "healthy": false,
            "status": null,
            "url": url_str,
            "error": e.to_string(),
        }),
    };
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "workflow",
    name = "build_request",
    description = "Pure tool: compose a request spec from method, url, auth_type enum, credential, \
                   and optional body. AuthType constrains credential format (Select pattern). \
                   Returns a RequestSpec JSON object ready for request_builder__send. \
                   No network call is made; no propositions established."
)]
#[instrument(skip(ctx, p), fields(method = %p.method, url = %p.url.get()))]
async fn wf_build_request(
    ctx: Arc<PluginContext>,
    p: BuildRequestParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = &ctx; // pure — no HTTP call
    let mut headers: HashMap<String, String> = HashMap::new();

    match &p.auth_type {
        AuthType::None => {}
        AuthType::Bearer => {
            if let Some(t) = &p.token {
                headers.insert("Authorization".to_string(), format!("Bearer {t}"));
            }
        }
        AuthType::Basic => {
            if let Some(t) = &p.token {
                headers.insert("Authorization".to_string(), format!("Basic {t}"));
            }
        }
        AuthType::ApiKey => {
            if let Some(t) = &p.token {
                headers.insert("X-Api-Key".to_string(), t.clone());
            }
        }
    }

    if let Some(ct) = &p.content_type {
        headers.insert("Content-Type".to_string(), ct.as_mime().to_string());
    }

    let spec = serde_json::json!({
        "method": p.method.to_uppercase(),
        "url": p.url.get().as_str(),
        "headers": headers,
        "body": p.body,
        "timeout_secs": p.timeout_secs.map(|t| t.get()),
    });
    Ok(CallToolResult::success(vec![Content::text(
        spec.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "workflow",
    name = "status_summary",
    description = "Convert a status code into a rich classification object: \
                   { code, reason, class, is_success, is_redirect, is_client_error, is_server_error }. \
                   Assumes: status is in range 100–599. \
                   Composes status_code__from_u16 + canonical_reason + all is_* checks in one call."
)]
#[instrument(skip(ctx, p), fields(status = p.status))]
async fn wf_status_summary(
    ctx: Arc<PluginContext>,
    p: StatusSummaryParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = &ctx; // pure — no HTTP call
    match reqwest::StatusCode::from_u16(p.status) {
        Err(_) => Ok(CallToolResult::error(vec![Content::text(format!(
            "StatusClassified not established: {} is not a valid status code",
            p.status
        ))])),
        Ok(sc) => {
            let class = match p.status {
                100..=199 => "informational",
                200..=299 => "success",
                300..=399 => "redirection",
                400..=499 => "client_error",
                500..=599 => "server_error",
                _ => "unknown",
            };
            let result = serde_json::json!({
                "code": p.status,
                "reason": sc.canonical_reason().unwrap_or("Unknown"),
                "class": class,
                "is_success": sc.is_success(),
                "is_redirect": sc.is_redirection(),
                "is_client_error": sc.is_client_error(),
                "is_server_error": sc.is_server_error(),
                "is_informational": sc.is_informational(),
            });
            Ok(CallToolResult::success(vec![Content::text(
                result.to_string(),
            )]))
        }
    }
}

#[elicit_tool(
    plugin = "workflow",
    name = "paginated_get",
    description = "GET a URL and parse the RFC 5988 Link header for a next-page URL. \
                   Returns { body, next_url, has_more }. Optional bearer token. \
                   Assumes: url is valid; response is 2xx. \
                   Establishes: FetchSucceeded. If has_more is true, call again with next_url."
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_paginated_get(
    ctx: Arc<PluginContext>,
    p: PaginatedGetParams,
) -> Result<CallToolResult, ErrorData> {
    let _url_proof: Established<UrlValid> = Established::assert();

    let rb = ctx
        .http
        .get(p.url.get().as_str())
        .timeout(timeout(p.timeout_secs));
    let rb = if let Some(t) = &p.token {
        rb.bearer_auth(t)
    } else {
        rb
    };

    let resp = match rb.send().await {
        Ok(r) => r,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "RequestCompleted not established: {e}"
            ))]));
        }
    };
    let req_proof: Established<RequestCompleted> = Established::assert();

    if !resp.status().is_success() {
        let s = resp.status().as_u16();
        return Ok(CallToolResult::error(vec![Content::text(format!(
            "StatusSuccess not established: got {s}"
        ))]));
    }
    let status_proof: Established<StatusSuccess> = Established::assert();
    let _combined: Established<FetchSucceeded> = both(_url_proof, both(req_proof, status_proof));

    let next_url = extract_link_next(resp.headers());
    let has_more = next_url.is_some();
    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();
    let body = resp.text().await.unwrap_or_default();

    let result = serde_json::json!({
        "status": status,
        "url": final_url,
        "body": body,
        "next_url": next_url,
        "has_more": has_more,
        "contract": "UrlValid ∧ RequestCompleted ∧ StatusSuccess",
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

// ── EmitCode impls ────────────────────────────────────────────────────────────

#[cfg(feature = "emit")]
use elicitation::emit_code::{CrateDep, EmitCode};
#[cfg(feature = "emit")]
use elicitation::proc_macro2::TokenStream;

#[cfg(feature = "emit")]
const ELICIT_REQWEST_DEP: CrateDep = CrateDep::new("elicit_reqwest", "0.8");
#[cfg(feature = "emit")]
const ELICITATION_DEP: CrateDep = CrateDep::new("elicitation", "0.8");

/// `fetch` → `WorkflowPlugin::default_client() → .fetch(url, timeout)`
#[cfg(feature = "emit")]
impl EmitCode for FetchParams {
    fn emit_code(&self) -> TokenStream {
        let url = self.url.get().as_str();
        let timeout = self.timeout_secs.map(|t| t.get()).unwrap_or(30.0);
        quote::quote! {
            let _plugin = elicit_reqwest::WorkflowPlugin::default_client();
            let (_resp, _proof) = _plugin.fetch(
                #url,
                std::time::Duration::from_secs_f64(#timeout),
            ).await.map_err(|e| format!("Fetch failed: {}", e))?;
            println!("Status: {}", _resp.status);
            println!("{}", _resp.body);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

/// `auth_fetch` → fetch with Authorization header
#[cfg(feature = "emit")]
impl EmitCode for AuthFetchParams {
    fn emit_code(&self) -> TokenStream {
        let url = self.url.get().as_str();
        let token = &self.token;
        let timeout = self.timeout_secs.map(|t| t.get()).unwrap_or(30.0);
        let auth_expr = match self.auth_type {
            AuthType::Bearer => quote::quote! { elicit_reqwest::AuthType::Bearer },
            AuthType::Basic => quote::quote! { elicit_reqwest::AuthType::Basic },
            AuthType::ApiKey => quote::quote! { elicit_reqwest::AuthType::ApiKey },
            AuthType::None => quote::quote! { elicit_reqwest::AuthType::None },
        };
        quote::quote! {
            let _plugin = elicit_reqwest::WorkflowPlugin::default_client();
            let (_resp, _proof) = _plugin.auth_fetch(
                #url,
                #token,
                #auth_expr,
                std::time::Duration::from_secs_f64(#timeout),
            ).await.map_err(|e| format!("Auth fetch failed: {}", e))?;
            println!("Status: {}", _resp.status);
            println!("{}", _resp.body);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

/// `post` → POST with body
#[cfg(feature = "emit")]
impl EmitCode for PostParams {
    fn emit_code(&self) -> TokenStream {
        let url = self.url.get().as_str();
        let body = &self.body;
        let timeout = self.timeout_secs.map(|t| t.get()).unwrap_or(30.0);
        let ct_expr = match self.content_type {
            ContentType::Json => quote::quote! { elicit_reqwest::ContentType::Json },
            ContentType::FormUrlEncoded => {
                quote::quote! { elicit_reqwest::ContentType::FormUrlEncoded }
            }
            ContentType::PlainText => quote::quote! { elicit_reqwest::ContentType::PlainText },
            ContentType::OctetStream => quote::quote! { elicit_reqwest::ContentType::OctetStream },
        };
        quote::quote! {
            let _plugin = elicit_reqwest::WorkflowPlugin::default_client();
            let (_resp, _proof) = _plugin.post(
                #url,
                #body,
                #ct_expr,
                std::time::Duration::from_secs_f64(#timeout),
            ).await.map_err(|e| format!("POST failed: {}", e))?;
            println!("Status: {}", _resp.status);
            println!("{}", _resp.body);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

/// `api_call` → authenticated JSON POST
#[cfg(feature = "emit")]
impl EmitCode for ApiCallParams {
    fn emit_code(&self) -> TokenStream {
        let url = self.url.get().as_str();
        let token = &self.token;
        let body = &self.body;
        let timeout = self.timeout_secs.map(|t| t.get()).unwrap_or(30.0);
        quote::quote! {
            let _plugin = elicit_reqwest::WorkflowPlugin::default_client();
            let (_resp, _proof) = _plugin.api_call(
                #url,
                #token,
                #body,
                std::time::Duration::from_secs_f64(#timeout),
            ).await.map_err(|e| format!("API call failed: {}", e))?;
            println!("Status: {}", _resp.status);
            println!("{}", _resp.body);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

/// `health_check` → probe URL, emit status
#[cfg(feature = "emit")]
impl EmitCode for HealthCheckParams {
    fn emit_code(&self) -> TokenStream {
        let url = self.url.get().as_str();
        let timeout = self.timeout_secs.map(|t| t.get()).unwrap_or(10.0);
        quote::quote! {
            let _plugin = elicit_reqwest::WorkflowPlugin::default_client();
            let _healthy = _plugin.health_check(
                #url,
                std::time::Duration::from_secs_f64(#timeout),
            ).await;
            println!("Healthy: {}", _healthy);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

/// `url_build` → construct URL from base + path + query
#[cfg(feature = "emit")]
impl EmitCode for UrlBuildParams {
    fn emit_code(&self) -> TokenStream {
        let base = self.base.get().as_str();
        let path_expr = match &self.path {
            Some(p) => quote::quote! { Some(#p) },
            None => quote::quote! { None::<&str> },
        };
        let query_pairs: Vec<TokenStream> = self
            .query
            .as_ref()
            .map(|q| q.iter().map(|(k, v)| quote::quote! { (#k, #v) }).collect())
            .unwrap_or_default();
        quote::quote! {
            let _url = elicit_reqwest::WorkflowPlugin::build_url(
                #base,
                #path_expr,
                &[ #( #query_pairs ),* ],
            ).map_err(|e| format!("URL build failed: {}", e))?;
            println!("{}", _url);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

/// `status_summary` → classify HTTP status code
#[cfg(feature = "emit")]
impl EmitCode for StatusSummaryParams {
    fn emit_code(&self) -> TokenStream {
        let status = self.status;
        quote::quote! {
            let _code = elicit_reqwest::StatusCode::from_u16(#status)
                .map_err(|e| format!("Invalid status code: {}", e))?;
            let _class = match #status {
                100..=199 => "informational",
                200..=299 => "success",
                300..=399 => "redirection",
                400..=499 => "client_error",
                500..=599 => "server_error",
                _ => "unknown",
            };
            println!(
                "status={} reason={} class={}",
                _code.as_u16(),
                _code.canonical_reason().unwrap_or("Unknown"),
                _class,
            );
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

/// `build_request` → construct a full request and send it
#[cfg(feature = "emit")]
impl EmitCode for BuildRequestParams {
    fn emit_code(&self) -> TokenStream {
        let method = &self.method;
        let url = self.url.get().as_str();
        let auth_expr = match self.auth_type {
            AuthType::Bearer => quote::quote! { elicit_reqwest::AuthType::Bearer },
            AuthType::Basic => quote::quote! { elicit_reqwest::AuthType::Basic },
            AuthType::ApiKey => quote::quote! { elicit_reqwest::AuthType::ApiKey },
            AuthType::None => quote::quote! { elicit_reqwest::AuthType::None },
        };
        let token_stmt = match &self.token {
            Some(t) => quote::quote! { .token(#t) },
            None => quote::quote! {},
        };
        let body_stmt = match &self.body {
            Some(b) => quote::quote! { .body(#b) },
            None => quote::quote! {},
        };
        let timeout_stmt = match self.timeout_secs {
            Some(t) => {
                let t = t.get();
                quote::quote! { .timeout_secs(#t) }
            }
            None => quote::quote! {},
        };
        let ct_stmt = match &self.content_type {
            Some(ContentType::Json) => {
                quote::quote! { .content_type(elicit_reqwest::ContentType::Json) }
            }
            Some(ContentType::FormUrlEncoded) => {
                quote::quote! { .content_type(elicit_reqwest::ContentType::FormUrlEncoded) }
            }
            Some(ContentType::PlainText) => {
                quote::quote! { .content_type(elicit_reqwest::ContentType::PlainText) }
            }
            Some(ContentType::OctetStream) => {
                quote::quote! { .content_type(elicit_reqwest::ContentType::OctetStream) }
            }
            None => quote::quote! {},
        };
        quote::quote! {
            let _plugin = elicit_reqwest::WorkflowPlugin::default_client();
            let _params = elicit_reqwest::BuildRequestParamsBuilder::default()
                .method(#method)
                .url(#url)
                .auth_type(#auth_expr)
                #token_stmt
                #body_stmt
                #ct_stmt
                #timeout_stmt
                .build()
                .map_err(|e| format!("BuildRequestParams build error: {}", e))?;
            let (_resp, _proof) = _plugin.build_request(_params)
                .await.map_err(|e| format!("Request failed: {}", e))?;
            println!("Status: {}", _resp.status);
            println!("{}", _resp.body);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

/// `paginated_get` → follow next-page links
#[cfg(feature = "emit")]
impl EmitCode for PaginatedGetParams {
    fn emit_code(&self) -> TokenStream {
        let url = self.url.get().as_str();
        let token_expr = match &self.token {
            Some(t) => quote::quote! { Some(#t) },
            None => quote::quote! { None::<&str> },
        };
        let timeout = self.timeout_secs.map(|t| t.get()).unwrap_or(30.0);
        quote::quote! {
            let _plugin = elicit_reqwest::WorkflowPlugin::default_client();
            let _pages = _plugin.paginated_get(
                #url,
                #token_expr,
                std::time::Duration::from_secs_f64(#timeout),
            ).await.map_err(|e| format!("Paginated GET failed: {}", e))?;
            for (_i, _page) in _pages.iter().enumerate() {
                println!("--- Page {} ---", _i + 1);
                println!("{}", _page);
            }
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP, ELICIT_REQWEST_DEP]
    }
}

// ── Public dispatch for cross-crate EmitCode recovery ────────────────────────

/// Deserialize a tool's params from JSON and return its [`EmitCode`] impl.
///
/// Used by `elicit_server` to recover reqwest workflow steps without
/// exposing internal param structs.
///
/// Returns `Err` if `tool_name` is unknown or `params` fails to deserialize.
#[cfg(feature = "emit")]
pub fn dispatch_emit(
    tool_name: &str,
    params: serde_json::Value,
) -> Result<Box<dyn EmitCode>, String> {
    match tool_name {
        "fetch" => serde_json::from_value::<FetchParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "auth_fetch" => serde_json::from_value::<AuthFetchParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "post" => serde_json::from_value::<PostParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "api_call" => serde_json::from_value::<ApiCallParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "health_check" => serde_json::from_value::<HealthCheckParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "url_build" => serde_json::from_value::<UrlBuildParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "status_summary" => serde_json::from_value::<StatusSummaryParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "build_request" => serde_json::from_value::<BuildRequestParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "paginated_get" => serde_json::from_value::<PaginatedGetParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        other => Err(format!("Unknown reqwest tool: '{other}'")),
    }
}

// ── Global emit registry ──────────────────────────────────────────────────────

#[cfg(feature = "emit")]
elicitation::register_emit!("fetch", FetchParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("auth_fetch", AuthFetchParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("post", PostParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("api_call", ApiCallParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("health_check", HealthCheckParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("url_build", UrlBuildParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("status_summary", StatusSummaryParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("build_request", BuildRequestParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("paginated_get", PaginatedGetParams);
