//! Elicitation support for [`geojson::Geometry`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geojson::Geometry;

use super::helpers::{
    elicit_optional_bbox, elicit_optional_json_object, optional_json_object_prompt_tree,
    serde_json_code_literal,
};

crate::default_style!(Geometry => GeoJsonGeometryStyle);

impl Prompt for Geometry {
    fn prompt() -> Option<&'static str> {
        Some("Build a GeoJSON geometry:")
    }
}

impl Elicitation for Geometry {
    type Style = GeoJsonGeometryStyle;

    #[tracing::instrument(skip(communicator))]
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            let value = geojson::GeometryValue::elicit(communicator).await?;
            let bbox =
                elicit_optional_bbox(communicator, "Add a bounding box to this geometry?").await?;
            let foreign_members = elicit_optional_json_object(
                communicator,
                "Add a foreign-members JSON object to this geometry?",
            )
            .await?;
            Ok(Self {
                bbox,
                value,
                foreign_members,
            })
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("geojson::Geometry")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("geojson::Geometry")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("geojson::Geometry")
    }
}

impl ElicitIntrospect for Geometry {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geojson::Geometry",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "value",
                        type_name: "geojson::GeometryValue",
                        prompt: Some("Geometry value:"),
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

impl crate::ElicitPromptTree for Geometry {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "geojson::Geometry".to_string(),
            fields: vec![
                (
                    "value".to_string(),
                    Box::new(geojson::GeometryValue::prompt_tree()),
                ),
                (
                    "bbox".to_string(),
                    Box::new(Option::<Vec<f64>>::prompt_tree()),
                ),
                (
                    "foreign_members".to_string(),
                    Box::new(optional_json_object_prompt_tree(
                        "Add a foreign-members JSON object to this geometry?",
                    )),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for Geometry {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        serde_json_code_literal(self, quote::quote!(geojson::Geometry))
    }
}
