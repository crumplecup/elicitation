//! `elicit_regex` — elicitation-enabled wrapper around `regex::Regex`.
//!
//! Provides a [`Regex`] newtype with:
//! - [`schemars::JsonSchema`] (serializes as pattern string)
//! - [`serde::Serialize`] / [`serde::Deserialize`]
//! - MCP reflect methods for matching and inspection

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod regex_type;

pub use regex_type::Regex;
