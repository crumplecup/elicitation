//! Elicitation support for [`geojson::GeoJson`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use geojson::GeoJson;

use super::helpers::serde_json_code_literal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GeoJsonKind {
    Geometry,
    Feature,
    FeatureCollection,
}

impl Prompt for GeoJsonKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose a top-level GeoJSON document type:")
    }
}

impl Select for GeoJsonKind {
    fn options() -> Vec<Self> {
        vec![Self::Geometry, Self::Feature, Self::FeatureCollection]
    }

    fn labels() -> Vec<String> {
        vec![
            "Geometry".to_string(),
            "Feature".to_string(),
            "FeatureCollection".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Geometry" => Some(Self::Geometry),
            "Feature" => Some(Self::Feature),
            "FeatureCollection" => Some(Self::FeatureCollection),
            _ => None,
        }
    }
}

crate::default_style!(GeoJson => GeoJsonStyle);

impl Prompt for GeoJson {
    fn prompt() -> Option<&'static str> {
        Some("Choose a top-level GeoJSON document:")
    }
}

impl Elicitation for GeoJson {
    type Style = GeoJsonStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(mcp::select_params(
                        GeoJsonKind::prompt()
                            .unwrap_or("Choose a top-level GeoJSON document type:"),
                        &GeoJsonKind::labels(),
                    )),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        match GeoJsonKind::from_label(&label) {
            Some(GeoJsonKind::Geometry) => Ok(Self::Geometry(
                geojson::Geometry::elicit(communicator).await?,
            )),
            Some(GeoJsonKind::Feature) => {
                Ok(Self::Feature(geojson::Feature::elicit(communicator).await?))
            }
            Some(GeoJsonKind::FeatureCollection) => Ok(Self::FeatureCollection(
                geojson::FeatureCollection::elicit(communicator).await?,
            )),
            None => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(label))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("geojson::GeoJson", "Geometry")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("geojson::GeoJson", "Geometry")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("geojson::GeoJson", "Geometry")
    }
}

impl ElicitIntrospect for GeoJson {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geojson::GeoJson",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: GeoJsonKind::labels()
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

impl crate::ElicitPromptTree for GeoJson {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a top-level GeoJSON document:")
                .to_string(),
            type_name: "geojson::GeoJson".to_string(),
            options: GeoJsonKind::labels(),
            branches: vec![
                Some(Box::new(geojson::Geometry::prompt_tree())),
                Some(Box::new(geojson::Feature::prompt_tree())),
                Some(Box::new(geojson::FeatureCollection::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeoJson {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        serde_json_code_literal(self, quote::quote!(geojson::GeoJson))
    }
}
