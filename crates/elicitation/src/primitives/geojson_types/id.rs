//! Elicitation support for [`geojson::feature::Id`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use geojson::feature::Id as GeoJsonId;

use super::helpers::serde_json_code_literal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GeoJsonIdKind {
    String,
    Number,
}

impl Prompt for GeoJsonIdKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose a GeoJSON feature id type:")
    }
}

impl Select for GeoJsonIdKind {
    fn options() -> Vec<Self> {
        vec![Self::String, Self::Number]
    }

    fn labels() -> Vec<String> {
        vec!["String".to_string(), "Number".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "String" => Some(Self::String),
            "Number" => Some(Self::Number),
            _ => None,
        }
    }
}

crate::default_style!(GeoJsonId => GeoJsonIdStyle);

impl Prompt for GeoJsonId {
    fn prompt() -> Option<&'static str> {
        Some("Choose a GeoJSON feature id:")
    }
}

impl Elicitation for GeoJsonId {
    type Style = GeoJsonIdStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(mcp::select_params(
                        GeoJsonIdKind::prompt().unwrap_or("Choose a GeoJSON feature id type:"),
                        &GeoJsonIdKind::labels(),
                    )),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        match GeoJsonIdKind::from_label(&label) {
            Some(GeoJsonIdKind::String) => Ok(Self::String(String::elicit(communicator).await?)),
            Some(GeoJsonIdKind::Number) => {
                let raw = String::elicit(communicator).await?;
                let parsed: serde_json::Value = serde_json::from_str(&raw).map_err(|error| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid GeoJSON numeric id: {error}"
                    )))
                })?;
                match parsed {
                    serde_json::Value::Number(number) => Ok(Self::Number(number)),
                    _ => Err(ElicitError::new(ElicitErrorKind::ParseError(
                        "GeoJSON numeric id must be a JSON number literal".to_string(),
                    ))),
                }
            }
            None => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(label))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("geojson::feature::Id", "String")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("geojson::feature::Id", "String")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("geojson::feature::Id", "String")
    }
}

impl ElicitIntrospect for GeoJsonId {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geojson::feature::Id",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: GeoJsonIdKind::labels()
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

impl crate::ElicitPromptTree for GeoJsonId {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a GeoJSON feature id:")
                .to_string(),
            type_name: "geojson::feature::Id".to_string(),
            options: GeoJsonIdKind::labels(),
            branches: vec![
                Some(Box::new(String::prompt_tree())),
                Some(Box::new(String::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeoJsonId {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        serde_json_code_literal(self, quote::quote!(geojson::feature::Id))
    }
}
