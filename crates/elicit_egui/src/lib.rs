//! `elicit_egui` — dual-mode MCP tools for egui widget creation.
//!
//! Provides tools that operate in two modes:
//!
//! 1. **Runtime mode** — each tool returns a JSON description
//!    that can be rendered by an egui runtime or inspected as JSON.
//! 2. **Emit mode** — each tool's parameters can generate idiomatic
//!    egui Rust code via the elicitation code-emission pipeline.
//!
//! # Tool categories
//!
//! ## Widgets (32 tools)
//!
//! | Tool | Widget | Category |
//! |------|--------|----------|
//! | `widget_label` | Plain text | Display |
//! | `widget_heading` | Heading text | Display |
//! | `widget_monospace` | Monospace text | Display |
//! | `widget_code` | Code with background | Display |
//! | `widget_small` | Small text | Display |
//! | `widget_strong` | Bold text | Display |
//! | `widget_weak` | Faint text | Display |
//! | `widget_colored_label` | Coloured text | Display |
//! | `widget_button` | Clickable button | Interactive |
//! | `widget_small_button` | Compact button | Interactive |
//! | `widget_checkbox` | Boolean toggle | Interactive |
//! | `widget_radio_value` | Radio button (auto-update) | Interactive |
//! | `widget_radio` | Radio button (display only) | Interactive |
//! | `widget_selectable_label` | Toggle label | Interactive |
//! | `widget_toggle_value` | Boolean toggle (simple) | Interactive |
//! | `widget_link` | Clickable text link | Interactive |
//! | `widget_hyperlink` | Web link | Interactive |
//! | `widget_separator` | Divider line | Layout |
//! | `widget_spinner` | Loading spinner | Feedback |
//! | `widget_text_edit_singleline` | Single-line input | Text |
//! | `widget_text_edit_multiline` | Multi-line input | Text |
//! | `widget_code_editor` | Code editor | Text |
//! | `widget_slider` | Numeric slider | Numeric |
//! | `widget_slider_vertical` | Vertical slider | Numeric |
//! | `widget_drag_value` | Drag-to-edit value | Numeric |
//! | `widget_drag_angle` | Drag-to-edit angle (degrees) | Numeric |
//! | `widget_drag_angle_tau` | Drag-to-edit angle (tau) | Numeric |
//! | `widget_progress_bar` | Progress indicator | Feedback |
//! | `widget_color_edit_button_srgba` | sRGBA colour picker | Colour |
//! | `widget_color_edit_button_hsva` | HSVA colour picker | Colour |
//! | `widget_image` | Image display | Media |
//!
//! ## Containers (14 tools)
//!
//! | Tool | Container | Notes |
//! |------|-----------|-------|
//! | `container_window` | Floating window | Title, pos, size, collapsible |
//! | `container_left_panel` | Left side panel | Resizable with min/max width |
//! | `container_right_panel` | Right side panel | Resizable |
//! | `container_top_panel` | Top panel | Resizable |
//! | `container_bottom_panel` | Bottom panel | Resizable |
//! | `container_central_panel` | Central panel | Fills remaining space |
//! | `container_scroll_area` | Scroll region | Vertical/horizontal |
//! | `container_collapsing` | Collapsible section | Header text, default open |
//! | `container_group` | Visual group | Box around content |
//! | `container_frame` | Styled frame | Fill, stroke, margins |
//! | `container_menu_bar` | Menu bar | Top-level menus |
//! | `container_menu` | Menu | Within menu bar |
//! | `container_tooltip` | Tooltip | Hover text |
//! | `container_popup` | Popup area | Context menu, dropdown |
//!
//! ## Layout (11 tools)
//!
//! | Tool | Layout | Notes |
//! |------|--------|-------|
//! | `layout_horizontal` | Left-to-right | Optional alignment |
//! | `layout_vertical` | Top-to-bottom | Optional alignment |
//! | `layout_horizontal_centered` | Centred horizontal | — |
//! | `layout_vertical_centered` | Centred vertical | — |
//! | `layout_horizontal_justified` | Justified horizontal | Items stretch |
//! | `layout_vertical_justified` | Justified vertical | — |
//! | `layout_horizontal_wrapped` | Wrapping horizontal | Next line on overflow |
//! | `layout_columns` | Column layout | N columns |
//! | `layout_grid` | Grid layout | Striped, column count |
//! | `layout_indent` | Indentation | Pixel amount |
//! | `layout_add_space` | Spacing | Pixel amount |
//!
//! ## Styling (9 tools)
//!
//! | Tool | Style | Notes |
//! |------|-------|-------|
//! | `style_spacing` | Global spacing | Item, window, button |
//! | `style_dark_mode` | Dark theme | — |
//! | `style_light_mode` | Light theme | — |
//! | `style_visual` | Colour property | Hyperlink, bg, panel, etc. |
//! | `style_window_rounding` | Window corners | Corner radius |
//! | `style_window_shadow` | Window shadow | Offset, blur, colour |
//! | `style_widget_visuals` | Widget state visuals | Fill, stroke per state |
//! | `style_selection` | Selection highlight | Background, stroke |
//! | `style_text_cursor` | Text cursor | Colour, width |
//!
//! ## Response (21 tools)
//!
//! | Tool | Query | Notes |
//! |------|-------|-------|
//! | `response_clicked` | Was clicked | — |
//! | `response_double_clicked` | Was double-clicked | — |
//! | `response_secondary_clicked` | Was right-clicked | — |
//! | `response_clicked_n` | Clicked N times | Count param |
//! | `response_hovered` | Is hovered | — |
//! | `response_has_focus` | Has focus | — |
//! | `response_gained_focus` | Gained focus | — |
//! | `response_lost_focus` | Lost focus | — |
//! | `response_request_focus` | Request focus | — |
//! | `response_surrender_focus` | Release focus | — |
//! | `response_dragged` | Is dragged | — |
//! | `response_drag_released` | Drag released | — |
//! | `response_drag_delta` | Drag delta | Vec2 result |
//! | `response_changed` | Value changed | — |
//! | `response_rect` | Bounding rect | Rect result |
//! | `response_hover_pos` | Hover position | Optional Vec2 |
//! | `response_show_tooltip` | Show tooltip | Text param |
//! | `response_set_enabled` | Set enabled state | Bool param |
//! | `response_highlight` | Highlight widget | — |
//! | `response_scroll_to_me` | Scroll into view | — |
//! | `response_context_menu` | Context menu | — |
//!
//! # JSON interchange
//!
//! Tools communicate via tagged enums that serialize to compact JSON:
//! [`WidgetJson`], [`ContainerJson`], [`LayoutJson`],
//! [`StyleJson`](style_tools::StyleJson), and
//! [`ResponseQueryJson`](response_tools::ResponseQueryJson).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod container_tools;
mod layout_tools;
mod response_tools;
#[cfg(feature = "runtime")]
pub mod runtime;
pub mod serde_types;
pub mod style_tools;
pub mod widget_tools;

pub use container_tools::{
    BottomPanelParams, CollapsingParams, EmptyContainerParams, FrameParams, LeftPanelParams,
    MenuParams, PopupParams, RightPanelParams, ScrollAreaParams, TopPanelParams, TooltipParams,
    WindowParams,
};

pub use layout_tools::{
    AddSpaceParams, ColumnsParams, EmptyLayoutParams, GridParams, HorizontalParams, IndentParams,
    VerticalParams,
};

pub use response_tools::{
    ClickedNParams, EmptyResponseParams, ResponseInfoJson, ResponseQueryJson, SetEnabledParams,
    ShowTooltipParams,
};

pub use serde_types::{
    ColorJson, ContainerJson, CornerRadiusJson, LayoutAlign, LayoutDirection, LayoutJson,
    MarginJson, RangeJson, RectJson, ResponseJson, StrokeJson, UiNode, Vec2Json, WidgetJson,
};

pub use style_tools::{
    EmptyStyleParams, SelectionParams, SpacingParams, StyleJson, TextCursorParams,
    VisualParams, VisualProperty, WidgetState, WidgetVisualsParams, WindowRoundingParams,
    WindowShadowParams,
};

#[cfg(feature = "runtime")]
pub use runtime::{
    ApplyStyleParams, EguiRuntimeContext, EguiRuntimePlugin, EmptyRuntimeParams,
    FrameOutputJson, RunFrameParams, SessionIdParams, SessionInfo,
};

pub use widget_tools::{
    ButtonParams, CheckboxParams, CodeEditorParams, CodeParams, ColorEditButtonHsvaParams,
    ColorEditButtonSrgbaParams, ColoredLabelParams, DragAngleParams, DragAngleTauParams,
    DragValueParams, EmptyParams, HeadingParams, HyperlinkParams, ImageParams, LabelParams,
    LinkParams, MonospaceParams, ProgressBarParams, RadioParams, RadioValueParams,
    SelectableLabelParams, SimpleTextParams, SliderParams, SliderVerticalParams, SmallButtonParams,
    TextEditMultilineParams, TextEditSinglelineParams, ToggleValueParams,
};
