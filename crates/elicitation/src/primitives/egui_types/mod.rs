//! Elicitation implementations for [`egui`] types.
//!
//! Provides [`Elicitation`](crate::Elicitation) for the egui 0.33 types
//! that can be interactively constructed from an agent — enumeration types via
//! [`Select`](crate::Select).
//!
//! # Enabled by the `egui-types` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["egui-types"] }
//! ```
//!
//! # Supported types
//!
//! | Type | Pattern | Notes |
//! |------|---------|-------|
//! | [`egui::Align`] | Select | Min / Center / Max |
//! | [`egui::CursorIcon`] | Select | 35 cursor icon variants |
//! | [`egui::Direction`] | Select | 4 layout direction variants |
//! | [`egui::FontFamily`] | Select | Monospace / Proportional (unit variants only) |
//! | [`egui::Key`] | Select | 97 keyboard key variants |
//! | [`egui::Order`] | Select | 5 paint layer ordering variants |
//! | [`egui::PointerButton`] | Select | 5 mouse button variants |
//! | [`egui::TextStyle`] | Select | 5 text style variants (unit variants only) |
//! | [`egui::TextWrapMode`] | Select | Extend / Wrap / Truncate |
//! | [`egui::Theme`] | Select | Dark / Light |
//! | [`egui::ThemePreference`] | Select | Dark / Light / System |
//! | [`egui::TouchPhase`] | Select | Start / Move / End / Cancel |
//! | [`egui::UiKind`] | Select | 17 UI region kinds |
//! | [`egui::WidgetType`] | Select | 18 widget type variants |
//! | [`egui::epaint::textures::TextureFilter`] | Select | Nearest / Linear |
//! | [`egui::epaint::textures::TextureWrapMode`] | Select | ClampToEdge / Repeat / MirroredRepeat |

mod align;
mod cursor_icon;
mod direction;
mod font_family;
mod key;
mod order;
mod pointer_button;
mod text_style;
mod text_wrap_mode;
mod texture_filter;
mod texture_wrap_mode;
mod theme;
mod theme_preference;
mod touch_phase;
mod trenchcoats;
mod ui_kind;
mod widget_type;

pub use align::AlignStyle;
pub use cursor_icon::CursorIconStyle;
pub use direction::DirectionStyle;
pub use font_family::FontFamilyStyle;
pub use key::KeyStyle;
pub use order::OrderStyle;
pub use pointer_button::PointerButtonStyle;
pub use text_style::TextStyleStyle;
pub use text_wrap_mode::TextWrapModeStyle;
pub use texture_filter::TextureFilterStyle;
pub use texture_wrap_mode::TextureWrapModeStyle;
pub use theme::ThemeStyle;
pub use theme_preference::ThemePreferenceStyle;
pub use touch_phase::TouchPhaseStyle;
pub use trenchcoats::{
    AlignSelect, CursorIconSelect, DirectionSelect, FontFamilySelect, KeySelect, OrderSelect,
    PointerButtonSelect, TextStyleSelect, TextWrapModeSelect, TextureFilterSelect,
    TextureWrapModeSelect, ThemePreferenceSelect, ThemeSelect, TouchPhaseSelect, UiKindSelect,
    WidgetTypeSelect,
};
pub use ui_kind::UiKindStyle;
pub use widget_type::WidgetTypeStyle;
