//! Bevy animation shadow types.
//!
//! Covers [`bevy::animation::RepeatAnimation`], [`bevy::animation::AnimationTargetId`],
//! [`bevy::animation::AnimationPlayer`], and [`bevy::animation::AnimationTransitions`].

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── shadow_elicitation macro ──────────────────────────────────────────────────

/// Generates the full `ElicitComplete` trait chain for a local shadow type that
/// already has `#[derive(Serialize, Deserialize, JsonSchema)]`.
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

elicit_newtype!(bevy::animation::RepeatAnimation, as RepeatAnimation);
elicit_newtype_traits!(RepeatAnimation, bevy::animation::RepeatAnimation, []);

impl serde::Serialize for RepeatAnimation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.variant_name())
    }
}
impl<'de> serde::Deserialize<'de> for RepeatAnimation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = String::deserialize(d)?;
        let repeat = match value.as_str() {
            "Never" => bevy::animation::RepeatAnimation::Never,
            "Forever" => bevy::animation::RepeatAnimation::Forever,
            "Count" => bevy::animation::RepeatAnimation::Count(1),
            _ => {
                return Err(D::Error::unknown_variant(
                    &value,
                    &["Never", "Forever", "Count"],
                ));
            }
        };
        Ok(RepeatAnimation(std::sync::Arc::new(repeat)))
    }
}
impl From<RepeatAnimation> for bevy::animation::RepeatAnimation {
    fn from(v: RepeatAnimation) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl RepeatAnimation {
    /// Returns `true` if the animation never loops.
    #[tracing::instrument(skip(self))]
    pub fn is_forever(&self) -> bool {
        *self.0 == bevy::animation::RepeatAnimation::Forever
    }
    /// Returns `true` if the animation plays only once.
    #[tracing::instrument(skip(self))]
    pub fn is_never(&self) -> bool {
        *self.0 == bevy::animation::RepeatAnimation::Never
    }
    /// Returns the remaining count if this is `Count`, otherwise `None`.
    #[tracing::instrument(skip(self))]
    pub fn count(&self) -> Option<u32> {
        match *self.0 {
            bevy::animation::RepeatAnimation::Count(n) => Some(n),
            _ => None,
        }
    }
    /// Returns the variant name: `"Never"`, `"Forever"`, or `"Count"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::animation::RepeatAnimation::Never => "Never",
            bevy::animation::RepeatAnimation::Forever => "Forever",
            bevy::animation::RepeatAnimation::Count(_) => "Count",
        }
    }
}

mod emit_impls {
    use super::RepeatAnimation;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for RepeatAnimation {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::animation::RepeatAnimation::Never => {
                    quote::quote! { ::bevy::animation::RepeatAnimation::Never }
                }
                bevy::animation::RepeatAnimation::Forever => {
                    quote::quote! { ::bevy::animation::RepeatAnimation::Forever }
                }
                bevy::animation::RepeatAnimation::Count(n) => {
                    quote::quote! { ::bevy::animation::RepeatAnimation::Count(#n) }
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for RepeatAnimation {}

// ── AnimationTargetId ─────────────────────────────────────────────────────────

elicit_newtype!(bevy::animation::AnimationTargetId, as AnimationTargetId);
elicit_newtype_traits!(AnimationTargetId, bevy::animation::AnimationTargetId, [eq]);

impl serde::Serialize for AnimationTargetId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0.0.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for AnimationTargetId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = String::deserialize(d)?;
        let uuid = uuid::Uuid::parse_str(&value).map_err(|e| D::Error::custom(e.to_string()))?;
        Ok(AnimationTargetId(std::sync::Arc::new(
            bevy::animation::AnimationTargetId(uuid),
        )))
    }
}
impl From<AnimationTargetId> for bevy::animation::AnimationTargetId {
    fn from(v: AnimationTargetId) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl AnimationTargetId {
    /// Returns the inner UUID as a hyphenated string.
    #[tracing::instrument(skip(self))]
    pub fn to_uuid_string(&self) -> String {
        self.0.0.to_string()
    }
}

mod emit_impls_animation_target {
    use super::AnimationTargetId;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for AnimationTargetId {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.0.to_string();
            quote::quote! {
                ::bevy::animation::AnimationTargetId(
                    ::uuid::Uuid::parse_str(#s).unwrap()
                )
            }
        }
    }
}
impl elicitation::ElicitComplete for AnimationTargetId {}

// ── AnimationPlayer ───────────────────────────────────────────────────────────

/// Shadow for [`bevy::animation::AnimationPlayer`].
///
/// `AnimationPlayer` holds private runtime state managed by Bevy's animation
/// systems. Users always spawn it as `AnimationPlayer::default()` and then
/// drive playback via ECS system calls. This shadow type serializes as `{}`
/// and emits that default constructor.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct AnimationPlayer {}

mod emit_animation_player {
    use super::AnimationPlayer;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for AnimationPlayer {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::animation::AnimationPlayer::default() }
        }
    }
}

shadow_elicitation!(AnimationPlayer);

// ── AnimationTransitions ──────────────────────────────────────────────────────

/// Shadow for [`bevy::animation::AnimationTransitions`].
///
/// `AnimationTransitions` holds private transition state managed by Bevy's
/// animation system. Users always add it as `AnimationTransitions::default()`
/// and then drive transitions via ECS system calls. This shadow type
/// serializes as `{}` and emits that default constructor.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct AnimationTransitions {}

mod emit_animation_transitions {
    use super::AnimationTransitions;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for AnimationTransitions {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::animation::AnimationTransitions::default() }
        }
    }
}

shadow_elicitation!(AnimationTransitions);
