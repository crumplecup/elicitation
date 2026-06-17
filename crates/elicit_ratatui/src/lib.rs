//! `elicit_ratatui` — MCP shadow crate for ratatui.
//!
//! Wraps every ratatui type with [`elicit_newtype!`] (builder types) or a
//! hand-written trenchcoat (value/enum types).  [`#[reflect_methods]`] mines
//! the builder API and generates MCP tools automatically — the tool vocabulary
//! IS the ratatui vocabulary.
//!
//! # Shadow types
//!
//! | Module | Types |
//! |--------|-------|
//! | `primitives` | `Color`, `Modifier`, `Constraint`, `Rect`, `Direction`, `Alignment`, `Borders`, `BorderType`, `Padding`, `Margin` |
//! | `style` | `Style` |
//! | `text` | `Span`, `Line`, `Text` |
//! | `layout` | `Layout` |
//! | `widgets` | `Block`, `Paragraph`, `List`, `ListItem`, `Table`, `Row`, `Cell`, `Gauge`, `LineGauge`, `Sparkline`, `BarChart`, `Chart`, `Dataset`, `Axis`, `Tabs`, `Scrollbar`, `ScrollbarOrientation` |
//! | `state` | UUID tools for `ListState`, `TableState`, `ScrollbarState` |
//! | `events` | `Event`, `KeyEvent`, `KeyCode`, `MouseEvent` trenchcoats |
//! | `plugin` | `RatatuiPlugin`, `RatatuiCtx` |
//!
//! [`elicit_newtype!`]: elicitation::elicit_newtype
//! [`#[reflect_methods]`]: elicitation_derive::reflect_methods

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod events;
mod layout;
mod plugin;
mod primitives;
mod render_backend;
mod render_context;
mod state;
mod style;
#[cfg(feature = "runtime")]
mod terminal_tools;
mod text;
pub mod tui_node;
pub mod tui_accesskit_convert;
pub(crate) mod wcag_verify;
mod widgets;

pub use elicit_ui::{verify_wcag_contrast_proofs, wcag_contrast_ratio};
pub use events::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
pub use layout::Layout;
pub use plugin::{RatatuiCtx, RatatuiPlugin};
pub use primitives::{
    Alignment, BorderType, Borders, Color, Constraint, Direction, Margin, Modifier, Padding, Rect,
    ScrollbarOrientation,
};
pub use render_backend::RatatuiBackend;
pub use render_context::{RatatuiRenderContext, RenderVerifiable, ratatui_verify_in_debug};
pub use state::{
    ListStateIdParams, ListStateNewParams, ListStateSelectParams, ScrollbarStateIdParams,
    ScrollbarStateNewParams, ScrollbarStateSetPositionParams, TableStateIdParams,
    TableStateNewParams, TableStateSelectParams, list_state_destroy, list_state_new,
    list_state_offset, list_state_select, list_state_selected, scrollbar_state_destroy,
    scrollbar_state_new, scrollbar_state_position, scrollbar_state_set_position,
    table_state_destroy, table_state_new, table_state_select_column, table_state_select_row,
    table_state_selected_row,
};
pub use style::Style;
pub use text::{Line, Span, Text};
pub use tui_accesskit_convert::{tree_update_to_tui_node, tui_node_to_tree_update};
pub use tui_node::{TuiNode, WidgetJson};
#[cfg(feature = "runtime")]
pub use terminal_tools::{render_node, render_widget};
pub use widgets::{
    Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph, Row,
    Scrollbar, Sparkline, Table, Tabs,
};
