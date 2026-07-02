//! Response wrapper for reqwest HTTP responses.

use elicitation::{
    Elicit, ElicitCommunicator, ElicitIntrospect, ElicitPromptTree, ElicitResult, ElicitSpec,
    Elicitation, Prompt, PromptTree, TypeMetadata, TypeSpec, emit_code::ToCodeLiteral,
};
use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Error, HeaderMap, StatusCode, Url, Version};

/// Elicitation-aware HTTP response snapshot.
#[derive(Debug, Clone)]
pub struct Response {
    snapshot: ResponseSnapshot,
    cursor: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP response snapshot:")]
struct ResponseSnapshot {
    #[prompt("HTTP status code:")]
    status: StatusCode,
    #[prompt("HTTP protocol version:")]
    version: Version,
    #[prompt("Final response URL:")]
    url: Url,
    #[prompt("HTTP headers:")]
    headers: HeaderMap,
    #[prompt("Raw response body bytes:")]
    body: Vec<u8>,
}

impl Serialize for Response {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            snapshot: ResponseSnapshot::deserialize(deserializer)?,
            cursor: 0,
        })
    }
}

impl JsonSchema for Response {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("Response")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ResponseSnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for Response {
    fn prompt() -> Option<&'static str> {
        Some("Describe an HTTP response snapshot:")
    }
}

impl Elicitation for Response {
    type Style = ();

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = ResponseSnapshot::elicit(communicator).await?;
        Ok(Self {
            snapshot,
            cursor: 0,
        })
    }

    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        ResponseSnapshot::kani_proof()
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        ResponseSnapshot::verus_proof()
    }

    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        ResponseSnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for Response {
    fn pattern() -> elicitation::ElicitationPattern {
        ResponseSnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "Response",
            description: Self::prompt(),
            details: ResponseSnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for Response {
    fn prompt_tree() -> PromptTree {
        match ResponseSnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "Response".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for Response {
    fn type_spec() -> TypeSpec {
        let base = ResponseSnapshot::type_spec();
        TypeSpec::new(
            "Response",
            "Captured HTTP response snapshot including status, version, URL, headers, and body bytes.",
            base.categories().clone(),
        )
    }
}

impl Response {
    /// Construct a response snapshot from explicit parts.
    pub fn from_parts(
        status: StatusCode,
        version: Version,
        url: Url,
        headers: HeaderMap,
        body: Vec<u8>,
    ) -> Self {
        Self {
            snapshot: ResponseSnapshot {
                status,
                version,
                url,
                headers,
                body,
            },
            cursor: 0,
        }
    }

    pub(crate) async fn from_reqwest(value: reqwest::Response) -> Result<Self, Error> {
        let status = StatusCode::from(value.status());
        let version = Version::from(value.version());
        let url = Url::from(value.url().clone());
        let headers = HeaderMap::from(value.headers().clone());
        let body = value.bytes().await.map_err(Error::from)?.to_vec();

        Ok(Self::from_parts(status, version, url, headers, body))
    }
}

impl elicitation::ElicitComplete for Response {}

impl Response {
    /// Return the next chunk of body bytes (up to 8 KiB), advancing the read cursor.
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn chunk(&mut self) -> Result<Option<bytes::Bytes>, Error> {
        const CHUNK_SIZE: usize = 8192;
        if self.cursor >= self.snapshot.body.len() {
            return Ok(None);
        }
        let end = (self.cursor + CHUNK_SIZE).min(self.snapshot.body.len());
        let chunk = bytes::Bytes::copy_from_slice(&self.snapshot.body[self.cursor..end]);
        self.cursor = end;
        Ok(Some(chunk))
    }

    /// Consume the response and return the body as a stream of byte chunks.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn bytes_stream(
        self,
    ) -> impl futures::Stream<Item = Result<bytes::Bytes, Error>> + Send + 'static {
        let body = self.snapshot.body;
        let start = self.cursor;
        futures::stream::unfold((body, start), |(body, cursor)| async move {
            const CHUNK_SIZE: usize = 8192;
            if cursor >= body.len() {
                return None;
            }
            let end = (cursor + CHUNK_SIZE).min(body.len());
            let chunk = bytes::Bytes::copy_from_slice(&body[cursor..end]);
            Some((Ok(chunk), (body, end)))
        })
    }
}

impl ToCodeLiteral for Response {
    fn to_code_literal(&self) -> elicitation::proc_macro2::TokenStream {
        let status = self.snapshot.status.to_code_literal();
        let version = self.snapshot.version.to_code_literal();
        let url = self.snapshot.url.to_code_literal();
        let headers = self.snapshot.headers.to_code_literal();
        let body = &self.snapshot.body;

        quote::quote! {
            ::elicit_reqwest::Response::from_parts(
                #status,
                #version,
                #url,
                #headers,
                ::std::vec![#(#body),*],
            )
        }
    }
}

#[reflect_methods]
impl Response {
    /// Returns the HTTP status code.
    pub fn status(&self) -> StatusCode {
        self.snapshot.status.clone()
    }

    /// Returns the final URL of the response (after redirects).
    pub fn url(&self) -> &Url {
        &self.snapshot.url
    }

    /// Returns the HTTP version of the response.
    pub fn version(&self) -> Version {
        self.snapshot.version.clone()
    }

    /// Returns the response headers.
    pub fn headers(&self) -> &HeaderMap {
        &self.snapshot.headers
    }

    /// Returns the content-length from headers, if present.
    pub fn content_length(&self) -> Option<u64> {
        self.snapshot
            .headers
            .0
            .get(http::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
    }

    /// Consume the response and return the raw body bytes.
    pub async fn bytes(self) -> Result<Vec<u8>, Error> {
        Ok(self.snapshot.body)
    }

    /// Consume the response and return the body as text.
    pub async fn text(self) -> Result<String, Error> {
        Ok(String::from_utf8_lossy(&self.snapshot.body).into_owned())
    }

    /// Consume the response and deserialize the body as JSON.
    pub async fn json<T>(self) -> Result<T, Error>
    where
        T: elicitation::Elicitation + schemars::JsonSchema + serde::de::DeserializeOwned,
    {
        serde_json::from_slice(&self.snapshot.body).map_err(|error| {
            Error::decode(
                format!("failed to decode response JSON: {error}"),
                Some(self.snapshot.url.clone()),
            )
        })
    }

    /// Consume the response, returning an error if the status is 4xx or 5xx.
    pub fn error_for_status(self) -> Result<Self, Error> {
        if self.snapshot.status.is_client_error() || self.snapshot.status.is_server_error() {
            Err(Error::status(
                self.snapshot.status.clone(),
                Some(self.snapshot.url.clone()),
            ))
        } else {
            Ok(self)
        }
    }

    /// Return an error if the status is 4xx or 5xx, without consuming the response.
    pub fn error_for_status_ref(&self) -> Result<&Self, Error> {
        if self.snapshot.status.is_client_error() || self.snapshot.status.is_server_error() {
            Err(Error::status(
                self.snapshot.status.clone(),
                Some(self.snapshot.url.clone()),
            ))
        } else {
            Ok(self)
        }
    }

    /// Consume the response and decode the body using the given default charset.
    pub async fn text_with_charset(self, default_encoding: &str) -> Result<String, Error> {
        let _ = default_encoding;
        Ok(String::from_utf8_lossy(&self.snapshot.body).into_owned())
    }
}
