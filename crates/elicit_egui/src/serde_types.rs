//! JSON-serializable types for egui widget interchange.
//!
//! These types form the wire format between MCP tool calls and Rust code
//! generation. Each widget tool returns a [`WidgetJson`] variant; the emit
//! layer converts it to idiomatic egui code.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Primitive JSON wrappers
// ---------------------------------------------------------------------------

/// RGBA colour in sRGB space (0–255 per channel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ColorJson {
    /// Red channel (0–255).
    pub r: u8,
    /// Green channel (0–255).
    pub g: u8,
    /// Blue channel (0–255).
    pub b: u8,
    /// Alpha channel (0–255, 255 = opaque).
    #[serde(default = "default_alpha")]
    pub a: u8,
}

fn default_alpha() -> u8 {
    255
}

impl ColorJson {
    /// Opaque colour from RGB.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Colour with explicit alpha.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<egui::Color32> for ColorJson {
    fn from(c: egui::Color32) -> Self {
        Self {
            r: c.r(),
            g: c.g(),
            b: c.b(),
            a: c.a(),
        }
    }
}

impl From<ColorJson> for egui::Color32 {
    fn from(c: ColorJson) -> Self {
        egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
    }
}

/// Line style: width + colour.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct StrokeJson {
    /// Line width in logical pixels.
    pub width: f32,
    /// Line colour.
    pub color: ColorJson,
}

impl From<egui::Stroke> for StrokeJson {
    fn from(s: egui::Stroke) -> Self {
        Self {
            width: s.width,
            color: s.color.into(),
        }
    }
}

impl From<StrokeJson> for egui::Stroke {
    fn from(s: StrokeJson) -> Self {
        egui::Stroke::new(s.width, egui::Color32::from(s.color))
    }
}

/// Numeric range (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RangeJson {
    /// Minimum value (inclusive).
    pub min: f64,
    /// Maximum value (inclusive).
    pub max: f64,
}

/// 2D point in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Vec2Json {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
}

impl From<egui::Vec2> for Vec2Json {
    fn from(v: egui::Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Vec2Json> for egui::Vec2 {
    fn from(v: Vec2Json) -> Self {
        egui::Vec2::new(v.x, v.y)
    }
}

/// Axis-aligned rectangle in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RectJson {
    /// Minimum x (left edge).
    pub min_x: f32,
    /// Minimum y (top edge).
    pub min_y: f32,
    /// Maximum x (right edge).
    pub max_x: f32,
    /// Maximum y (bottom edge).
    pub max_y: f32,
}

impl From<egui::Rect> for RectJson {
    fn from(r: egui::Rect) -> Self {
        Self {
            min_x: r.min.x,
            min_y: r.min.y,
            max_x: r.max.x,
            max_y: r.max.y,
        }
    }
}

impl From<RectJson> for egui::Rect {
    fn from(r: RectJson) -> Self {
        egui::Rect::from_min_max(
            egui::pos2(r.min_x, r.min_y),
            egui::pos2(r.max_x, r.max_y),
        )
    }
}

/// Corner radii for rounded rectangles.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CornerRadiusJson {
    /// North-west corner radius.
    pub nw: u8,
    /// North-east corner radius.
    pub ne: u8,
    /// South-west corner radius.
    pub sw: u8,
    /// South-east corner radius.
    pub se: u8,
}

impl From<egui::CornerRadius> for CornerRadiusJson {
    fn from(r: egui::CornerRadius) -> Self {
        Self {
            nw: r.nw,
            ne: r.ne,
            sw: r.sw,
            se: r.se,
        }
    }
}

impl From<CornerRadiusJson> for egui::CornerRadius {
    fn from(r: CornerRadiusJson) -> Self {
        egui::CornerRadius {
            nw: r.nw,
            ne: r.ne,
            sw: r.sw,
            se: r.se,
        }
    }
}

/// Box margins in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MarginJson {
    /// Left margin.
    pub left: f32,
    /// Right margin.
    pub right: f32,
    /// Top margin.
    pub top: f32,
    /// Bottom margin.
    pub bottom: f32,
}

impl From<egui::Margin> for MarginJson {
    fn from(m: egui::Margin) -> Self {
        Self {
            left: m.left as f32,
            right: m.right as f32,
            top: m.top as f32,
            bottom: m.bottom as f32,
        }
    }
}

impl From<MarginJson> for egui::Margin {
    fn from(m: MarginJson) -> Self {
        egui::Margin {
            left: m.left as i8,
            right: m.right as i8,
            top: m.top as i8,
            bottom: m.bottom as i8,
        }
    }
}

// ---------------------------------------------------------------------------
// Widget JSON — the main interchange enum
// ---------------------------------------------------------------------------

/// Serializable description of an egui widget.
///
/// Each variant captures the parameters needed to recreate the widget in
/// either runtime (immediate mode) or code-emission mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum WidgetJson {
    /// Plain text label.
    Label {
        /// Display text.
        text: String,
        /// Whether to wrap long text.
        #[serde(default)]
        wrap: bool,
        /// Optional text colour override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color: Option<ColorJson>,
    },

    /// Clickable button.
    Button {
        /// Button label text.
        text: String,
        /// Whether to wrap long text.
        #[serde(default)]
        wrap: bool,
        /// Optional fill colour.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fill: Option<ColorJson>,
        /// Optional border stroke.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        stroke: Option<StrokeJson>,
        /// Whether the button appears "selected" (toggled on).
        #[serde(default)]
        selected: bool,
        /// Whether to draw a frame around the button.
        #[serde(default = "default_true")]
        frame: bool,
        /// Minimum widget size.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_size: Option<Vec2Json>,
    },

    /// Small button (less padding).
    SmallButton {
        /// Button label text.
        text: String,
    },

    /// Boolean checkbox.
    Checkbox {
        /// Label text beside the checkbox.
        text: String,
        /// Current checked state.
        checked: bool,
    },

    /// Radio button (one of a group).
    RadioValue {
        /// Label text.
        text: String,
        /// Whether this radio is currently selected.
        selected: bool,
    },

    /// Selectable label (click to toggle selection).
    SelectableLabel {
        /// Label text.
        text: String,
        /// Whether currently selected.
        selected: bool,
    },

    /// Hyperlink-styled text that opens a URL.
    Hyperlink {
        /// Display text.
        text: String,
        /// Target URL.
        url: String,
    },

    /// Heading text (large, bold).
    Heading {
        /// Heading text.
        text: String,
    },

    /// Monospace text.
    Monospace {
        /// Text content.
        text: String,
    },

    /// Code text with background.
    Code {
        /// Code content.
        text: String,
    },

    /// Small text.
    Small {
        /// Text content.
        text: String,
    },

    /// Strong (bold) text.
    Strong {
        /// Text content.
        text: String,
    },

    /// Weak (faint) text.
    Weak {
        /// Text content.
        text: String,
    },

    /// Coloured text label.
    ColoredLabel {
        /// Text content.
        text: String,
        /// Text colour.
        color: ColorJson,
    },

    /// Horizontal or vertical separator line.
    Separator,

    /// Loading spinner animation.
    Spinner,

    /// Single-line text input.
    TextEditSingleline {
        /// Current text value.
        text: String,
        /// Placeholder hint text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hint: Option<String>,
        /// Whether the text input is interactive.
        #[serde(default = "default_true")]
        interactive: bool,
    },

    /// Multi-line text input.
    TextEditMultiline {
        /// Current text value.
        text: String,
        /// Placeholder hint text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hint: Option<String>,
        /// Whether the text input is interactive.
        #[serde(default = "default_true")]
        interactive: bool,
    },

    /// Code editor (monospace, tab support).
    CodeEditor {
        /// Current code text.
        text: String,
        /// Programming language hint for potential syntax highlighting.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        language: Option<String>,
        /// Whether the editor is interactive.
        #[serde(default = "default_true")]
        interactive: bool,
    },

    /// Numeric slider.
    Slider {
        /// Current value.
        value: f64,
        /// Value range (inclusive).
        range: RangeJson,
        /// Step size between values.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        /// Label text beside the slider.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        /// Prefix string prepended to the value display.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prefix: Option<String>,
        /// Suffix string appended to the value display.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        suffix: Option<String>,
        /// Whether to use logarithmic scale.
        #[serde(default)]
        logarithmic: bool,
        /// Whether to clamp value to range.
        #[serde(default = "default_true")]
        clamping: bool,
        /// Whether to show the current value.
        #[serde(default = "default_true")]
        show_value: bool,
    },

    /// Drag-to-edit numeric value.
    DragValue {
        /// Current value.
        value: f64,
        /// Value range (inclusive).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        range: Option<RangeJson>,
        /// Drag speed multiplier.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        speed: Option<f64>,
        /// Label prefix.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        prefix: Option<String>,
        /// Label suffix.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        suffix: Option<String>,
        /// Minimum number of decimal places to display.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_decimals: Option<usize>,
        /// Maximum number of decimal places to display.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_decimals: Option<usize>,
    },

    /// Progress bar (0.0–1.0).
    ProgressBar {
        /// Progress fraction (0.0 = empty, 1.0 = full).
        progress: f32,
        /// Optional overlay text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        /// Whether to animate the progress bar.
        #[serde(default)]
        animate: bool,
        /// Optional fill colour override.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fill: Option<ColorJson>,
        /// Optional desired width in logical pixels.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        desired_width: Option<f32>,
        /// Optional corner rounding.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        corner_radius: Option<CornerRadiusJson>,
    },

    /// Image display.
    Image {
        /// Image URI (e.g. `bytes://name` or `file://path`).
        uri: String,
        /// Optional size constraint.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        size: Option<Vec2Json>,
        /// Whether to maintain aspect ratio when resizing.
        #[serde(default = "default_true")]
        maintain_aspect_ratio: bool,
        /// Optional tint colour applied over the image.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tint: Option<ColorJson>,
        /// Optional corner rounding.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        corner_radius: Option<CornerRadiusJson>,
    },

    /// Clickable text link (you handle the click action).
    Link {
        /// Link display text.
        text: String,
    },

    /// Boolean toggle (auto-toggles on click, simpler than checkbox).
    ToggleValue {
        /// Label text.
        text: String,
        /// Current toggle state.
        selected: bool,
    },

    /// Simple radio button (displays state, does not auto-update).
    Radio {
        /// Label text.
        text: String,
        /// Whether this radio is currently selected.
        selected: bool,
    },

    /// Drag-to-edit angle in degrees (stored as radians).
    DragAngle {
        /// Angle value in radians.
        radians: f64,
    },

    /// Drag-to-edit angle as fraction of tau (2π).
    DragAngleTau {
        /// Angle value in radians.
        radians: f64,
    },

    /// sRGBA colour picker button.
    ColorEditButtonSrgba {
        /// Current colour value.
        color: ColorJson,
        /// Whether to show the alpha channel.
        #[serde(default = "default_true")]
        alpha: bool,
    },

    /// HSVA colour picker button.
    ColorEditButtonHsva {
        /// Current colour as RGBA (converted to HSVA internally).
        color: ColorJson,
        /// Whether to show the alpha channel.
        #[serde(default = "default_true")]
        alpha: bool,
    },

    /// Numeric slider with vertical orientation.
    SliderVertical {
        /// Current value.
        value: f64,
        /// Value range (inclusive).
        range: RangeJson,
        /// Step size between values.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        step: Option<f64>,
        /// Label text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        /// Suffix string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        suffix: Option<String>,
        /// Whether to use logarithmic scale.
        #[serde(default)]
        logarithmic: bool,
    },
}

fn default_true() -> bool {
    true
}

// ---------------------------------------------------------------------------
// Container JSON — layout containers
// ---------------------------------------------------------------------------

/// Serializable description of an egui layout container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum ContainerJson {
    /// Floating window.
    Window {
        /// Window title.
        title: String,
        /// Initial position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default_pos: Option<Vec2Json>,
        /// Initial size.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default_size: Option<Vec2Json>,
        /// Whether the window is resizable.
        #[serde(default = "default_true")]
        resizable: bool,
        /// Whether the window is collapsible.
        #[serde(default = "default_true")]
        collapsible: bool,
        /// Whether to enable scrolling.
        #[serde(default)]
        scroll: bool,
        /// Whether to show the title bar.
        #[serde(default = "default_true")]
        title_bar: bool,
    },

    /// Left side panel.
    LeftPanel {
        /// Panel identifier.
        id: String,
        /// Default panel width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default_width: Option<f32>,
        /// Whether the panel is resizable.
        #[serde(default = "default_true")]
        resizable: bool,
        /// Minimum panel width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_width: Option<f32>,
        /// Maximum panel width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_width: Option<f32>,
    },

    /// Right side panel.
    RightPanel {
        /// Panel identifier.
        id: String,
        /// Default panel width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default_width: Option<f32>,
        /// Whether the panel is resizable.
        #[serde(default = "default_true")]
        resizable: bool,
    },

    /// Top panel.
    TopPanel {
        /// Panel identifier.
        id: String,
        /// Default panel height.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default_height: Option<f32>,
        /// Whether the panel is resizable.
        #[serde(default)]
        resizable: bool,
    },

    /// Bottom panel.
    BottomPanel {
        /// Panel identifier.
        id: String,
        /// Default panel height.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default_height: Option<f32>,
        /// Whether the panel is resizable.
        #[serde(default)]
        resizable: bool,
    },

    /// Central panel (fills remaining space).
    CentralPanel,

    /// Scrollable region.
    ScrollArea {
        /// Enable vertical scrolling.
        #[serde(default = "default_true")]
        vertical: bool,
        /// Enable horizontal scrolling.
        #[serde(default)]
        horizontal: bool,
        /// Maximum height before scrolling.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_height: Option<f32>,
        /// Maximum width before scrolling.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_width: Option<f32>,
        /// Whether to auto-shrink to content.
        #[serde(default)]
        auto_shrink: bool,
        /// Whether to always show scroll bars.
        #[serde(default)]
        always_show_scroll: bool,
    },

    /// Collapsible section with header.
    CollapsingHeader {
        /// Header text.
        text: String,
        /// Whether the section starts open.
        #[serde(default)]
        default_open: bool,
    },

    /// Visual grouping (box around content).
    Group,

    /// Framed region with custom styling.
    Frame {
        /// Optional fill colour.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fill: Option<ColorJson>,
        /// Optional border stroke.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        stroke: Option<StrokeJson>,
        /// Optional corner rounding.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        corner_radius: Option<CornerRadiusJson>,
        /// Optional inner margin.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        inner_margin: Option<MarginJson>,
        /// Optional outer margin.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        outer_margin: Option<MarginJson>,
    },

    /// Menu bar container.
    MenuBar,

    /// Menu within a menu bar.
    Menu {
        /// Menu title.
        title: String,
    },

    /// Tooltip container.
    Tooltip {
        /// Tooltip text.
        text: String,
    },

    /// Popup area (context menu, dropdown, etc.).
    Popup {
        /// Popup identifier.
        id: String,
    },
}

// ---------------------------------------------------------------------------
// Layout JSON — layout configuration
// ---------------------------------------------------------------------------

/// Layout direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum LayoutDirection {
    /// Left to right.
    LeftToRight,
    /// Right to left.
    RightToLeft,
    /// Top to bottom.
    TopDown,
    /// Bottom to top.
    BottomUp,
}

/// Cross-axis alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum LayoutAlign {
    /// Align to minimum (left or top).
    Min,
    /// Centre alignment.
    Center,
    /// Align to maximum (right or bottom).
    Max,
}

/// Serializable layout description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum LayoutJson {
    /// Horizontal layout (left to right).
    Horizontal {
        /// Cross-axis alignment.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        align: Option<LayoutAlign>,
    },

    /// Vertical layout (top to bottom).
    Vertical {
        /// Cross-axis alignment.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        align: Option<LayoutAlign>,
    },

    /// Horizontal layout, centred cross-axis.
    HorizontalCentered,

    /// Vertical layout, centred cross-axis.
    VerticalCentered,

    /// Horizontal layout, justified (items stretch to fill).
    HorizontalJustified,

    /// Vertical layout, justified.
    VerticalJustified,

    /// Horizontal layout, wrapping to next line.
    HorizontalWrapped,

    /// Column-based layout.
    Columns {
        /// Number of columns.
        count: usize,
    },

    /// Grid layout.
    Grid {
        /// Grid identifier.
        id: String,
        /// Number of columns.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        num_columns: Option<usize>,
        /// Whether to stripe alternating rows.
        #[serde(default)]
        striped: bool,
        /// Minimum column width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_col_width: Option<f32>,
        /// Maximum column width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_col_width: Option<f32>,
        /// Cell spacing.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        spacing: Option<Vec2Json>,
    },

    /// Indented section.
    Indent {
        /// Indentation amount in logical pixels.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        indent: Option<f32>,
    },

    /// Explicit spacing between widgets.
    AddSpace {
        /// Space amount in logical pixels.
        amount: f32,
    },
}

// ---------------------------------------------------------------------------
// Response JSON — widget interaction state
// ---------------------------------------------------------------------------

/// Serializable representation of widget interaction state.
///
/// Captures the most commonly queried fields from [`egui::Response`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ResponseJson {
    /// Widget was clicked this frame.
    pub clicked: bool,
    /// Widget was double-clicked this frame.
    pub double_clicked: bool,
    /// Pointer is hovering over the widget.
    pub hovered: bool,
    /// Widget has keyboard focus.
    pub has_focus: bool,
    /// Widget value was changed this frame.
    pub changed: bool,
    /// Widget was dragged this frame.
    pub dragged: bool,
    /// Drag delta since last frame.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_delta: Option<Vec2Json>,
    /// Widget bounding rectangle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect: Option<RectJson>,
}

// ---------------------------------------------------------------------------
// UI tree — compositional description of a full egui frame
// ---------------------------------------------------------------------------

/// A node in a declarative UI tree.
///
/// Agents compose `UiNode` trees to describe an entire egui frame.
/// The runtime module renders these into actual egui calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "node_type")]
pub enum UiNode {
    /// A leaf widget.
    Widget {
        /// The widget description.
        widget: WidgetJson,
    },

    /// A container with children.
    Container {
        /// The container description.
        container: ContainerJson,
        /// Child nodes rendered inside the container.
        #[serde(default)]
        children: Vec<UiNode>,
    },

    /// A layout wrapper around children.
    Layout {
        /// The layout description.
        layout: LayoutJson,
        /// Child nodes arranged by the layout.
        #[serde(default)]
        children: Vec<UiNode>,
    },
}
