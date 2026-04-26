//! Trenchcoat wrapper for [`surrealdb_types::Number`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A SurrealDB numeric value.
///
/// Wraps an upstream `surrealdb_types::Number` to add [`JsonSchema`] for MCP
/// boundary crossing. Numbers can be 64-bit integers, 64-bit floats, or
/// arbitrary-precision decimals (represented as strings).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum Number {
    /// A 64-bit signed integer.
    Int(i64),
    /// A 64-bit floating-point number.
    Float(f64),
    /// An arbitrary-precision decimal, represented as a string.
    ///
    /// Use standard decimal notation, e.g. `"3.14159265358979323846"`.
    Decimal(String),
}

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::Number> for Number {
    fn from(n: surrealdb_types::Number) -> Self {
        match n {
            surrealdb_types::Number::Int(i) => Number::Int(i),
            surrealdb_types::Number::Float(f) => Number::Float(f),
            surrealdb_types::Number::Decimal(d) => Number::Decimal(d.to_string()),
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<Number> for surrealdb_types::Number {
    fn from(n: Number) -> Self {
        match n {
            Number::Int(i) => surrealdb_types::Number::Int(i),
            Number::Float(f) => surrealdb_types::Number::Float(f),
            Number::Decimal(s) => surrealdb_types::Number::Decimal(
                s.parse::<rust_decimal::Decimal>().unwrap_or_default(),
            ),
        }
    }
}

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

impl Prompt for Number {
    fn prompt() -> Option<&'static str> {
        Some("Choose the numeric value type:")
    }
}

impl Select for Number {
    fn options() -> Vec<Self> {
        vec![
            Number::Int(0),
            Number::Float(0.0),
            Number::Decimal("0".to_string()),
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Int (64-bit signed integer)".to_string(),
            "Float (64-bit float)".to_string(),
            "Decimal (arbitrary-precision string)".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Int (64-bit signed integer)" => Some(Number::Int(0)),
            "Float (64-bit float)" => Some(Number::Float(0.0)),
            "Decimal (arbitrary-precision string)" => Some(Number::Decimal("0".to_string())),
            _ => None,
        }
    }
}

crate::default_style!(Number => NumberStyle);

impl Elicitation for Number {
    type Style = NumberStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Number");
        let type_params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the numeric value type:"),
            &Self::labels(),
        );
        let type_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(type_params),
            )
            .await?;
        let type_label = mcp::parse_string(mcp::extract_value(type_result)?)?;
        tracing::debug!(type_label = %type_label, "Selected number type");

        let val_params = mcp::text_params("Enter the numeric value:");
        let val_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(val_params),
            )
            .await?;
        let val_str = mcp::parse_string(mcp::extract_value(val_result)?)?;
        let val_str = val_str.trim();
        tracing::debug!(value = %val_str, "Entered numeric value");

        match type_label.as_str() {
            "Int (64-bit signed integer)" => {
                let i: i64 = val_str.parse().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid integer \"{}\": {}",
                        val_str, e
                    )))
                })?;
                Ok(Number::Int(i))
            }
            "Float (64-bit float)" => {
                let f: f64 = val_str.parse().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid float \"{}\": {}",
                        val_str, e
                    )))
                })?;
                Ok(Number::Float(f))
            }
            _ => Ok(Number::Decimal(val_str.to_string())),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "Number",
            "Int (64-bit signed integer)",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "Number",
            "Int (64-bit signed integer)",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "Number",
            "Int (64-bit signed integer)",
        )
    }
}

impl ElicitIntrospect for Number {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "SurrealNumber",
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

impl crate::ElicitPromptTree for Number {
    fn prompt_tree() -> crate::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the numeric value type:")
                .to_string(),
            type_name: "SurrealNumber".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for Number {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let json = serde_json::to_string(self).expect("Number should serialize");
        quote::quote! {
            ::serde_json::from_str::<elicitation::SurrealNumber>(#json)
                .expect("serialized SurrealNumber should deserialize")
        }
    }
}
