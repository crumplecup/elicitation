//! Shadows for `reqwest::Proxy` and `reqwest::NoProxy`.

use std::{borrow::Cow, str::FromStr};

use elicitation::{
    Elicit, ElicitCommunicator, ElicitIntrospect, ElicitPromptTree, ElicitResult, ElicitSpec,
    Elicitation, Prompt, PromptTree, TypeMetadata, TypeSpec, proc_macro2::TokenStream,
};
use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};

use crate::{Error, HeaderMap, Url};

/// Owned no-proxy exclusion list.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a no-proxy exclusion list:")]
#[to_code_literal(path = "::elicit_reqwest::NoProxy::from_string", tuple)]
pub struct NoProxy {
    #[prompt("Raw NO_PROXY list string:")]
    raw: String,
}

impl NoProxy {
    /// Construct a shadow no-proxy list from its raw comma-separated form.
    pub fn from_string(raw: impl Into<String>) -> Self {
        Self { raw: raw.into() }
    }

    /// Borrow the raw exclusion-list string.
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// Rebuild a raw `reqwest::NoProxy`.
    pub fn build_raw(&self) -> Option<reqwest::NoProxy> {
        reqwest::NoProxy::from_string(&self.raw)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe how the proxy is initially constructed:")]
enum ProxyCtor {
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
        #[serde(default, skip_serializing_if = "String::is_empty")]
        #[prompt("Original Rust expression used to build the custom proxy constructor, when available:")]
        code: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a proxy builder step:")]
enum ProxyStep {
    BasicAuth {
        #[prompt("Basic-auth username:")]
        username: String,
        #[prompt("Basic-auth password:")]
        password: String,
    },
    CustomHttpAuth {
        #[prompt("Raw Proxy-Authorization header value:")]
        header_value: String,
    },
    Headers {
        #[prompt("Custom proxy headers:")]
        headers: HeaderMap,
    },
    NoProxy {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[prompt("Optional no-proxy exclusions:")]
        no_proxy: Option<NoProxy>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP proxy configuration:")]
struct ProxySnapshot {
    #[prompt("Initial proxy constructor:")]
    ctor: ProxyCtor,
    #[serde(default)]
    #[prompt("Ordered proxy builder steps:")]
    steps: Vec<ProxyStep>,
}

/// Elicitation-aware `reqwest::Proxy` wrapper.
#[derive(Clone)]
pub struct Proxy {
    raw: Option<reqwest::Proxy>,
    snapshot: ProxySnapshot,
}

impl std::fmt::Debug for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Proxy")
            .field("snapshot", &self.snapshot)
            .finish_non_exhaustive()
    }
}

impl Serialize for Proxy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Proxy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            raw: None,
            snapshot: ProxySnapshot::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for Proxy {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("Proxy")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ProxySnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for Proxy {
    fn prompt() -> Option<&'static str> {
        Some("Describe an HTTP proxy configuration:")
    }
}

impl Elicitation for Proxy {
    type Style = ();

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = ProxySnapshot::elicit(communicator).await?;
        Ok(Self {
            raw: None,
            snapshot,
        })
    }

    fn kani_proof() -> TokenStream {
        ProxySnapshot::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        ProxySnapshot::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        ProxySnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for Proxy {
    fn pattern() -> elicitation::ElicitationPattern {
        ProxySnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "Proxy",
            description: Self::prompt(),
            details: ProxySnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for Proxy {
    fn prompt_tree() -> PromptTree {
        match ProxySnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "Proxy".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for Proxy {
    fn type_spec() -> TypeSpec {
        let base = ProxySnapshot::type_spec();
        TypeSpec::new(
            "Proxy",
            "Owned reqwest proxy wrapper with a constructor snapshot and ordered builder steps.",
            base.categories().clone(),
        )
    }
}

impl Proxy {
    fn from_ctor(raw: reqwest::Proxy, ctor: ProxyCtor) -> Self {
        Self {
            raw: Some(raw),
            snapshot: ProxySnapshot {
                ctor,
                steps: Vec::new(),
            },
        }
    }

    fn with_step(mut self, step: ProxyStep) -> Self {
        match &step {
            ProxyStep::BasicAuth { username, password } => {
                self.raw = self
                    .raw
                    .take()
                    .map(|raw| raw.basic_auth(username, password));
            }
            ProxyStep::CustomHttpAuth { header_value } => {
                self.raw = self.raw.take().and_then(|raw| {
                    http::HeaderValue::from_str(header_value)
                        .ok()
                        .map(|header| raw.custom_http_auth(header))
                });
            }
            ProxyStep::Headers { headers } => {
                self.raw = self
                    .raw
                    .take()
                    .map(|raw| raw.headers(http::HeaderMap::from(headers.clone())));
            }
            ProxyStep::NoProxy { no_proxy } => {
                self.raw = self
                    .raw
                    .take()
                    .map(|raw| raw.no_proxy(no_proxy.as_ref().and_then(NoProxy::build_raw)));
            }
        }

        self.snapshot.steps.push(step);
        self
    }

    fn ctor_tokens(&self) -> TokenStream {
        match &self.snapshot.ctor {
            ProxyCtor::Http { url } => {
                let url = url.to_code_literal();
                quote::quote! {
                    match ::elicit_reqwest::Proxy::http(#url) {
                        ::std::result::Result::Ok(proxy) => proxy,
                        ::std::result::Result::Err(error) => {
                            return ::std::result::Result::Err(error.into());
                        }
                    }
                }
            }
            ProxyCtor::Https { url } => {
                let url = url.to_code_literal();
                quote::quote! {
                    match ::elicit_reqwest::Proxy::https(#url) {
                        ::std::result::Result::Ok(proxy) => proxy,
                        ::std::result::Result::Err(error) => {
                            return ::std::result::Result::Err(error.into());
                        }
                    }
                }
            }
            ProxyCtor::All { url } => {
                let url = url.to_code_literal();
                quote::quote! {
                    match ::elicit_reqwest::Proxy::all(#url) {
                        ::std::result::Result::Ok(proxy) => proxy,
                        ::std::result::Result::Err(error) => {
                            return ::std::result::Result::Err(error.into());
                        }
                    }
                }
            }
            ProxyCtor::Custom { code } => {
                if code.trim().is_empty() {
                    let message =
                        "elicit_reqwest::Proxy::custom is missing constructor code provenance; call Proxy::with_ctor_code on the shadow wrapper";
                    quote::quote! {
                        {
                            return ::std::result::Result::Err(
                                elicitation::ElicitError::new(
                                    elicitation::ElicitErrorKind::ParseError(
                                        #message.to_string(),
                                    ),
                                )
                                .into(),
                            );
                        }
                    }
                } else {
                    let invalid_message =
                        "elicit_reqwest::Proxy::custom carried invalid constructor code provenance";
                    TokenStream::from_str(code).unwrap_or_else(|_error| {
                        quote::quote! {
                            {
                                return ::std::result::Result::Err(
                                    elicitation::ElicitError::new(
                                        elicitation::ElicitErrorKind::ParseError(
                                            #invalid_message.to_string(),
                                        ),
                                    )
                                    .into(),
                                );
                            }
                        }
                    })
                }
            }
        }
    }

    /// Proxy all HTTP traffic to `url`.
    pub fn http(url: Url) -> Result<Self, Error> {
        let raw = reqwest::Proxy::http(url.clone()).map_err(Error::from)?;
        Ok(Self::from_ctor(raw, ProxyCtor::Http { url }))
    }

    /// Proxy all HTTPS traffic to `url`.
    pub fn https(url: Url) -> Result<Self, Error> {
        let raw = reqwest::Proxy::https(url.clone()).map_err(Error::from)?;
        Ok(Self::from_ctor(raw, ProxyCtor::Https { url }))
    }

    /// Proxy all traffic to `url`.
    pub fn all(url: Url) -> Result<Self, Error> {
        let raw = reqwest::Proxy::all(url.clone()).map_err(Error::from)?;
        Ok(Self::from_ctor(raw, ProxyCtor::All { url }))
    }

    /// Provide a custom function to determine what traffic to proxy to where.
    pub fn custom<F, U>(fun: F) -> Self
    where
        F: Fn(&reqwest::Url) -> Option<U> + Send + Sync + 'static,
        U: Into<reqwest::Url>,
    {
        let fun = move |url: &reqwest::Url| fun(url).map(Into::into);
        Self::from_ctor(
            reqwest::Proxy::custom(fun),
            ProxyCtor::Custom {
                code: String::new(),
            },
        )
    }

    /// Attach the original Rust constructor expression for a custom proxy.
    pub fn with_ctor_code(mut self, code: impl Into<String>) -> Self {
        if let ProxyCtor::Custom { code: ctor_code } = &mut self.snapshot.ctor {
            *ctor_code = code.into();
        }
        self
    }

    /// Rebuild a raw `reqwest::Proxy`.
    pub fn build_raw(&self) -> Result<reqwest::Proxy, Error> {
        if let Some(raw) = &self.raw {
            return Ok(raw.clone());
        }

        let mut proxy = match &self.snapshot.ctor {
            ProxyCtor::Http { url } => reqwest::Proxy::http(url.clone()).map_err(Error::from)?,
            ProxyCtor::Https { url } => reqwest::Proxy::https(url.clone()).map_err(Error::from)?,
            ProxyCtor::All { url } => reqwest::Proxy::all(url.clone()).map_err(Error::from)?,
            ProxyCtor::Custom { .. } => {
                return Err(Error::builder(
                    "custom proxy closures require a live runtime handle and cannot be rebuilt from snapshot alone",
                ));
            }
        };

        for step in &self.snapshot.steps {
            proxy = match step {
                ProxyStep::BasicAuth { username, password } => proxy.basic_auth(username, password),
                ProxyStep::CustomHttpAuth { header_value } => {
                    let header = http::HeaderValue::from_str(header_value).map_err(|error| {
                        Error::builder(format!(
                            "invalid proxy authorization header value `{header_value}`: {error}"
                        ))
                    })?;
                    proxy.custom_http_auth(header)
                }
                ProxyStep::Headers { headers } => {
                    proxy.headers(http::HeaderMap::from(headers.clone()))
                }
                ProxyStep::NoProxy { no_proxy } => {
                    proxy.no_proxy(no_proxy.as_ref().and_then(NoProxy::build_raw))
                }
            };
        }

        Ok(proxy)
    }
}

#[reflect_methods]
impl Proxy {
    /// Set the `Proxy-Authorization` header using Basic auth.
    pub fn basic_auth(self, username: String, password: String) -> Self {
        self.with_step(ProxyStep::BasicAuth { username, password })
    }

    /// Set the `Proxy-Authorization` header to a specified value.
    pub fn custom_http_auth(self, header_value: String) -> Self {
        self.with_step(ProxyStep::CustomHttpAuth { header_value })
    }

    /// Attach custom proxy headers.
    pub fn headers(self, headers: HeaderMap) -> Self {
        self.with_step(ProxyStep::Headers { headers })
    }

    /// Attach no-proxy exclusions.
    pub fn no_proxy(self, no_proxy: Option<NoProxy>) -> Self {
        self.with_step(ProxyStep::NoProxy { no_proxy })
    }

    /// Borrow the configured no-proxy list, if any.
    pub fn no_proxy_ref(&self) -> Option<&NoProxy> {
        self.snapshot.steps.iter().rev().find_map(|step| match step {
            ProxyStep::NoProxy { no_proxy } => no_proxy.as_ref(),
            _ => None,
        })
    }
}
