//! `elicit_regex` — elicitation-enabled wrapper around `regex::Regex`.
//!
//! Provides a [`Regex`] newtype with:
//! - [`schemars::JsonSchema`] (serializes as pattern string)
//! - [`serde::Serialize`] / [`serde::Deserialize`]
//! - MCP reflect methods for matching and inspection
//!
//! Also exposes [`RegexWorkflowPlugin`] — a stateless MCP plugin with
//! contract-verified compile, match, search, replace, and capture tools.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod regex_type;
mod workflow;

pub use regex_type::Regex;
pub use workflow::{
    CaptureGroupsParams, CompileParams, FindAllParams, IsMatchParams, MatchProof, PatternMatched,
    RegexValid, RegexWorkflowPlugin, ReplaceAllParams, UnvalidatedPattern,
};
