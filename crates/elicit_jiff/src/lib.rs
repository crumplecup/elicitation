//! `elicit_jiff` — elicitation-enabled wrappers around `jiff` datetime types.
//!
//! Provides [`Zoned`] and [`Timestamp`] newtypes with:
//! - [`schemars::JsonSchema`] (delegated to inner jiff types)
//! - [`serde::Serialize`] / [`serde::Deserialize`] (transparent)
//! - MCP reflect methods for datetime field access and timezone operations
//! - [`JiffWorkflowPlugin`]: contract-verified datetime composition tools

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod timestamp;
pub mod workflow;
mod zoned;

pub use timestamp::Timestamp;
pub use workflow::{
    AssertFutureParams, ComputeSpanParams, ConvertTzParams, ConvertedZonedProof,
    ConvertedZonedState, FutureTimestampProof, FutureTimestampState, JiffWorkflowPlugin,
    ParseTimestampParams, ParseZonedParams, ParsedTimestamp, ParsedZoned, TimestampFuture,
    TimestampParsed, TimezoneConverted, UnvalidatedTimestampStr, UnvalidatedZonedStr, ZonedParsed,
    parse_ts,
};
pub use zoned::Zoned;
