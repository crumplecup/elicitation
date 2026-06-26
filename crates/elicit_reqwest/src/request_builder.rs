//! Structured request builder shadow for reqwest requests.

use std::str::FromStr;

use elicitation::{
    Elicit, ElicitCommunicator, ElicitIntrospect, ElicitPromptTree, ElicitResult, ElicitSpec,
    Elicitation, Prompt, PromptTree, TypeMetadata, TypeSpec, emit_code::ToCodeLiteral,
    proc_macro2::TokenStream,
};
use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Client, Error, Method, Response, Url};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a request timeout:")]
struct TimeoutSpec {
    #[prompt("Whole seconds component:")]
    secs: u64,
    #[prompt("Nanoseconds component:")]
    nanos: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Original Rust code used to build this timeout, when available:")]
    code: Option<String>,
}

impl TimeoutSpec {
    fn from_duration(timeout: std::time::Duration) -> Self {
        Self {
            secs: timeout.as_secs(),
            nanos: timeout.subsec_nanos(),
            code: Some(timeout.to_code_literal().to_string()),
        }
    }

    fn to_duration(&self) -> std::time::Duration {
        std::time::Duration::new(self.secs, self.nanos)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a request body snapshot:")]
enum BodySpec {
    Json {
        #[prompt("Serialized JSON payload bytes:")]
        bytes: Vec<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[prompt("Original Rust expression used to build the JSON payload, when available:")]
        code: Option<String>,
    },
    Bytes {
        #[prompt("Raw request body bytes:")]
        bytes: Vec<u8>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a request builder step:")]
enum RequestStep {
    Timeout {
        #[prompt("Timeout configuration:")]
        timeout: TimeoutSpec,
    },
    BearerAuth {
        #[prompt("Bearer token:")]
        token: String,
    },
    BasicAuth {
        #[prompt("Basic auth username:")]
        username: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[prompt("Optional basic auth password:")]
        password: Option<String>,
    },
    Header {
        #[prompt("Header name:")]
        key: String,
        #[prompt("Header value:")]
        value: String,
    },
    Body {
        #[prompt("Body configuration:")]
        body: BodySpec,
    },
}

/// Elicitation-aware reqwest request builder shadow.
#[derive(Debug, Clone)]
pub struct RequestBuilder {
    snapshot: RequestBuilderSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP request under construction:")]
struct RequestBuilderSnapshot {
    #[prompt("Client configuration used to build the request:")]
    client: Client,
    #[prompt("HTTP method:")]
    method: Method,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Parsed request URL, when the URL was valid:")]
    url: Option<Url>,
    #[prompt("Original Rust expression used for the request URL:")]
    url_code: String,
    #[serde(default)]
    #[prompt("Ordered request builder steps:")]
    steps: Vec<RequestStep>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Builder error captured before send, when present:")]
    build_error: Option<Error>,
}

impl Serialize for RequestBuilder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RequestBuilder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            snapshot: RequestBuilderSnapshot::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for RequestBuilder {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("RequestBuilder")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <RequestBuilderSnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for RequestBuilder {
    fn prompt() -> Option<&'static str> {
        Some("Describe an HTTP request under construction:")
    }
}

impl Elicitation for RequestBuilder {
    type Style = ();

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = RequestBuilderSnapshot::elicit(communicator).await?;
        Ok(Self { snapshot })
    }

    fn kani_proof() -> TokenStream {
        RequestBuilderSnapshot::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        RequestBuilderSnapshot::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        RequestBuilderSnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for RequestBuilder {
    fn pattern() -> elicitation::ElicitationPattern {
        RequestBuilderSnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "RequestBuilder",
            description: Self::prompt(),
            details: RequestBuilderSnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for RequestBuilder {
    fn prompt_tree() -> PromptTree {
        match RequestBuilderSnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "RequestBuilder".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for RequestBuilder {
    fn type_spec() -> TypeSpec {
        let base = RequestBuilderSnapshot::type_spec();
        TypeSpec::new(
            "RequestBuilder",
            "Ordered HTTP request builder snapshot with captured URL provenance, mutations, and pending builder errors.",
            base.categories().clone(),
        )
    }
}

impl RequestBuilder {
    pub(crate) fn from_input<U>(client: Client, method: Method, url: U) -> Self
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        let url_code = url.to_code_literal().to_string();
        match url.into_url() {
            Ok(parsed) => Self::from_url_with_code(client, method, parsed, url_code),
            Err(error) => Self {
                snapshot: RequestBuilderSnapshot {
                    client,
                    method,
                    url: None,
                    url_code,
                    steps: Vec::new(),
                    build_error: Some(Error::from(error)),
                },
            },
        }
    }

    pub(crate) fn from_url(client: Client, method: Method, url: Url) -> Self {
        let url_code = url.to_code_literal().to_string();

        Self::from_url_with_code(client, method, url, url_code)
    }

    fn from_url_with_code(client: Client, method: Method, url: Url, url_code: String) -> Self {
        Self {
            snapshot: RequestBuilderSnapshot {
                client,
                method,
                url: Some(url),
                url_code,
                steps: Vec::new(),
                build_error: None,
            },
        }
    }

    fn with_step(mut self, step: RequestStep) -> Self {
        self.snapshot.steps.push(step);
        self
    }

    fn rebuild_raw(&self) -> Result<reqwest::RequestBuilder, Error> {
        if let Some(error) = &self.snapshot.build_error {
            return Err(error.clone());
        }

        let url = self.snapshot.url.clone().ok_or_else(|| {
            Error::builder("request builder is missing a parsed URL and cannot be sent")
        })?;

        let mut builder = self
            .snapshot
            .client
            .build_raw()
            .request((*self.snapshot.method).clone(), url);

        for step in &self.snapshot.steps {
            builder = match step {
                RequestStep::Timeout { timeout } => builder.timeout(timeout.to_duration()),
                RequestStep::BearerAuth { token } => builder.bearer_auth(token),
                RequestStep::BasicAuth { username, password } => {
                    builder.basic_auth(username, password.clone())
                }
                RequestStep::Header { key, value } => builder.header(key, value),
                RequestStep::Body { body } => match body {
                    BodySpec::Json { bytes, .. } => builder
                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                        .body(bytes.clone()),
                    BodySpec::Bytes { bytes } => builder.body(bytes.clone()),
                },
            };
        }

        Ok(builder)
    }

    fn url_tokens(&self) -> TokenStream {
        TokenStream::from_str(&self.snapshot.url_code).unwrap_or_else(|_| {
            let fallback = self
                .snapshot
                .url
                .as_ref()
                .map(|url| url.as_str())
                .unwrap_or_default()
                .to_string();
            quote::quote! {
                match ::elicit_reqwest::Url::parse(#fallback) {
                    ::std::result::Result::Ok(url) => url,
                    ::std::result::Result::Err(error) => {
                        return ::std::result::Result::Err(error.into());
                    }
                }
            }
        })
    }
}

impl elicitation::ElicitComplete for RequestBuilder {}

impl ToCodeLiteral for RequestBuilder {
    fn to_code_literal(&self) -> TokenStream {
        let client = self.snapshot.client.to_code_literal();
        let method = self.snapshot.method.to_code_literal();
        let url_tokens = self.url_tokens();

        let mut tokens = match self.snapshot.method.as_str() {
            "GET" => quote::quote! { (#client).get(#url_tokens) },
            "POST" => quote::quote! { (#client).post(#url_tokens) },
            "PUT" => quote::quote! { (#client).put(#url_tokens) },
            "DELETE" => quote::quote! { (#client).delete(#url_tokens) },
            "PATCH" => quote::quote! { (#client).patch(#url_tokens) },
            "HEAD" => quote::quote! { (#client).head(#url_tokens) },
            _ => quote::quote! { (#client).request(#method, #url_tokens) },
        };

        for step in &self.snapshot.steps {
            tokens = match step {
                RequestStep::Timeout { timeout } => {
                    let timeout_tokens = timeout
                        .code
                        .as_deref()
                        .and_then(|code| TokenStream::from_str(code).ok())
                        .unwrap_or_else(|| {
                            let secs = timeout.secs;
                            let nanos = timeout.nanos;
                            quote::quote! { ::std::time::Duration::new(#secs, #nanos) }
                        });
                    quote::quote! { (#tokens).timeout(#timeout_tokens) }
                }
                RequestStep::BearerAuth { token } => {
                    quote::quote! { (#tokens).bearer_auth(#token.to_string()) }
                }
                RequestStep::BasicAuth { username, password } => {
                    let password_tokens = match password {
                        Some(value) => quote::quote! { ::std::option::Option::Some(#value.to_string()) },
                        None => quote::quote! { ::std::option::Option::None },
                    };
                    quote::quote! {
                        (#tokens).basic_auth(#username.to_string(), #password_tokens)
                    }
                }
                RequestStep::Header { key, value } => {
                    quote::quote! { (#tokens).header(#key.to_string(), #value.to_string()) }
                }
                RequestStep::Body { body } => match body {
                    BodySpec::Json { bytes, code } => {
                        if let Some(payload_tokens) = code
                            .as_deref()
                            .and_then(|code| TokenStream::from_str(code).ok())
                        {
                            quote::quote! { (#tokens).json(&(#payload_tokens)) }
                        } else {
                            quote::quote! { (#tokens).json_bytes(::std::vec![#(#bytes),*]) }
                        }
                    }
                    BodySpec::Bytes { bytes } => {
                        quote::quote! { (#tokens).body_bytes(::std::vec![#(#bytes),*]) }
                    }
                },
            };
        }

        tokens
    }
}

#[reflect_methods]
impl RequestBuilder {
    /// Set request timeout.
    pub fn timeout(self, timeout: std::time::Duration) -> Self {
        self.with_step(RequestStep::Timeout {
            timeout: TimeoutSpec::from_duration(timeout),
        })
    }

    /// Set Bearer token authorization header.
    pub fn bearer_auth(self, token: String) -> Self {
        self.with_step(RequestStep::BearerAuth { token })
    }

    /// Set Basic authorization credentials.
    pub fn basic_auth(self, username: String, password: Option<String>) -> Self {
        self.with_step(RequestStep::BasicAuth { username, password })
    }

    /// Append a header to the request.
    pub fn header(self, key: String, value: String) -> Self {
        self.with_step(RequestStep::Header { key, value })
    }

    /// Set the JSON body (serializes `value` as JSON).
    pub fn json<T>(self, value: &T) -> Self
    where
        T: elicitation::ElicitComplete + serde::Serialize,
    {
        match serde_json::to_vec(value) {
            Ok(bytes) => self.with_step(RequestStep::Body {
                body: BodySpec::Json {
                    bytes,
                    code: Some(value.to_code_literal().to_string()),
                },
            }),
            Err(error) => {
                let mut builder = self;
                builder.snapshot.build_error = Some(Error::builder(format!(
                    "failed to serialize request JSON body: {error}"
                )));
                builder
            }
        }
    }

    /// Set the request body directly from raw bytes.
    pub fn body_bytes(self, body: Vec<u8>) -> Self {
        self.with_step(RequestStep::Body {
            body: BodySpec::Bytes { bytes: body },
        })
    }

    /// Set the request body directly from already-serialized JSON bytes.
    pub fn json_bytes(self, body: Vec<u8>) -> Self {
        self.with_step(RequestStep::Body {
            body: BodySpec::Json {
                bytes: body,
                code: None,
            },
        })
    }

    /// Send the request and await the response.
    pub async fn send(self) -> Result<Response, Error> {
        let builder = self.rebuild_raw()?;
        let response = builder.send().await.map_err(Error::from)?;
        Response::from_reqwest(response).await
    }
}
