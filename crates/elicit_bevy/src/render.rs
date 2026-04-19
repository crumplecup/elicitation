//! Shadow types for user-facing render components.
//!
//! Covers [`NoAutomaticBatching`], [`MipBias`], and [`OcclusionCulling`].

// ── shadow_elicitation / unit_elicitation macros (module-local) ───────────────

macro_rules! shadow_elicitation {
    ($name:ident) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl elicitation::Elicitation for $name {
            type Style = ();

            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                let response = communicator
                    .send_prompt(concat!("Enter value for ", stringify!($name)))
                    .await?;
                serde_json::from_str(&response)
                    .or_else(|_| serde_json::from_str::<Self>(&format!("\"{}\"", response)))
                    .map_err(|e| {
                        elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                            format!("Invalid {}: {}", stringify!($name), e),
                        ))
                    })
            }

            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }

            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }

            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }

        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }

        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }

        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(concat!("Shadow type for `", stringify!($name), "`.").to_string())
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        impl elicitation::ElicitComplete for $name {}
    };
}

macro_rules! unit_elicitation {
    ($name:ident, $inner_path:path) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl elicitation::Elicitation for $name {
            type Style = ();

            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Ok(Self)
            }

            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }

            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }

            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }

        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }

        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }

        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(
                        concat!(
                            "Marker component shadow for `",
                            stringify!($inner_path),
                            "`."
                        )
                        .to_string(),
                    )
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        impl elicitation::ElicitComplete for $name {}
    };
}

// ── NoAutomaticBatching ───────────────────────────────────────────────────────

/// Shadow of [`bevy::render::batching::NoAutomaticBatching`].
///
/// Marker component that disables automatic draw-call batching for a mesh entity.
/// Useful when per-entity rendering order or draw call isolation is required.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct NoAutomaticBatching;

impl From<NoAutomaticBatching> for bevy::render::batching::NoAutomaticBatching {
    fn from(_: NoAutomaticBatching) -> Self {
        bevy::render::batching::NoAutomaticBatching
    }
}

mod emit_impls_no_automatic_batching {
    use super::NoAutomaticBatching;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for NoAutomaticBatching {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::NoAutomaticBatching }
        }
    }
}

unit_elicitation!(
    NoAutomaticBatching,
    bevy::render::batching::NoAutomaticBatching
);

// ── MipBias ───────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::render::camera::MipBias`].
///
/// Camera component specifying a mip bias when sampling material textures.
/// Typically used alongside TAA or other temporal upscaling techniques
/// to counteract texture blurriness introduced by jitter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct MipBias(pub f32);

impl Default for MipBias {
    fn default() -> Self {
        Self(bevy::render::camera::MipBias::default().0)
    }
}

impl From<MipBias> for bevy::render::camera::MipBias {
    fn from(v: MipBias) -> Self {
        bevy::render::camera::MipBias(v.0)
    }
}

impl From<bevy::render::camera::MipBias> for MipBias {
    fn from(v: bevy::render::camera::MipBias) -> Self {
        MipBias(v.0)
    }
}

mod emit_impls_mip_bias {
    use super::MipBias;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for MipBias {
        fn to_code_literal(&self) -> TokenStream {
            let bias = self.0;
            quote::quote! { ::elicit_bevy::MipBias(#bias) }
        }
    }
}

shadow_elicitation!(MipBias);

// ── OcclusionCulling ──────────────────────────────────────────────────────────

/// Shadow of [`bevy::render::experimental::occlusion_culling::OcclusionCulling`].
///
/// Marker component enabling GPU-driven occlusion culling for a camera.
/// Entities occluded by closer geometry are skipped during rendering.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct OcclusionCulling;

impl From<OcclusionCulling> for bevy::render::experimental::occlusion_culling::OcclusionCulling {
    fn from(_: OcclusionCulling) -> Self {
        bevy::render::experimental::occlusion_culling::OcclusionCulling
    }
}

mod emit_impls_occlusion_culling {
    use super::OcclusionCulling;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for OcclusionCulling {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::OcclusionCulling }
        }
    }
}

unit_elicitation!(
    OcclusionCulling,
    bevy::render::experimental::occlusion_culling::OcclusionCulling
);
