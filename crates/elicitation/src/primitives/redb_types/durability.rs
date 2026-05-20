//! Trenchcoat wrapper for [`redb::Durability`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Write durability level for a redb write transaction.
///
/// Wraps `redb::Durability` to add [`JsonSchema`] for MCP boundary crossing.
///
/// Note: `redb::Durability` is `#[non_exhaustive]` with two public variants.
/// Future redb releases may add more; unknown variants map to [`Durability::Immediate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Durability {
    /// Commits will not be persisted to disk unless followed by an
    /// [`Immediate`](Durability::Immediate) commit. Fastest writes.
    None,
    /// Commits are guaranteed persistent as soon as
    /// [`WriteTransaction::commit`](https://docs.rs/redb/latest/redb/struct.WriteTransaction.html#method.commit)
    /// returns.
    Immediate,
}

#[cfg(feature = "redb-types")]
impl From<redb::Durability> for Durability {
    fn from(d: redb::Durability) -> Self {
        match d {
            redb::Durability::None => Durability::None,
            redb::Durability::Immediate => Durability::Immediate,
            _ => Durability::Immediate,
        }
    }
}

#[cfg(feature = "redb-types")]
impl From<Durability> for redb::Durability {
    fn from(d: Durability) -> Self {
        match d {
            Durability::None => redb::Durability::None,
            Durability::Immediate => redb::Durability::Immediate,
        }
    }
}

impl Prompt for Durability {
    fn prompt() -> Option<&'static str> {
        Some("Choose the write durability level for this redb transaction:")
    }
}

impl Select for Durability {
    fn options() -> Vec<Self> {
        vec![Durability::None, Durability::Immediate]
    }

    fn labels() -> Vec<String> {
        Self::options()
            .iter()
            .map(|v| {
                serde_json::to_string(v)
                    .unwrap()
                    .trim_matches('"')
                    .to_string()
            })
            .collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(Durability => DurabilityStyle);

impl Elicitation for Durability {
    type Style = DurabilityStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RedbDurability");
        let params = crate::mcp::select_params(
            Self::prompt().unwrap_or("Choose durability:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(
                    crate::mcp::tool_names::elicit_select(),
                )
                .with_arguments(params),
            )
            .await?;
        let value = crate::mcp::extract_value(result)?;
        let label = crate::mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            crate::ElicitError::new(crate::ElicitErrorKind::ParseError(format!(
                "Invalid RedbDurability: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum(
            "elicitation::RedbDurability",
            "none",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum("elicitation::RedbDurability")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum(
            "elicitation::RedbDurability",
        )
    }
}

impl ElicitIntrospect for Durability {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::RedbDurability",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for Durability {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: "RedbDurability".to_string(),
            type_name: "RedbDurability".to_string(),
            options: Self::labels(),
            branches: vec![None, None],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for Durability {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Durability::None => {
                quote::quote! { elicitation::RedbDurability::None }
            }
            Durability::Immediate => {
                quote::quote! { elicitation::RedbDurability::Immediate }
            }
        }
    }
}
