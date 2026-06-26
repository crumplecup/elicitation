//! Client wrapper for reqwest HTTP clients.
//!
//! The shadow stores reconstructable client configuration rather than a live
//! connection pool handle. Request creation records semantic request state so
//! downstream coverage reports can identify real support gaps instead of
//! placeholder shims.

use elicitation::{
    Elicit, ElicitCommunicator, ElicitIntrospect, ElicitPromptTree, ElicitResult, ElicitSpec,
    Elicitation, Prompt, PromptTree, TypeMetadata, TypeSpec, emit_code::ToCodeLiteral,
};
use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Method, RequestBuilder};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit)]
enum ClientRecipe {
    #[default]
    Default,
}

impl ClientRecipe {
    fn build(&self) -> reqwest::Client {
        match self {
            Self::Default => reqwest::Client::new(),
        }
    }
}

/// Elicitation-aware reqwest client shadow.
#[derive(Debug, Clone)]
pub struct Client {
    snapshot: ClientSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP client configuration:")]
struct ClientSnapshot {
    #[serde(default)]
    #[prompt("Client recipe:")]
    recipe: ClientRecipe,
}

impl Serialize for Client {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Client {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            snapshot: ClientSnapshot::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for Client {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("Client")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ClientSnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for Client {
    fn prompt() -> Option<&'static str> {
        Some("Describe an HTTP client configuration:")
    }
}

impl Elicitation for Client {
    type Style = ();

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = ClientSnapshot::elicit(communicator).await?;
        Ok(Self { snapshot })
    }

    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        ClientSnapshot::kani_proof()
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        ClientSnapshot::verus_proof()
    }

    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        ClientSnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for Client {
    fn pattern() -> elicitation::ElicitationPattern {
        ClientSnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "Client",
            description: Self::prompt(),
            details: ClientSnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for Client {
    fn prompt_tree() -> PromptTree {
        match ClientSnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "Client".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for Client {
    fn type_spec() -> TypeSpec {
        let base = ClientSnapshot::type_spec();
        TypeSpec::new(
            "Client",
            "Owned HTTP client configuration reconstructed from a serializable recipe.",
            base.categories().clone(),
        )
    }
}

impl elicitation::ElicitComplete for Client {}

impl Client {
    /// Creates a new HTTP client with default settings.
    pub fn new() -> Self {
        Self {
            snapshot: ClientSnapshot {
                recipe: ClientRecipe::Default,
            },
        }
    }

    pub(crate) fn build_raw(&self) -> reqwest::Client {
        self.snapshot.recipe.build()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl ToCodeLiteral for Client {
    fn to_code_literal(&self) -> elicitation::proc_macro2::TokenStream {
        quote::quote! { ::elicit_reqwest::Client::new() }
    }
}

#[reflect_methods]
impl Client {
    /// Start building a request to `url` with the given HTTP method.
    pub fn request<U>(&self, method: Method, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        RequestBuilder::from_input(self.clone(), method, url)
    }

    /// Start building a request to an already-parsed URL with the given HTTP method.
    pub fn request_url(&self, method: Method, url: crate::Url) -> RequestBuilder {
        RequestBuilder::from_url(self.clone(), method, url)
    }

    /// Start building a GET request to `url`.
    pub fn get<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.request(Method::from(reqwest::Method::GET), url)
    }

    /// Start building a POST request to `url`.
    pub fn post<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.request(Method::from(reqwest::Method::POST), url)
    }

    /// Start building a PUT request to `url`.
    pub fn put<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.request(Method::from(reqwest::Method::PUT), url)
    }

    /// Start building a DELETE request to `url`.
    pub fn delete<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.request(Method::from(reqwest::Method::DELETE), url)
    }

    /// Start building a PATCH request to `url`.
    pub fn patch<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.request(Method::from(reqwest::Method::PATCH), url)
    }

    /// Start building a HEAD request to `url`.
    pub fn head<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.request(Method::from(reqwest::Method::HEAD), url)
    }
}
