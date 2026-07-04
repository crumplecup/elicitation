//! `elicit_chrono` — elicitation-enabled wrappers around `chrono` datetime types.
//!
//! Provides [`DateTime`], [`DateTimeFixed`], and [`NaiveDateTime`] newtypes with:
//! - [`schemars::JsonSchema`] (delegated to inner chrono types)
//! - [`serde::Serialize`] / [`serde::Deserialize`] (transparent, RFC 3339)
//! - MCP reflect methods for field access and formatting
//! - [`ChronoWorkflowPlugin`]: contract-verified datetime composition tools

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod date_time;
mod date_time_fixed;
mod date_time_registry;
mod duration;
mod naive_date;
mod naive_date_iter;
mod naive_date_registry;
mod naive_date_time;
mod naive_date_time_registry;
mod naive_time;
mod naive_time_registry;
mod utc;
pub mod workflow;

pub use date_time::DateTime;
pub use date_time_fixed::DateTimeFixed;
pub use date_time_registry::{DateTimeRegistryPlugin, UtcCreateParams};
pub use duration::Duration;
pub use naive_date::NaiveDate;
pub use naive_date_iter::{
    IterDaysParams, IterDropParams, IterNextParams, IterWeeksParams, NaiveDateIterPlugin,
};
pub use naive_date_registry::{DateCreateParams, NaiveDateRegistryPlugin};
pub use naive_date_time::NaiveDateTime;
pub use naive_date_time_registry::NaiveDateTimeRegistryPlugin;
pub use naive_time::NaiveTime;
pub use naive_time_registry::{NaiveTimeRegistryPlugin, TimeCreateParams};
pub use utc::Utc;
pub use workflow::{
    AddSecondsParams, AssertFutureParams, AssertInRangeParams, ChronoWorkflowPlugin,
    ComputeDurationParams, DateTimeFuture, DateTimeInRange, DateTimeParsed, FutureDateTimeProof,
    FutureDateTimeState, ParseDatetimeParams, ParsedDateTime, RangedDateTimeProof,
    RangedDateTimeState, UnvalidatedDateStr,
};
