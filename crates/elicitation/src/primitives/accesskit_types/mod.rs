//! Elicitation implementations for [`accesskit`] types.
//!
//! Provides [`Elicitation`](crate::Elicitation) for the accesskit 0.24 types
//! that can be interactively constructed from an agent — enumeration types via
//! [`Select`](crate::Select).
//!
//! # Enabled by the `accesskit` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["accesskit"] }
//! ```
//!
//! # Supported types
//!
//! | Type | Pattern | Notes |
//! |------|---------|-------|
//! | [`accesskit::Role`] | Select | 182 accessibility role variants |
//! | [`accesskit::Action`] | Select | 22 unit action variants |
//! | [`accesskit::Invalid`] | Select | 3 invalidity states |
//! | [`accesskit::Toggled`] | Select | False / True / Mixed |
//! | [`accesskit::Orientation`] | Select | Horizontal / Vertical |
//! | [`accesskit::TextDirection`] | Select | 4 text direction variants |
//! | [`accesskit::SortDirection`] | Select | Ascending / Descending / Other |
//! | [`accesskit::AriaCurrent`] | Select | 7 aria-current values |
//! | [`accesskit::AutoComplete`] | Select | Inline / List / Both |
//! | [`accesskit::Live`] | Select | Off / Polite / Assertive |
//! | [`accesskit::HasPopup`] | Select | 5 popup type variants |
//! | [`accesskit::ListStyle`] | Select | 6 list item style variants |
//! | [`accesskit::TextAlign`] | Select | Left / Right / Center / Justify |
//! | [`accesskit::VerticalOffset`] | Select | Subscript / Superscript |
//! | [`accesskit::TextDecorationStyle`] | Select | 5 decoration style variants |
//! | [`accesskit::ScrollUnit`] | Select | Item / Page |
//! | [`accesskit::ScrollHint`] | Select | 6 scroll hint positions |

mod action;
mod aria_current;
mod auto_complete;
mod has_popup;
mod invalid;
mod list_style;
mod live;
mod orientation;
mod role;
mod scroll_hint;
mod scroll_unit;
mod sort_direction;
mod text_align;
mod text_decoration_style;
mod text_direction;
mod toggled;
mod vertical_offset;

pub use action::ActionStyle;
pub use aria_current::AriaCurrentStyle;
pub use auto_complete::AutoCompleteStyle;
pub use has_popup::HasPopupStyle;
pub use invalid::InvalidStyle;
pub use list_style::ListStyleStyle;
pub use live::LiveStyle;
pub use orientation::OrientationStyle;
pub use role::RoleStyle;
pub use scroll_hint::ScrollHintStyle;
pub use scroll_unit::ScrollUnitStyle;
pub use sort_direction::SortDirectionStyle;
pub use text_align::TextAlignStyle;
pub use text_decoration_style::TextDecorationStyleStyle;
pub use text_direction::TextDirectionStyle;
pub use toggled::ToggledStyle;
pub use vertical_offset::VerticalOffsetStyle;
