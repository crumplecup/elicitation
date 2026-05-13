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
//! ## Styling (29 tools)
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
//! | `egui_set_fonts` | Font families | Proportional, monospace |
//! | `egui_override_text_style` | Text style override | Family + size per style |
//! | `egui_set_text_valign` | Text vertical align | Top, center, bottom |
//! | `egui_set_interaction` | Interaction settings | Click time, drag threshold |
//! | `egui_set_animation_time` | Animation duration | Transition timing |
//! | `egui_set_debug_options` | Debug rendering | Widget hits, hover debug |
//! | `egui_set_hyperlink_color` | Hyperlink colour | — |
//! | `egui_set_faint_bg_color` | Faint background | Alternating rows |
//! | `egui_set_extreme_bg_color` | Extreme background | Text input fields |
//! | `egui_set_code_bg_color` | Code background | Monospace background |
//! | `egui_set_warn_fg_color` | Warning foreground | — |
//! | `egui_set_error_fg_color` | Error foreground | — |
//! | `egui_set_widget_stroke` | Widget state stroke | Per-state border |
//! | `egui_set_window_stroke` | Window border stroke | Width + colour |
//! | `egui_set_menu_margin` | Menu margin | Left, right, top, bottom |
//! | `egui_set_button_padding` | Button padding | Horizontal, vertical |
//! | `egui_set_indent` | Indentation | Pixel distance |
//! | `egui_set_scroll_bar_width` | Scroll bar | Width, handle, margins |
//! | `egui_set_resize_grip_size` | Resize grip | Corner size |
//! | `egui_set_text_cursor_width` | Cursor blink | Width, blink timing |
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
//! ## Menus & Popups (13 tools)
//!
//! | Tool | Action | Notes |
//! |------|--------|-------|
//! | `egui_context_menu` | Right-click menu | Region ID |
//! | `egui_context_menu_item` | Menu item | Label + shortcut |
//! | `egui_context_menu_separator` | Menu separator | — |
//! | `egui_popup` | Popup at position | ID, position, content |
//! | `egui_popup_below_widget` | Popup below widget | Anchor ID |
//! | `egui_close_popup` | Close popup | — |
//! | `egui_tooltip` | Hover tooltip | Widget ID, text |
//! | `egui_tooltip_rich` | Rich tooltip | Custom UI content |
//! | `egui_tooltip_at_pointer` | Pointer tooltip | Text at cursor |
//! | `egui_modal` | Modal dialog | Title, content, buttons |
//! | `egui_confirm_dialog` | Confirm dialog | Yes/no |
//! | `egui_alert_dialog` | Alert dialog | OK button |
//! | `egui_notification` | Toast message | Text, duration, position |
//!
//! ## Input (14 tools)
//!
//! | Tool | Query | Notes |
//! |------|-------|-------|
//! | `egui_key_pressed` | Key pressed | This frame |
//! | `egui_key_released` | Key released | This frame |
//! | `egui_key_down` | Key held down | Current state |
//! | `egui_modifiers` | Modifier keys | Ctrl, Shift, Alt, Cmd |
//! | `egui_pointer_pos` | Pointer position | — |
//! | `egui_pointer_button_pressed` | Mouse pressed | Button name |
//! | `egui_pointer_button_released` | Mouse released | Button name |
//! | `egui_pointer_delta` | Pointer delta | This frame |
//! | `egui_scroll_delta` | Scroll delta | — |
//! | `egui_clipboard_get` | Get clipboard | Text |
//! | `egui_clipboard_set` | Set clipboard | Text |
//! | `egui_request_focus` | Request focus | Widget ID |
//! | `egui_surrender_focus` | Release focus | Widget ID |
//! | `egui_has_focus` | Check focus | Widget ID |
//!
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
pub mod fragment_tools;
mod input_tools;
mod layout_tools;
mod menu_tools;
mod response_tools;
#[cfg(feature = "runtime")]
pub mod runtime;
pub mod serde_types;
pub mod style_tools;
pub mod widget_tools;

pub mod accesskit_bridge;
pub mod egui_accesskit_convert;
pub mod winit_plugin;

pub use accesskit_bridge::{EguiBackend, bounds_to_size, render_tree};
pub use egui_accesskit_convert::{tree_update_to_ui_node, ui_node_to_tree_update};
pub use winit_plugin::EguiWinitPlugin;

pub use container_tools::{
    BottomPanelParams, CollapsingParams, EmptyContainerParams, FrameParams, LeftPanelParams,
    MenuParams, PopupParams, RightPanelParams, ScrollAreaParams, TooltipParams, TopPanelParams,
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
    AnimationTimeParams, ButtonPaddingParams, ColorOverrideParams, DebugOptionsParams,
    EmptyStyleParams, FontFamily, InteractionParams, MenuMarginParams, OverrideTextStyleParams,
    ResizeGripSizeParams, ScrollBarWidthParams, SelectionParams, SetFontsParams,
    SetTextValignParams, SpacingParams, StyleIndentParams, StyleJson, TextCursorBlinkParams,
    TextCursorParams, TextStyleName, TextValign, VisualParams, VisualProperty, WidgetState,
    WidgetStrokeParams, WidgetVisualsParams, WindowRoundingParams, WindowShadowParams,
    WindowStrokeParams,
};

#[cfg(feature = "runtime")]
pub use runtime::{
    ApplyStyleParams, EguiRuntimeContext, EguiRuntimePlugin, EmptyRuntimeParams, FrameOutputJson,
    RunFrameParams, SessionIdParams, SessionInfo,
};

pub use widget_tools::{
    ButtonParams, CheckboxParams, CodeEditorParams, CodeParams, ColorEditButtonHsvaParams,
    ColorEditButtonSrgbaParams, ColoredLabelParams, DragAngleParams, DragAngleTauParams,
    DragValueParams, EmptyParams, HeadingParams, HyperlinkParams, ImageParams, LabelParams,
    LinkParams, MonospaceParams, ProgressBarParams, RadioParams, RadioValueParams,
    SelectableLabelParams, SimpleTextParams, SliderParams, SliderVerticalParams, SmallButtonParams,
    TextEditMultilineParams, TextEditSinglelineParams, ToggleValueParams,
};

pub use input_tools::{
    ClipboardSetParams, EmptyInputParams, FocusParams, InputActionJson, KeyParams, ModifiersJson,
    PointerButtonParams,
};

pub use menu_tools::{
    AlertDialogParams, ConfirmDialogParams, ContextMenuItemParams, ContextMenuParams,
    EmptyMenuParams, MenuActionJson, MenuPopupParams, MenuTooltipParams, ModalParams,
    NotificationParams, PopupBelowWidgetParams, TooltipAtPointerParams, TooltipRichParams,
};

pub use fragment_tools::{
    AppStateParams, FormFieldDef, FormParams, MessageEnumParams, MessageVariantDef,
    NativeAppParams, SettingsFieldDef, SettingsPanelParams, SettingsSectionDef,
    SidebarLayoutParams, StateFieldDef, TabDef, TabPanelParams, TableColumnDef, TableParams,
    ToolbarButtonDef, ToolbarParams, WebAppParams,
};
