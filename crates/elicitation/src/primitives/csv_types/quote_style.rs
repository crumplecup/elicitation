//! Trenchcoat wrapper for [`csv::QuoteStyle`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Quoting style used when writing CSV fields.
///
/// Wraps `csv::QuoteStyle` to add [`JsonSchema`] for MCP boundary crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CsvQuoteStyle {
    /// Always quote every field, regardless of content.
    Always,
    /// Quote fields only when necessary (contains delimiter, quote char, or record terminator).
    Necessary,
    /// Quote fields that are non-numeric (not an integer or float).
    NonNumeric,
    /// Never quote fields. Writer errors if quoting would be required.
    Never,
}

#[cfg(feature = "csv-types")]
impl From<csv::QuoteStyle> for CsvQuoteStyle {
    fn from(s: csv::QuoteStyle) -> Self {
        match s {
            csv::QuoteStyle::Always => CsvQuoteStyle::Always,
            csv::QuoteStyle::Necessary => CsvQuoteStyle::Necessary,
            csv::QuoteStyle::NonNumeric => CsvQuoteStyle::NonNumeric,
            csv::QuoteStyle::Never => CsvQuoteStyle::Never,
            _ => CsvQuoteStyle::Necessary,
        }
    }
}

#[cfg(feature = "csv-types")]
impl From<CsvQuoteStyle> for csv::QuoteStyle {
    fn from(s: CsvQuoteStyle) -> Self {
        match s {
            CsvQuoteStyle::Always => csv::QuoteStyle::Always,
            CsvQuoteStyle::Necessary => csv::QuoteStyle::Necessary,
            CsvQuoteStyle::NonNumeric => csv::QuoteStyle::NonNumeric,
            CsvQuoteStyle::Never => csv::QuoteStyle::Never,
        }
    }
}

impl Prompt for CsvQuoteStyle {
    fn prompt() -> Option<&'static str> {
        Some("Choose the CSV quoting style:")
    }
}

impl Select for CsvQuoteStyle {
    fn options() -> Vec<Self> {
        vec![
            CsvQuoteStyle::Always,
            CsvQuoteStyle::Necessary,
            CsvQuoteStyle::NonNumeric,
            CsvQuoteStyle::Never,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Always".to_string(),
            "Necessary".to_string(),
            "NonNumeric".to_string(),
            "Never".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Always" => Some(CsvQuoteStyle::Always),
            "Necessary" => Some(CsvQuoteStyle::Necessary),
            "NonNumeric" => Some(CsvQuoteStyle::NonNumeric),
            "Never" => Some(CsvQuoteStyle::Never),
            _ => None,
        }
    }
}

crate::default_style!(CsvQuoteStyle => CsvQuoteStyleStyle);

impl Elicitation for CsvQuoteStyle {
    type Style = CsvQuoteStyleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CsvQuoteStyle");
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose quote style:"), &Self::labels());
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
                "Invalid CsvQuoteStyle: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("CsvQuoteStyle", "Always")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("CsvQuoteStyle", "Always")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("CsvQuoteStyle", "Always")
    }
}

impl ElicitIntrospect for CsvQuoteStyle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "csv::QuoteStyle",
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

impl crate::ElicitPromptTree for CsvQuoteStyle {
    fn prompt_tree() -> crate::PromptTree {
        let labels = Self::labels();
        let branch_count = labels.len();
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose quote style:").to_string(),
            type_name: "CsvQuoteStyle".to_string(),
            options: labels,
            branches: vec![None; branch_count],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for CsvQuoteStyle {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            CsvQuoteStyle::Always => quote::quote! { ::elicitation::CsvQuoteStyle::Always },
            CsvQuoteStyle::Necessary => {
                quote::quote! { ::elicitation::CsvQuoteStyle::Necessary }
            }
            CsvQuoteStyle::NonNumeric => {
                quote::quote! { ::elicitation::CsvQuoteStyle::NonNumeric }
            }
            CsvQuoteStyle::Never => quote::quote! { ::elicitation::CsvQuoteStyle::Never },
        }
    }
}
