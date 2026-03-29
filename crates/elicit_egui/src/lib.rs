//! `elicit_egui` — dual-mode MCP tools for egui widget creation.
//!
//! Provides tools that operate in two modes:
//!
//! 1. **Runtime mode** — each tool returns a [`WidgetJson`] description
//!    that can be rendered by an egui runtime or inspected as JSON.
//! 2. **Emit mode** — each tool's parameters can generate idiomatic
//!    egui Rust code via the elicitation code-emission pipeline.
//!
//! # Supported widgets
//!
//! | Tool | Widget | Category |
//! |------|--------|----------|
//! | [`widget_label`] | Plain text | Display |
//! | [`widget_heading`] | Heading text | Display |
//! | [`widget_monospace`] | Monospace text | Display |
//! | [`widget_code`] | Code with background | Display |
//! | [`widget_small`] | Small text | Display |
//! | [`widget_strong`] | Bold text | Display |
//! | [`widget_weak`] | Faint text | Display |
//! | [`widget_colored_label`] | Coloured text | Display |
//! | [`widget_button`] | Clickable button | Interactive |
//! | [`widget_small_button`] | Compact button | Interactive |
//! | [`widget_checkbox`] | Boolean toggle | Interactive |
//! | [`widget_radio_value`] | Radio button (auto-update) | Interactive |
//! | [`widget_radio`] | Radio button (display only) | Interactive |
//! | [`widget_selectable_label`] | Toggle label | Interactive |
//! | [`widget_toggle_value`] | Boolean toggle (simple) | Interactive |
//! | [`widget_link`] | Clickable text link | Interactive |
//! | [`widget_hyperlink`] | Web link | Interactive |
//! | [`widget_separator`] | Divider line | Layout |
//! | [`widget_spinner`] | Loading spinner | Feedback |
//! | [`widget_text_edit_singleline`] | Single-line input | Text |
//! | [`widget_text_edit_multiline`] | Multi-line input | Text |
//! | [`widget_code_editor`] | Code editor | Text |
//! | [`widget_slider`] | Numeric slider | Numeric |
//! | [`widget_slider_vertical`] | Vertical slider | Numeric |
//! | [`widget_drag_value`] | Drag-to-edit value | Numeric |
//! | [`widget_drag_angle`] | Drag-to-edit angle (degrees) | Numeric |
//! | [`widget_drag_angle_tau`] | Drag-to-edit angle (tau) | Numeric |
//! | [`widget_progress_bar`] | Progress indicator | Feedback |
//! | [`widget_color_edit_button_srgba`] | sRGBA colour picker | Colour |
//! | [`widget_color_edit_button_hsva`] | HSVA colour picker | Colour |
//! | [`widget_image`] | Image display | Media |
//!
//! # JSON interchange
//!
//! All tools communicate via [`WidgetJson`], a tagged enum that serializes
//! to compact JSON. Helper types ([`ColorJson`], [`StrokeJson`], [`RangeJson`])
//! provide ergonomic access to egui primitives.
//!
//! [`WidgetJson`]: serde_types::WidgetJson
//! [`ColorJson`]: serde_types::ColorJson
//! [`StrokeJson`]: serde_types::StrokeJson
//! [`RangeJson`]: serde_types::RangeJson

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod serde_types;
pub mod widget_tools;

pub use serde_types::{
    ColorJson, CornerRadiusJson, MarginJson, RangeJson, RectJson, ResponseJson, StrokeJson,
    Vec2Json, WidgetJson,
};

pub use widget_tools::{
    ButtonParams, CheckboxParams, CodeEditorParams, CodeParams, ColorEditButtonHsvaParams,
    ColorEditButtonSrgbaParams, ColoredLabelParams, DragAngleParams, DragAngleTauParams,
    DragValueParams, EmptyParams, HeadingParams, HyperlinkParams, ImageParams, LabelParams,
    LinkParams, MonospaceParams, ProgressBarParams, RadioParams, RadioValueParams,
    SelectableLabelParams, SimpleTextParams, SliderParams, SliderVerticalParams, SmallButtonParams,
    TextEditMultilineParams, TextEditSinglelineParams, ToggleValueParams,
};
