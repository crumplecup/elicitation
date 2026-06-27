//! Trenchcoat for `reqwest::retry::Builder`.

use std::borrow::Cow;

use crate::{
    ElicitCommunicator, ElicitComplete, ElicitIntrospect, ElicitPromptTree, ElicitResult,
    ElicitSpec, Elicitation, Prompt, PromptTree, SpecCategory, SpecEntry, TypeMetadata, TypeSpec,
    emit_code::ToCodeLiteral, type_spec::TypeSpecInventoryKey,
};
use proc_macro2::TokenStream;
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};

const DEFAULT_MAX_RETRIES: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Select a retry budget policy:")]
enum ReqwestRetryBudget {
    /// Use the default budget (20 % extra load).
    Default,
    /// Disable the budget entirely.
    NoBudget,
    /// Allow at most this percentage of extra load above normal traffic.
    MaxExtraLoad(f32),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Configure a reqwest retry policy:")]
enum ReqwestRetryBuilderRecipe {
    /// Scope retries to requests targeting the given host.
    ForHost {
        #[prompt("Hostname to scope retries to (e.g. \"api.example.com\"):")]
        host: String,
        #[prompt("Budget policy for retries:")]
        budget: ReqwestRetryBudget,
        #[prompt("Maximum number of retries per request (reqwest default: 2):")]
        max_retries_per_request: u32,
    },
    /// Never retry any request.
    Never,
}

/// Owned trenchcoat for `reqwest::retry::Builder`.
///
/// Covers the two public constructors (`for_host` and `never`) and the three
/// chainable modifiers (`no_budget`, `with_max_extra_load`, `with_max_retries`).
/// `build_raw` always succeeds because the recipe contains no closures.
///
/// `classify_fn` — the custom classifier — uses private reqwest types and
/// cannot be wrapped here. Apply it directly to the `reqwest::retry::Builder`
/// returned by `build_raw`.
#[derive(Debug, Clone)]
pub struct ReqwestRetryBuilder {
    recipe: ReqwestRetryBuilderRecipe,
}

impl Serialize for ReqwestRetryBuilder {
    #[tracing::instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.recipe.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ReqwestRetryBuilder {
    #[tracing::instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            recipe: ReqwestRetryBuilderRecipe::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for ReqwestRetryBuilder {
    #[tracing::instrument(level = "trace")]
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("ReqwestRetryBuilder")
    }

    #[tracing::instrument(skip(generator), level = "trace")]
    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ReqwestRetryBuilderRecipe as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for ReqwestRetryBuilder {
    fn prompt() -> Option<&'static str> {
        Some("Configure a reqwest retry policy:")
    }
}

impl Elicitation for ReqwestRetryBuilder {
    type Style = ();

    #[tracing::instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let recipe = ReqwestRetryBuilderRecipe::elicit(communicator).await?;
        Ok(Self { recipe })
    }

    fn kani_proof() -> TokenStream {
        ReqwestRetryBuilderRecipe::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        ReqwestRetryBuilderRecipe::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        ReqwestRetryBuilderRecipe::creusot_proof()
    }
}

impl ElicitIntrospect for ReqwestRetryBuilder {
    fn pattern() -> crate::ElicitationPattern {
        ReqwestRetryBuilderRecipe::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ReqwestRetryBuilder",
            description: Self::prompt(),
            details: ReqwestRetryBuilderRecipe::metadata().details,
        }
    }
}

impl ElicitPromptTree for ReqwestRetryBuilder {
    fn prompt_tree() -> PromptTree {
        match ReqwestRetryBuilderRecipe::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "elicitation::ReqwestRetryBuilder".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for ReqwestRetryBuilder {
    fn type_spec() -> TypeSpec {
        TypeSpec::new(
            "elicitation::ReqwestRetryBuilder",
            "Owned trenchcoat for `reqwest::retry::Builder`. Serializable recipe for \
             `for_host` and `never` constructors plus budget and retry-count modifiers. \
             `build_raw` always succeeds. Apply `classify_fn` directly to the returned \
             builder when a custom classifier is needed.",
            vec![
                SpecCategory::new(
                    "construction",
                    vec![
                        SpecEntry::new(
                            "for_host",
                            "Scope retries to requests targeting the given hostname.",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestRetryBuilder::for_host(host)".to_string(),
                        )),
                        SpecEntry::new("never", "Disable retries entirely.").with_expression(
                            Some("::elicitation::ReqwestRetryBuilder::never()".to_string()),
                        ),
                    ],
                ),
                SpecCategory::new(
                    "modifiers",
                    vec![
                        SpecEntry::new("no_budget", "Disable the retry budget.")
                            .with_expression(Some("builder.no_budget()".to_string())),
                        SpecEntry::new(
                            "with_max_extra_load",
                            "Allow at most `pct` percent extra load above normal traffic.",
                        )
                        .with_expression(Some("builder.with_max_extra_load(pct)".to_string())),
                        SpecEntry::new(
                            "with_max_retries",
                            "Set the maximum number of retries per request.",
                        )
                        .with_expression(Some("builder.with_max_retries(max)".to_string())),
                    ],
                ),
                SpecCategory::new(
                    "recovery",
                    vec![SpecEntry::new(
                        "build_raw",
                        "Build a live `reqwest::retry::Builder` from the recipe. Always succeeds.",
                    )
                    .with_expression(Some("retry_builder.build_raw()".to_string()))],
                ),
            ],
        )
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestRetryBuilder",
    <ReqwestRetryBuilder as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestRetryBuilder>
));

impl ToCodeLiteral for ReqwestRetryBuilder {
    #[tracing::instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        match &self.recipe {
            ReqwestRetryBuilderRecipe::Never => quote::quote! {
                ::elicitation::ReqwestRetryBuilder::never()
            },
            ReqwestRetryBuilderRecipe::ForHost { host, budget, max_retries_per_request } => {
                let mut tokens = quote::quote! {
                    ::elicitation::ReqwestRetryBuilder::for_host(#host)
                };
                match budget {
                    ReqwestRetryBudget::Default => {}
                    ReqwestRetryBudget::NoBudget => {
                        tokens = quote::quote! { #tokens.no_budget() };
                    }
                    ReqwestRetryBudget::MaxExtraLoad(pct) => {
                        tokens = quote::quote! { #tokens.with_max_extra_load(#pct) };
                    }
                }
                if *max_retries_per_request != DEFAULT_MAX_RETRIES {
                    let max = *max_retries_per_request;
                    tokens = quote::quote! { #tokens.with_max_retries(#max) };
                }
                tokens
            }
        }
    }
}

impl ElicitComplete for ReqwestRetryBuilder {}

impl ReqwestRetryBuilder {
    /// Scope retries to requests targeting the given hostname.
    #[tracing::instrument(skip(host), level = "debug")]
    pub fn for_host(host: impl Into<String>) -> Self {
        Self {
            recipe: ReqwestRetryBuilderRecipe::ForHost {
                host: host.into(),
                budget: ReqwestRetryBudget::Default,
                max_retries_per_request: DEFAULT_MAX_RETRIES,
            },
        }
    }

    /// Disable retries entirely.
    #[tracing::instrument(level = "debug")]
    pub fn never() -> Self {
        Self { recipe: ReqwestRetryBuilderRecipe::Never }
    }

    /// Disable the retry budget. No-op on [`never`](Self::never) builders.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn no_budget(mut self) -> Self {
        if let ReqwestRetryBuilderRecipe::ForHost { budget, .. } = &mut self.recipe {
            *budget = ReqwestRetryBudget::NoBudget;
        }
        self
    }

    /// Allow at most `pct` percent extra load above normal traffic.
    ///
    /// No-op on [`never`](Self::never) builders.
    #[tracing::instrument(level = "debug")]
    pub fn with_max_extra_load(mut self, pct: f32) -> Self {
        if let ReqwestRetryBuilderRecipe::ForHost { budget, .. } = &mut self.recipe {
            *budget = ReqwestRetryBudget::MaxExtraLoad(pct);
        }
        self
    }

    /// Set the maximum number of retries per request.
    ///
    /// No-op on [`never`](Self::never) builders.
    #[tracing::instrument(level = "debug")]
    pub fn with_max_retries(mut self, max: u32) -> Self {
        if let ReqwestRetryBuilderRecipe::ForHost { max_retries_per_request, .. } =
            &mut self.recipe
        {
            *max_retries_per_request = max;
        }
        self
    }

    /// Build a live `reqwest::retry::Builder` from the recipe.
    ///
    /// Always succeeds. Apply `classify_fn` directly to the returned builder
    /// when a custom classifier is needed — `classify_fn` uses private reqwest
    /// types and cannot be wrapped in this trenchcoat.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> reqwest::retry::Builder {
        match &self.recipe {
            ReqwestRetryBuilderRecipe::Never => reqwest::retry::never(),
            ReqwestRetryBuilderRecipe::ForHost { host, budget, max_retries_per_request } => {
                let b = reqwest::retry::for_host(host.clone());
                let b = match budget {
                    ReqwestRetryBudget::Default => b,
                    ReqwestRetryBudget::NoBudget => b.no_budget(),
                    ReqwestRetryBudget::MaxExtraLoad(pct) => b.max_extra_load(*pct),
                };
                b.max_retries_per_request(*max_retries_per_request)
            }
        }
    }
}
