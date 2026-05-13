//! Trenchcoat wrapper for [`proj::Area`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use proj::Area;

/// Elicitable wrapper for [`proj::Area`].
///
/// In the case of an area of use crossing the antimeridian (longitude +/- 180
/// degrees), `west` must be greater than `east`.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct ProjArea {
    /// Western boundary of the bounding box.
    pub west: f64,
    /// Southern boundary of the bounding box.
    pub south: f64,
    /// Eastern boundary of the bounding box.
    pub east: f64,
    /// Northern boundary of the bounding box.
    pub north: f64,
}

impl ProjArea {
    /// Creates a new PROJ area-of-use bounding box.
    ///
    /// In the case of an area of use crossing the antimeridian (longitude +/-
    /// 180 degrees), `west` must be greater than `east`.
    #[tracing::instrument]
    pub fn new(west: f64, south: f64, east: f64, north: f64) -> Self {
        Self {
            west,
            south,
            east,
            north,
        }
    }

    /// Converts this wrapper into the upstream `proj` type.
    #[tracing::instrument]
    pub fn into_inner(self) -> Area {
        self.into()
    }
}

impl From<Area> for ProjArea {
    fn from(value: Area) -> Self {
        Self::new(value.west, value.south, value.east, value.north)
    }
}

impl From<ProjArea> for Area {
    fn from(value: ProjArea) -> Self {
        Self::new(value.west, value.south, value.east, value.north)
    }
}

crate::default_style!(ProjArea => ProjAreaStyle);

impl Prompt for ProjArea {
    fn prompt() -> Option<&'static str> {
        Some("Specify a PROJ area of use bounding box (west, south, east, north):")
    }
}

impl Elicitation for ProjArea {
    type Style = ProjAreaStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self::new(
            f64::elicit(communicator).await?,
            f64::elicit(communicator).await?,
            f64::elicit(communicator).await?,
            f64::elicit(communicator).await?,
        ))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let proof = <f64 as Elicitation>::kani_proof();
        let mut tokens = proc_macro2::TokenStream::new();
        tokens.extend(proof.clone());
        tokens.extend(proof.clone());
        tokens.extend(proof.clone());
        tokens.extend(proof);
        tokens
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let proof = <f64 as Elicitation>::verus_proof();
        let mut tokens = proc_macro2::TokenStream::new();
        tokens.extend(proof.clone());
        tokens.extend(proof.clone());
        tokens.extend(proof.clone());
        tokens.extend(proof);
        tokens
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let proof = <f64 as Elicitation>::creusot_proof();
        let mut tokens = proc_macro2::TokenStream::new();
        tokens.extend(proof.clone());
        tokens.extend(proof.clone());
        tokens.extend(proof.clone());
        tokens.extend(proof);
        tokens
    }
}

impl ElicitIntrospect for ProjArea {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "proj::Area",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "west",
                        type_name: "f64",
                        prompt: Some("Western boundary:"),
                    },
                    FieldInfo {
                        name: "south",
                        type_name: "f64",
                        prompt: Some("Southern boundary:"),
                    },
                    FieldInfo {
                        name: "east",
                        type_name: "f64",
                        prompt: Some("Eastern boundary:"),
                    },
                    FieldInfo {
                        name: "north",
                        type_name: "f64",
                        prompt: Some("Northern boundary:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for ProjArea {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "ProjArea".to_string(),
            fields: vec![
                ("west".to_string(), Box::new(f64::prompt_tree())),
                ("south".to_string(), Box::new(f64::prompt_tree())),
                ("east".to_string(), Box::new(f64::prompt_tree())),
                ("north".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for ProjArea {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let west = self.west;
        let south = self.south;
        let east = self.east;
        let north = self.north;
        quote::quote! {
            ::elicitation::ProjArea::new(#west, #south, #east, #north)
        }
    }
}
