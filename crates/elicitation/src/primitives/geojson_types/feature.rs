//! Elicitation support for [`geojson::Feature`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geojson::Feature;

use super::helpers::{
    elicit_optional_bbox, elicit_optional_geometry, elicit_optional_id,
    elicit_optional_json_object, optional_json_object_prompt_tree, serde_json_code_literal,
};

crate::default_style!(Feature => GeoJsonFeatureStyle);

impl Prompt for Feature {
    fn prompt() -> Option<&'static str> {
        Some("Build a GeoJSON feature:")
    }
}

impl Elicitation for Feature {
    type Style = GeoJsonFeatureStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let geometry =
            elicit_optional_geometry(communicator, "Include a geometry in this feature?").await?;
        let id = elicit_optional_id(communicator, "Include an id for this feature?").await?;
        let properties = elicit_optional_json_object(
            communicator,
            "Include a properties JSON object for this feature?",
        )
        .await?;
        let bbox =
            elicit_optional_bbox(communicator, "Add a bounding box to this feature?").await?;
        let foreign_members = elicit_optional_json_object(
            communicator,
            "Add a foreign-members JSON object to this feature?",
        )
        .await?;
        Ok(Self {
            bbox,
            geometry,
            id,
            properties,
            foreign_members,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("geojson::Feature")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("geojson::Feature")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("geojson::Feature")
    }
}

impl ElicitIntrospect for Feature {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geojson::Feature",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "geometry",
                        type_name: "Option<geojson::Geometry>",
                        prompt: Some("Optional geometry:"),
                    },
                    FieldInfo {
                        name: "id",
                        type_name: "Option<geojson::feature::Id>",
                        prompt: Some("Optional feature id:"),
                    },
                    FieldInfo {
                        name: "properties",
                        type_name: "Option<geojson::JsonObject>",
                        prompt: Some("Optional properties object:"),
                    },
                    FieldInfo {
                        name: "bbox",
                        type_name: "Option<geojson::Bbox>",
                        prompt: Some("Optional bounding box:"),
                    },
                    FieldInfo {
                        name: "foreign_members",
                        type_name: "Option<geojson::JsonObject>",
                        prompt: Some("Optional foreign-members object:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Feature {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "geojson::Feature".to_string(),
            fields: vec![
                (
                    "geometry".to_string(),
                    Box::new(Option::<geojson::Geometry>::prompt_tree()),
                ),
                (
                    "id".to_string(),
                    Box::new(Option::<geojson::feature::Id>::prompt_tree()),
                ),
                (
                    "properties".to_string(),
                    Box::new(optional_json_object_prompt_tree(
                        "Include a properties JSON object for this feature?",
                    )),
                ),
                (
                    "bbox".to_string(),
                    Box::new(Option::<Vec<f64>>::prompt_tree()),
                ),
                (
                    "foreign_members".to_string(),
                    Box::new(optional_json_object_prompt_tree(
                        "Add a foreign-members JSON object to this feature?",
                    )),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for Feature {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        serde_json_code_literal(self, quote::quote!(geojson::Feature))
    }
}
