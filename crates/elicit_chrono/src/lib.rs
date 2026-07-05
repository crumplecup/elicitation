//! `elicit_chrono` — elicitation-enabled wrappers around `chrono` datetime types.
//!
//! Provides [`DateTime`], [`DateTimeFixed`], and [`NaiveDateTime`] newtypes with:
//! - [`schemars::JsonSchema`] (delegated to inner chrono types)
//! - [`serde::Serialize`] / [`serde::Deserialize`] (transparent, RFC 3339)
//! - MCP reflect methods for field access and formatting
//! - [`ChronoWorkflowPlugin`]: contract-verified datetime composition tools

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod constructors;
mod date_iters;
mod date_time;
mod date_time_fixed;
mod date_time_registry;
mod days;
mod duration;
mod fixed_offset;
mod format_types;
mod iso_week;
mod local;
mod local_result;
mod month;
mod months;
mod naive_date;
mod naive_date_iter;
mod naive_date_registry;
mod naive_date_time;
mod naive_date_time_registry;
mod naive_time;
mod naive_time_registry;
mod parsed;
mod strftime_items;
mod strftime_registry;
mod time_delta;
mod utc;
mod weekday;
mod weekday_set;
pub mod workflow;

pub use constructors::ChronoConstructorsPlugin;
pub use date_iters::{NaiveDateDaysIterator, NaiveDateWeeksIterator};
pub use date_time::DateTime;
pub use date_time_fixed::DateTimeFixed;
pub use date_time_registry::{DateTimeRegistryPlugin, UtcCreateParams};
pub use days::Days;
pub use duration::Duration;
pub use fixed_offset::FixedOffset;
pub use format_types::{
    Colons, Fixed, InternalFixed, InternalNumeric, Numeric, OffsetFormat, OffsetPrecision,
    OutOfRange, OutOfRangeError, Pad, ParseError, ParseErrorKind, ParseMonthError,
    ParseWeekdayError, RoundingError, SecondsFormat,
};
pub use iso_week::{IsoWeek, NaiveWeek};
pub use local::Local;
pub use local_result::LocalResult;
pub use month::Month;
pub use months::Months;
pub use naive_date::NaiveDate;
pub use naive_date_iter::{
    IterDaysParams, IterDropParams, IterNextParams, IterWeeksParams, NaiveDateIterPlugin,
};
pub use naive_date_registry::{DateCreateParams, NaiveDateRegistryPlugin};
pub use naive_date_time::NaiveDateTime;
pub use naive_date_time_registry::NaiveDateTimeRegistryPlugin;
pub use naive_time::NaiveTime;
pub use naive_time_registry::{NaiveTimeRegistryPlugin, TimeCreateParams};
pub use parsed::Parsed;
pub use strftime_items::{Item, StrftimeItems};
pub use strftime_registry::{
    StrftimeCreateParams, StrftimeDropParams, StrftimeItemsPlugin, StrftimeNextParams,
};
pub use time_delta::TimeDelta;
pub use utc::Utc;
pub use weekday::Weekday;
pub use weekday_set::WeekdaySet;
pub use workflow::{
    AddSecondsParams, AssertFutureParams, AssertInRangeParams, ChronoWorkflowPlugin,
    ComputeDurationParams, DateTimeFuture, DateTimeInRange, DateTimeParsed, FutureDateTimeProof,
    FutureDateTimeState, ParseDatetimeParams, ParsedDateTime, RangedDateTimeProof,
    RangedDateTimeState, UnvalidatedDateStr,
};
