//! `elicit_time` — elicitation-enabled wrappers around `time` datetime types.
//!
//! Provides [`OffsetDateTime`] and [`PrimitiveDateTime`] newtypes with:
//! - [`schemars::JsonSchema`] (serializes as ISO 8601 string)
//! - [`serde::Serialize`] / [`serde::Deserialize`]
//! - MCP reflect methods for field access
//! - [`TimeWorkflowPlugin`]: contract-verified datetime composition tools

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod offset_date_time;
mod primitive_date_time;
pub mod workflow;

pub use offset_date_time::OffsetDateTime;
pub use primitive_date_time::PrimitiveDateTime;
pub use time::{Duration, format_description::well_known::Rfc3339};
pub use workflow::{
    AddSecondsParams, AssertFutureParams, ComputeDurationParams, FutureOffsetDateTimeState,
    FutureOffsetProof, OffsetDateTimeFuture, OffsetDateTimeParsed, ParseOffsetParams,
    ParsePrimitiveParams, ParsedOffsetDateTime, ParsedPrimitiveDateTime, PrimitiveDateTimeParsed,
    TimeWorkflowPlugin, UnvalidatedOffsetStr, UnvalidatedPrimitiveStr, parse_rfc3339,
};
