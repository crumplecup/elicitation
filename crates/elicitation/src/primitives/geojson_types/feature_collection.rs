//! Elicitation support for [`geojson::FeatureCollection`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geojson::FeatureCollection;

use super::helpers::{
    elicit_optional_bbox, elicit_optional_json_object, optional_json_object_prompt_tree,
    serde_json_code_literal,
};

crate::default_style!(FeatureCollection => GeoJsonFeatureCollectionStyle);

impl Prompt for FeatureCollection {
    fn prompt() -> Option<&'static str> {
        Some("Build a GeoJSON feature collection:")
    }
}

impl Elicitation for FeatureCollection {
    type Style = GeoJsonFeatureCollectionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let features = Vec::<geojson::Feature>::elicit(communicator).await?;
        let bbox = elicit_optional_bbox(
            communicator,
            "Add a bounding box to this feature collection?",
        )
        .await?;
        let foreign_members = elicit_optional_json_object(
            communicator,
            "Add a foreign-members JSON object to this feature collection?",
        )
        .await?;
        Ok(Self {
            bbox,
            features,
            foreign_members,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("geojson::FeatureCollection")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("geojson::FeatureCollection")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("geojson::FeatureCollection")
    }
}

impl ElicitIntrospect for FeatureCollection {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geojson::FeatureCollection",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "features",
                        type_name: "Vec<geojson::Feature>",
                        prompt: Some("Features:"),
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

impl crate::ElicitPromptTree for FeatureCollection {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "geojson::FeatureCollection".to_string(),
            fields: vec![
                (
                    "features".to_string(),
                    Box::new(Vec::<geojson::Feature>::prompt_tree()),
                ),
                (
                    "bbox".to_string(),
                    Box::new(Option::<Vec<f64>>::prompt_tree()),
                ),
                (
                    "foreign_members".to_string(),
                    Box::new(optional_json_object_prompt_tree(
                        "Add a foreign-members JSON object to this feature collection?",
                    )),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for FeatureCollection {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        serde_json_code_literal(self, quote::quote!(geojson::FeatureCollection))
    }
}
