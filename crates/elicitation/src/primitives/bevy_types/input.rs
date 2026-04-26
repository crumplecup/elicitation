//! Bevy input type elicitation trenchcoats.
//!
//! Covers:
//! - [`BevyButtonState`] — select-trenchcoat for `bevy::input::ButtonState`
//! - [`BevyTouchPhase`] — select-trenchcoat for `bevy::input::touch::TouchPhase`
//! - [`BevyKeyCode`] — local mirror enum for `bevy::input::keyboard::KeyCode` (186 unit variants)
//! - [`BevyMouseButton`] — local mirror enum for `bevy::input::mouse::MouseButton`
//! - [`BevyGamepadButton`] — local mirror enum for `bevy::input::gamepad::GamepadButton`
//! - [`BevyGamepadAxis`] — local mirror enum for `bevy::input::gamepad::GamepadAxis`

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Shared helper macro for unit-enum input selects ──────────────────────────

macro_rules! impl_input_select {
    (
        type       = $ty:ty,
        style      = $style:ident,
        prompt     = $prompt:literal,
        kani_var   = $kani_var:literal,
        variants   = [ $($v:expr),+ $(,)? ]
    ) => {
        impl Prompt for $ty {
            fn prompt() -> Option<&'static str> { Some($prompt) }
        }

        impl Select for $ty {
            fn options() -> Vec<Self> { vec![$($v),+] }

            fn labels() -> Vec<String> {
                Self::options()
                    .iter()
                    .map(|v| serde_json::to_string(v).unwrap().trim_matches('"').to_string())
                    .collect()
            }

            fn from_label(label: &str) -> Option<Self> {
                serde_json::from_str(&format!("\"{}\"", label)).ok()
            }
        }

        crate::default_style!($ty => $style);

        impl Elicitation for $ty {
            type Style = $style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                let params = mcp::select_params(
                    Self::prompt().unwrap_or("Choose a value:"),
                    &Self::labels(),
                );
                let result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                            .with_arguments(params),
                    )
                    .await?;
                let value = mcp::extract_value(result)?;
                let label = mcp::parse_string(value)?;
                Self::from_label(&label).ok_or_else(|| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid {}: {label}", stringify!($ty)
                    )))
                })
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_select_wrapper(stringify!($ty), $kani_var)
            }
            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_select_wrapper(stringify!($ty), $kani_var)
            }
            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_select_wrapper(stringify!($ty), $kani_var)
            }
        }

        impl ElicitIntrospect for $ty {
            fn pattern() -> ElicitationPattern { ElicitationPattern::Select }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: stringify!($ty),
                    description: None,
                    details: PatternDetails::Select {
                        variants: Self::labels()
                            .into_iter()
                            .map(|label| VariantMetadata { label, fields: vec![] })
                            .collect(),
                    },
                }
            }
        }
    };
}

// ── ButtonState ───────────────────────────────────────────────────────────────

impl_input_select!(
    type     = bevy::input::ButtonState,
    style    = BevyButtonStateStyle,
    prompt   = "Choose button state:",
    kani_var = "bevy::input::ButtonState::Pressed",
    variants = [Self::Pressed, Self::Released]
);

crate::select_trenchcoat!(bevy::input::ButtonState, as BevyButtonState, serde);
crate::select_trenchcoat_traits!(BevyButtonState, bevy::input::ButtonState, [copy, eq, hash]);

// ── TouchPhase ────────────────────────────────────────────────────────────────

impl_input_select!(
    type     = bevy::input::touch::TouchPhase,
    style    = BevyTouchPhaseStyle,
    prompt   = "Choose touch phase:",
    kani_var = "bevy::input::touch::TouchPhase::Started",
    variants = [
        Self::Started,
        Self::Moved,
        Self::Ended,
        Self::Canceled,
    ]
);

crate::select_trenchcoat!(bevy::input::touch::TouchPhase, as BevyTouchPhase, serde);
crate::select_trenchcoat_traits!(
    BevyTouchPhase,
    bevy::input::touch::TouchPhase,
    [copy, eq, hash]
);

// ── BevyKeyCode ───────────────────────────────────────────────────────────────
//
// bevy::input::keyboard::KeyCode has an Unidentified(NativeKeyCode) data
// variant which cannot be elicited.  We mirror all 186 unit variants and
// provide From conversions (Unidentified is excluded from the select options).

/// Mirror of [`bevy::input::keyboard::KeyCode`] covering all 186 unit variants.
///
/// The upstream `Unidentified(NativeKeyCode)` variant is excluded from selection.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum BevyKeyCode {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Meta,
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}

impl From<BevyKeyCode> for bevy::input::keyboard::KeyCode {
    fn from(k: BevyKeyCode) -> Self {
        match k {
            BevyKeyCode::Backquote => Self::Backquote,
            BevyKeyCode::Backslash => Self::Backslash,
            BevyKeyCode::BracketLeft => Self::BracketLeft,
            BevyKeyCode::BracketRight => Self::BracketRight,
            BevyKeyCode::Comma => Self::Comma,
            BevyKeyCode::Digit0 => Self::Digit0,
            BevyKeyCode::Digit1 => Self::Digit1,
            BevyKeyCode::Digit2 => Self::Digit2,
            BevyKeyCode::Digit3 => Self::Digit3,
            BevyKeyCode::Digit4 => Self::Digit4,
            BevyKeyCode::Digit5 => Self::Digit5,
            BevyKeyCode::Digit6 => Self::Digit6,
            BevyKeyCode::Digit7 => Self::Digit7,
            BevyKeyCode::Digit8 => Self::Digit8,
            BevyKeyCode::Digit9 => Self::Digit9,
            BevyKeyCode::Equal => Self::Equal,
            BevyKeyCode::IntlBackslash => Self::IntlBackslash,
            BevyKeyCode::IntlRo => Self::IntlRo,
            BevyKeyCode::IntlYen => Self::IntlYen,
            BevyKeyCode::KeyA => Self::KeyA,
            BevyKeyCode::KeyB => Self::KeyB,
            BevyKeyCode::KeyC => Self::KeyC,
            BevyKeyCode::KeyD => Self::KeyD,
            BevyKeyCode::KeyE => Self::KeyE,
            BevyKeyCode::KeyF => Self::KeyF,
            BevyKeyCode::KeyG => Self::KeyG,
            BevyKeyCode::KeyH => Self::KeyH,
            BevyKeyCode::KeyI => Self::KeyI,
            BevyKeyCode::KeyJ => Self::KeyJ,
            BevyKeyCode::KeyK => Self::KeyK,
            BevyKeyCode::KeyL => Self::KeyL,
            BevyKeyCode::KeyM => Self::KeyM,
            BevyKeyCode::KeyN => Self::KeyN,
            BevyKeyCode::KeyO => Self::KeyO,
            BevyKeyCode::KeyP => Self::KeyP,
            BevyKeyCode::KeyQ => Self::KeyQ,
            BevyKeyCode::KeyR => Self::KeyR,
            BevyKeyCode::KeyS => Self::KeyS,
            BevyKeyCode::KeyT => Self::KeyT,
            BevyKeyCode::KeyU => Self::KeyU,
            BevyKeyCode::KeyV => Self::KeyV,
            BevyKeyCode::KeyW => Self::KeyW,
            BevyKeyCode::KeyX => Self::KeyX,
            BevyKeyCode::KeyY => Self::KeyY,
            BevyKeyCode::KeyZ => Self::KeyZ,
            BevyKeyCode::Minus => Self::Minus,
            BevyKeyCode::Period => Self::Period,
            BevyKeyCode::Quote => Self::Quote,
            BevyKeyCode::Semicolon => Self::Semicolon,
            BevyKeyCode::Slash => Self::Slash,
            BevyKeyCode::AltLeft => Self::AltLeft,
            BevyKeyCode::AltRight => Self::AltRight,
            BevyKeyCode::Backspace => Self::Backspace,
            BevyKeyCode::CapsLock => Self::CapsLock,
            BevyKeyCode::ContextMenu => Self::ContextMenu,
            BevyKeyCode::ControlLeft => Self::ControlLeft,
            BevyKeyCode::ControlRight => Self::ControlRight,
            BevyKeyCode::Enter => Self::Enter,
            BevyKeyCode::SuperLeft => Self::SuperLeft,
            BevyKeyCode::SuperRight => Self::SuperRight,
            BevyKeyCode::ShiftLeft => Self::ShiftLeft,
            BevyKeyCode::ShiftRight => Self::ShiftRight,
            BevyKeyCode::Space => Self::Space,
            BevyKeyCode::Tab => Self::Tab,
            BevyKeyCode::Convert => Self::Convert,
            BevyKeyCode::KanaMode => Self::KanaMode,
            BevyKeyCode::Lang1 => Self::Lang1,
            BevyKeyCode::Lang2 => Self::Lang2,
            BevyKeyCode::Lang3 => Self::Lang3,
            BevyKeyCode::Lang4 => Self::Lang4,
            BevyKeyCode::Lang5 => Self::Lang5,
            BevyKeyCode::NonConvert => Self::NonConvert,
            BevyKeyCode::Delete => Self::Delete,
            BevyKeyCode::End => Self::End,
            BevyKeyCode::Help => Self::Help,
            BevyKeyCode::Home => Self::Home,
            BevyKeyCode::Insert => Self::Insert,
            BevyKeyCode::PageDown => Self::PageDown,
            BevyKeyCode::PageUp => Self::PageUp,
            BevyKeyCode::ArrowDown => Self::ArrowDown,
            BevyKeyCode::ArrowLeft => Self::ArrowLeft,
            BevyKeyCode::ArrowRight => Self::ArrowRight,
            BevyKeyCode::ArrowUp => Self::ArrowUp,
            BevyKeyCode::NumLock => Self::NumLock,
            BevyKeyCode::Numpad0 => Self::Numpad0,
            BevyKeyCode::Numpad1 => Self::Numpad1,
            BevyKeyCode::Numpad2 => Self::Numpad2,
            BevyKeyCode::Numpad3 => Self::Numpad3,
            BevyKeyCode::Numpad4 => Self::Numpad4,
            BevyKeyCode::Numpad5 => Self::Numpad5,
            BevyKeyCode::Numpad6 => Self::Numpad6,
            BevyKeyCode::Numpad7 => Self::Numpad7,
            BevyKeyCode::Numpad8 => Self::Numpad8,
            BevyKeyCode::Numpad9 => Self::Numpad9,
            BevyKeyCode::NumpadAdd => Self::NumpadAdd,
            BevyKeyCode::NumpadBackspace => Self::NumpadBackspace,
            BevyKeyCode::NumpadClear => Self::NumpadClear,
            BevyKeyCode::NumpadClearEntry => Self::NumpadClearEntry,
            BevyKeyCode::NumpadComma => Self::NumpadComma,
            BevyKeyCode::NumpadDecimal => Self::NumpadDecimal,
            BevyKeyCode::NumpadDivide => Self::NumpadDivide,
            BevyKeyCode::NumpadEnter => Self::NumpadEnter,
            BevyKeyCode::NumpadEqual => Self::NumpadEqual,
            BevyKeyCode::NumpadHash => Self::NumpadHash,
            BevyKeyCode::NumpadMemoryAdd => Self::NumpadMemoryAdd,
            BevyKeyCode::NumpadMemoryClear => Self::NumpadMemoryClear,
            BevyKeyCode::NumpadMemoryRecall => Self::NumpadMemoryRecall,
            BevyKeyCode::NumpadMemoryStore => Self::NumpadMemoryStore,
            BevyKeyCode::NumpadMemorySubtract => Self::NumpadMemorySubtract,
            BevyKeyCode::NumpadMultiply => Self::NumpadMultiply,
            BevyKeyCode::NumpadParenLeft => Self::NumpadParenLeft,
            BevyKeyCode::NumpadParenRight => Self::NumpadParenRight,
            BevyKeyCode::NumpadStar => Self::NumpadStar,
            BevyKeyCode::NumpadSubtract => Self::NumpadSubtract,
            BevyKeyCode::Escape => Self::Escape,
            BevyKeyCode::Fn => Self::Fn,
            BevyKeyCode::FnLock => Self::FnLock,
            BevyKeyCode::PrintScreen => Self::PrintScreen,
            BevyKeyCode::ScrollLock => Self::ScrollLock,
            BevyKeyCode::Pause => Self::Pause,
            BevyKeyCode::BrowserBack => Self::BrowserBack,
            BevyKeyCode::BrowserFavorites => Self::BrowserFavorites,
            BevyKeyCode::BrowserForward => Self::BrowserForward,
            BevyKeyCode::BrowserHome => Self::BrowserHome,
            BevyKeyCode::BrowserRefresh => Self::BrowserRefresh,
            BevyKeyCode::BrowserSearch => Self::BrowserSearch,
            BevyKeyCode::BrowserStop => Self::BrowserStop,
            BevyKeyCode::Eject => Self::Eject,
            BevyKeyCode::LaunchApp1 => Self::LaunchApp1,
            BevyKeyCode::LaunchApp2 => Self::LaunchApp2,
            BevyKeyCode::LaunchMail => Self::LaunchMail,
            BevyKeyCode::MediaPlayPause => Self::MediaPlayPause,
            BevyKeyCode::MediaSelect => Self::MediaSelect,
            BevyKeyCode::MediaStop => Self::MediaStop,
            BevyKeyCode::MediaTrackNext => Self::MediaTrackNext,
            BevyKeyCode::MediaTrackPrevious => Self::MediaTrackPrevious,
            BevyKeyCode::Power => Self::Power,
            BevyKeyCode::Sleep => Self::Sleep,
            BevyKeyCode::AudioVolumeDown => Self::AudioVolumeDown,
            BevyKeyCode::AudioVolumeMute => Self::AudioVolumeMute,
            BevyKeyCode::AudioVolumeUp => Self::AudioVolumeUp,
            BevyKeyCode::WakeUp => Self::WakeUp,
            BevyKeyCode::Meta => Self::Meta,
            BevyKeyCode::Hyper => Self::Hyper,
            BevyKeyCode::Turbo => Self::Turbo,
            BevyKeyCode::Abort => Self::Abort,
            BevyKeyCode::Resume => Self::Resume,
            BevyKeyCode::Suspend => Self::Suspend,
            BevyKeyCode::Again => Self::Again,
            BevyKeyCode::Copy => Self::Copy,
            BevyKeyCode::Cut => Self::Cut,
            BevyKeyCode::Find => Self::Find,
            BevyKeyCode::Open => Self::Open,
            BevyKeyCode::Paste => Self::Paste,
            BevyKeyCode::Props => Self::Props,
            BevyKeyCode::Select => Self::Select,
            BevyKeyCode::Undo => Self::Undo,
            BevyKeyCode::Hiragana => Self::Hiragana,
            BevyKeyCode::Katakana => Self::Katakana,
            BevyKeyCode::F1 => Self::F1,
            BevyKeyCode::F2 => Self::F2,
            BevyKeyCode::F3 => Self::F3,
            BevyKeyCode::F4 => Self::F4,
            BevyKeyCode::F5 => Self::F5,
            BevyKeyCode::F6 => Self::F6,
            BevyKeyCode::F7 => Self::F7,
            BevyKeyCode::F8 => Self::F8,
            BevyKeyCode::F9 => Self::F9,
            BevyKeyCode::F10 => Self::F10,
            BevyKeyCode::F11 => Self::F11,
            BevyKeyCode::F12 => Self::F12,
            BevyKeyCode::F13 => Self::F13,
            BevyKeyCode::F14 => Self::F14,
            BevyKeyCode::F15 => Self::F15,
            BevyKeyCode::F16 => Self::F16,
            BevyKeyCode::F17 => Self::F17,
            BevyKeyCode::F18 => Self::F18,
            BevyKeyCode::F19 => Self::F19,
            BevyKeyCode::F20 => Self::F20,
            BevyKeyCode::F21 => Self::F21,
            BevyKeyCode::F22 => Self::F22,
            BevyKeyCode::F23 => Self::F23,
            BevyKeyCode::F24 => Self::F24,
            BevyKeyCode::F25 => Self::F25,
            BevyKeyCode::F26 => Self::F26,
            BevyKeyCode::F27 => Self::F27,
            BevyKeyCode::F28 => Self::F28,
            BevyKeyCode::F29 => Self::F29,
            BevyKeyCode::F30 => Self::F30,
            BevyKeyCode::F31 => Self::F31,
            BevyKeyCode::F32 => Self::F32,
            BevyKeyCode::F33 => Self::F33,
            BevyKeyCode::F34 => Self::F34,
            BevyKeyCode::F35 => Self::F35,
        }
    }
}

impl Prompt for BevyKeyCode {
    fn prompt() -> Option<&'static str> {
        Some("Choose a key code:")
    }
}

impl Select for BevyKeyCode {
    fn options() -> Vec<Self> {
        vec![
            Self::Backquote,
            Self::Backslash,
            Self::BracketLeft,
            Self::BracketRight,
            Self::Comma,
            Self::Digit0,
            Self::Digit1,
            Self::Digit2,
            Self::Digit3,
            Self::Digit4,
            Self::Digit5,
            Self::Digit6,
            Self::Digit7,
            Self::Digit8,
            Self::Digit9,
            Self::Equal,
            Self::IntlBackslash,
            Self::IntlRo,
            Self::IntlYen,
            Self::KeyA,
            Self::KeyB,
            Self::KeyC,
            Self::KeyD,
            Self::KeyE,
            Self::KeyF,
            Self::KeyG,
            Self::KeyH,
            Self::KeyI,
            Self::KeyJ,
            Self::KeyK,
            Self::KeyL,
            Self::KeyM,
            Self::KeyN,
            Self::KeyO,
            Self::KeyP,
            Self::KeyQ,
            Self::KeyR,
            Self::KeyS,
            Self::KeyT,
            Self::KeyU,
            Self::KeyV,
            Self::KeyW,
            Self::KeyX,
            Self::KeyY,
            Self::KeyZ,
            Self::Minus,
            Self::Period,
            Self::Quote,
            Self::Semicolon,
            Self::Slash,
            Self::AltLeft,
            Self::AltRight,
            Self::Backspace,
            Self::CapsLock,
            Self::ContextMenu,
            Self::ControlLeft,
            Self::ControlRight,
            Self::Enter,
            Self::SuperLeft,
            Self::SuperRight,
            Self::ShiftLeft,
            Self::ShiftRight,
            Self::Space,
            Self::Tab,
            Self::Convert,
            Self::KanaMode,
            Self::Lang1,
            Self::Lang2,
            Self::Lang3,
            Self::Lang4,
            Self::Lang5,
            Self::NonConvert,
            Self::Delete,
            Self::End,
            Self::Help,
            Self::Home,
            Self::Insert,
            Self::PageDown,
            Self::PageUp,
            Self::ArrowDown,
            Self::ArrowLeft,
            Self::ArrowRight,
            Self::ArrowUp,
            Self::NumLock,
            Self::Numpad0,
            Self::Numpad1,
            Self::Numpad2,
            Self::Numpad3,
            Self::Numpad4,
            Self::Numpad5,
            Self::Numpad6,
            Self::Numpad7,
            Self::Numpad8,
            Self::Numpad9,
            Self::NumpadAdd,
            Self::NumpadBackspace,
            Self::NumpadClear,
            Self::NumpadClearEntry,
            Self::NumpadComma,
            Self::NumpadDecimal,
            Self::NumpadDivide,
            Self::NumpadEnter,
            Self::NumpadEqual,
            Self::NumpadHash,
            Self::NumpadMemoryAdd,
            Self::NumpadMemoryClear,
            Self::NumpadMemoryRecall,
            Self::NumpadMemoryStore,
            Self::NumpadMemorySubtract,
            Self::NumpadMultiply,
            Self::NumpadParenLeft,
            Self::NumpadParenRight,
            Self::NumpadStar,
            Self::NumpadSubtract,
            Self::Escape,
            Self::Fn,
            Self::FnLock,
            Self::PrintScreen,
            Self::ScrollLock,
            Self::Pause,
            Self::BrowserBack,
            Self::BrowserFavorites,
            Self::BrowserForward,
            Self::BrowserHome,
            Self::BrowserRefresh,
            Self::BrowserSearch,
            Self::BrowserStop,
            Self::Eject,
            Self::LaunchApp1,
            Self::LaunchApp2,
            Self::LaunchMail,
            Self::MediaPlayPause,
            Self::MediaSelect,
            Self::MediaStop,
            Self::MediaTrackNext,
            Self::MediaTrackPrevious,
            Self::Power,
            Self::Sleep,
            Self::AudioVolumeDown,
            Self::AudioVolumeMute,
            Self::AudioVolumeUp,
            Self::WakeUp,
            Self::Meta,
            Self::Hyper,
            Self::Turbo,
            Self::Abort,
            Self::Resume,
            Self::Suspend,
            Self::Again,
            Self::Copy,
            Self::Cut,
            Self::Find,
            Self::Open,
            Self::Paste,
            Self::Props,
            Self::Select,
            Self::Undo,
            Self::Hiragana,
            Self::Katakana,
            Self::F1,
            Self::F2,
            Self::F3,
            Self::F4,
            Self::F5,
            Self::F6,
            Self::F7,
            Self::F8,
            Self::F9,
            Self::F10,
            Self::F11,
            Self::F12,
            Self::F13,
            Self::F14,
            Self::F15,
            Self::F16,
            Self::F17,
            Self::F18,
            Self::F19,
            Self::F20,
            Self::F21,
            Self::F22,
            Self::F23,
            Self::F24,
            Self::F25,
            Self::F26,
            Self::F27,
            Self::F28,
            Self::F29,
            Self::F30,
            Self::F31,
            Self::F32,
            Self::F33,
            Self::F34,
            Self::F35,
        ]
    }

    fn labels() -> Vec<String> {
        Self::options()
            .iter()
            .map(|v| {
                serde_json::to_string(v)
                    .unwrap()
                    .trim_matches('"')
                    .to_string()
            })
            .collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(BevyKeyCode => BevyKeyCodeStyle);

impl Elicitation for BevyKeyCode {
    type Style = BevyKeyCodeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose a key code:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid BevyKeyCode: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("BevyKeyCode", "BevyKeyCode::Space")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyKeyCode",
            "BevyKeyCode::Space",
        )
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyKeyCode",
            "BevyKeyCode::Space",
        )
    }
}

impl ElicitIntrospect for BevyKeyCode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyKeyCode",
            description: Some("Keyboard key code (186 named keys; Unidentified excluded)"),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyKeyCode {
    fn prompt_tree() -> crate::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose a key code:").to_string(),
            type_name: "BevyKeyCode".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyKeyCode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let label = serde_json::to_string(self).unwrap_or_default();
        let label = label.trim_matches('"');
        let variant = proc_macro2::Ident::new(label, proc_macro2::Span::call_site());
        quote::quote! { bevy::input::keyboard::KeyCode::#variant }
    }
}

// ── BevyMouseButton ───────────────────────────────────────────────────────────
//
// bevy::input::mouse::MouseButton has an Other(u16) data variant.
// We mirror all named variants plus Other.

/// Mirror of [`bevy::input::mouse::MouseButton`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum BevyMouseButton {
    /// Primary (left) button.
    Left,
    /// Secondary (right) button.
    Right,
    /// Middle (wheel-click) button.
    Middle,
    /// Back/side button.
    Back,
    /// Forward/side button.
    Forward,
    /// Another mouse button identified by index.
    Other(u16),
}

impl From<BevyMouseButton> for bevy::input::mouse::MouseButton {
    fn from(b: BevyMouseButton) -> Self {
        match b {
            BevyMouseButton::Left => Self::Left,
            BevyMouseButton::Right => Self::Right,
            BevyMouseButton::Middle => Self::Middle,
            BevyMouseButton::Back => Self::Back,
            BevyMouseButton::Forward => Self::Forward,
            BevyMouseButton::Other(n) => Self::Other(n),
        }
    }
}

impl From<bevy::input::mouse::MouseButton> for BevyMouseButton {
    fn from(b: bevy::input::mouse::MouseButton) -> Self {
        match b {
            bevy::input::mouse::MouseButton::Left => Self::Left,
            bevy::input::mouse::MouseButton::Right => Self::Right,
            bevy::input::mouse::MouseButton::Middle => Self::Middle,
            bevy::input::mouse::MouseButton::Back => Self::Back,
            bevy::input::mouse::MouseButton::Forward => Self::Forward,
            bevy::input::mouse::MouseButton::Other(n) => Self::Other(n),
        }
    }
}

impl Prompt for BevyMouseButton {
    fn prompt() -> Option<&'static str> {
        Some("Choose a mouse button:")
    }
}

impl Select for BevyMouseButton {
    fn options() -> Vec<Self> {
        vec![
            Self::Left,
            Self::Right,
            Self::Middle,
            Self::Back,
            Self::Forward,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Left".into(),
            "Right".into(),
            "Middle".into(),
            "Back".into(),
            "Forward".into(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Left" => Some(Self::Left),
            "Right" => Some(Self::Right),
            "Middle" => Some(Self::Middle),
            "Back" => Some(Self::Back),
            "Forward" => Some(Self::Forward),
            _ => None,
        }
    }
}

crate::default_style!(BevyMouseButton => BevyMouseButtonStyle);

impl Elicitation for BevyMouseButton {
    type Style = BevyMouseButtonStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose a mouse button:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid BevyMouseButton: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "BevyMouseButton",
            "BevyMouseButton::Left",
        )
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyMouseButton",
            "BevyMouseButton::Left",
        )
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyMouseButton",
            "BevyMouseButton::Left",
        )
    }
}

impl ElicitIntrospect for BevyMouseButton {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyMouseButton",
            description: Some("Mouse button (named buttons; Other(u16) excluded from select)"),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyMouseButton {
    fn prompt_tree() -> crate::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a mouse button:")
                .to_string(),
            type_name: "BevyMouseButton".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyMouseButton {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let label = serde_json::to_string(self).unwrap_or_default();
        let label = label.trim_matches('"');
        let variant = proc_macro2::Ident::new(label, proc_macro2::Span::call_site());
        quote::quote! { bevy::input::mouse::MouseButton::#variant }
    }
}

// ── BevyGamepadButton ─────────────────────────────────────────────────────────
//
// bevy::input::gamepad::GamepadButton has an Other(u8) data variant.
// We mirror the 19 standard named variants.

/// Mirror of [`bevy::input::gamepad::GamepadButton`] (standard variants).
///
/// The `Other(u8)` variant is excluded from selection.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum BevyGamepadButton {
    South,
    East,
    North,
    West,
    C,
    Z,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    Select,
    Start,
    Mode,
    LeftThumb,
    RightThumb,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    /// Non-standard button by index.
    Other(u8),
}

impl From<BevyGamepadButton> for bevy::input::gamepad::GamepadButton {
    fn from(b: BevyGamepadButton) -> Self {
        match b {
            BevyGamepadButton::South => Self::South,
            BevyGamepadButton::East => Self::East,
            BevyGamepadButton::North => Self::North,
            BevyGamepadButton::West => Self::West,
            BevyGamepadButton::C => Self::C,
            BevyGamepadButton::Z => Self::Z,
            BevyGamepadButton::LeftTrigger => Self::LeftTrigger,
            BevyGamepadButton::LeftTrigger2 => Self::LeftTrigger2,
            BevyGamepadButton::RightTrigger => Self::RightTrigger,
            BevyGamepadButton::RightTrigger2 => Self::RightTrigger2,
            BevyGamepadButton::Select => Self::Select,
            BevyGamepadButton::Start => Self::Start,
            BevyGamepadButton::Mode => Self::Mode,
            BevyGamepadButton::LeftThumb => Self::LeftThumb,
            BevyGamepadButton::RightThumb => Self::RightThumb,
            BevyGamepadButton::DPadUp => Self::DPadUp,
            BevyGamepadButton::DPadDown => Self::DPadDown,
            BevyGamepadButton::DPadLeft => Self::DPadLeft,
            BevyGamepadButton::DPadRight => Self::DPadRight,
            BevyGamepadButton::Other(n) => Self::Other(n),
        }
    }
}

impl From<bevy::input::gamepad::GamepadButton> for BevyGamepadButton {
    fn from(b: bevy::input::gamepad::GamepadButton) -> Self {
        match b {
            bevy::input::gamepad::GamepadButton::South => Self::South,
            bevy::input::gamepad::GamepadButton::East => Self::East,
            bevy::input::gamepad::GamepadButton::North => Self::North,
            bevy::input::gamepad::GamepadButton::West => Self::West,
            bevy::input::gamepad::GamepadButton::C => Self::C,
            bevy::input::gamepad::GamepadButton::Z => Self::Z,
            bevy::input::gamepad::GamepadButton::LeftTrigger => Self::LeftTrigger,
            bevy::input::gamepad::GamepadButton::LeftTrigger2 => Self::LeftTrigger2,
            bevy::input::gamepad::GamepadButton::RightTrigger => Self::RightTrigger,
            bevy::input::gamepad::GamepadButton::RightTrigger2 => Self::RightTrigger2,
            bevy::input::gamepad::GamepadButton::Select => Self::Select,
            bevy::input::gamepad::GamepadButton::Start => Self::Start,
            bevy::input::gamepad::GamepadButton::Mode => Self::Mode,
            bevy::input::gamepad::GamepadButton::LeftThumb => Self::LeftThumb,
            bevy::input::gamepad::GamepadButton::RightThumb => Self::RightThumb,
            bevy::input::gamepad::GamepadButton::DPadUp => Self::DPadUp,
            bevy::input::gamepad::GamepadButton::DPadDown => Self::DPadDown,
            bevy::input::gamepad::GamepadButton::DPadLeft => Self::DPadLeft,
            bevy::input::gamepad::GamepadButton::DPadRight => Self::DPadRight,
            bevy::input::gamepad::GamepadButton::Other(n) => Self::Other(n),
        }
    }
}

impl Prompt for BevyGamepadButton {
    fn prompt() -> Option<&'static str> {
        Some("Choose a gamepad button:")
    }
}

impl Select for BevyGamepadButton {
    fn options() -> Vec<Self> {
        vec![
            Self::South,
            Self::East,
            Self::North,
            Self::West,
            Self::C,
            Self::Z,
            Self::LeftTrigger,
            Self::LeftTrigger2,
            Self::RightTrigger,
            Self::RightTrigger2,
            Self::Select,
            Self::Start,
            Self::Mode,
            Self::LeftThumb,
            Self::RightThumb,
            Self::DPadUp,
            Self::DPadDown,
            Self::DPadLeft,
            Self::DPadRight,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "South".into(),
            "East".into(),
            "North".into(),
            "West".into(),
            "C".into(),
            "Z".into(),
            "LeftTrigger".into(),
            "LeftTrigger2".into(),
            "RightTrigger".into(),
            "RightTrigger2".into(),
            "Select".into(),
            "Start".into(),
            "Mode".into(),
            "LeftThumb".into(),
            "RightThumb".into(),
            "DPadUp".into(),
            "DPadDown".into(),
            "DPadLeft".into(),
            "DPadRight".into(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "South" => Some(Self::South),
            "East" => Some(Self::East),
            "North" => Some(Self::North),
            "West" => Some(Self::West),
            "C" => Some(Self::C),
            "Z" => Some(Self::Z),
            "LeftTrigger" => Some(Self::LeftTrigger),
            "LeftTrigger2" => Some(Self::LeftTrigger2),
            "RightTrigger" => Some(Self::RightTrigger),
            "RightTrigger2" => Some(Self::RightTrigger2),
            "Select" => Some(Self::Select),
            "Start" => Some(Self::Start),
            "Mode" => Some(Self::Mode),
            "LeftThumb" => Some(Self::LeftThumb),
            "RightThumb" => Some(Self::RightThumb),
            "DPadUp" => Some(Self::DPadUp),
            "DPadDown" => Some(Self::DPadDown),
            "DPadLeft" => Some(Self::DPadLeft),
            "DPadRight" => Some(Self::DPadRight),
            _ => None,
        }
    }
}

crate::default_style!(BevyGamepadButton => BevyGamepadButtonStyle);

impl Elicitation for BevyGamepadButton {
    type Style = BevyGamepadButtonStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose a gamepad button:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid BevyGamepadButton: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "BevyGamepadButton",
            "BevyGamepadButton::South",
        )
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyGamepadButton",
            "BevyGamepadButton::South",
        )
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyGamepadButton",
            "BevyGamepadButton::South",
        )
    }
}

impl ElicitIntrospect for BevyGamepadButton {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyGamepadButton",
            description: Some("Gamepad button (19 standard; Other(u8) excluded from select)"),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyGamepadButton {
    fn prompt_tree() -> crate::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a gamepad button:")
                .to_string(),
            type_name: "BevyGamepadButton".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyGamepadButton {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let label = serde_json::to_string(self).unwrap_or_default();
        let label = label.trim_matches('"');
        let variant = proc_macro2::Ident::new(label, proc_macro2::Span::call_site());
        quote::quote! { bevy::input::gamepad::GamepadButton::#variant }
    }
}

// ── BevyGamepadAxis ───────────────────────────────────────────────────────────
//
// bevy::input::gamepad::GamepadAxis has an Other(u8) data variant.
// We mirror the 6 standard axes.

/// Mirror of [`bevy::input::gamepad::GamepadAxis`] (standard axes).
///
/// The `Other(u8)` variant is excluded from selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum BevyGamepadAxis {
    /// Horizontal left stick.
    LeftStickX,
    /// Vertical left stick.
    LeftStickY,
    /// Left throttle / Z-axis.
    LeftZ,
    /// Horizontal right stick.
    RightStickX,
    /// Vertical right stick.
    RightStickY,
    /// Right throttle / Z-axis.
    RightZ,
    /// Non-standard axis by index.
    Other(u8),
}

impl From<BevyGamepadAxis> for bevy::input::gamepad::GamepadAxis {
    fn from(a: BevyGamepadAxis) -> Self {
        match a {
            BevyGamepadAxis::LeftStickX => Self::LeftStickX,
            BevyGamepadAxis::LeftStickY => Self::LeftStickY,
            BevyGamepadAxis::LeftZ => Self::LeftZ,
            BevyGamepadAxis::RightStickX => Self::RightStickX,
            BevyGamepadAxis::RightStickY => Self::RightStickY,
            BevyGamepadAxis::RightZ => Self::RightZ,
            BevyGamepadAxis::Other(n) => Self::Other(n),
        }
    }
}

impl From<bevy::input::gamepad::GamepadAxis> for BevyGamepadAxis {
    fn from(a: bevy::input::gamepad::GamepadAxis) -> Self {
        match a {
            bevy::input::gamepad::GamepadAxis::LeftStickX => Self::LeftStickX,
            bevy::input::gamepad::GamepadAxis::LeftStickY => Self::LeftStickY,
            bevy::input::gamepad::GamepadAxis::LeftZ => Self::LeftZ,
            bevy::input::gamepad::GamepadAxis::RightStickX => Self::RightStickX,
            bevy::input::gamepad::GamepadAxis::RightStickY => Self::RightStickY,
            bevy::input::gamepad::GamepadAxis::RightZ => Self::RightZ,
            bevy::input::gamepad::GamepadAxis::Other(n) => Self::Other(n),
        }
    }
}

impl Prompt for BevyGamepadAxis {
    fn prompt() -> Option<&'static str> {
        Some("Choose a gamepad axis:")
    }
}

impl Select for BevyGamepadAxis {
    fn options() -> Vec<Self> {
        vec![
            Self::LeftStickX,
            Self::LeftStickY,
            Self::LeftZ,
            Self::RightStickX,
            Self::RightStickY,
            Self::RightZ,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "LeftStickX".into(),
            "LeftStickY".into(),
            "LeftZ".into(),
            "RightStickX".into(),
            "RightStickY".into(),
            "RightZ".into(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "LeftStickX" => Some(Self::LeftStickX),
            "LeftStickY" => Some(Self::LeftStickY),
            "LeftZ" => Some(Self::LeftZ),
            "RightStickX" => Some(Self::RightStickX),
            "RightStickY" => Some(Self::RightStickY),
            "RightZ" => Some(Self::RightZ),
            _ => None,
        }
    }
}

crate::default_style!(BevyGamepadAxis => BevyGamepadAxisStyle);

impl Elicitation for BevyGamepadAxis {
    type Style = BevyGamepadAxisStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose a gamepad axis:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid BevyGamepadAxis: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "BevyGamepadAxis",
            "BevyGamepadAxis::LeftStickX",
        )
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyGamepadAxis",
            "BevyGamepadAxis::LeftStickX",
        )
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyGamepadAxis",
            "BevyGamepadAxis::LeftStickX",
        )
    }
}

impl ElicitIntrospect for BevyGamepadAxis {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyGamepadAxis",
            description: Some("Gamepad axis (6 standard; Other(u8) excluded from select)"),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyGamepadAxis {
    fn prompt_tree() -> crate::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a gamepad axis:")
                .to_string(),
            type_name: "BevyGamepadAxis".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyGamepadAxis {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let label = serde_json::to_string(self).unwrap_or_default();
        let label = label.trim_matches('"');
        let variant = proc_macro2::Ident::new(label, proc_macro2::Span::call_site());
        quote::quote! { bevy::input::gamepad::GamepadAxis::#variant }
    }
}
