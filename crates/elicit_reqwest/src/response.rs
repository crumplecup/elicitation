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
        Ok(Self { snapshot })
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
        }
    }

    pub(crate) async fn from_reqwest(value: reqwest::Response) -> Result<Self, Error> {
        let status = StatusCode::from(value.status());
        let version = Version::from(value.version());
        let url = value.url().clone();
        let headers = HeaderMap::from(value.headers().clone());
        let body = value.bytes().await.map_err(Error::from)?.to_vec();

        Ok(Self::from_parts(status, version, url, headers, body))
    }
}

impl elicitation::ElicitComplete for Response {}

impl ToCodeLiteral for Response {
    fn to_code_literal(&self) -> elicitation::proc_macro2::TokenStream {
        let status = self.snapshot.status.to_code_literal();
        let version = self.snapshot.version.to_code_literal();
        let url = {
            let url = self.snapshot.url.as_str();
            quote::quote! {
                match ::elicit_reqwest::Url::parse(#url) {
                    ::std::result::Result::Ok(url) => url,
                    ::std::result::Result::Err(error) => {
                        return ::std::result::Result::Err(error.into());
                    }
                }
            }
        };
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
    /// Returns the HTTP status code as a `u16`.
    pub fn status(&self) -> u16 {
        self.snapshot.status.0.as_u16()
    }

    /// Returns the final URL of the response (after redirects).
    pub fn url(&self) -> String {
        self.snapshot.url.to_string()
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
}
