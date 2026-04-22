//! Trenchcoat wrapper for [`csv::Trim`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Whitespace trimming behaviour when reading CSV fields.
///
/// Wraps `csv::Trim` to add [`JsonSchema`] for MCP boundary crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CsvTrim {
    /// Trim whitespace from all fields and headers.
    All,
    /// Trim whitespace from field values only (not header names).
    Fields,
    /// Trim whitespace from header names only (not field values).
    Headers,
    /// Do not trim any whitespace (default).
    None,
}

#[cfg(feature = "csv-types")]
impl From<csv::Trim> for CsvTrim {
    fn from(t: csv::Trim) -> Self {
        match t {
            csv::Trim::All => CsvTrim::All,
            csv::Trim::Fields => CsvTrim::Fields,
            csv::Trim::Headers => CsvTrim::Headers,
            csv::Trim::None => CsvTrim::None,
            _ => CsvTrim::None,
        }
    }
}

#[cfg(feature = "csv-types")]
impl From<CsvTrim> for csv::Trim {
    fn from(t: CsvTrim) -> Self {
        match t {
            CsvTrim::All => csv::Trim::All,
            CsvTrim::Fields => csv::Trim::Fields,
            CsvTrim::Headers => csv::Trim::Headers,
            CsvTrim::None => csv::Trim::None,
        }
    }
}

impl Prompt for CsvTrim {
    fn prompt() -> Option<&'static str> {
        Some("Choose whitespace trimming behaviour for CSV reading:")
    }
}

impl Select for CsvTrim {
    fn options() -> Vec<Self> {
        vec![CsvTrim::All, CsvTrim::Fields, CsvTrim::Headers, CsvTrim::None]
    }

    fn labels() -> Vec<String> {
        vec![
            "All".to_string(),
            "Fields".to_string(),
            "Headers".to_string(),
            "None".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "All" => Some(CsvTrim::All),
            "Fields" => Some(CsvTrim::Fields),
            "Headers" => Some(CsvTrim::Headers),
            "None" => Some(CsvTrim::None),
            _ => None,
        }
    }
}

crate::default_style!(CsvTrim => CsvTrimStyle);

impl Elicitation for CsvTrim {
    type Style = CsvTrimStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CsvTrim");
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose trim mode:"), &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid CsvTrim: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("CsvTrim", "All")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("CsvTrim", "All")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("CsvTrim", "All")
    }
}

impl ElicitIntrospect for CsvTrim {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "csv::Trim",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for CsvTrim {
    fn prompt_tree() -> crate::PromptTree {
        let labels = Self::labels();
        let branch_count = labels.len();
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose trim mode:").to_string(),
            type_name: "CsvTrim".to_string(),
            options: labels,
            branches: vec![None; branch_count],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for CsvTrim {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            CsvTrim::All => quote::quote! { ::elicitation::CsvTrim::All },
            CsvTrim::Fields => quote::quote! { ::elicitation::CsvTrim::Fields },
            CsvTrim::Headers => quote::quote! { ::elicitation::CsvTrim::Headers },
            CsvTrim::None => quote::quote! { ::elicitation::CsvTrim::None },
        }
    }
}
