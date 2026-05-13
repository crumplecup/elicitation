//! Elicitation support for [`georaster::Coordinate`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use georaster::Coordinate;

crate::default_style!(Coordinate => GeoRasterCoordinateStyle);

impl Prompt for Coordinate {
    fn prompt() -> Option<&'static str> {
        Some("Enter a georaster coordinate (x = longitude, y = latitude):")
    }
}

impl Elicitation for Coordinate {
    type Style = GeoRasterCoordinateStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            x: f64::elicit(communicator).await?,
            y: f64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("georaster::Coordinate")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("georaster::Coordinate")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("georaster::Coordinate")
    }
}

impl ElicitIntrospect for Coordinate {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "georaster::Coordinate",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f64",
                        prompt: Some("Longitude / X coordinate:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f64",
                        prompt: Some("Latitude / Y coordinate:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Coordinate {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "georaster::Coordinate".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f64::prompt_tree())),
                ("y".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for Coordinate {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        quote::quote! {
            ::georaster::Coordinate { x: #x, y: #y }
        }
    }
}
