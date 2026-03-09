//! `UrlWorkflowPlugin` — contract-verified URL composition tools.
//!
//! While the atomic `Url` newtype wraps `url::Url` for MCP reflection, this
//! plugin provides **phrase-level** tools: parse-and-validate, HTTPS enforcement,
//! building canonical URLs from parts, and relative resolution.
//!
//! # Typestate Design
//!
//! ```text
//! UnvalidatedUrl ──parse()──→ ParsedUrl + Established<UrlParsed>
//!                                  │
//!                        assert_https()
//!                                  │
//!                                  ↓
//!                          SecureUrl + Established<HttpsRequired>
//! ```
//!
//! **Key invariant**: `SecureUrl` cannot be constructed without first proving
//! `UrlParsed`. The scheme check is a second transition on an already-parsed
//! URL — you can never skip parsing.
//!
//! # Propositions and Contracts
//!
//! ```text
//! parse_url:        UrlParsed
//! assert_https:     UrlParsed ∧ HttpsRequired
//! validate_scheme:  UrlParsed ∧ SchemeAllowed
//! build_url:        UrlParsed
//! join_url:         UrlParsed(base) ∧ UrlParsed(result)
//! ```
//!
//! Registered under the `"url_workflow"` namespace.

use elicitation::contracts::{And, Established, Prop};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: the input string is a syntactically valid URL.
pub struct UrlParsed;
impl Prop for UrlParsed {}

/// Proposition: the URL scheme is specifically `https`.
pub struct HttpsRequired;
impl Prop for HttpsRequired {}

/// Proposition: the URL scheme is in the caller-supplied allow-list.
pub struct SchemeAllowed;
impl Prop for SchemeAllowed {}

/// Composite: URL is parsed AND scheme is https.
pub type SecureUrl = And<UrlParsed, HttpsRequired>;

// ── Typestate structs ─────────────────────────────────────────────────────────

/// An unvalidated URL string — the initial state.
///
/// Nothing has been proved yet. The only transition is `parse()`.
pub struct UnvalidatedUrl {
    src: String,
}

/// A successfully parsed URL.
///
/// Carries the parsed `url::Url` internally. Can transition to `ValidatedSchemeUrl`
/// via `assert_https()` or `validate_scheme()`.
pub struct ParsedUrl {
    /// The inner parsed URL value.
    pub inner: url::Url,
}

/// A parsed URL additionally proven to use HTTPS.
///
/// Cannot be constructed without first proving `UrlParsed`.
pub struct SecureUrlState {
    /// The inner validated HTTPS URL value.
    pub inner: url::Url,
}

// ── Typestate transitions ─────────────────────────────────────────────────────

impl UnvalidatedUrl {
    /// Wrap a raw string as an unvalidated URL.
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }

    /// Parse the input, establishing `UrlParsed` proof on success.
    ///
    /// This is the only way to enter the validated URL space.
    pub fn parse(self) -> Result<(ParsedUrl, Established<UrlParsed>), String> {
        url::Url::parse(&self.src)
            .map(|inner| (ParsedUrl { inner }, Established::assert()))
            .map_err(|e| format!("UrlParsed not established: {e}"))
    }
}

impl ParsedUrl {
    /// Return the canonical URL string.
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    /// Assert that the scheme is `https`, establishing `HttpsRequired`.
    ///
    /// Requires an existing `UrlParsed` proof — you cannot skip parsing.
    pub fn assert_https(
        self,
        parsed: Established<UrlParsed>,
    ) -> Result<(SecureUrlState, Established<SecureUrl>), String> {
        if self.inner.scheme() == "https" {
            let secure_proof =
                elicitation::contracts::both(parsed, Established::<HttpsRequired>::assert());
            Ok((SecureUrlState { inner: self.inner }, secure_proof))
        } else {
            Err(format!(
                "HttpsRequired not established: scheme is '{}', expected 'https'",
                self.inner.scheme()
            ))
        }
    }

    /// Assert that the scheme is in `allowed`, establishing `SchemeAllowed`.
    pub fn validate_scheme(
        self,
        allowed: &[&str],
        _parsed: Established<UrlParsed>,
    ) -> Result<(ParsedUrl, Established<And<UrlParsed, SchemeAllowed>>), String> {
        if allowed.contains(&self.inner.scheme()) {
            let proof = elicitation::contracts::both(
                Established::<UrlParsed>::assert(),
                Established::<SchemeAllowed>::assert(),
            );
            Ok((self, proof))
        } else {
            Err(format!(
                "SchemeAllowed not established: scheme '{}' not in {:?}",
                self.inner.scheme(),
                allowed,
            ))
        }
    }
}

impl SecureUrlState {
    /// Return the canonical URL string.
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

// ── Params structs ────────────────────────────────────────────────────────────

/// Parameters for [`UrlWorkflowPlugin::parse_url`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ParseUrlParams {
    /// Raw URL string to parse. Assumes: non-empty string.
    pub url: String,
}

/// Parameters for [`UrlWorkflowPlugin::assert_https`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AssertHttpsParams {
    /// Raw URL string. Assumes: syntactically valid URL.
    pub url: String,
}

/// Parameters for [`UrlWorkflowPlugin::validate_scheme`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ValidateSchemeParams {
    /// Raw URL string. Assumes: syntactically valid URL.
    pub url: String,
    /// Allowed scheme values (e.g. `["https", "wss"]`).
    pub allowed_schemes: Vec<String>,
}

/// Parameters for [`UrlWorkflowPlugin::build_url`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct BuildUrlParams {
    /// Base URL string. Assumes: syntactically valid URL.
    pub base: String,
    /// Optional path to append (e.g. `"/v1/users"`).
    pub path: Option<String>,
    /// Optional query parameters to append.
    pub params: Option<HashMap<String, String>>,
}

/// Parameters for [`UrlWorkflowPlugin::join_url`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct JoinUrlParams {
    /// Base URL string. Assumes: syntactically valid URL.
    pub base: String,
    /// Relative URL or path to resolve against the base.
    pub relative: String,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing contract-verified URL composition tools.
///
/// Each tool documents the propositions it **assumes** and **establishes**.
/// The Rust implementation carries proofs internally via
/// [`elicitation::contracts::Established`] — zero-cost `PhantomData` markers.
///
/// Register under the `"url_workflow"` namespace:
///
/// ```ignore
/// use elicitation::PluginRegistry;
/// use elicit_url::UrlWorkflowPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("url_workflow", UrlWorkflowPlugin);
/// ```
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "url_workflow")]
pub struct UrlWorkflowPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "url_workflow",
    name = "parse_url",
    description = "Parse a raw URL string and validate its syntax. \
                   Establishes: UrlParsed. \
                   Returns scheme, host, port, path, query, and fragment."
)]
#[instrument(skip_all)]
async fn parse_url(p: ParseUrlParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, _proof) = match UnvalidatedUrl::new(p.url).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let inner = &parsed.inner;
    let summary = format!(
        "UrlParsed established.\n\
         url:      {}\n\
         scheme:   {}\n\
         host:     {}\n\
         port:     {}\n\
         path:     {}\n\
         query:    {}\n\
         fragment: {}",
        inner.as_str(),
        inner.scheme(),
        inner.host_str().unwrap_or(""),
        inner
            .port()
            .map(|port| port.to_string())
            .unwrap_or_default(),
        inner.path(),
        inner.query().unwrap_or(""),
        inner.fragment().unwrap_or(""),
    );
    Ok(CallToolResult::success(vec![Content::text(summary)]))
}

#[elicit_tool(
    plugin = "url_workflow",
    name = "assert_https",
    description = "Parse a URL and assert that its scheme is 'https'. \
                   Establishes: UrlParsed ∧ HttpsRequired. \
                   Fails if the URL is invalid OR the scheme is not https."
)]
#[instrument(skip_all)]
async fn assert_https(p: AssertHttpsParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, parsed_proof) = match UnvalidatedUrl::new(p.url).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let (secure, _proof) = match parsed.assert_https(parsed_proof) {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    Ok(CallToolResult::success(vec![Content::text(format!(
        "UrlParsed ∧ HttpsRequired established.\nurl: {}",
        secure.as_str()
    ))]))
}

#[elicit_tool(
    plugin = "url_workflow",
    name = "validate_scheme",
    description = "Parse a URL and assert that its scheme is in the supplied allow-list. \
                   Establishes: UrlParsed ∧ SchemeAllowed. \
                   Useful for restricting to ['https', 'wss'] or similar safe sets."
)]
#[instrument(skip_all)]
async fn validate_scheme(p: ValidateSchemeParams) -> Result<CallToolResult, ErrorData> {
    let allowed: Vec<&str> = p.allowed_schemes.iter().map(String::as_str).collect();
    let (parsed, parsed_proof) = match UnvalidatedUrl::new(p.url).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let (validated, _proof) = match parsed.validate_scheme(&allowed, parsed_proof) {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    Ok(CallToolResult::success(vec![Content::text(format!(
        "UrlParsed ∧ SchemeAllowed established.\nscheme: {}\nurl: {}",
        validated.inner.scheme(),
        validated.inner.as_str()
    ))]))
}

#[elicit_tool(
    plugin = "url_workflow",
    name = "build_url",
    description = "Build a canonical URL from a base, optional path, and optional query params. \
                   Establishes: UrlParsed on the resulting URL. \
                   The result is percent-encoded and normalized."
)]
#[instrument(skip_all)]
async fn build_url(p: BuildUrlParams) -> Result<CallToolResult, ErrorData> {
    let mut url = match url::Url::parse(&p.base) {
        Ok(u) => u,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "UrlParsed not established for base: {e}"
            ))]));
        }
    };
    if let Some(path) = p.path {
        // Merge path: ensure base has trailing slash then join
        let base_str = url.as_str().trim_end_matches('/').to_string() + "/";
        let base = url::Url::parse(&base_str).unwrap_or(url.clone());
        let path_clean = path.trim_start_matches('/');
        url = match base.join(path_clean) {
            Ok(u) => u,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Path join failed: {e}"
                ))]));
            }
        };
    }
    if let Some(qp) = p.params {
        let mut pairs: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        for (k, v) in qp {
            pairs.push((k, v));
        }
        let mut new_url = url.clone();
        new_url.set_query(None);
        {
            let mut serializer = new_url.query_pairs_mut();
            for (k, v) in &pairs {
                serializer.append_pair(k, v);
            }
        }
        url = new_url;
    }
    Ok(CallToolResult::success(vec![Content::text(format!(
        "UrlParsed established.\nurl: {}",
        url.as_str()
    ))]))
}

#[elicit_tool(
    plugin = "url_workflow",
    name = "join_url",
    description = "Resolve a relative URL or path against a base URL (RFC 3986). \
                   Establishes: UrlParsed(base) ∧ UrlParsed(result). \
                   Handles `../`, query strings, and fragment identifiers correctly."
)]
#[instrument(skip_all)]
async fn join_url(p: JoinUrlParams) -> Result<CallToolResult, ErrorData> {
    let base = match url::Url::parse(&p.base) {
        Ok(u) => u,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "UrlParsed not established for base: {e}"
            ))]));
        }
    };
    let result = match base.join(&p.relative) {
        Ok(u) => u,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Join failed: {e}"
            ))]));
        }
    };
    Ok(CallToolResult::success(vec![Content::text(format!(
        "UrlParsed(base) ∧ UrlParsed(result) established.\nresult: {}",
        result.as_str()
    ))]))
}
