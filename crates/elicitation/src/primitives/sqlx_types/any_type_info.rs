//! [`sqlx::any::AnyTypeInfo`] elicitation.
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use sqlx::any::{AnyTypeInfo, AnyTypeInfoKind};

crate::default_style!(AnyTypeInfo => AnyTypeInfoStyle);

impl Prompt for AnyTypeInfo {
    fn prompt() -> Option<&'static str> {
        Some("Specify the SQL column type info:")
    }
}

impl Elicitation for AnyTypeInfo {
    type Style = AnyTypeInfoStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AnyTypeInfo");
        let kind = AnyTypeInfoKind::elicit(communicator).await?;
        Ok(AnyTypeInfo { kind })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

impl ElicitIntrospect for AnyTypeInfo {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "sqlx::any::AnyTypeInfo",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "kind",
                    type_name: "AnyTypeInfoKind",
                    prompt: Some("The SQL type category"),
                }],
            },
        }
    }
}
