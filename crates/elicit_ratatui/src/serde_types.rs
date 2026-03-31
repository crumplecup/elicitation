//! JSON-serialisable representations of ratatui types.
//!
//! These types cross the MCP JSON boundary. Each has bidirectional
//! conversion to/from the corresponding ratatui type and derives
//! `Serialize`, `Deserialize`, `JsonSchema` for MCP transport.

use elicitation::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Colour
// ---------------------------------------------------------------------------

/// JSON representation of a ratatui `Color`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(tag = "type")]
pub enum ColorJson {
    /// Reset to terminal default.
    Reset,
    /// Standard ANSI black.
    Black,
    /// Standard ANSI red.
    Red,
    /// Standard ANSI green.
    Green,
    /// Standard ANSI yellow.
    Yellow,
    /// Standard ANSI blue.
    Blue,
    /// Standard ANSI magenta.
    Magenta,
    /// Standard ANSI cyan.
    Cyan,
    /// Standard ANSI white.
    White,
    /// Dark gray (bright black).
    DarkGray,
    /// Light (bright) red.
    LightRed,
    /// Light (bright) green.
    LightGreen,
    /// Light (bright) yellow.
    LightYellow,
    /// Light (bright) blue.
    LightBlue,
    /// Light (bright) magenta.
    LightMagenta,
    /// Light (bright) cyan.
    LightCyan,
    /// Bright white (gray).
    Gray,
    /// 24-bit RGB colour.
    Rgb {
        /// Red channel (0–255).
        r: u8,
        /// Green channel (0–255).
        g: u8,
        /// Blue channel (0–255).
        b: u8,
    },
    /// 256-colour palette index.
    Indexed {
        /// Palette index (0–255).
        index: u8,
    },
}

impl From<ColorJson> for ratatui::style::Color {
    fn from(c: ColorJson) -> Self {
        match c {
            ColorJson::Reset => Self::Reset,
            ColorJson::Black => Self::Black,
            ColorJson::Red => Self::Red,
            ColorJson::Green => Self::Green,
            ColorJson::Yellow => Self::Yellow,
            ColorJson::Blue => Self::Blue,
            ColorJson::Magenta => Self::Magenta,
            ColorJson::Cyan => Self::Cyan,
            ColorJson::White => Self::White,
            ColorJson::DarkGray => Self::DarkGray,
            ColorJson::LightRed => Self::LightRed,
            ColorJson::LightGreen => Self::LightGreen,
            ColorJson::LightYellow => Self::LightYellow,
            ColorJson::LightBlue => Self::LightBlue,
            ColorJson::LightMagenta => Self::LightMagenta,
            ColorJson::LightCyan => Self::LightCyan,
            ColorJson::Gray => Self::Gray,
            ColorJson::Rgb { r, g, b } => Self::Rgb(r, g, b),
            ColorJson::Indexed { index } => Self::Indexed(index),
        }
    }
}

impl From<ratatui::style::Color> for ColorJson {
    fn from(c: ratatui::style::Color) -> Self {
        match c {
            ratatui::style::Color::Reset => Self::Reset,
            ratatui::style::Color::Black => Self::Black,
            ratatui::style::Color::Red => Self::Red,
            ratatui::style::Color::Green => Self::Green,
            ratatui::style::Color::Yellow => Self::Yellow,
            ratatui::style::Color::Blue => Self::Blue,
            ratatui::style::Color::Magenta => Self::Magenta,
            ratatui::style::Color::Cyan => Self::Cyan,
            ratatui::style::Color::White => Self::White,
            ratatui::style::Color::DarkGray => Self::DarkGray,
            ratatui::style::Color::LightRed => Self::LightRed,
            ratatui::style::Color::LightGreen => Self::LightGreen,
            ratatui::style::Color::LightYellow => Self::LightYellow,
            ratatui::style::Color::LightBlue => Self::LightBlue,
            ratatui::style::Color::LightMagenta => Self::LightMagenta,
            ratatui::style::Color::LightCyan => Self::LightCyan,
            ratatui::style::Color::Gray => Self::Gray,
            ratatui::style::Color::Rgb(r, g, b) => Self::Rgb { r, g, b },
            ratatui::style::Color::Indexed(index) => Self::Indexed { index },
        }
    }
}

// ---------------------------------------------------------------------------
// Modifier
// ---------------------------------------------------------------------------

/// JSON representation of a ratatui text `Modifier`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum ModifierJson {
    /// Bold text.
    Bold,
    /// Dim/faint text.
    Dim,
    /// Italic text.
    Italic,
    /// Underlined text.
    Underlined,
    /// Slow blink.
    SlowBlink,
    /// Rapid blink.
    RapidBlink,
    /// Reversed foreground/background.
    Reversed,
    /// Hidden text.
    Hidden,
    /// Crossed-out (strikethrough) text.
    CrossedOut,
}

impl ModifierJson {
    /// Convert to the corresponding ratatui `Modifier` bitflag.
    pub fn to_modifier(self) -> ratatui::style::Modifier {
        match self {
            Self::Bold => ratatui::style::Modifier::BOLD,
            Self::Dim => ratatui::style::Modifier::DIM,
            Self::Italic => ratatui::style::Modifier::ITALIC,
            Self::Underlined => ratatui::style::Modifier::UNDERLINED,
            Self::SlowBlink => ratatui::style::Modifier::SLOW_BLINK,
            Self::RapidBlink => ratatui::style::Modifier::RAPID_BLINK,
            Self::Reversed => ratatui::style::Modifier::REVERSED,
            Self::Hidden => ratatui::style::Modifier::HIDDEN,
            Self::CrossedOut => ratatui::style::Modifier::CROSSED_OUT,
        }
    }
}

// ---------------------------------------------------------------------------
// Style
// ---------------------------------------------------------------------------

/// JSON representation of a ratatui `Style`.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral,
)]
pub struct StyleJson {
    /// Foreground colour.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fg: Option<ColorJson>,
    /// Background colour.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg: Option<ColorJson>,
    /// Active text modifiers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<ModifierJson>,
}

impl From<StyleJson> for ratatui::style::Style {
    fn from(s: StyleJson) -> Self {
        let mut style = Self::default();
        if let Some(fg) = s.fg {
            style = style.fg(ratatui::style::Color::from(fg));
        }
        if let Some(bg) = s.bg {
            style = style.bg(ratatui::style::Color::from(bg));
        }
        for m in &s.modifiers {
            style = style.add_modifier(m.to_modifier());
        }
        style
    }
}

// ---------------------------------------------------------------------------
// Borders
// ---------------------------------------------------------------------------

/// JSON representation of ratatui border edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum BordersJson {
    /// No borders.
    None,
    /// All four borders.
    All,
    /// Top border only.
    Top,
    /// Right border only.
    Right,
    /// Bottom border only.
    Bottom,
    /// Left border only.
    Left,
}

impl From<BordersJson> for ratatui::widgets::Borders {
    fn from(b: BordersJson) -> Self {
        match b {
            BordersJson::None => Self::NONE,
            BordersJson::All => Self::ALL,
            BordersJson::Top => Self::TOP,
            BordersJson::Right => Self::RIGHT,
            BordersJson::Bottom => Self::BOTTOM,
            BordersJson::Left => Self::LEFT,
        }
    }
}

/// JSON representation of a ratatui `BorderType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum BorderTypeJson {
    /// Plain ASCII borders.
    Plain,
    /// Rounded corner borders.
    Rounded,
    /// Double-line borders.
    Double,
    /// Thick borders.
    Thick,
}

impl From<BorderTypeJson> for ratatui::widgets::BorderType {
    fn from(bt: BorderTypeJson) -> Self {
        match bt {
            BorderTypeJson::Plain => Self::Plain,
            BorderTypeJson::Rounded => Self::Rounded,
            BorderTypeJson::Double => Self::Double,
            BorderTypeJson::Thick => Self::Thick,
        }
    }
}

// ---------------------------------------------------------------------------
// Padding
// ---------------------------------------------------------------------------

/// JSON representation of inner padding for a Block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct PaddingJson {
    /// Left padding.
    #[serde(default)]
    pub left: u16,
    /// Right padding.
    #[serde(default)]
    pub right: u16,
    /// Top padding.
    #[serde(default)]
    pub top: u16,
    /// Bottom padding.
    #[serde(default)]
    pub bottom: u16,
}

impl From<PaddingJson> for ratatui::widgets::Padding {
    fn from(p: PaddingJson) -> Self {
        Self::new(p.left, p.right, p.top, p.bottom)
    }
}

// ---------------------------------------------------------------------------
// Margin
// ---------------------------------------------------------------------------

/// JSON representation of layout margin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct MarginJson {
    /// Horizontal margin.
    #[serde(default)]
    pub horizontal: u16,
    /// Vertical margin.
    #[serde(default)]
    pub vertical: u16,
}

// ---------------------------------------------------------------------------
// Layout
// ---------------------------------------------------------------------------

/// Layout direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum DirectionJson {
    /// Top to bottom.
    Vertical,
    /// Left to right.
    Horizontal,
}

impl From<DirectionJson> for ratatui::layout::Direction {
    fn from(d: DirectionJson) -> Self {
        match d {
            DirectionJson::Vertical => Self::Vertical,
            DirectionJson::Horizontal => Self::Horizontal,
        }
    }
}

/// JSON representation of a layout constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(tag = "type")]
pub enum ConstraintJson {
    /// Fixed number of rows/columns.
    Length {
        /// Exact length.
        value: u16,
    },
    /// Percentage of available space (0–100).
    Percentage {
        /// Percentage value.
        value: u16,
    },
    /// Minimum length.
    Min {
        /// Minimum value.
        value: u16,
    },
    /// Maximum length.
    Max {
        /// Maximum value.
        value: u16,
    },
    /// Fill remaining space with proportional weight.
    Fill {
        /// Weight (higher = more space).
        value: u16,
    },
    /// Ratio of available space.
    Ratio {
        /// Numerator.
        num: u32,
        /// Denominator.
        den: u32,
    },
}

impl From<ConstraintJson> for ratatui::layout::Constraint {
    fn from(c: ConstraintJson) -> Self {
        match c {
            ConstraintJson::Length { value } => Self::Length(value),
            ConstraintJson::Percentage { value } => Self::Percentage(value),
            ConstraintJson::Min { value } => Self::Min(value),
            ConstraintJson::Max { value } => Self::Max(value),
            ConstraintJson::Fill { value } => Self::Fill(value),
            ConstraintJson::Ratio { num, den } => Self::Ratio(num, den),
        }
    }
}

// ---------------------------------------------------------------------------
// Block
// ---------------------------------------------------------------------------

/// JSON representation of a ratatui `Block`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct BlockJson {
    /// Which borders to draw.
    #[serde(default = "default_borders_all")]
    pub borders: BordersJson,
    /// Border line style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_type: Option<BorderTypeJson>,
    /// Block title text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Block style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
    /// Border style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_style: Option<StyleJson>,
    /// Inner padding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<PaddingJson>,
}

fn default_borders_all() -> BordersJson {
    BordersJson::All
}

// ---------------------------------------------------------------------------
// Table helpers
// ---------------------------------------------------------------------------

/// JSON representation of a table row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct RowJson {
    /// Cell contents.
    pub cells: Vec<CellJson>,
    /// Row height (lines).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    /// Row style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
}

/// JSON representation of a table cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct CellJson {
    /// Cell text content.
    pub content: String,
    /// Cell style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
}

// ---------------------------------------------------------------------------
// Stateful widget state
// ---------------------------------------------------------------------------

/// JSON representation of `ListState`.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral,
)]
pub struct ListStateJson {
    /// Currently selected index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<usize>,
    /// Scroll offset.
    #[serde(default)]
    pub offset: usize,
}

/// JSON representation of `TableState`.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral,
)]
pub struct TableStateJson {
    /// Currently selected row index.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<usize>,
    /// Scroll offset.
    #[serde(default)]
    pub offset: usize,
}

// ---------------------------------------------------------------------------
// Text composition
// ---------------------------------------------------------------------------

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum AlignmentJson {
    /// Left aligned.
    Left,
    /// Center aligned.
    Center,
    /// Right aligned.
    Right,
}

impl From<AlignmentJson> for ratatui::layout::Alignment {
    fn from(a: AlignmentJson) -> Self {
        match a {
            AlignmentJson::Left => Self::Left,
            AlignmentJson::Center => Self::Center,
            AlignmentJson::Right => Self::Right,
        }
    }
}

impl From<ratatui::layout::Alignment> for AlignmentJson {
    fn from(a: ratatui::layout::Alignment) -> Self {
        match a {
            ratatui::layout::Alignment::Left => Self::Left,
            ratatui::layout::Alignment::Center => Self::Center,
            ratatui::layout::Alignment::Right => Self::Right,
        }
    }
}

/// JSON representation of a styled text span.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct SpanJson {
    /// Span text content.
    pub content: String,
    /// Span style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
}

/// JSON representation of a line of styled spans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LineJson {
    /// Spans composing the line.
    pub spans: Vec<SpanJson>,
    /// Line style (applied to entire line).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
    /// Line alignment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<AlignmentJson>,
}

/// JSON representation of multi-line text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct TextJson {
    /// Lines of text.
    pub lines: Vec<LineJson>,
    /// Text style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
    /// Text alignment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<AlignmentJson>,
}

// ---------------------------------------------------------------------------
// Widget enum (tagged union of all widget descriptions)
// ---------------------------------------------------------------------------

/// Top-level tagged enum describing any ratatui widget as JSON.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(tag = "type")]
pub enum WidgetJson {
    /// A bordered container block.
    Block {
        /// Block description.
        #[serde(flatten)]
        block: BlockJson,
    },
    /// A text paragraph.
    Paragraph {
        /// Display text.
        text: String,
        /// Text style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
        /// Enable text wrapping.
        #[serde(default)]
        wrap: bool,
        /// Scroll offset (row, col).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scroll: Option<(u16, u16)>,
        /// Text alignment.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        alignment: Option<String>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
    },
    /// A selectable list.
    List {
        /// List item texts.
        items: Vec<String>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
        /// Item style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
        /// Selected-item highlight style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        highlight_style: Option<StyleJson>,
        /// Selection indicator string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        highlight_symbol: Option<String>,
        /// Widget state.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        state: Option<ListStateJson>,
    },
    /// A multi-column table.
    Table {
        /// Header row.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        header: Option<RowJson>,
        /// Data rows.
        rows: Vec<RowJson>,
        /// Column width constraints.
        widths: Vec<ConstraintJson>,
        /// Gap between columns.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        column_spacing: Option<u16>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
        /// Selected-row highlight style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        highlight_style: Option<StyleJson>,
        /// Selection indicator string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        highlight_symbol: Option<String>,
        /// Widget state.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        state: Option<TableStateJson>,
    },
    /// A progress gauge.
    Gauge {
        /// Progress ratio (0.0–1.0).
        ratio: f64,
        /// Optional label text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
        /// Gauge style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
        /// Filled-portion style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gauge_style: Option<StyleJson>,
    },
    /// A compact sparkline chart.
    Sparkline {
        /// Data points.
        data: Vec<u64>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
        /// Sparkline style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
        /// Maximum value (auto-scaled if absent).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u64>,
        /// Render direction.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        direction: Option<DirectionJson>,
    },
    /// Horizontal tab selector.
    Tabs {
        /// Tab title texts.
        titles: Vec<String>,
        /// Currently selected tab index.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        selected: Option<usize>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
        /// Tab style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
        /// Selected-tab highlight style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        highlight_style: Option<StyleJson>,
        /// Divider character between tabs.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        divider: Option<String>,
    },
    /// Clear a rectangular area.
    Clear,
    /// A bar chart with grouped bars.
    BarChart {
        /// Bar groups.
        data: Vec<BarGroupJson>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
        /// Maximum bar value (auto-calculated if absent).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_value: Option<u64>,
        /// Bar width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bar_width: Option<u16>,
        /// Gap between bars in a group.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bar_gap: Option<u16>,
        /// Gap between groups.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        group_gap: Option<u16>,
        /// Bar style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bar_style: Option<StyleJson>,
        /// Value label style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value_style: Option<StyleJson>,
        /// Label style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label_style: Option<StyleJson>,
        /// Layout direction.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        direction: Option<DirectionJson>,
        /// Bar chart style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
    },
    /// A line/scatter chart with axes.
    Chart {
        /// Datasets to plot.
        datasets: Vec<DatasetJson>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
        /// X axis configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        x_axis: Option<AxisJson>,
        /// Y axis configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        y_axis: Option<AxisJson>,
        /// Chart style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
        /// Legend position.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        legend_position: Option<LegendPositionJson>,
    },
    /// A linear progress gauge.
    LineGauge {
        /// Progress ratio (0.0–1.0).
        ratio: f64,
        /// Optional label text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        /// Optional surrounding block.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block: Option<BlockJson>,
        /// Line gauge style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
        /// Filled portion style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filled_style: Option<StyleJson>,
        /// Unfilled portion style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unfilled_style: Option<StyleJson>,
    },
    /// A scrollbar indicator.
    Scrollbar {
        /// Scrollbar orientation.
        orientation: ScrollbarOrientationJson,
        /// Thumb symbol.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thumb_symbol: Option<String>,
        /// Track symbol.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        track_symbol: Option<String>,
        /// Begin symbol (arrow at start).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        begin_symbol: Option<String>,
        /// End symbol (arrow at end).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        end_symbol: Option<String>,
        /// Scrollbar style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<StyleJson>,
        /// Thumb style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thumb_style: Option<StyleJson>,
        /// Track style.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        track_style: Option<StyleJson>,
        /// State.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        state: Option<ScrollbarStateJson>,
    },
}

// ---------------------------------------------------------------------------
// BarChart helpers
// ---------------------------------------------------------------------------

/// JSON representation of a single bar in a bar chart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct BarJson {
    /// Bar label text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Bar value.
    pub value: u64,
    /// Bar style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
    /// Value label style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_style: Option<StyleJson>,
    /// Text value to display (overrides numeric).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
}

/// JSON representation of a group of bars.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct BarGroupJson {
    /// Group label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Bars in this group.
    pub bars: Vec<BarJson>,
}

// ---------------------------------------------------------------------------
// Chart helpers
// ---------------------------------------------------------------------------

/// Chart graph type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum GraphTypeJson {
    /// Scatter plot.
    Scatter,
    /// Line chart.
    Line,
}

/// Chart marker style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum MarkerJson {
    /// Dot marker.
    Dot,
    /// Braille pattern marker.
    Braille,
    /// Block marker.
    Block,
    /// Bar marker.
    Bar,
    /// Half-block marker.
    HalfBlock,
}

/// JSON representation of a chart dataset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct DatasetJson {
    /// Dataset name (shown in legend).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Data points as (x, y) pairs.
    pub data: Vec<(f64, f64)>,
    /// Point/line style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
    /// Marker type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<MarkerJson>,
    /// Graph type (Line or Scatter).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_type: Option<GraphTypeJson>,
}

/// JSON representation of a chart axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AxisJson {
    /// Axis title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Axis value bounds [min, max].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounds: Option<(f64, f64)>,
    /// Axis tick labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    /// Axis style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleJson>,
}

/// Legend position for charts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum LegendPositionJson {
    /// Top left.
    TopLeft,
    /// Top right.
    TopRight,
    /// Bottom left.
    BottomLeft,
    /// Bottom right.
    BottomRight,
}

// ---------------------------------------------------------------------------
// Scrollbar
// ---------------------------------------------------------------------------

/// Scrollbar orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum ScrollbarOrientationJson {
    /// Vertical scrollbar on the right.
    VerticalRight,
    /// Vertical scrollbar on the left.
    VerticalLeft,
    /// Horizontal scrollbar at bottom.
    HorizontalBottom,
    /// Horizontal scrollbar at top.
    HorizontalTop,
}

/// JSON representation of `ScrollbarState`.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral,
)]
pub struct ScrollbarStateJson {
    /// Total content length.
    #[serde(default)]
    pub content_length: usize,
    /// Current scroll position.
    #[serde(default)]
    pub position: usize,
    /// Viewport content length.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport_content_length: Option<usize>,
}

// ---------------------------------------------------------------------------
// Event types
// ---------------------------------------------------------------------------

/// JSON representation of a key event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct KeyEventJson {
    /// Key code (e.g. "Char(q)", "Enter", "Esc", "Up", "Down", "Left", "Right", "Tab", "Backspace", "F(1)").
    pub code: String,
    /// Key modifiers (e.g. "CONTROL", "SHIFT", "ALT").
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<String>,
}

/// JSON representation of a mouse event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct MouseEventJson {
    /// Mouse event kind (e.g. "Down(Left)", "Up(Left)", "Drag(Left)", "Moved", "ScrollDown", "ScrollUp").
    pub kind: String,
    /// Column position.
    pub column: u16,
    /// Row position.
    pub row: u16,
    /// Key modifiers active during mouse event.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<String>,
}

/// JSON representation of a terminal event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(tag = "type")]
pub enum EventJson {
    /// A key was pressed.
    Key {
        /// Key event details.
        event: KeyEventJson,
    },
    /// A mouse event occurred.
    Mouse {
        /// Mouse event details.
        event: MouseEventJson,
    },
    /// The terminal was resized.
    Resize {
        /// New width.
        width: u16,
        /// New height.
        height: u16,
    },
    /// Focus gained.
    FocusGained,
    /// Focus lost.
    FocusLost,
    /// Paste event.
    Paste {
        /// Pasted text.
        text: String,
    },
}

// ---------------------------------------------------------------------------
// TUI node tree (compositional)
// ---------------------------------------------------------------------------

/// A node in a declarative TUI tree.
///
/// Agents build a tree of `TuiNode` values describing the complete TUI layout,
/// which can then be rendered by a ratatui backend or emitted as Rust source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(tag = "node_type")]
pub enum TuiNode {
    /// A leaf widget.
    Widget {
        /// Widget description.
        widget: Box<WidgetJson>,
    },
    /// A layout split containing child nodes.
    Layout {
        /// Split direction.
        direction: DirectionJson,
        /// Size constraints for each child.
        constraints: Vec<ConstraintJson>,
        /// Child nodes (one per constraint).
        children: Vec<TuiNode>,
        /// Outer margin.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        margin: Option<MarginJson>,
    },
}
