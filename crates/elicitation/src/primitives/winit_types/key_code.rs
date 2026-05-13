//! [`winit::keyboard::KeyCode`] elicitation.
//!
//! All 161 unit variants of the physical key-code enum are supported.
//! Variants that carry data (none in KeyCode) are excluded by construction.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use winit::keyboard::KeyCode;

impl Prompt for KeyCode {
    fn prompt() -> Option<&'static str> {
        Some("Choose physical key code:")
    }
}

impl Select for KeyCode {
    #[allow(clippy::too_many_lines)]
    fn options() -> Vec<Self> {
        vec![
            // Writing keys
            KeyCode::Backquote,
            KeyCode::Backslash,
            KeyCode::BracketLeft,
            KeyCode::BracketRight,
            KeyCode::Comma,
            KeyCode::Digit0,
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
            KeyCode::Equal,
            KeyCode::IntlBackslash,
            KeyCode::IntlRo,
            KeyCode::IntlYen,
            KeyCode::KeyA,
            KeyCode::KeyB,
            KeyCode::KeyC,
            KeyCode::KeyD,
            KeyCode::KeyE,
            KeyCode::KeyF,
            KeyCode::KeyG,
            KeyCode::KeyH,
            KeyCode::KeyI,
            KeyCode::KeyJ,
            KeyCode::KeyK,
            KeyCode::KeyL,
            KeyCode::KeyM,
            KeyCode::KeyN,
            KeyCode::KeyO,
            KeyCode::KeyP,
            KeyCode::KeyQ,
            KeyCode::KeyR,
            KeyCode::KeyS,
            KeyCode::KeyT,
            KeyCode::KeyU,
            KeyCode::KeyV,
            KeyCode::KeyW,
            KeyCode::KeyX,
            KeyCode::KeyY,
            KeyCode::KeyZ,
            KeyCode::Minus,
            KeyCode::Period,
            KeyCode::Quote,
            KeyCode::Semicolon,
            KeyCode::Slash,
            // Functional keys
            KeyCode::AltLeft,
            KeyCode::AltRight,
            KeyCode::Backspace,
            KeyCode::CapsLock,
            KeyCode::ContextMenu,
            KeyCode::ControlLeft,
            KeyCode::ControlRight,
            KeyCode::Enter,
            KeyCode::SuperLeft,
            KeyCode::SuperRight,
            KeyCode::ShiftLeft,
            KeyCode::ShiftRight,
            KeyCode::Space,
            KeyCode::Tab,
            KeyCode::Convert,
            KeyCode::KanaMode,
            KeyCode::Lang1,
            KeyCode::Lang2,
            KeyCode::Lang3,
            KeyCode::Lang4,
            KeyCode::Lang5,
            KeyCode::NonConvert,
            // Navigation keys
            KeyCode::Delete,
            KeyCode::End,
            KeyCode::Help,
            KeyCode::Home,
            KeyCode::Insert,
            KeyCode::PageDown,
            KeyCode::PageUp,
            // Arrow keys
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::ArrowUp,
            // Numpad
            KeyCode::NumLock,
            KeyCode::Numpad0,
            KeyCode::Numpad1,
            KeyCode::Numpad2,
            KeyCode::Numpad3,
            KeyCode::Numpad4,
            KeyCode::Numpad5,
            KeyCode::Numpad6,
            KeyCode::Numpad7,
            KeyCode::Numpad8,
            KeyCode::Numpad9,
            KeyCode::NumpadAdd,
            KeyCode::NumpadBackspace,
            KeyCode::NumpadClear,
            KeyCode::NumpadClearEntry,
            KeyCode::NumpadComma,
            KeyCode::NumpadDecimal,
            KeyCode::NumpadDivide,
            KeyCode::NumpadEnter,
            KeyCode::NumpadEqual,
            KeyCode::NumpadHash,
            KeyCode::NumpadMemoryAdd,
            KeyCode::NumpadMemoryClear,
            KeyCode::NumpadMemoryRecall,
            KeyCode::NumpadMemoryStore,
            KeyCode::NumpadMemorySubtract,
            KeyCode::NumpadMultiply,
            KeyCode::NumpadParenLeft,
            KeyCode::NumpadParenRight,
            KeyCode::NumpadStar,
            KeyCode::NumpadSubtract,
            // Control keys
            KeyCode::Escape,
            KeyCode::Fn,
            KeyCode::FnLock,
            KeyCode::PrintScreen,
            KeyCode::ScrollLock,
            KeyCode::Pause,
            // Browser keys
            KeyCode::BrowserBack,
            KeyCode::BrowserFavorites,
            KeyCode::BrowserForward,
            KeyCode::BrowserHome,
            KeyCode::BrowserRefresh,
            KeyCode::BrowserSearch,
            KeyCode::BrowserStop,
            // Media keys
            KeyCode::Eject,
            KeyCode::LaunchApp1,
            KeyCode::LaunchApp2,
            KeyCode::LaunchMail,
            KeyCode::MediaPlayPause,
            KeyCode::MediaSelect,
            KeyCode::MediaStop,
            KeyCode::MediaTrackNext,
            KeyCode::MediaTrackPrevious,
            KeyCode::Power,
            KeyCode::Sleep,
            KeyCode::AudioVolumeDown,
            KeyCode::AudioVolumeMute,
            KeyCode::AudioVolumeUp,
            KeyCode::WakeUp,
            // Legacy / modifier aliases
            KeyCode::Meta,
            KeyCode::Hyper,
            KeyCode::Turbo,
            KeyCode::Abort,
            KeyCode::Resume,
            KeyCode::Suspend,
            // Editing
            KeyCode::Again,
            KeyCode::Copy,
            KeyCode::Cut,
            KeyCode::Find,
            KeyCode::Open,
            KeyCode::Paste,
            KeyCode::Props,
            KeyCode::Select,
            KeyCode::Undo,
            // Japanese
            KeyCode::Hiragana,
            KeyCode::Katakana,
            // Function keys
            KeyCode::F1,
            KeyCode::F2,
            KeyCode::F3,
            KeyCode::F4,
            KeyCode::F5,
            KeyCode::F6,
            KeyCode::F7,
            KeyCode::F8,
            KeyCode::F9,
            KeyCode::F10,
            KeyCode::F11,
            KeyCode::F12,
            KeyCode::F13,
            KeyCode::F14,
            KeyCode::F15,
            KeyCode::F16,
            KeyCode::F17,
            KeyCode::F18,
            KeyCode::F19,
            KeyCode::F20,
            KeyCode::F21,
            KeyCode::F22,
            KeyCode::F23,
            KeyCode::F24,
            KeyCode::F25,
            KeyCode::F26,
            KeyCode::F27,
            KeyCode::F28,
            KeyCode::F29,
            KeyCode::F30,
            KeyCode::F31,
            KeyCode::F32,
            KeyCode::F33,
            KeyCode::F34,
            KeyCode::F35,
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

crate::default_style!(winit::keyboard::KeyCode => KeyCodeStyle);

impl Elicitation for KeyCode {
    type Style = KeyCodeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting winit::keyboard::KeyCode");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose key code:"),
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
                "Invalid winit::keyboard::KeyCode: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("winit::keyboard::KeyCode", "Space")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "winit::keyboard::KeyCode",
            "Space",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "winit::keyboard::KeyCode",
            "Space",
        )
    }
}

impl ElicitIntrospect for KeyCode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "winit::keyboard::KeyCode",
            description: Self::prompt(),
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
