//! MCP (Model Context Protocol) integration.

mod parsing;
mod tools;

pub use parsing::{parse_bool, parse_integer, parse_string};
pub use tools::{bool_params, number_params, select_params, text_params, tool_names};
