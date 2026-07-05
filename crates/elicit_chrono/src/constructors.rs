//! Static constructor tools for `elicit_chrono` shadow types.
//!
//! Instance methods are exposed via `#[reflect_methods]` on each shadow type.
//! Static methods (no `self` receiver) live here as `#[elicit_tool]` functions.

use crate::{
    DateTime, Days, FixedOffset, Local, LocalResult, Month, Months, NaiveDateDaysIterator,
    NaiveDateWeeksIterator, Parsed, StrftimeItems, TimeDelta, Weekday, WeekdaySet,
};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn json_result<T: Serialize>(value: &T) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(rmcp::model::CallToolResult::success(vec![
        rmcp::model::Content::text(
            serde_json::to_string(value)
                .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
        ),
    ]))
}

/// Parameters for [`TimeDelta::zero`].
#[derive(Debug, Deserialize, JsonSchema)]
struct TimeDeltaZeroParams {}

/// Parameters for [`TimeDelta::max_value`].
#[derive(Debug, Deserialize, JsonSchema)]
struct TimeDeltaMaxValueParams {}

/// Parameters for [`TimeDelta::min_value`].
#[derive(Debug, Deserialize, JsonSchema)]
struct TimeDeltaMinValueParams {}

/// Parameters for [`Local::now`].
#[derive(Debug, Deserialize, JsonSchema)]
struct LocalNowParams {}

/// Parameters for [`Local::today`].
#[derive(Debug, Deserialize, JsonSchema)]
struct LocalTodayParams {}

/// Parameters for [`Local::from_offset`].
#[derive(Debug, Deserialize, JsonSchema)]
struct LocalFromOffsetParams {}

/// Parameters for [`Parsed::new`].
#[derive(Debug, Deserialize, JsonSchema)]
struct ParsedNewParams {}

// ── Weekday constructors ──────────────────────────────────────────────────────

/// Parameters for [`Weekday::from_i64`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WeekdayFromI64Params {
    /// Monday-indexed day number (0 = Monday, 6 = Sunday).
    pub n: i64,
}

/// Parameters for [`Weekday::from_u64`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WeekdayFromU64Params {
    /// Monday-indexed day number (0 = Monday, 6 = Sunday).
    pub n: u64,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "weekday__from_i64",
    description = "Convert a Monday-indexed integer (0–6) to a Weekday. Returns null if out of range."
)]
#[instrument]
async fn weekday_from_i64(
    p: WeekdayFromI64Params,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let w = Weekday::from_i64(p.n);
    json_result(&w)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "weekday__from_u64",
    description = "Convert a Monday-indexed unsigned integer (0–6) to a Weekday. Returns null if out of range."
)]
#[instrument]
async fn weekday_from_u64(
    p: WeekdayFromU64Params,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let w = Weekday::from_u64(p.n);
    json_result(&w)
}

// ── WeekdaySet constructors ───────────────────────────────────────────────────

/// Parameters for [`WeekdaySet::from_array`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WeekdaySetFromArrayParams {
    /// List of weekdays to include in the set.
    pub days: Vec<Weekday>,
}

/// Parameters for [`WeekdaySet::single`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WeekdaySetSingleParams {
    /// The single weekday for the set.
    pub day: Weekday,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "weekday_set__from_array",
    description = "Build a WeekdaySet from a list of Weekday values."
)]
#[instrument]
async fn weekday_set_from_array(
    p: WeekdaySetFromArrayParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = WeekdaySet::from_array(p.days);
    json_result(&s)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "weekday_set__single",
    description = "Build a WeekdaySet containing exactly one weekday."
)]
#[instrument]
async fn weekday_set_single(
    p: WeekdaySetSingleParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = WeekdaySet::single(p.day);
    json_result(&s)
}

// ── Month constructors ────────────────────────────────────────────────────────

/// Parameters for [`Month::from_u32`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MonthFromU32Params {
    /// 1-indexed month number (1 = January, 12 = December).
    pub n: u32,
}

/// Parameters for [`Month::from_i64`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MonthFromI64Params {
    /// 1-indexed month number (1 = January, 12 = December).
    pub n: i64,
}

/// Parameters for [`Month::from_u64`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MonthFromU64Params {
    /// 1-indexed month number (1 = January, 12 = December).
    pub n: u64,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "month__from_u32",
    description = "Convert a 1-indexed month number (1–12) to a Month. Returns null if out of range."
)]
#[instrument]
async fn month_from_u32(p: MonthFromU32Params) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let m = Month::from_u32(p.n);
    json_result(&m)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "month__from_i64",
    description = "Convert a signed 1-indexed month number (1–12) to a Month. Returns null if out of range."
)]
#[instrument]
async fn month_from_i64(p: MonthFromI64Params) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let m = Month::from_i64(p.n);
    json_result(&m)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "month__from_u64",
    description = "Convert an unsigned 1-indexed month number (1–12) to a Month. Returns null if out of range."
)]
#[instrument]
async fn month_from_u64(p: MonthFromU64Params) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let m = Month::from_u64(p.n);
    json_result(&m)
}

// ── Months constructors ───────────────────────────────────────────────────────

/// Parameters for [`Months::new`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MonthsNewParams {
    /// Number of months.
    pub num: u32,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "months__new",
    description = "Create a Months value representing a count of calendar months."
)]
#[instrument]
async fn months_new(p: MonthsNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let m = Months::new(p.num);
    json_result(&m)
}

// ── Days constructors ─────────────────────────────────────────────────────────

/// Parameters for [`Days::new`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DaysNewParams {
    /// Number of days.
    pub days: u64,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "days__new",
    description = "Create a Days value representing a count of calendar days."
)]
#[instrument]
async fn days_new(p: DaysNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = Days::new(p.days);
    json_result(&d)
}

// ── FixedOffset constructors ──────────────────────────────────────────────────

/// Parameters for [`FixedOffset::east_opt`] and [`FixedOffset::west_opt`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FixedOffsetEastOptParams {
    /// Seconds east of UTC (must be in range (-86400, 86400)).
    pub secs: i32,
}

/// Parameters for [`FixedOffset::west_opt`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FixedOffsetWestOptParams {
    /// Seconds west of UTC (must be in range (-86400, 86400)).
    pub secs: i32,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "fixed_offset__east_opt",
    description = "Construct a FixedOffset for the given seconds east of UTC. Returns null if out of range."
)]
#[instrument]
async fn fixed_offset_east_opt(
    p: FixedOffsetEastOptParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let o = FixedOffset::east_opt(p.secs);
    json_result(&o)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "fixed_offset__west_opt",
    description = "Construct a FixedOffset for the given seconds west of UTC. Returns null if out of range."
)]
#[instrument]
async fn fixed_offset_west_opt(
    p: FixedOffsetWestOptParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let o = FixedOffset::west_opt(p.secs);
    json_result(&o)
}

// ── TimeDelta constructors ────────────────────────────────────────────────────

/// Parameters for [`TimeDelta::new`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaNewParams {
    /// Whole seconds.
    pub secs: i64,
    /// Subsecond nanoseconds (0–999_999_999).
    pub nanos: u32,
}

/// Parameters for a single-`i64` TimeDelta constructor (weeks, days, hours, etc.).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaWeeksParams {
    /// Number of weeks.
    pub weeks: i64,
}

/// Parameters for [`TimeDelta::try_days`] / [`TimeDelta::days`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaDaysParams {
    /// Number of days.
    pub days: i64,
}

/// Parameters for [`TimeDelta::try_hours`] / [`TimeDelta::hours`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaHoursParams {
    /// Number of hours.
    pub hours: i64,
}

/// Parameters for [`TimeDelta::try_minutes`] / [`TimeDelta::minutes`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaMinutesParams {
    /// Number of minutes.
    pub mins: i64,
}

/// Parameters for [`TimeDelta::try_seconds`] / [`TimeDelta::seconds`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaSecondsParams {
    /// Number of seconds.
    pub secs: i64,
}

/// Parameters for [`TimeDelta::try_milliseconds`] / [`TimeDelta::milliseconds`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaMillisecondsParams {
    /// Number of milliseconds.
    pub millis: i64,
}

/// Parameters for [`TimeDelta::microseconds`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaMicrosecondsParams {
    /// Number of microseconds.
    pub micros: i64,
}

/// Parameters for [`TimeDelta::nanoseconds`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaNanosecondsParams {
    /// Number of nanoseconds.
    pub nanos: i64,
}

/// Parameters for [`TimeDelta::from_std`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeDeltaFromStdParams {
    /// `std::time::Duration` as whole seconds.
    pub secs: u64,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__zero",
    description = "Return the zero TimeDelta (duration of exactly zero)."
)]
#[instrument]
async fn time_delta_zero(
    _p: TimeDeltaZeroParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::zero();
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__new",
    description = "Construct a TimeDelta from whole seconds and subsecond nanoseconds. Returns null on overflow."
)]
#[instrument]
async fn time_delta_new(p: TimeDeltaNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::new(p.secs, p.nanos);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__try_weeks",
    description = "Construct a TimeDelta from whole weeks. Returns null on overflow."
)]
#[instrument]
async fn time_delta_try_weeks(
    p: TimeDeltaWeeksParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::try_weeks(p.weeks);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__try_days",
    description = "Construct a TimeDelta from whole days. Returns null on overflow."
)]
#[instrument]
async fn time_delta_try_days(
    p: TimeDeltaDaysParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::try_days(p.days);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__try_hours",
    description = "Construct a TimeDelta from whole hours. Returns null on overflow."
)]
#[instrument]
async fn time_delta_try_hours(
    p: TimeDeltaHoursParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::try_hours(p.hours);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__try_minutes",
    description = "Construct a TimeDelta from whole minutes. Returns null on overflow."
)]
#[instrument]
async fn time_delta_try_minutes(
    p: TimeDeltaMinutesParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::try_minutes(p.mins);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__try_seconds",
    description = "Construct a TimeDelta from whole seconds. Returns null on overflow."
)]
#[instrument]
async fn time_delta_try_seconds(
    p: TimeDeltaSecondsParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::try_seconds(p.secs);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__try_milliseconds",
    description = "Construct a TimeDelta from whole milliseconds. Returns null on overflow."
)]
#[instrument]
async fn time_delta_try_milliseconds(
    p: TimeDeltaMillisecondsParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::try_milliseconds(p.millis);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__microseconds",
    description = "Construct a TimeDelta from whole microseconds."
)]
#[instrument]
async fn time_delta_microseconds(
    p: TimeDeltaMicrosecondsParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::microseconds(p.micros);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__nanoseconds",
    description = "Construct a TimeDelta from whole nanoseconds."
)]
#[instrument]
async fn time_delta_nanoseconds(
    p: TimeDeltaNanosecondsParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::nanoseconds(p.nanos);
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__max_value",
    description = "Return the maximum representable TimeDelta."
)]
#[instrument]
async fn time_delta_max_value(
    _p: TimeDeltaMaxValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::max_value();
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__min_value",
    description = "Return the minimum representable TimeDelta."
)]
#[instrument]
async fn time_delta_min_value(
    _p: TimeDeltaMinValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::min_value();
    json_result(&d)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "time_delta__from_std",
    description = "Convert a std::time::Duration (expressed as whole seconds) to a TimeDelta. Returns null on overflow or negative."
)]
#[instrument]
async fn time_delta_from_std(
    p: TimeDeltaFromStdParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = TimeDelta::from_std(p.secs);
    json_result(&d)
}

// ── Local constructors ────────────────────────────────────────────────────────

/// Parameters for [`Local::offset_from_local_date`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocalOffsetFromLocalDateParams {
    /// The local (wall-clock) date to look up.
    pub date: crate::NaiveDate,
}

/// Parameters for [`Local::offset_from_local_datetime`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocalOffsetFromLocalDatetimeParams {
    /// The local (wall-clock) datetime to look up.
    pub dt: crate::NaiveDateTime,
}

/// Parameters for [`Local::offset_from_utc_date`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocalOffsetFromUtcDateParams {
    /// The UTC date to look up.
    pub date: crate::NaiveDate,
}

/// Parameters for [`Local::offset_from_utc_datetime`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocalOffsetFromUtcDatetimeParams {
    /// The UTC datetime to look up.
    pub dt: crate::NaiveDateTime,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local__now",
    description = "Return the current local time as a UTC DateTime."
)]
#[instrument]
async fn local_now(_p: LocalNowParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let dt = Local::now();
    json_result(&dt)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local__today",
    description = "Return the current local date as an ISO 8601 string (deprecated; prefer local__now)."
)]
#[instrument]
async fn local_today(_p: LocalTodayParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = Local::today();
    json_result(&s)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local__from_offset",
    description = "Return a Local timezone marker instance."
)]
#[instrument]
async fn local_from_offset(
    _p: LocalFromOffsetParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let l = Local::from_offset();
    json_result(&l)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local__offset_from_local_date",
    description = "Return the local UTC offset that applies to the given naive (wall-clock) date, as a formatted string."
)]
#[instrument]
async fn local_offset_from_local_date(
    p: LocalOffsetFromLocalDateParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = Local::offset_from_local_date(p.date);
    json_result(&s)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local__offset_from_local_datetime",
    description = "Return the local UTC offset that applies to the given naive (wall-clock) datetime, as a formatted string."
)]
#[instrument]
async fn local_offset_from_local_datetime(
    p: LocalOffsetFromLocalDatetimeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = Local::offset_from_local_datetime(p.dt);
    json_result(&s)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local__offset_from_utc_date",
    description = "Return the local UTC offset that applies to the given UTC date, as a formatted string."
)]
#[instrument]
async fn local_offset_from_utc_date(
    p: LocalOffsetFromUtcDateParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = Local::offset_from_utc_date(p.date);
    json_result(&s)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local__offset_from_utc_datetime",
    description = "Return the local UTC offset that applies to the given UTC datetime, as a formatted string."
)]
#[instrument]
async fn local_offset_from_utc_datetime(
    p: LocalOffsetFromUtcDatetimeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = Local::offset_from_utc_datetime(p.dt);
    json_result(&s)
}

// ── LocalResult constructors ──────────────────────────────────────────────────

/// Parameters for [`LocalResult::from_single`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocalResultFromSingleParams {
    /// The UTC DateTime to wrap.
    pub dt: DateTime,
}

/// Parameters for [`LocalResult::from_wrap`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocalResultFromWrapParams {
    /// The LocalResultWrap to convert.
    pub wrap: elicitation::LocalResultWrap<DateTime>,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local_result__from_single",
    description = "Build a LocalResult::Single from a UTC DateTime."
)]
#[instrument]
async fn local_result_from_single(
    p: LocalResultFromSingleParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let r = LocalResult::from_single(p.dt);
    json_result(&r)
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "local_result__from_wrap",
    description = "Convert a LocalResultWrap<DateTime> to a LocalResult."
)]
#[instrument]
async fn local_result_from_wrap(
    p: LocalResultFromWrapParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let r = LocalResult::from_wrap(p.wrap);
    json_result(&r)
}

// ── Parsed constructors ───────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "parsed__new",
    description = "Create a new, empty Parsed date/time field collector."
)]
#[instrument]
async fn parsed_new(_p: ParsedNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let parsed = Parsed::new();
    json_result(&parsed)
}

// ── StrftimeItems constructors ────────────────────────────────────────────────

/// Parameters for [`StrftimeItems::new`] and variants.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StrftimeItemsNewParams {
    /// strftime format string (e.g. `"%Y-%m-%d"`).
    pub fmt: String,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "strftime_items__new",
    description = "Parse a strftime format string into a StrftimeItems sequence."
)]
#[instrument]
async fn strftime_items_new(
    p: StrftimeItemsNewParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = StrftimeItems::new(p.fmt);
    json_result(&s)
}

/// Parameters for [`StrftimeItems::new_lenient`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StrftimeItemsNewLenientParams {
    /// strftime format string; unknown specs become literals.
    pub fmt: String,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "strftime_items__new_lenient",
    description = "Parse a strftime format string leniently; unknown specifiers become literal text."
)]
#[instrument]
async fn strftime_items_new_lenient(
    p: StrftimeItemsNewLenientParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = StrftimeItems::new_lenient(p.fmt);
    json_result(&s)
}

/// Parameters for [`StrftimeItems::parse`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StrftimeItemsParseParams {
    /// strftime format string to parse strictly.
    pub fmt: String,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "strftime_items__parse",
    description = "Parse a strftime format string strictly; returns an error string if any item is invalid."
)]
#[instrument]
async fn strftime_items_parse(
    p: StrftimeItemsParseParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = StrftimeItems::parse(p.fmt).map_err(|e| ErrorData::invalid_params(e, None))?;
    json_result(&s)
}

/// Parameters for [`StrftimeItems::parse_to_owned`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StrftimeItemsParseToOwnedParams {
    /// strftime format string to parse into owned static items.
    pub fmt: String,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "strftime_items__parse_to_owned",
    description = "Parse a strftime format string into owned (static) items; returns an error string if invalid."
)]
#[instrument]
async fn strftime_items_parse_to_owned(
    p: StrftimeItemsParseToOwnedParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let s = StrftimeItems::parse_to_owned(p.fmt).map_err(|e| ErrorData::invalid_params(e, None))?;
    json_result(&s)
}

// ── NaiveDateDaysIterator constructors ────────────────────────────────────────

/// Parameters for [`NaiveDateDaysIterator::new`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NaiveDateDaysIteratorNewParams {
    /// Starting date for the days iterator.
    pub start: crate::NaiveDate,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "naive_date_days_iterator__new",
    description = "Create a NaiveDateDaysIterator starting at the given date."
)]
#[instrument]
async fn naive_date_days_iterator_new(
    p: NaiveDateDaysIteratorNewParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let it = NaiveDateDaysIterator::new(p.start);
    json_result(&it)
}

// ── NaiveDateWeeksIterator constructors ───────────────────────────────────────

/// Parameters for [`NaiveDateWeeksIterator::new`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NaiveDateWeeksIteratorNewParams {
    /// Starting date for the weeks iterator.
    pub start: crate::NaiveDate,
}

#[elicit_tool(
    plugin = "chrono_constructors",
    name = "naive_date_weeks_iterator__new",
    description = "Create a NaiveDateWeeksIterator starting at the given date."
)]
#[instrument]
async fn naive_date_weeks_iterator_new(
    p: NaiveDateWeeksIteratorNewParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let it = NaiveDateWeeksIterator::new(p.start);
    json_result(&it)
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing static constructors for all new `elicit_chrono` shadow types.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "chrono_constructors")]
pub struct ChronoConstructorsPlugin;
