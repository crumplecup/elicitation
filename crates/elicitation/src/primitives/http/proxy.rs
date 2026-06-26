//! Trenchcoats for `reqwest::Proxy` and `reqwest::NoProxy`.

use std::{borrow::Cow, str::FromStr};

use crate::{
    ElicitCommunicator, ElicitComplete, ElicitIntrospect, ElicitPromptTree, ElicitResult,
    ElicitSpec, Elicitation, Prompt, PromptTree, SpecCategory, SpecEntry, TypeMetadata, TypeSpec,
    emit_code::ToCodeLiteral, type_spec::TypeSpecInventoryKey,
};
use http::{HeaderMap, HeaderValue};
use proc_macro2::TokenStream;
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use url::Url;

/// Owned `NO_PROXY` exclusion-list trenchcoat.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a reqwest no-proxy exclusion list:")]
#[to_code_literal(path = "::elicitation::ReqwestNoProxy::from_string", tuple)]
pub struct ReqwestNoProxy {
    #[prompt("Raw NO_PROXY exclusion-list string:")]
    raw: String,
}

impl ReqwestNoProxy {
    /// Record a raw comma-separated exclusion list.
    #[tracing::instrument(skip(raw), level = "debug")]
    pub fn from_string(raw: impl Into<String>) -> Self {
        Self { raw: raw.into() }
    }

    /// Borrow the raw exclusion-list string.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Rebuild a raw `reqwest::NoProxy`.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> ElicitResult<reqwest::NoProxy> {
        reqwest::NoProxy::from_string(&self.raw).ok_or_else(|| {
            crate::ElicitError::new(crate::ElicitErrorKind::ParseError(format!(
                "reqwest::NoProxy could not parse exclusion list: {}",
                self.raw
            )))
        })
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestNoProxy",
    <ReqwestNoProxy as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestNoProxy>
));

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a preserved HTTP header value for proxy configuration:")]
struct ProxyHeaderValue {
    #[prompt("Exact header bytes:")]
    bytes: Vec<u8>,
}

impl ProxyHeaderValue {
    #[tracing::instrument(skip(header_value), level = "debug")]
    fn from_header_value(header_value: HeaderValue) -> Self {
        Self {
            bytes: header_value.as_bytes().to_vec(),
        }
    }

    #[tracing::instrument(skip(self), level = "debug")]
    fn build_raw(&self) -> ElicitResult<HeaderValue> {
        HeaderValue::from_bytes(&self.bytes).map_err(crate::ElicitError::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a preserved HTTP header entry for proxy configuration:")]
struct ProxyHeaderEntry {
    #[prompt("Header name:")]
    name: String,
    #[prompt("Header value:")]
    value: ProxyHeaderValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe preserved proxy headers:")]
struct ProxyHeaderMap {
    #[serde(default)]
    #[prompt("Ordered header entries:")]
    entries: Vec<ProxyHeaderEntry>,
}

impl ProxyHeaderMap {
    #[tracing::instrument(skip(headers), level = "debug")]
    fn from_header_map(headers: HeaderMap) -> Self {
        let mut entries = Vec::new();
        for (name, value) in &headers {
            entries.push(ProxyHeaderEntry {
                name: name.as_str().to_string(),
                value: ProxyHeaderValue::from_header_value(value.clone()),
            });
        }
        Self { entries }
    }

    #[tracing::instrument(skip(self), level = "debug")]
    fn build_raw(&self) -> ElicitResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        for entry in &self.entries {
            let name =
                http::HeaderName::from_bytes(entry.name.as_bytes()).map_err(crate::ElicitError::from)?;
            let value = entry.value.build_raw()?;
            headers.append(name, value);
        }
        Ok(headers)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe how the reqwest proxy is initially constructed:")]
enum ReqwestProxyCtor {
    Http {
        #[prompt("Proxy URL:")]
        url: Url,
    },
    Https {
        #[prompt("Proxy URL:")]
        url: Url,
    },
    All {
        #[prompt("Proxy URL:")]
        url: Url,
    },
    Custom {
        #[prompt("Original Rust expression used to build the custom proxy selector:")]
        code: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a reqwest proxy builder step:")]
enum ReqwestProxyStep {
    BasicAuth {
        #[prompt("Basic-auth username:")]
        username: String,
        #[prompt("Basic-auth password:")]
        password: String,
    },
    CustomHttpAuth {
        #[prompt("Proxy-Authorization header value:")]
        header_value: ProxyHeaderValue,
    },
    Headers {
        #[prompt("Custom proxy headers:")]
        headers: ProxyHeaderMap,
    },
    NoProxy {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[prompt("Optional no-proxy exclusion list:")]
        no_proxy: Option<ReqwestNoProxy>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a reqwest proxy trenchcoat:")]
struct ReqwestProxySnapshot {
    #[prompt("Initial proxy constructor:")]
    ctor: ReqwestProxyCtor,
    #[serde(default)]
    #[prompt("Ordered proxy builder steps:")]
    steps: Vec<ReqwestProxyStep>,
}

/// Elicitation-aware trenchcoat for `reqwest::Proxy`.
#[derive(Clone)]
pub struct ReqwestProxy {
    raw: Option<reqwest::Proxy>,
    snapshot: ReqwestProxySnapshot,
}

impl std::fmt::Debug for ReqwestProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReqwestProxy")
            .field("snapshot", &self.snapshot)
            .finish_non_exhaustive()
    }
}

impl Serialize for ReqwestProxy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ReqwestProxy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            raw: None,
            snapshot: ReqwestProxySnapshot::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for ReqwestProxy {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("ReqwestProxy")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ReqwestProxySnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for ReqwestProxy {
    fn prompt() -> Option<&'static str> {
        Some("Describe a reqwest proxy trenchcoat:")
    }
}

impl Elicitation for ReqwestProxy {
    type Style = ();

    #[tracing::instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = ReqwestProxySnapshot::elicit(communicator).await?;
        Ok(Self {
            raw: None,
            snapshot,
        })
    }

    fn kani_proof() -> TokenStream {
        ReqwestProxySnapshot::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        ReqwestProxySnapshot::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        ReqwestProxySnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for ReqwestProxy {
    fn pattern() -> crate::ElicitationPattern {
        ReqwestProxySnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ReqwestProxy",
            description: Self::prompt(),
            details: ReqwestProxySnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for ReqwestProxy {
    fn prompt_tree() -> PromptTree {
        match ReqwestProxySnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "elicitation::ReqwestProxy".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for ReqwestProxy {
    fn type_spec() -> TypeSpec {
        TypeSpec::new(
            "elicitation::ReqwestProxy",
            "Owned trenchcoat for reqwest::Proxy that preserves constructor provenance and ordered builder mutations.",
            vec![
                SpecCategory::new(
                    "construction",
                    vec![
                        SpecEntry::new(
                            "http",
                            "Constructs an HTTP-only proxy from a URL.",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestProxy::http(url)".to_string(),
                        )),
                        SpecEntry::new(
                            "https",
                            "Constructs an HTTPS-only proxy from a URL.",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestProxy::https(url)".to_string(),
                        )),
                        SpecEntry::new(
                            "all",
                            "Constructs a proxy used for all schemes from a URL.",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestProxy::all(url)".to_string(),
                        )),
                        SpecEntry::new(
                            "custom",
                            "Records the original Rust proxy-selector expression for a custom proxy constructor.",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestProxy::custom(code)".to_string(),
                        )),
                    ],
                ),
                SpecCategory::new(
                    "mutation",
                    vec![
                        SpecEntry::new(
                            "basic_auth",
                            "Adds basic-auth credentials to the proxy builder recipe.",
                        )
                        .with_expression(Some("proxy.basic_auth(username, password)".to_string())),
                        SpecEntry::new(
                            "custom_http_auth",
                            "Adds a Proxy-Authorization header value to the proxy builder recipe.",
                        )
                        .with_expression(Some(
                            "proxy.custom_http_auth(header_value)".to_string(),
                        )),
                        SpecEntry::new(
                            "headers",
                            "Adds custom headers to the proxy builder recipe.",
                        )
                        .with_expression(Some("proxy.headers(headers)".to_string())),
                        SpecEntry::new(
                            "no_proxy",
                            "Adds an optional no-proxy exclusion list to the proxy builder recipe.",
                        )
                        .with_expression(Some("proxy.no_proxy(no_proxy)".to_string())),
                    ],
                ),
                SpecCategory::new(
                    "recovery",
                    vec![
                        SpecEntry::new(
                            "build_raw",
                            "Rebuilds a raw reqwest::Proxy from the recorded constructor and ordered builder steps. Custom constructors require a live wrapped proxy to still be present.",
                        )
                        .with_expression(Some("proxy.build_raw()".to_string())),
                    ],
                ),
            ],
        )
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestProxy",
    <ReqwestProxy as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestProxy>
));

impl ToCodeLiteral for ReqwestProxy {
    fn to_code_literal(&self) -> TokenStream {
        let ctor = ctor_tokens(&self.snapshot.ctor);
        let steps = self.snapshot.steps.iter().map(step_tokens);

        quote::quote! {{
            let proxy = #ctor;
            #(#steps)*
            proxy
        }}
    }
}

impl ElicitComplete for ReqwestProxy {}

impl ReqwestProxy {
    #[tracing::instrument(level = "debug")]
    fn from_ctor(raw: Option<reqwest::Proxy>, ctor: ReqwestProxyCtor) -> Self {
        Self {
            raw,
            snapshot: ReqwestProxySnapshot {
                ctor,
                steps: Vec::new(),
            },
        }
    }

    #[tracing::instrument(skip(self), level = "debug")]
    fn with_step(mut self, step: ReqwestProxyStep) -> Self {
        self.raw = match (&self.raw, &step) {
            (Some(raw), ReqwestProxyStep::BasicAuth { username, password }) => {
                Some(raw.clone().basic_auth(username, password))
            }
            (Some(raw), ReqwestProxyStep::CustomHttpAuth { header_value }) => {
                match header_value.build_raw() {
                    Ok(header_value) => Some(raw.clone().custom_http_auth(header_value)),
                    Err(error) => {
                        tracing::warn!(?error, "Dropping live reqwest::Proxy after invalid header value snapshot");
                        None
                    }
                }
            }
            (Some(raw), ReqwestProxyStep::Headers { headers }) => {
                match headers.build_raw() {
                    Ok(headers) => Some(raw.clone().headers(headers)),
                    Err(error) => {
                        tracing::warn!(?error, "Dropping live reqwest::Proxy after invalid header map snapshot");
                        None
                    }
                }
            }
            (Some(raw), ReqwestProxyStep::NoProxy { no_proxy }) => match no_proxy {
                Some(entry) => match entry.build_raw() {
                    Ok(no_proxy) => Some(raw.clone().no_proxy(Some(no_proxy))),
                    Err(error) => {
                        tracing::warn!(?error, "Dropping live reqwest::Proxy after invalid no_proxy snapshot");
                        None
                    }
                },
                None => Some(raw.clone().no_proxy(None)),
            },
            (None, _) => None,
        };
        self.snapshot.steps.push(step);
        self
    }

    /// Construct an HTTP-only proxy trenchcoat.
    #[tracing::instrument(level = "debug")]
    pub fn http(url: Url) -> ElicitResult<Self> {
        let raw = reqwest::Proxy::http(url.as_str())?;
        Ok(Self::from_ctor(Some(raw), ReqwestProxyCtor::Http { url }))
    }

    /// Construct an HTTPS-only proxy trenchcoat.
    #[tracing::instrument(level = "debug")]
    pub fn https(url: Url) -> ElicitResult<Self> {
        let raw = reqwest::Proxy::https(url.as_str())?;
        Ok(Self::from_ctor(Some(raw), ReqwestProxyCtor::Https { url }))
    }

    /// Construct an all-schemes proxy trenchcoat.
    #[tracing::instrument(level = "debug")]
    pub fn all(url: Url) -> ElicitResult<Self> {
        let raw = reqwest::Proxy::all(url.as_str())?;
        Ok(Self::from_ctor(Some(raw), ReqwestProxyCtor::All { url }))
    }

    /// Record a custom proxy-selector expression.
    ///
    /// This preserves the constructor for elicitation, schema, and code recovery.
    /// Runtime reconstruction of a raw `reqwest::Proxy` from this recipe requires
    /// a live wrapped proxy, which can be provided via [`ReqwestProxy::custom_raw`].
    #[tracing::instrument(skip(code), level = "debug")]
    pub fn custom(code: impl Into<String>) -> Self {
        Self::from_ctor(
            None,
            ReqwestProxyCtor::Custom { code: code.into() },
        )
    }

    /// Wrap a live custom proxy together with the original constructor expression.
    #[tracing::instrument(skip(raw, code), level = "debug")]
    pub fn custom_raw(raw: reqwest::Proxy, code: impl Into<String>) -> Self {
        Self::from_ctor(
            Some(raw),
            ReqwestProxyCtor::Custom { code: code.into() },
        )
    }

    /// Rebuild a raw `reqwest::Proxy`.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> ElicitResult<reqwest::Proxy> {
        let mut raw = match &self.snapshot.ctor {
            ReqwestProxyCtor::Http { url } => reqwest::Proxy::http(url.as_str())?,
            ReqwestProxyCtor::Https { url } => reqwest::Proxy::https(url.as_str())?,
            ReqwestProxyCtor::All { url } => reqwest::Proxy::all(url.as_str())?,
            ReqwestProxyCtor::Custom { code } => {
                if let Some(raw) = &self.raw {
                    raw.clone()
                } else {
                    return Err(crate::ElicitError::new(crate::ElicitErrorKind::ParseError(
                        format!(
                            "reqwest::Proxy custom constructor cannot be rebuilt without a live wrapped proxy: {}",
                            code
                        ),
                    )));
                }
            }
        };

        for step in &self.snapshot.steps {
            raw = apply_step(raw, step)?;
        }

        Ok(raw)
    }

    /// Add proxy basic-auth credentials to the trenchcoat recipe.
    #[tracing::instrument(skip(self, username, password), level = "debug")]
    pub fn basic_auth(
        self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.with_step(ReqwestProxyStep::BasicAuth {
            username: username.into(),
            password: password.into(),
        })
    }

    /// Add a `Proxy-Authorization` header value to the trenchcoat recipe.
    #[tracing::instrument(skip(self, header_value), level = "debug")]
    pub fn custom_http_auth(self, header_value: HeaderValue) -> Self {
        self.with_step(ReqwestProxyStep::CustomHttpAuth {
            header_value: ProxyHeaderValue::from_header_value(header_value),
        })
    }

    /// Add custom proxy headers to the trenchcoat recipe.
    #[tracing::instrument(skip(self, headers), level = "debug")]
    pub fn headers(self, headers: HeaderMap) -> Self {
        self.with_step(ReqwestProxyStep::Headers {
            headers: ProxyHeaderMap::from_header_map(headers),
        })
    }

    /// Add an optional no-proxy exclusion list to the trenchcoat recipe.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn no_proxy(self, no_proxy: Option<ReqwestNoProxy>) -> Self {
        self.with_step(ReqwestProxyStep::NoProxy { no_proxy })
    }
}

#[tracing::instrument(skip(step), level = "debug")]
fn apply_step(raw: reqwest::Proxy, step: &ReqwestProxyStep) -> ElicitResult<reqwest::Proxy> {
    let updated = match step {
        ReqwestProxyStep::BasicAuth { username, password } => raw.basic_auth(username, password),
        ReqwestProxyStep::CustomHttpAuth { header_value } => {
            raw.custom_http_auth(header_value.build_raw()?)
        }
        ReqwestProxyStep::Headers { headers } => raw.headers(headers.build_raw()?),
        ReqwestProxyStep::NoProxy { no_proxy } => {
            raw.no_proxy(no_proxy.as_ref().map(ReqwestNoProxy::build_raw).transpose()?)
        }
    };

    Ok(updated)
}

#[tracing::instrument(skip(ctor), level = "trace")]
fn ctor_tokens(ctor: &ReqwestProxyCtor) -> TokenStream {
    match ctor {
        ReqwestProxyCtor::Http { url } => {
            let url = url.to_code_literal();
            quote::quote! {
                match ::elicitation::ReqwestProxy::http(#url) {
                    ::std::result::Result::Ok(proxy) => proxy,
                    ::std::result::Result::Err(error) => {
                        return ::std::result::Result::Err(error);
                    }
                }
            }
        }
        ReqwestProxyCtor::Https { url } => {
            let url = url.to_code_literal();
            quote::quote! {
                match ::elicitation::ReqwestProxy::https(#url) {
                    ::std::result::Result::Ok(proxy) => proxy,
                    ::std::result::Result::Err(error) => {
                        return ::std::result::Result::Err(error);
                    }
                }
            }
        }
        ReqwestProxyCtor::All { url } => {
            let url = url.to_code_literal();
            quote::quote! {
                match ::elicitation::ReqwestProxy::all(#url) {
                    ::std::result::Result::Ok(proxy) => proxy,
                    ::std::result::Result::Err(error) => {
                        return ::std::result::Result::Err(error);
                    }
                }
            }
        }
        ReqwestProxyCtor::Custom { code } => {
            let literal = code.clone();
            match TokenStream::from_str(code) {
                Ok(custom_ctor) => quote::quote! {
                    ::elicitation::ReqwestProxy::custom_raw(
                        reqwest::Proxy::custom(#custom_ctor),
                        ::std::string::String::from(#literal),
                    )
                },
                Err(error) => {
                    let message = format!(
                        "reqwest::Proxy custom constructor code could not be parsed back into Rust tokens: {error}"
                    );
                    quote::quote! {
                        compile_error!(#message);
                    }
                }
            }
        }
    }
}

#[tracing::instrument(skip(step), level = "trace")]
fn step_tokens(step: &ReqwestProxyStep) -> TokenStream {
    match step {
        ReqwestProxyStep::BasicAuth { username, password } => quote::quote! {
            let proxy = proxy.basic_auth(
                ::std::string::String::from(#username),
                ::std::string::String::from(#password),
            );
        },
        ReqwestProxyStep::CustomHttpAuth { header_value } => {
            let header_value = header_value.to_code_literal();
            quote::quote! {
                let proxy = proxy.custom_http_auth(#header_value);
            }
        }
        ReqwestProxyStep::Headers { headers } => {
            let headers = headers.to_code_literal();
            quote::quote! {
                let proxy = proxy.headers(#headers);
            }
        }
        ReqwestProxyStep::NoProxy { no_proxy } => {
            let no_proxy = match no_proxy {
                Some(no_proxy) => {
                    let no_proxy = no_proxy.to_code_literal();
                    quote::quote! { ::std::option::Option::Some(#no_proxy) }
                }
                None => quote::quote! { ::std::option::Option::None },
            };
            quote::quote! {
                let proxy = proxy.no_proxy(#no_proxy);
            }
        }
    }
}
