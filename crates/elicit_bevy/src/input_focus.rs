//! Bevy input focus shadow types.
//!
//! Covers [`bevy::input_focus::AutoFocus`],
//! [`bevy::input_focus::InputFocusVisible`],
//! [`bevy::input_focus::tab_navigation::TabIndex`],
//! [`bevy::input_focus::tab_navigation::TabGroup`], and
//! [`bevy::input_focus::directional_navigation::AutoNavigationConfig`].

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

// ── AutoFocus ─────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::input_focus::AutoFocus`].
///
/// Marker component: when added to an entity, immediately sets that entity as
/// the current input focus via a component hook.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct AutoFocus;

impl From<AutoFocus> for bevy::input_focus::AutoFocus {
    fn from(_: AutoFocus) -> Self {
        bevy::input_focus::AutoFocus
    }
}

impl From<bevy::input_focus::AutoFocus> for AutoFocus {
    fn from(_: bevy::input_focus::AutoFocus) -> Self {
        AutoFocus
    }
}

mod emit_auto_focus {
    use super::AutoFocus;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AutoFocus {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::input_focus::AutoFocus }
        }
    }
}

unit_elicitation!(AutoFocus, bevy::input_focus::AutoFocus);

// ── InputFocusVisible ─────────────────────────────────────────────────────────

/// Shadow of [`bevy::input_focus::InputFocusVisible`].
///
/// Resource that controls whether the focus indicator is visible.
/// Set to `true` to show the focus ring on the currently focused entity.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct InputFocusVisible(pub bool);

impl From<InputFocusVisible> for bevy::input_focus::InputFocusVisible {
    fn from(v: InputFocusVisible) -> Self {
        bevy::input_focus::InputFocusVisible(v.0)
    }
}

impl From<bevy::input_focus::InputFocusVisible> for InputFocusVisible {
    fn from(v: bevy::input_focus::InputFocusVisible) -> Self {
        InputFocusVisible(v.0)
    }
}

mod emit_input_focus_visible {
    use super::InputFocusVisible;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for InputFocusVisible {
        fn to_code_literal(&self) -> TokenStream {
            let v = self.0;
            quote::quote! { ::bevy::input_focus::InputFocusVisible(#v) }
        }
    }
}

shadow_elicitation!(InputFocusVisible);

// ── TabIndex ──────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::input_focus::tab_navigation::TabIndex`].
///
/// Component placed on a focusable entity to control its position within a
/// tab group.  Lower values are focused first; negative values exclude the
/// entity from tab order.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TabIndex(pub i32);

impl From<TabIndex> for bevy::input_focus::tab_navigation::TabIndex {
    fn from(v: TabIndex) -> Self {
        bevy::input_focus::tab_navigation::TabIndex(v.0)
    }
}

impl From<bevy::input_focus::tab_navigation::TabIndex> for TabIndex {
    fn from(v: bevy::input_focus::tab_navigation::TabIndex) -> Self {
        TabIndex(v.0)
    }
}

mod emit_tab_index {
    use super::TabIndex;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for TabIndex {
        fn to_code_literal(&self) -> TokenStream {
            let idx = self.0;
            quote::quote! { ::bevy::input_focus::tab_navigation::TabIndex(#idx) }
        }
    }
}

shadow_elicitation!(TabIndex);

// ── TabGroup ──────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::input_focus::tab_navigation::TabGroup`].
///
/// Component that marks a subtree of entities as a tab group.  Entities with
/// [`TabIndex`] inside this subtree participate in the tab order.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TabGroup {
    /// Order of this group relative to other tab groups (lower = focused first).
    pub order: i32,
    /// If `true`, tabbing wraps within this group (modal behaviour).
    pub modal: bool,
}

impl From<TabGroup> for bevy::input_focus::tab_navigation::TabGroup {
    fn from(v: TabGroup) -> Self {
        bevy::input_focus::tab_navigation::TabGroup {
            order: v.order,
            modal: v.modal,
        }
    }
}

impl From<bevy::input_focus::tab_navigation::TabGroup> for TabGroup {
    fn from(v: bevy::input_focus::tab_navigation::TabGroup) -> Self {
        TabGroup {
            order: v.order,
            modal: v.modal,
        }
    }
}

mod emit_tab_group {
    use super::TabGroup;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for TabGroup {
        fn to_code_literal(&self) -> TokenStream {
            let order = self.order;
            let modal = self.modal;
            quote::quote! {
                ::bevy::input_focus::tab_navigation::TabGroup {
                    order: #order,
                    modal: #modal,
                }
            }
        }
    }
}

shadow_elicitation!(TabGroup);

// ── AutoNavigationConfig ──────────────────────────────────────────────────────

/// Shadow of [`bevy::input_focus::directional_navigation::AutoNavigationConfig`].
///
/// Resource that configures the automatic directional navigation heuristics.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct AutoNavigationConfig {
    /// Minimum perpendicular alignment factor (0.0–1.0) for cardinal directions.
    pub min_alignment_factor: f32,
    /// Maximum search distance (in pixels) for finding neighbours; `None` is unlimited.
    pub max_search_distance: Option<f32>,
    /// Prefer well-aligned neighbours over closer but off-axis ones.
    pub prefer_aligned: bool,
}

impl Default for AutoNavigationConfig {
    fn default() -> Self {
        let inner = bevy::input_focus::directional_navigation::AutoNavigationConfig::default();
        Self {
            min_alignment_factor: inner.min_alignment_factor,
            max_search_distance: inner.max_search_distance,
            prefer_aligned: inner.prefer_aligned,
        }
    }
}

impl From<AutoNavigationConfig>
    for bevy::input_focus::directional_navigation::AutoNavigationConfig
{
    fn from(v: AutoNavigationConfig) -> Self {
        bevy::input_focus::directional_navigation::AutoNavigationConfig {
            min_alignment_factor: v.min_alignment_factor,
            max_search_distance: v.max_search_distance,
            prefer_aligned: v.prefer_aligned,
        }
    }
}

impl From<bevy::input_focus::directional_navigation::AutoNavigationConfig>
    for AutoNavigationConfig
{
    fn from(v: bevy::input_focus::directional_navigation::AutoNavigationConfig) -> Self {
        AutoNavigationConfig {
            min_alignment_factor: v.min_alignment_factor,
            max_search_distance: v.max_search_distance,
            prefer_aligned: v.prefer_aligned,
        }
    }
}

mod emit_auto_navigation_config {
    use super::AutoNavigationConfig;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AutoNavigationConfig {
        fn to_code_literal(&self) -> TokenStream {
            let factor = self.min_alignment_factor;
            let max_dist = match self.max_search_distance {
                None => quote::quote! { None },
                Some(d) => quote::quote! { Some(#d) },
            };
            let prefer = self.prefer_aligned;
            quote::quote! {
                ::bevy::input_focus::directional_navigation::AutoNavigationConfig {
                    min_alignment_factor: #factor,
                    max_search_distance: #max_dist,
                    prefer_aligned: #prefer,
                }
            }
        }
    }
}

shadow_elicitation!(AutoNavigationConfig);
