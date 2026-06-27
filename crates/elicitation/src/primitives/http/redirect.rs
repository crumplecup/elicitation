//! Trenchcoats for `reqwest::redirect::{Policy, Attempt}` and the associated
//! `ReqwestRedirectAction` type used by custom policy handlers.

use std::{borrow::Cow, str::FromStr, sync::Arc};

use crate::{
    ElicitCommunicator, ElicitComplete, ElicitIntrospect, ElicitPromptTree, ElicitResult,
    ElicitSpec, Elicitation, Prompt, PromptTree, SpecCategory, SpecEntry, TypeMetadata, TypeSpec,
    emit_code::ToCodeLiteral, type_spec::TypeSpecInventoryKey,
};
use proc_macro2::TokenStream;
use schemars::{JsonSchema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use url::Url;

// ── ReqwestRedirectAction ─────────────────────────────────────────────────────

/// Owned shadow for the decision returned by a custom redirect policy handler.
///
/// Used as the return type of the handler closure stored in
/// [`ReqwestRedirectPolicy`] custom closures. The closure receives a
/// [`ReqwestRedirectAttempt`] and returns one of these variants, which is then
/// converted back to the raw `reqwest::redirect::Action` inside the closure
/// adapter.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Select a redirect action:")]
pub enum ReqwestRedirectAction {
    /// Follow the redirect to the next URL.
    Follow,
    /// Stop following redirects and return the current response.
    Stop,
    /// Fail the redirect chain with the given error message.
    Error {
        /// Human-readable error message attached to the failed redirect.
        #[prompt("Error message for the failed redirect:")]
        message: String,
    },
}

// Derive generates ToCodeLiteral and ElicitComplete for ReqwestRedirectAction.
inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestRedirectAction",
    <ReqwestRedirectAction as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestRedirectAction>
));

// ── ReqwestRedirectPolicy ─────────────────────────────────────────────────────

/// Serializable recipe captured from a `reqwest::redirect::Policy`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Select a redirect policy:")]
enum ReqwestRedirectPolicyRecipe {
    /// Follow at most `max` redirects.
    Limited {
        #[prompt("Maximum number of redirects to follow:")]
        max: usize,
    },
    /// Never follow any redirects.
    None,
    /// A custom policy. The closure is not serializable; `description` records
    /// its intent and `code` optionally stores the original Rust source so
    /// `to_code_literal` can recover it.
    Custom {
        #[prompt("Human-readable description of the custom redirect policy logic:")]
        description: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        #[prompt(
            "Original Rust closure expression used to build this custom policy, when available:"
        )]
        code: String,
    },
}

type PolicyFn =
    Arc<dyn Fn(ReqwestRedirectAttempt) -> ReqwestRedirectAction + Send + Sync + 'static>;

/// Owned trenchcoat for `reqwest::redirect::Policy`.
///
/// `Policy::custom` takes an opaque closure that cannot be cloned or
/// serialized. This trenchcoat caches the live handler as an `Arc<dyn Fn>`
/// alongside a serializable recipe, so `build_raw` can reconstruct a working
/// policy even for the `Custom` variant.
pub struct ReqwestRedirectPolicy {
    recipe: ReqwestRedirectPolicyRecipe,
    /// Cached handler for `Custom` variants. `None` after deserialization.
    handler: Option<PolicyFn>,
}

impl std::fmt::Debug for ReqwestRedirectPolicy {
    #[tracing::instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReqwestRedirectPolicy")
            .field("recipe", &self.recipe)
            .finish_non_exhaustive()
    }
}

impl Clone for ReqwestRedirectPolicy {
    fn clone(&self) -> Self {
        Self {
            recipe: self.recipe.clone(),
            handler: self.handler.clone(),
        }
    }
}

impl Default for ReqwestRedirectPolicy {
    fn default() -> Self {
        Self::limited(10)
    }
}

impl Serialize for ReqwestRedirectPolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.recipe.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ReqwestRedirectPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            recipe: ReqwestRedirectPolicyRecipe::deserialize(deserializer)?,
            handler: None,
        })
    }
}

impl JsonSchema for ReqwestRedirectPolicy {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("ReqwestRedirectPolicy")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ReqwestRedirectPolicyRecipe as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for ReqwestRedirectPolicy {
    fn prompt() -> Option<&'static str> {
        Some("Select a redirect policy:")
    }
}

impl Elicitation for ReqwestRedirectPolicy {
    type Style = ();

    #[tracing::instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let recipe = ReqwestRedirectPolicyRecipe::elicit(communicator).await?;
        Ok(Self {
            recipe,
            handler: None,
        })
    }

    fn kani_proof() -> TokenStream {
        ReqwestRedirectPolicyRecipe::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        ReqwestRedirectPolicyRecipe::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        ReqwestRedirectPolicyRecipe::creusot_proof()
    }
}

impl ElicitIntrospect for ReqwestRedirectPolicy {
    fn pattern() -> crate::ElicitationPattern {
        ReqwestRedirectPolicyRecipe::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ReqwestRedirectPolicy",
            description: Self::prompt(),
            details: ReqwestRedirectPolicyRecipe::metadata().details,
        }
    }
}

impl ElicitPromptTree for ReqwestRedirectPolicy {
    fn prompt_tree() -> PromptTree {
        match ReqwestRedirectPolicyRecipe::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "elicitation::ReqwestRedirectPolicy".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for ReqwestRedirectPolicy {
    fn type_spec() -> TypeSpec {
        TypeSpec::new(
            "elicitation::ReqwestRedirectPolicy",
            "Trenchcoat for `reqwest::redirect::Policy`. Caches the live handler for \
             `Custom` variants so `build_raw` can reconstruct a working policy. \
             The optional `code` field enables code recovery via `to_code_literal`.",
            vec![
                SpecCategory::new(
                    "construction",
                    vec![
                        SpecEntry::new(
                            "limited",
                            "Follow at most `max` redirects (reqwest default is 10).",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestRedirectPolicy::limited(max)".to_string(),
                        )),
                        SpecEntry::new("none", "Disable redirect following entirely.")
                            .with_expression(Some(
                                "::elicitation::ReqwestRedirectPolicy::none()".to_string(),
                            )),
                        SpecEntry::new(
                            "custom",
                            "Record a custom handler with a prose description. \
                             Supply the closure source via `with_code` for full code recovery.",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestRedirectPolicy::custom(description, handler)"
                                .to_string(),
                        )),
                    ],
                ),
                SpecCategory::new(
                    "recovery",
                    vec![
                        SpecEntry::new(
                            "build_raw",
                            "Rebuild a raw `reqwest::redirect::Policy`. \
                         `Custom` variants require the cached live handler.",
                        )
                        .with_expression(Some("policy.build_raw()".to_string())),
                    ],
                ),
            ],
        )
    }
}

impl ToCodeLiteral for ReqwestRedirectPolicy {
    #[tracing::instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        match &self.recipe {
            ReqwestRedirectPolicyRecipe::Limited { max } => quote::quote! {
                ::elicitation::ReqwestRedirectPolicy::limited(#max)
            },
            ReqwestRedirectPolicyRecipe::None => quote::quote! {
                ::elicitation::ReqwestRedirectPolicy::none()
            },
            ReqwestRedirectPolicyRecipe::Custom { description, code } => {
                let handler_tokens = TokenStream::from_str(code).unwrap_or_default();
                quote::quote! {
                    ::elicitation::ReqwestRedirectPolicy::custom(
                        ::std::string::String::from(#description),
                        #handler_tokens,
                    )
                }
            }
        }
    }
}

impl ElicitComplete for ReqwestRedirectPolicy {}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestRedirectPolicy",
    <ReqwestRedirectPolicy as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestRedirectPolicy>
));

impl ReqwestRedirectPolicy {
    /// Shadow of `reqwest::redirect::Policy::limited`.
    #[tracing::instrument(level = "debug")]
    pub fn limited(max: usize) -> Self {
        Self {
            recipe: ReqwestRedirectPolicyRecipe::Limited { max },
            handler: None,
        }
    }

    /// Shadow of `reqwest::redirect::Policy::none`.
    #[tracing::instrument(level = "debug")]
    pub fn none() -> Self {
        Self {
            recipe: ReqwestRedirectPolicyRecipe::None,
            handler: None,
        }
    }

    /// Construct a custom redirect policy from a handler closure and a prose
    /// description. Supply the closure source via [`with_code`] for full code
    /// recovery through `to_code_literal`.
    ///
    /// [`with_code`]: ReqwestRedirectPolicy::with_code
    #[tracing::instrument(skip(description, handler), level = "debug")]
    pub fn custom<F>(description: impl Into<String>, handler: F) -> Self
    where
        F: Fn(ReqwestRedirectAttempt) -> ReqwestRedirectAction + Send + Sync + 'static,
    {
        Self {
            recipe: ReqwestRedirectPolicyRecipe::Custom {
                description: description.into(),
                code: String::new(),
            },
            handler: Some(Arc::new(handler)),
        }
    }

    /// Attach the original Rust closure source to a `Custom` policy for code
    /// recovery. No-op on `Limited` or `None` variants.
    #[tracing::instrument(skip(self, code), level = "debug")]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        if let ReqwestRedirectPolicyRecipe::Custom { code: slot, .. } = &mut self.recipe {
            *slot = code.into();
        }
        self
    }

    /// Rebuild a raw `reqwest::redirect::Policy`.
    ///
    /// `Custom` variants require the live handler cached at construction time.
    /// After deserialization the handler is absent and this returns an error.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> ElicitResult<reqwest::redirect::Policy> {
        match &self.recipe {
            ReqwestRedirectPolicyRecipe::Limited { max } => {
                Ok(reqwest::redirect::Policy::limited(*max))
            }
            ReqwestRedirectPolicyRecipe::None => Ok(reqwest::redirect::Policy::none()),
            ReqwestRedirectPolicyRecipe::Custom { description, .. } => {
                let handler = self.handler.clone().ok_or_else(|| {
                    crate::ElicitError::new(crate::ElicitErrorKind::ParseError(format!(
                        "custom redirect policy handler is absent after deserialization; \
                         policy described as: {description}"
                    )))
                })?;
                Ok(reqwest::redirect::Policy::custom(
                    move |attempt: reqwest::redirect::Attempt| {
                        let snapshot = ReqwestRedirectAttempt::from(&attempt);
                        match handler(snapshot) {
                            ReqwestRedirectAction::Follow => attempt.follow(),
                            ReqwestRedirectAction::Stop => attempt.stop(),
                            ReqwestRedirectAction::Error { message } => attempt.error(message),
                        }
                    },
                ))
            }
        }
    }
}

// ── ReqwestRedirectAttempt helpers ────────────────────────────────────────────

fn serialize_status_code<S>(code: &reqwest::StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_u16(code.as_u16())
}

fn deserialize_status_code<'de, D>(d: D) -> Result<reqwest::StatusCode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let n = <u16 as serde::Deserialize>::deserialize(d)?;
    reqwest::StatusCode::from_u16(n).map_err(serde::de::Error::custom)
}

fn status_code_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
    json_schema!({
        "type": "integer",
        "minimum": 100,
        "maximum": 999,
        "description": "HTTP status code (100–999)"
    })
}

// ── ReqwestRedirectAttempt ────────────────────────────────────────────────────

/// Owned shadow of `reqwest::redirect::Attempt<'_>`.
///
/// Captures all observable state from the lifetime-bound `Attempt` reference
/// into an owned, fully serializable form. Used as the argument type for
/// handlers stored in [`ReqwestRedirectPolicy`] custom closures.
///
/// Stores `status` as `reqwest::StatusCode` using per-field serde helpers that
/// serialize to/from the raw integer, avoiding an orphan `impl Serialize for
/// reqwest::StatusCode` while keeping the public API fully typed.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a redirect attempt snapshot:")]
#[to_code_literal(path = "::elicitation::ReqwestRedirectAttempt::new", tuple)]
pub struct ReqwestRedirectAttempt {
    #[prompt("The URL being redirected to:")]
    url: Url,
    #[prompt("Previously visited URLs in this redirect chain:")]
    previous: Vec<Url>,
    #[serde(
        serialize_with = "serialize_status_code",
        deserialize_with = "deserialize_status_code"
    )]
    #[schemars(schema_with = "status_code_schema")]
    #[prompt("HTTP status code that triggered this redirect (e.g. 301, 302):")]
    status: reqwest::StatusCode,
}

impl ReqwestRedirectAttempt {
    /// Construct a redirect attempt snapshot from its parts.
    #[tracing::instrument(skip(url, previous, status), level = "debug")]
    pub fn new(url: Url, previous: Vec<Url>, status: reqwest::StatusCode) -> Self {
        Self {
            url,
            previous,
            status,
        }
    }

    /// Borrow the redirect target URL.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Borrow the previously visited URLs in this chain.
    pub fn previous(&self) -> &[Url] {
        &self.previous
    }

    /// Return the HTTP status code that triggered this redirect.
    pub fn status(&self) -> reqwest::StatusCode {
        self.status
    }
}

impl<'a> From<&reqwest::redirect::Attempt<'a>> for ReqwestRedirectAttempt {
    #[tracing::instrument(skip(attempt), level = "debug")]
    fn from(attempt: &reqwest::redirect::Attempt<'a>) -> Self {
        Self::new(
            attempt.url().clone(),
            attempt.previous().to_vec(),
            attempt.status(),
        )
    }
}

// Derive generates ToCodeLiteral (reqwest::StatusCode implements ToCodeLiteral,
// so the tuple path works directly) and ElicitComplete.
inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestRedirectAttempt",
    <ReqwestRedirectAttempt as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestRedirectAttempt>
));
