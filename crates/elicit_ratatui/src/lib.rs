//! `elicit_ratatui` — dual-mode MCP tools for ratatui TUI creation.
//!
//! Provides tools that operate in two modes:
//!
//! 1. **Runtime mode** — each tool returns a JSON description
//!    that can be rendered by a ratatui terminal backend or inspected as JSON.
//! 2. **Emit mode** — each tool's parameters can generate idiomatic
//!    ratatui Rust code via the elicitation code-emission pipeline.
//!
//! # Tool categories
//!
//! ## Widgets
//!
//! | Tool | Widget | Description |
//! |------|--------|-------------|
//! | `widget_block` | Block | Bordered container with optional title |
//! | `widget_paragraph` | Paragraph | Text display with wrapping/scrolling |
//! | `widget_list` | List | Selectable item list |
//! | `widget_table` | Table | Multi-column data grid |
//! | `widget_gauge` | Gauge | Progress indicator |
//! | `widget_sparkline` | Sparkline | Compact inline chart |
//! | `widget_bar_chart` | BarChart | Grouped vertical/horizontal bars |
//! | `widget_chart` | Chart | Line/scatter chart with axes |
//! | `widget_line_gauge` | LineGauge | Linear progress bar |
//! | `widget_scrollbar` | Scrollbar | Scroll position indicator |
//! | `widget_tabs` | Tabs | Horizontal tab selector |
//! | `widget_clear` | Clear | Clear a rectangular area |
//!
//! ## Style
//!
//! | Tool | Description |
//! |------|-------------|
//! | `style_fg` | Set foreground colour |
//! | `style_bg` | Set background colour |
//! | `style_modifier` | Add text modifier (Bold, Italic, etc.) |
//! | `style_reset` | Reset to default style |
//! | `color_rgb` | Create RGB colour |
//! | `color_indexed` | Create 256-colour palette colour |
//! | `color_named` | Create named colour (Red, Green, etc.) |
//!
//! ## Layout
//!
//! | Tool | Description |
//! |------|-------------|
//! | `layout_vertical` | Vertical split with constraints |
//! | `layout_horizontal` | Horizontal split with constraints |
//! | `constraint_length` | Fixed-length constraint |
//! | `constraint_percentage` | Percentage constraint |
//! | `constraint_min` | Minimum length constraint |
//! | `constraint_max` | Maximum length constraint |
//! | `constraint_fill` | Fill remaining space constraint |
//! | `constraint_ratio` | Ratio constraint |
//!
//! ## Text
//!
//! | Tool | Description |
//! |------|-------------|
//! | `text_raw` | Create plain unstyled text |
//! | `text_styled` | Create styled text with a single span |
//! | `span_raw` | Create a plain unstyled span |
//! | `span_styled` | Create a styled span |
//! | `line_from_spans` | Create a line from spans |
//! | `text_from_lines` | Create multi-line text from lines |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod layout_tools;
mod property_tools;
mod serde_types;
mod style_tools;
mod text_tools;
mod widget_tools;

pub use layout_tools::{
    ConstraintFillParams, ConstraintLengthParams, ConstraintMaxParams, ConstraintMinParams,
    ConstraintPercentageParams, ConstraintRatioParams, LayoutHorizontalParams,
    LayoutVerticalParams,
};
pub use serde_types::{
    AlignmentJson, AxisJson, BarGroupJson, BarJson, BlockJson, BorderTypeJson, BordersJson,
    CellJson, ColorJson, ConstraintJson, DatasetJson, DirectionJson, GraphTypeJson,
    LegendPositionJson, LineJson, ListStateJson, MarginJson, MarkerJson, ModifierJson,
    PaddingJson, RowJson, ScrollbarOrientationJson, ScrollbarStateJson, SpanJson, StyleJson,
    TableStateJson, TextJson, TuiNode, WidgetJson,
};
pub use style_tools::{
    ColorIndexedParams, ColorNamedParams, ColorRgbParams, ModifierParams, StyleBgParams,
    StyleFgParams, StyleResetParams,
};
pub use text_tools::{
    LineFromSpansParams, SpanRawParams, SpanStyledParams, TextFromLinesParams, TextRawParams,
    TextStyledParams,
};
pub use widget_tools::{
    BarChartParams, BlockParams, ChartParams, ClearParams, GaugeParams, LineGaugeParams,
    ListParams, ParagraphParams, ScrollbarParams, SparklineParams, TableParams, TabsParams,
};
pub use property_tools::{
    BarChartSetBarGapParams, BarChartSetBarStyleParams, BarChartSetBarWidthParams,
    BarChartSetBlockParams, BarChartSetDirectionParams, BarChartSetMaxValueParams,
    BarChartSetStyleParams, BlockSetBorderStyleParams, BlockSetBorderTypeParams,
    BlockSetBordersParams, BlockSetPaddingParams, BlockSetStyleParams, BlockSetTitleParams,
    ChartSetBlockParams, ChartSetLegendPositionParams, ChartSetStyleParams, ChartSetXAxisParams,
    ChartSetYAxisParams, GaugeSetBlockParams, GaugeSetGaugeStyleParams, GaugeSetLabelParams,
    GaugeSetStyleParams, LineGaugeSetBlockParams, LineGaugeSetLabelParams, LineGaugeSetStyleParams,
    ListSetBlockParams, ListSetHighlightStyleParams, ListSetHighlightSymbolParams,
    ListSetStateParams, ListSetStyleParams, ParagraphSetAlignmentParams, ParagraphSetBlockParams,
    ParagraphSetScrollParams, ParagraphSetStyleParams, ParagraphSetWrapParams,
    ScrollbarSetStateParams, ScrollbarSetStyleParams, SparklineSetBlockParams,
    SparklineSetDirectionParams, SparklineSetMaxParams, SparklineSetStyleParams,
    TableSetBlockParams, TableSetColumnSpacingParams, TableSetHeaderParams,
    TableSetHighlightStyleParams, TableSetHighlightSymbolParams, TableSetStateParams,
    TabsSetBlockParams, TabsSetDividerParams, TabsSetHighlightStyleParams, TabsSetSelectedParams,
    TabsSetStyleParams,
};
