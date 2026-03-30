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

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod serde_types;
mod style_tools;
mod widget_tools;

pub use serde_types::{
    BlockJson, BorderTypeJson, BordersJson, CellJson, ColorJson, ConstraintJson, DirectionJson,
    ListStateJson, MarginJson, ModifierJson, PaddingJson, RowJson, StyleJson, TableStateJson,
    TuiNode, WidgetJson,
};
pub use style_tools::{
    ColorIndexedParams, ColorNamedParams, ColorRgbParams, ModifierParams, StyleBgParams,
    StyleFgParams, StyleResetParams,
};
pub use widget_tools::{
    BlockParams, ClearParams, GaugeParams, ListParams, ParagraphParams, SparklineParams,
    TableParams, TabsParams,
};
