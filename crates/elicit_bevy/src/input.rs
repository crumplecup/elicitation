//! Bevy input shadow types.
//!
//! Covers keyboard, mouse, and gamepad input enums plus `ButtonState`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── Helper macro for simple Copy input enums with derive serde ───────────────

macro_rules! input_enum {
    ($name:ident, $upstream:path, [$($extra_trait:ident),*]) => {
        elicit_newtype!($upstream, as $name);
        elicit_newtype_traits!($name, $upstream, [eq_hash $(, $extra_trait)*]);
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                (*self.0).serialize(s)
            }
        }
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                <$upstream>::deserialize(d).map(|v| $name(Arc::new(v)))
            }
        }
        impl From<$name> for $upstream {
            fn from(v: $name) -> Self { *v.0 }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

// ── ButtonState ───────────────────────────────────────────────────────────────

input_enum!(ButtonState, bevy::input::ButtonState, []);

#[reflect_methods]
impl ButtonState {
    /// Returns `true` if the button is pressed.
    #[tracing::instrument(skip(self))]
    pub fn is_pressed(&self) -> bool {
        self.0.is_pressed()
    }
    /// Variant name: `"Pressed"` or `"Released"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::input::ButtonState::Pressed => "Pressed",
            bevy::input::ButtonState::Released => "Released",
        }
    }
}

mod emit_buttonstate {
    use super::ButtonState;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ButtonState {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::input::ButtonState::#v }
        }
    }
}

// ── MouseButton ───────────────────────────────────────────────────────────────

input_enum!(MouseButton, bevy::input::mouse::MouseButton, []);

#[reflect_methods]
impl MouseButton {
    /// Returns `true` if this is the primary (left) button.
    #[tracing::instrument(skip(self))]
    pub fn is_left(&self) -> bool {
        *self.0 == bevy::input::mouse::MouseButton::Left
    }
    /// Returns `true` if this is the secondary (right) button.
    #[tracing::instrument(skip(self))]
    pub fn is_right(&self) -> bool {
        *self.0 == bevy::input::mouse::MouseButton::Right
    }
    /// Returns `true` if this is the middle (scroll wheel) button.
    #[tracing::instrument(skip(self))]
    pub fn is_middle(&self) -> bool {
        *self.0 == bevy::input::mouse::MouseButton::Middle
    }
    /// Variant name: `"Left"`, `"Right"`, `"Middle"`, `"Back"`, `"Forward"`, `"Other"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::input::mouse::MouseButton as M;
        match *self.0 {
            M::Left => "Left",
            M::Right => "Right",
            M::Middle => "Middle",
            M::Back => "Back",
            M::Forward => "Forward",
            M::Other(_) => "Other",
        }
    }
}

mod emit_mousebutton {
    use super::MouseButton;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for MouseButton {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::input::mouse::MouseButton as M;
            match *self.0 {
                M::Other(n) => quote::quote! { ::bevy::input::mouse::MouseButton::Other(#n) },
                _ => {
                    let v = proc_macro2::Ident::new(
                        self.variant_name(),
                        proc_macro2::Span::call_site(),
                    );
                    quote::quote! { ::bevy::input::mouse::MouseButton::#v }
                }
            }
        }
    }
}

// ── MouseScrollUnit ───────────────────────────────────────────────────────────

input_enum!(MouseScrollUnit, bevy::input::mouse::MouseScrollUnit, []);

#[reflect_methods]
impl MouseScrollUnit {
    /// Variant name: `"Line"` or `"Pixel"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::input::mouse::MouseScrollUnit::Line => "Line",
            bevy::input::mouse::MouseScrollUnit::Pixel => "Pixel",
        }
    }

    /// Returns `true` if this is line-based scrolling.
    #[tracing::instrument(skip(self))]
    pub fn is_line(&self) -> bool {
        matches!(*self.0, bevy::input::mouse::MouseScrollUnit::Line)
    }

    /// Returns `true` if this is pixel-based scrolling.
    #[tracing::instrument(skip(self))]
    pub fn is_pixel(&self) -> bool {
        matches!(*self.0, bevy::input::mouse::MouseScrollUnit::Pixel)
    }
}

mod emit_scroll {
    use super::MouseScrollUnit;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for MouseScrollUnit {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::input::mouse::MouseScrollUnit::#v }
        }
    }
}

// ── GamepadButton ─────────────────────────────────────────────────────────────

input_enum!(GamepadButton, bevy::input::gamepad::GamepadButton, []);

#[reflect_methods]
impl GamepadButton {
    /// Variant name, e.g. `"South"`, `"North"`, `"LeftTrigger2"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::input::gamepad::GamepadButton as G;
        match *self.0 {
            G::South => "South",
            G::East => "East",
            G::North => "North",
            G::West => "West",
            G::C => "C",
            G::Z => "Z",
            G::LeftTrigger => "LeftTrigger",
            G::LeftTrigger2 => "LeftTrigger2",
            G::RightTrigger => "RightTrigger",
            G::RightTrigger2 => "RightTrigger2",
            G::Select => "Select",
            G::Start => "Start",
            G::Mode => "Mode",
            G::LeftThumb => "LeftThumb",
            G::RightThumb => "RightThumb",
            G::DPadUp => "DPadUp",
            G::DPadDown => "DPadDown",
            G::DPadLeft => "DPadLeft",
            G::DPadRight => "DPadRight",
            G::Other(_) => "Other",
        }
    }

    /// Returns `true` if this is a face button (South/East/North/West).
    #[tracing::instrument(skip(self))]
    pub fn is_face_button(&self) -> bool {
        use bevy::input::gamepad::GamepadButton as G;
        matches!(*self.0, G::South | G::East | G::North | G::West)
    }

    /// Returns `true` if this is a shoulder button (LeftTrigger/LeftTrigger2/RightTrigger/RightTrigger2).
    #[tracing::instrument(skip(self))]
    pub fn is_shoulder(&self) -> bool {
        use bevy::input::gamepad::GamepadButton as G;
        matches!(
            *self.0,
            G::LeftTrigger | G::LeftTrigger2 | G::RightTrigger | G::RightTrigger2
        )
    }
}

mod emit_gamepadbutton {
    use super::GamepadButton;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for GamepadButton {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::input::gamepad::GamepadButton as G;
            match *self.0 {
                G::Other(n) => quote::quote! { ::bevy::input::gamepad::GamepadButton::Other(#n) },
                _ => {
                    let v = proc_macro2::Ident::new(
                        self.variant_name(),
                        proc_macro2::Span::call_site(),
                    );
                    quote::quote! { ::bevy::input::gamepad::GamepadButton::#v }
                }
            }
        }
    }
}

// ── GamepadAxis ───────────────────────────────────────────────────────────────

input_enum!(GamepadAxis, bevy::input::gamepad::GamepadAxis, []);

#[reflect_methods]
impl GamepadAxis {
    /// Variant name, e.g. `"LeftStickX"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::input::gamepad::GamepadAxis as G;
        match *self.0 {
            G::LeftStickX => "LeftStickX",
            G::LeftStickY => "LeftStickY",
            G::LeftZ => "LeftZ",
            G::RightStickX => "RightStickX",
            G::RightStickY => "RightStickY",
            G::RightZ => "RightZ",
            G::Other(_) => "Other",
        }
    }

    /// Returns `true` if this is a left stick axis.
    #[tracing::instrument(skip(self))]
    pub fn is_left_stick(&self) -> bool {
        use bevy::input::gamepad::GamepadAxis as G;
        matches!(*self.0, G::LeftStickX | G::LeftStickY)
    }

    /// Returns `true` if this is a right stick axis.
    #[tracing::instrument(skip(self))]
    pub fn is_right_stick(&self) -> bool {
        use bevy::input::gamepad::GamepadAxis as G;
        matches!(*self.0, G::RightStickX | G::RightStickY)
    }
}

mod emit_gamepadaxis {
    use super::GamepadAxis;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for GamepadAxis {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::input::gamepad::GamepadAxis as G;
            match *self.0 {
                G::Other(n) => quote::quote! { ::bevy::input::gamepad::GamepadAxis::Other(#n) },
                _ => {
                    let v = proc_macro2::Ident::new(
                        self.variant_name(),
                        proc_macro2::Span::call_site(),
                    );
                    quote::quote! { ::bevy::input::gamepad::GamepadAxis::#v }
                }
            }
        }
    }
}

// ── KeyCode ───────────────────────────────────────────────────────────────────

input_enum!(KeyCode, bevy::input::keyboard::KeyCode, []);

#[reflect_methods]
impl KeyCode {
    /// Variant name, e.g. `"KeyA"`, `"Space"`, `"ArrowUp"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> String {
        format!("{:?}", *self.0)
    }

    /// Returns `true` if this is a modifier key (Shift, Control, Alt, Super).
    #[tracing::instrument(skip(self))]
    pub fn is_modifier(&self) -> bool {
        use bevy::input::keyboard::KeyCode as K;
        matches!(
            *self.0,
            K::ShiftLeft
                | K::ShiftRight
                | K::ControlLeft
                | K::ControlRight
                | K::AltLeft
                | K::AltRight
                | K::SuperLeft
                | K::SuperRight
        )
    }

    /// Returns `true` if this is an arrow key.
    #[tracing::instrument(skip(self))]
    pub fn is_arrow(&self) -> bool {
        use bevy::input::keyboard::KeyCode as K;
        matches!(
            *self.0,
            K::ArrowLeft | K::ArrowRight | K::ArrowUp | K::ArrowDown
        )
    }

    /// Returns `true` if this is a function key (F1–F35).
    #[tracing::instrument(skip(self))]
    pub fn is_function_key(&self) -> bool {
        use bevy::input::keyboard::KeyCode as K;
        matches!(
            *self.0,
            K::F1
                | K::F2
                | K::F3
                | K::F4
                | K::F5
                | K::F6
                | K::F7
                | K::F8
                | K::F9
                | K::F10
                | K::F11
                | K::F12
                | K::F13
                | K::F14
                | K::F15
                | K::F16
                | K::F17
                | K::F18
                | K::F19
                | K::F20
                | K::F21
                | K::F22
                | K::F23
                | K::F24
                | K::F25
                | K::F26
                | K::F27
                | K::F28
                | K::F29
                | K::F30
                | K::F31
                | K::F32
                | K::F33
                | K::F34
                | K::F35
        )
    }

    /// Returns `true` if this is a numpad key.
    #[tracing::instrument(skip(self))]
    pub fn is_numpad(&self) -> bool {
        use bevy::input::keyboard::KeyCode as K;
        matches!(
            *self.0,
            K::Numpad0
                | K::Numpad1
                | K::Numpad2
                | K::Numpad3
                | K::Numpad4
                | K::Numpad5
                | K::Numpad6
                | K::Numpad7
                | K::Numpad8
                | K::Numpad9
                | K::NumpadAdd
                | K::NumpadSubtract
                | K::NumpadMultiply
                | K::NumpadDivide
                | K::NumpadDecimal
                | K::NumpadEqual
                | K::NumpadEnter
                | K::NumpadComma
                | K::NumpadParenLeft
                | K::NumpadParenRight
                | K::NumpadStar
                | K::NumpadHash
                | K::NumpadBackspace
                | K::NumpadClear
                | K::NumpadClearEntry
                | K::NumpadMemoryAdd
                | K::NumpadMemorySubtract
                | K::NumpadMemoryStore
                | K::NumpadMemoryRecall
                | K::NumpadMemoryClear
        )
    }
}

mod emit_keycode {
    use super::KeyCode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for KeyCode {
        fn to_code_literal(&self) -> TokenStream {
            let name = self.variant_name();
            let v = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
            quote::quote! { ::bevy::input::keyboard::KeyCode::#v }
        }
    }
}

// ── shadow_elicitation macro ──────────────────────────────────────────────────

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

// ── TouchPhase ────────────────────────────────────────────────────────────────

input_enum!(TouchPhase, bevy::input::touch::TouchPhase, []);

#[reflect_methods]
impl TouchPhase {
    /// Variant name: `"Started"`, `"Moved"`, `"Ended"`, or `"Canceled"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        use bevy::input::touch::TouchPhase as T;
        match *self.0 {
            T::Started => "Started",
            T::Moved => "Moved",
            T::Ended => "Ended",
            T::Canceled => "Canceled",
        }
    }
}

mod emit_touchphase {
    use super::TouchPhase;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for TouchPhase {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::input::touch::TouchPhase::#v }
        }
    }
}

// ── GamepadConnection ─────────────────────────────────────────────────────────

elicit_newtype!(bevy::input::gamepad::GamepadConnection, as GamepadConnection, forward_serde);
elicit_newtype_traits!(
    GamepadConnection,
    bevy::input::gamepad::GamepadConnection,
    []
);

impl From<GamepadConnection> for bevy::input::gamepad::GamepadConnection {
    fn from(v: GamepadConnection) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl GamepadConnection {
    /// Returns `true` if the gamepad is connected.
    #[tracing::instrument(skip(self))]
    pub fn is_connected(&self) -> bool {
        matches!(
            *self.0,
            bevy::input::gamepad::GamepadConnection::Connected { .. }
        )
    }

    /// Returns the gamepad name if connected.
    #[tracing::instrument(skip(self))]
    pub fn name(&self) -> Option<&str> {
        match &*self.0 {
            bevy::input::gamepad::GamepadConnection::Connected { name, .. } => Some(name.as_str()),
            _ => None,
        }
    }
}

mod emit_gamepadconnection {
    use super::GamepadConnection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GamepadConnection {
        fn to_code_literal(&self) -> TokenStream {
            match &*self.0 {
                bevy::input::gamepad::GamepadConnection::Connected {
                    name,
                    vendor_id,
                    product_id,
                } => {
                    let vendor = match vendor_id {
                        Some(id) => quote::quote! { Some(#id) },
                        None => quote::quote! { None },
                    };
                    let product = match product_id {
                        Some(id) => quote::quote! { Some(#id) },
                        None => quote::quote! { None },
                    };
                    quote::quote! {
                        ::bevy::input::gamepad::GamepadConnection::Connected {
                            name: #name.to_string(),
                            vendor_id: #vendor,
                            product_id: #product,
                        }
                    }
                }
                bevy::input::gamepad::GamepadConnection::Disconnected => {
                    quote::quote! { ::bevy::input::gamepad::GamepadConnection::Disconnected }
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for GamepadConnection {}

// ── GamepadInput ──────────────────────────────────────────────────────────────

/// Shadow of [`bevy::input::gamepad::GamepadInput`].
///
/// Discriminates between axis and button gamepad inputs.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum GamepadInput {
    /// An axis input.
    Axis(GamepadAxis),
    /// A button input.
    Button(GamepadButton),
}

impl From<bevy::input::gamepad::GamepadInput> for GamepadInput {
    fn from(v: bevy::input::gamepad::GamepadInput) -> Self {
        use bevy::input::gamepad::GamepadInput as GI;
        match v {
            GI::Axis(a) => GamepadInput::Axis(GamepadAxis(std::sync::Arc::new(a))),
            GI::Button(b) => GamepadInput::Button(GamepadButton(std::sync::Arc::new(b))),
        }
    }
}

impl From<GamepadInput> for bevy::input::gamepad::GamepadInput {
    fn from(v: GamepadInput) -> Self {
        use bevy::input::gamepad::GamepadInput as GI;
        match v {
            GamepadInput::Axis(a) => GI::Axis(*a.0),
            GamepadInput::Button(b) => GI::Button(*b.0),
        }
    }
}

impl GamepadInput {
    /// Returns `true` if this is an axis input.
    pub fn is_axis(&self) -> bool {
        matches!(self, GamepadInput::Axis(_))
    }

    /// Returns `true` if this is a button input.
    pub fn is_button(&self) -> bool {
        matches!(self, GamepadInput::Button(_))
    }
}

mod emit_gamepadinput {
    use super::GamepadInput;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GamepadInput {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                GamepadInput::Axis(a) => {
                    let inner = a.to_code_literal();
                    quote::quote! { ::bevy::input::gamepad::GamepadInput::Axis(#inner) }
                }
                GamepadInput::Button(b) => {
                    let inner = b.to_code_literal();
                    quote::quote! { ::bevy::input::gamepad::GamepadInput::Button(#inner) }
                }
            }
        }
    }
}

shadow_elicitation!(GamepadInput);

// ── GamepadRumbleIntensity ────────────────────────────────────────────────────

/// Shadow of [`bevy::input::gamepad::GamepadRumbleIntensity`].
///
/// Rumble intensity values range from `0.0` to `1.0`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct GamepadRumbleIntensity {
    /// Intensity of the strong (low-frequency) motor. Range: 0.0–1.0.
    pub strong_motor: f32,
    /// Intensity of the weak (high-frequency) motor. Range: 0.0–1.0.
    pub weak_motor: f32,
}

impl From<bevy::input::gamepad::GamepadRumbleIntensity> for GamepadRumbleIntensity {
    fn from(v: bevy::input::gamepad::GamepadRumbleIntensity) -> Self {
        Self {
            strong_motor: v.strong_motor,
            weak_motor: v.weak_motor,
        }
    }
}

impl From<GamepadRumbleIntensity> for bevy::input::gamepad::GamepadRumbleIntensity {
    fn from(v: GamepadRumbleIntensity) -> Self {
        Self {
            strong_motor: v.strong_motor,
            weak_motor: v.weak_motor,
        }
    }
}

mod emit_gamepadrumbleintensity {
    use super::GamepadRumbleIntensity;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GamepadRumbleIntensity {
        fn to_code_literal(&self) -> TokenStream {
            let strong = self.strong_motor;
            let weak = self.weak_motor;
            quote::quote! {
                ::bevy::input::gamepad::GamepadRumbleIntensity {
                    strong_motor: #strong,
                    weak_motor: #weak,
                }
            }
        }
    }
}

shadow_elicitation!(GamepadRumbleIntensity);
