//! Elicitation implementations for [`ratatui`] types.
//!
//! Provides [`Elicitation`](crate::Elicitation) for ratatui 0.30 types
//! that can be interactively constructed from an agent — enumeration types via
//! [`Select`](crate::Select) and composite types via Survey.
//!
//! # Enabled by the `ratatui` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["ratatui"] }
//! ```
//!
//! # Supported types
//!
//! ## Select enums
//!
//! | Type | Pattern | Notes |
//! |------|---------|-------|
//! | [`ratatui::style::Color`] | Select | 17 named + Reset (RGB/Indexed via text) |
//! | [`ratatui::layout::Alignment`] | Select | Left / Center / Right |
//! | [`ratatui::layout::Direction`] | Select | Vertical / Horizontal |
//! | [`ratatui::widgets::BorderType`] | Select | Plain / Rounded / Double / Thick |
//!
//! ## Select trenchcoat wrappers
//!
//! | Wrapper | Inner type | Notes |
//! |---------|-----------|-------|
//! | [`BordersSelect`] | [`ratatui::widgets::Borders`] | Bitflags — common presets only |
//! | [`ScrollbarOrientationSelect`] | [`ratatui::widgets::ScrollbarOrientation`] | 4 orientations |
//!
//! ## Composite structs
//!
//! | Type | Pattern | Notes |
//! |------|---------|-------|
//! | [`RatatuiStyle`] | Survey | fg, bg, modifier fields |
//! | [`RatatuiPadding`] | Survey | left, right, top, bottom |
//! | [`RatatuiMargin`] | Survey | horizontal, vertical |

// ── Select enum modules ──────────────────────────────────────────────
mod alignment;
mod border_type;
mod color;
mod direction;
mod trenchcoats;

// ── Composite struct modules ─────────────────────────────────────────
mod margin;
mod padding;
mod style;

pub use alignment::AlignmentStyle;
pub use border_type::BorderTypeStyle;
pub use color::ColorStyle;
pub use direction::RatatuiDirectionStyle;
pub use margin::{RatatuiMargin, RatatuiMarginStyle};
pub use padding::{RatatuiPadding, RatatuiPaddingStyle};
pub use style::{RatatuiStyle, RatatuiStyleStyle};
pub use trenchcoats::{
    AlignmentSelect, BorderTypeSelect, BordersSelect, ColorSelect, RatatuiDirectionSelect,
    ScrollbarOrientationSelect,
};
