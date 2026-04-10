//! UUID handle trenchcoats for non-serializable tower runtime services.
//!
//! Each handle is `{ id: String }` — a thin wrapper around a UUID that
//! identifies a live service stored in the shadow crate's registry.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

macro_rules! tower_handle {
    (
        $name:ident,
        type_name = $type_name:literal,
        prompt = $prompt:literal $(,)?
    ) => {
        #[doc = concat!("UUID handle for a live `", $type_name, "` stored in the plugin registry.")]
        #[derive(
            Debug,
            Clone,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
            elicitation_derive::ToCodeLiteral,
        )]
        pub struct $name {
            /// Registry key (UUID) for this service instance.
            pub id: String,
        }

        paste::paste! {
            crate::default_style!($name => [<$name Style>]);
        }

        impl Prompt for $name {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        impl Elicitation for $name {
            type Style = paste::paste! { [<$name Style>] };

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($name)));
                let id = String::elicit(communicator).await?;
                Ok(Self { id })
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                <String as Elicitation>::kani_proof()
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                <String as Elicitation>::verus_proof()
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                <String as Elicitation>::creusot_proof()
            }
        }

        impl ElicitIntrospect for $name {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Survey
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: $type_name,
                    description: Self::prompt(),
                    details: PatternDetails::Survey {
                        fields: vec![FieldInfo {
                            name: "id",
                            type_name: "String",
                            prompt: Some("Service UUID:"),
                        }],
                    },
                }
            }
        }

        impl crate::ElicitPromptTree for $name {
            fn prompt_tree() -> crate::PromptTree {
                crate::PromptTree::Survey {
                    prompt: Self::prompt().map(str::to_string),
                    type_name: stringify!($name).to_string(),
                    fields: vec![("id".to_string(), Box::new(String::prompt_tree()))],
                }
            }
        }
    };
}

tower_handle!(
    TowerServiceBuilderHandle,
    type_name = "tower::ServiceBuilder (handle)",
    prompt = "Enter the ServiceBuilder registry UUID:"
);

tower_handle!(
    TowerBufferHandle,
    type_name = "tower::buffer::Buffer (handle)",
    prompt = "Enter the Buffer service registry UUID:"
);

tower_handle!(
    TowerRateLimitHandle,
    type_name = "tower::limit::RateLimit (handle)",
    prompt = "Enter the RateLimit service registry UUID:"
);

tower_handle!(
    TowerConcurrencyLimitHandle,
    type_name = "tower::limit::ConcurrencyLimit (handle)",
    prompt = "Enter the ConcurrencyLimit service registry UUID:"
);

tower_handle!(
    TowerTimeoutHandle,
    type_name = "tower::timeout::Timeout (handle)",
    prompt = "Enter the Timeout service registry UUID:"
);

tower_handle!(
    TowerLoadShedHandle,
    type_name = "tower::load_shed::LoadShed (handle)",
    prompt = "Enter the LoadShed service registry UUID:"
);

tower_handle!(
    TowerRetryHandle,
    type_name = "tower::retry::Retry (handle)",
    prompt = "Enter the Retry service registry UUID:"
);

tower_handle!(
    TowerFilterHandle,
    type_name = "tower::filter::Filter (handle)",
    prompt = "Enter the Filter service registry UUID:"
);

tower_handle!(
    TowerBoxServiceHandle,
    type_name = "tower::util::BoxService (handle)",
    prompt = "Enter the BoxService registry UUID:"
);

tower_handle!(
    TowerMapRequestHandle,
    type_name = "tower::util::MapRequest (handle)",
    prompt = "Enter the MapRequest service registry UUID:"
);

tower_handle!(
    TowerMapResponseHandle,
    type_name = "tower::util::MapResponse (handle)",
    prompt = "Enter the MapResponse service registry UUID:"
);

tower_handle!(
    TowerMapErrHandle,
    type_name = "tower::util::MapErr (handle)",
    prompt = "Enter the MapErr service registry UUID:"
);

tower_handle!(
    TowerThenHandle,
    type_name = "tower::util::Then (handle)",
    prompt = "Enter the Then service registry UUID:"
);

tower_handle!(
    TowerAndThenHandle,
    type_name = "tower::util::AndThen (handle)",
    prompt = "Enter the AndThen service registry UUID:"
);

tower_handle!(
    TowerMapResultHandle,
    type_name = "tower::util::MapResult (handle)",
    prompt = "Enter the MapResult service registry UUID:"
);

tower_handle!(
    TowerBoxCloneServiceHandle,
    type_name = "tower::util::BoxCloneService (handle)",
    prompt = "Enter the BoxCloneService registry UUID:"
);

tower_handle!(
    TowerSteerHandle,
    type_name = "tower::steer::Steer (handle)",
    prompt = "Enter the Steer service registry UUID:"
);

tower_handle!(
    TowerBalanceHandle,
    type_name = "tower::balance::p2c::Balance (handle)",
    prompt = "Enter the Balance service registry UUID:"
);

tower_handle!(
    TowerPeakEwmaHandle,
    type_name = "tower::load::PeakEwma (handle)",
    prompt = "Enter the PeakEwma service registry UUID:"
);

tower_handle!(
    TowerPendingRequestsHandle,
    type_name = "tower::load::PendingRequests (handle)",
    prompt = "Enter the PendingRequests service registry UUID:"
);
