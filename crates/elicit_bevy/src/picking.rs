//! Bevy picking shadow types.
//!
//! Covers [`bevy::picking::mesh_picking::ray_cast::SimplifiedMesh`] and the
//! core picking enums [`Pickable`], [`PickingInteraction`].

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

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

// ── Pickable ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::picking::Pickable, as Pickable);
elicit_newtype_traits!(Pickable, bevy::picking::Pickable, [eq]);

impl serde::Serialize for Pickable {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(2))?;
        map.serialize_entry("should_block_lower", &self.0.should_block_lower)?;
        map.serialize_entry("is_hoverable", &self.0.is_hoverable)?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for Pickable {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Pickable;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Pickable object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Pickable, A::Error> {
                let mut block = true;
                let mut hover = true;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "should_block_lower" => block = map.next_value()?,
                        "is_hoverable" => hover = map.next_value()?,
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Pickable(Arc::new(bevy::picking::Pickable {
                    should_block_lower: block,
                    is_hoverable: hover,
                })))
            }
        }
        d.deserialize_map(V)
    }
}
impl From<Pickable> for bevy::picking::Pickable {
    fn from(v: Pickable) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Pickable {
    /// Whether this entity should block picking rays from reaching lower entities.
    #[tracing::instrument(skip(self))]
    pub fn should_block_lower(&self) -> bool {
        self.0.should_block_lower
    }
    /// Whether this entity can be hovered.
    #[tracing::instrument(skip(self))]
    pub fn is_hoverable(&self) -> bool {
        self.0.is_hoverable
    }
    /// Returns a `Pickable` that blocks picking and is hoverable (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn default_constructor(&self) -> Pickable {
        Pickable::from(bevy::picking::Pickable::default())
    }
    /// Returns a `Pickable` that ignores all picking (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn ignore_constructor(&self) -> Pickable {
        Pickable::from(bevy::picking::Pickable::IGNORE)
    }
}

mod emit_impls_pickable {
    use super::Pickable;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Pickable {
        fn to_code_literal(&self) -> TokenStream {
            let block = self.0.should_block_lower;
            let hover = self.0.is_hoverable;
            quote::quote! {
                ::bevy::picking::Pickable {
                    should_block_lower: #block,
                    is_hoverable: #hover,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Pickable {}

// ── PickingInteraction ────────────────────────────────────────────────────────

elicit_newtype!(bevy::picking::hover::PickingInteraction, as PickingInteraction);
elicit_newtype_traits!(
    PickingInteraction,
    bevy::picking::hover::PickingInteraction,
    []
);

impl serde::Serialize for PickingInteraction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.variant_name())
    }
}
impl<'de> serde::Deserialize<'de> for PickingInteraction {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = String::deserialize(d)?;
        let interaction = match value.as_str() {
            "Pressed" => bevy::picking::hover::PickingInteraction::Pressed,
            "Hovered" => bevy::picking::hover::PickingInteraction::Hovered,
            "None" => bevy::picking::hover::PickingInteraction::None,
            _ => {
                return Err(D::Error::unknown_variant(
                    &value,
                    &["Pressed", "Hovered", "None"],
                ));
            }
        };
        Ok(PickingInteraction(Arc::new(interaction)))
    }
}
impl From<PickingInteraction> for bevy::picking::hover::PickingInteraction {
    fn from(v: PickingInteraction) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl PickingInteraction {
    /// Returns `true` if the interaction is `Pressed`.
    #[tracing::instrument(skip(self))]
    pub fn is_pressed(&self) -> bool {
        *self.0 == bevy::picking::hover::PickingInteraction::Pressed
    }
    /// Returns `true` if the interaction is `Hovered`.
    #[tracing::instrument(skip(self))]
    pub fn is_hovered(&self) -> bool {
        *self.0 == bevy::picking::hover::PickingInteraction::Hovered
    }
    /// Returns `true` if the interaction is `None`.
    #[tracing::instrument(skip(self))]
    pub fn is_none(&self) -> bool {
        *self.0 == bevy::picking::hover::PickingInteraction::None
    }
    /// Variant name: `"Pressed"`, `"Hovered"`, or `"None"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::picking::hover::PickingInteraction::Pressed => "Pressed",
            bevy::picking::hover::PickingInteraction::Hovered => "Hovered",
            bevy::picking::hover::PickingInteraction::None => "None",
        }
    }
}

mod emit_impls_interaction {
    use super::PickingInteraction;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for PickingInteraction {
        fn to_code_literal(&self) -> TokenStream {
            let variant =
                proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::picking::hover::PickingInteraction::#variant }
        }
    }
}
impl elicitation::ElicitComplete for PickingInteraction {}

// ── PointerButton ─────────────────────────────────────────────────────────────

/// Shadow of [`bevy::picking::pointer::PointerButton`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum PointerButton {
    /// The primary pointer button (usually left mouse button).
    #[default]
    Primary,
    /// The secondary pointer button (usually right mouse button).
    Secondary,
    /// The tertiary pointer button (usually middle mouse button).
    Middle,
}

impl From<bevy::picking::pointer::PointerButton> for PointerButton {
    fn from(v: bevy::picking::pointer::PointerButton) -> Self {
        match v {
            bevy::picking::pointer::PointerButton::Primary => Self::Primary,
            bevy::picking::pointer::PointerButton::Secondary => Self::Secondary,
            bevy::picking::pointer::PointerButton::Middle => Self::Middle,
        }
    }
}

impl From<PointerButton> for bevy::picking::pointer::PointerButton {
    fn from(v: PointerButton) -> Self {
        match v {
            PointerButton::Primary => Self::Primary,
            PointerButton::Secondary => Self::Secondary,
            PointerButton::Middle => Self::Middle,
        }
    }
}

mod emit_impls_pointer_button {
    use super::PointerButton;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PointerButton {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                PointerButton::Primary => {
                    quote::quote! { ::bevy::picking::pointer::PointerButton::Primary }
                }
                PointerButton::Secondary => {
                    quote::quote! { ::bevy::picking::pointer::PointerButton::Secondary }
                }
                PointerButton::Middle => {
                    quote::quote! { ::bevy::picking::pointer::PointerButton::Middle }
                }
            }
        }
    }
}

shadow_elicitation!(PointerButton);

// ── PressDirection ────────────────────────────────────────────────────────────

/// Shadow of [`bevy::picking::pointer::PressDirection`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum PressDirection {
    /// The pointer button was just pressed.
    #[default]
    Pressed,
    /// The pointer button was just released.
    Released,
}

impl From<bevy::picking::pointer::PressDirection> for PressDirection {
    fn from(v: bevy::picking::pointer::PressDirection) -> Self {
        match v {
            bevy::picking::pointer::PressDirection::Pressed => Self::Pressed,
            bevy::picking::pointer::PressDirection::Released => Self::Released,
        }
    }
}

impl From<PressDirection> for bevy::picking::pointer::PressDirection {
    fn from(v: PressDirection) -> Self {
        match v {
            PressDirection::Pressed => Self::Pressed,
            PressDirection::Released => Self::Released,
        }
    }
}

mod emit_impls_press_direction {
    use super::PressDirection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PressDirection {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                PressDirection::Pressed => {
                    quote::quote! { ::bevy::picking::pointer::PressDirection::Pressed }
                }
                PressDirection::Released => {
                    quote::quote! { ::bevy::picking::pointer::PressDirection::Released }
                }
            }
        }
    }
}

shadow_elicitation!(PressDirection);

// ── PointerId ─────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::picking::pointer::PointerId`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum PointerId {
    /// The mouse pointer.
    #[default]
    Mouse,
    /// A touch input identified by its event index.
    Touch(u64),
    /// A custom pointer identified by a UUID.
    Custom(uuid::Uuid),
}

impl From<bevy::picking::pointer::PointerId> for PointerId {
    fn from(v: bevy::picking::pointer::PointerId) -> Self {
        match v {
            bevy::picking::pointer::PointerId::Mouse => Self::Mouse,
            bevy::picking::pointer::PointerId::Touch(id) => Self::Touch(id),
            bevy::picking::pointer::PointerId::Custom(uuid) => Self::Custom(uuid),
        }
    }
}

impl From<PointerId> for bevy::picking::pointer::PointerId {
    fn from(v: PointerId) -> Self {
        match v {
            PointerId::Mouse => Self::Mouse,
            PointerId::Touch(id) => Self::Touch(id),
            PointerId::Custom(uuid) => Self::Custom(uuid),
        }
    }
}

mod emit_impls_pointer_id {
    use super::PointerId;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PointerId {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                PointerId::Mouse => {
                    quote::quote! { ::bevy::picking::pointer::PointerId::Mouse }
                }
                PointerId::Touch(id) => {
                    quote::quote! { ::bevy::picking::pointer::PointerId::Touch(#id) }
                }
                PointerId::Custom(uuid) => {
                    let bits = uuid.as_u128();
                    quote::quote! {
                        ::bevy::picking::pointer::PointerId::Custom(
                            ::uuid::Uuid::from_u128(#bits)
                        )
                    }
                }
            }
        }
    }
}

shadow_elicitation!(PointerId);

// ── Hovered ───────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::picking::hover::Hovered`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct Hovered(pub bool);

impl From<bevy::picking::hover::Hovered> for Hovered {
    fn from(v: bevy::picking::hover::Hovered) -> Self {
        Self(v.0)
    }
}

impl From<Hovered> for bevy::picking::hover::Hovered {
    fn from(v: Hovered) -> Self {
        Self(v.0)
    }
}

mod emit_impls_hovered {
    use super::Hovered;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Hovered {
        fn to_code_literal(&self) -> TokenStream {
            let b = self.0;
            quote::quote! { ::bevy::picking::hover::Hovered(#b) }
        }
    }
}

shadow_elicitation!(Hovered);

// ── DirectlyHovered ───────────────────────────────────────────────────────────

/// Shadow of [`bevy::picking::hover::DirectlyHovered`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct DirectlyHovered(pub bool);

impl From<bevy::picking::hover::DirectlyHovered> for DirectlyHovered {
    fn from(v: bevy::picking::hover::DirectlyHovered) -> Self {
        Self(v.0)
    }
}

impl From<DirectlyHovered> for bevy::picking::hover::DirectlyHovered {
    fn from(v: DirectlyHovered) -> Self {
        Self(v.0)
    }
}

mod emit_impls_directly_hovered {
    use super::DirectlyHovered;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DirectlyHovered {
        fn to_code_literal(&self) -> TokenStream {
            let b = self.0;
            quote::quote! { ::bevy::picking::hover::DirectlyHovered(#b) }
        }
    }
}

shadow_elicitation!(DirectlyHovered);

// ── PickingSettings ───────────────────────────────────────────────────────────

/// Shadow of [`bevy::picking::PickingSettings`].
#[derive(
    Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct PickingSettings {
    /// Enables and disables all picking features.
    pub is_enabled: bool,
    /// Enables and disables input collection.
    pub is_input_enabled: bool,
    /// Enables and disables updating interaction states of entities.
    pub is_hover_enabled: bool,
    /// Enables or disables picking for window entities.
    pub is_window_picking_enabled: bool,
}

impl From<bevy::picking::PickingSettings> for PickingSettings {
    fn from(v: bevy::picking::PickingSettings) -> Self {
        Self {
            is_enabled: v.is_enabled,
            is_input_enabled: v.is_input_enabled,
            is_hover_enabled: v.is_hover_enabled,
            is_window_picking_enabled: v.is_window_picking_enabled,
        }
    }
}

impl From<PickingSettings> for bevy::picking::PickingSettings {
    fn from(v: PickingSettings) -> Self {
        Self {
            is_enabled: v.is_enabled,
            is_input_enabled: v.is_input_enabled,
            is_hover_enabled: v.is_hover_enabled,
            is_window_picking_enabled: v.is_window_picking_enabled,
        }
    }
}

mod emit_impls_picking_settings {
    use super::PickingSettings;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PickingSettings {
        fn to_code_literal(&self) -> TokenStream {
            let enabled = self.is_enabled;
            let input = self.is_input_enabled;
            let hover = self.is_hover_enabled;
            let window = self.is_window_picking_enabled;
            quote::quote! {
                ::bevy::picking::PickingSettings {
                    is_enabled: #enabled,
                    is_input_enabled: #input,
                    is_hover_enabled: #hover,
                    is_window_picking_enabled: #window,
                }
            }
        }
    }
}

shadow_elicitation!(PickingSettings);
