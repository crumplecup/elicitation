//! Wrapper for [`wkb::writer::WriteOptions`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, WkbEndianness,
};

/// Serializable mirror of [`wkb::writer::WriteOptions`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct WkbWriteOptions {
    /// Byte order to use when writing WKB bytes.
    pub endianness: WkbEndianness,
}

impl From<wkb::writer::WriteOptions> for WkbWriteOptions {
    fn from(value: wkb::writer::WriteOptions) -> Self {
        Self {
            endianness: value.endianness.into(),
        }
    }
}

impl From<WkbWriteOptions> for wkb::writer::WriteOptions {
    fn from(value: WkbWriteOptions) -> Self {
        Self {
            endianness: value.endianness.into(),
        }
    }
}

crate::default_style!(WkbWriteOptions => WkbWriteOptionsStyle);

impl Prompt for WkbWriteOptions {
    fn prompt() -> Option<&'static str> {
        Some("Configure WKB write options:")
    }
}

impl Elicitation for WkbWriteOptions {
    type Style = WkbWriteOptionsStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            endianness: WkbEndianness::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <WkbEndianness as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <WkbEndianness as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <WkbEndianness as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for WkbWriteOptions {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkb::writer::WriteOptions",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "endianness",
                    type_name: "WkbEndianness",
                    prompt: Some("Byte order to use for emitted WKB:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WkbWriteOptions {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WkbWriteOptions".to_string(),
            fields: vec![(
                "endianness".to_string(),
                Box::new(WkbEndianness::prompt_tree()),
            )],
        }
    }
}
