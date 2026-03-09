//! `elicit_chrono` — elicitation-enabled wrappers around `chrono` datetime types.
//!
//! Provides [`DateTimeUtc`], [`DateTimeFixed`], and [`NaiveDateTime`] newtypes with:
//! - [`schemars::JsonSchema`] (delegated to inner chrono types)
//! - [`serde::Serialize`] / [`serde::Deserialize`] (transparent, RFC 3339)
//! - MCP reflect methods for field access and formatting
//! - [`ChronoWorkflowPlugin`]: contract-verified datetime composition tools

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod date_time_fixed;
mod date_time_utc;
mod naive_date_time;
pub mod workflow;

pub use chrono::{Duration, Utc};
pub use date_time_fixed::DateTimeFixed;
pub use date_time_utc::DateTimeUtc;
pub use naive_date_time::NaiveDateTime;
pub use workflow::{
    AddSecondsParams, AssertFutureParams, AssertInRangeParams, ChronoWorkflowPlugin,
    ComputeDurationParams, DateTimeFuture, DateTimeInRange, DateTimeParsed, FutureDateTimeProof,
    FutureDateTimeState, ParseDateTimeParams, ParsedDateTime, RangedDateTimeProof,
    RangedDateTimeState, UnvalidatedDateStr, parse_rfc3339,
};
