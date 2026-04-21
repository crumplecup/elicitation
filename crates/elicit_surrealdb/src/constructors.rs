//! Static constructor tools for `surrealdb-types` wrapper types.
//!
//! Instance methods are exposed via `#[reflect_methods]` on each wrapper.
//! Static methods (no `self` receiver) live here as `#[elicit_tool]` functions.

use crate::{Datetime, Duration, Kind, Number, RecordId, Table};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Empty parameter struct for no-argument tools.
#[derive(Debug, Deserialize, JsonSchema)]
struct EmptyParams {}

fn json_result<T: Serialize>(value: &T) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(rmcp::model::CallToolResult::success(vec![
        rmcp::model::Content::text(
            serde_json::to_string(value)
                .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
        ),
    ]))
}

// ── Duration constructors ─────────────────────────────────────────────────────

/// Parameters for [`Duration::new`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DurationNewParams {
    /// Whole seconds.
    pub secs: u64,
    /// Subsecond nanoseconds.
    pub nanos: u32,
}

/// Parameters for [`Duration::from_secs`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DurationFromSecsParams {
    /// Whole seconds.
    pub secs: u64,
}

/// Parameters for [`Duration::from_millis`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DurationFromMillisParams {
    /// Milliseconds.
    pub millis: u64,
}

/// Parameters for [`Duration::from_mins`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DurationFromMinsParams {
    /// Minutes.
    pub mins: u64,
}

/// Parameters for [`Duration::from_hours`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DurationFromHoursParams {
    /// Hours.
    pub hours: u64,
}

/// Parameters for [`Duration::from_days`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DurationFromDaysParams {
    /// Days.
    pub days: u64,
}

/// Parameters for [`Duration::from_weeks`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DurationFromWeeksParams {
    /// Weeks.
    pub weeks: u64,
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "duration__new",
    description = "Create a SurrealDB Duration from whole seconds and subsecond nanoseconds."
)]
#[instrument]
async fn duration_new(p: DurationNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = Duration::from(surrealdb_types::Duration::new(p.secs, p.nanos));
    json_result(&d)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "duration__from_secs",
    description = "Create a SurrealDB Duration from a whole number of seconds."
)]
#[instrument]
async fn duration_from_secs(
    p: DurationFromSecsParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = Duration::from(surrealdb_types::Duration::from_secs(p.secs));
    json_result(&d)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "duration__from_millis",
    description = "Create a SurrealDB Duration from milliseconds."
)]
#[instrument]
async fn duration_from_millis(
    p: DurationFromMillisParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = Duration::from(surrealdb_types::Duration::from_millis(p.millis));
    json_result(&d)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "duration__from_mins",
    description = "Create a SurrealDB Duration from minutes. Returns null if the value overflows."
)]
#[instrument]
async fn duration_from_mins(
    p: DurationFromMinsParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = surrealdb_types::Duration::from_mins(p.mins).map(Duration::from);
    json_result(&d)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "duration__from_hours",
    description = "Create a SurrealDB Duration from hours. Returns null if the value overflows."
)]
#[instrument]
async fn duration_from_hours(
    p: DurationFromHoursParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = surrealdb_types::Duration::from_hours(p.hours).map(Duration::from);
    json_result(&d)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "duration__from_days",
    description = "Create a SurrealDB Duration from days. Returns null if the value overflows."
)]
#[instrument]
async fn duration_from_days(
    p: DurationFromDaysParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = surrealdb_types::Duration::from_days(p.days).map(Duration::from);
    json_result(&d)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "duration__from_weeks",
    description = "Create a SurrealDB Duration from weeks. Returns null if the value overflows."
)]
#[instrument]
async fn duration_from_weeks(
    p: DurationFromWeeksParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = surrealdb_types::Duration::from_weeks(p.weeks).map(Duration::from);
    json_result(&d)
}

// ── Datetime constructors ─────────────────────────────────────────────────────

/// Parameters for [`Datetime::from_timestamp`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeFromTimestampParams {
    /// Unix timestamp seconds.
    pub seconds: i64,
    /// Subsecond nanoseconds.
    pub nanos: u32,
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "datetime__now",
    description = "Create a SurrealDB Datetime representing the current UTC time."
)]
#[instrument]
async fn datetime_now(_p: EmptyParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = Datetime::from(surrealdb_types::Datetime::now());
    json_result(&d)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "datetime__from_timestamp",
    description = "Create a SurrealDB Datetime from a Unix timestamp. Returns null if out of range."
)]
#[instrument]
async fn datetime_from_timestamp(
    p: DatetimeFromTimestampParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let d = surrealdb_types::Datetime::from_timestamp(p.seconds, p.nanos).map(Datetime::from);
    json_result(&d)
}

// ── Table constructors ────────────────────────────────────────────────────────

/// Parameters for [`Table::new`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableNewParams {
    /// Table name.
    pub name: String,
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "table__new",
    description = "Create a SurrealDB Table identifier from a name string."
)]
#[instrument]
async fn table_new(p: TableNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let t = Table::from(surrealdb_types::Table::new(p.name));
    json_result(&t)
}

// ── RecordId constructors ─────────────────────────────────────────────────────

/// Parameters for creating a record ID from a table name and string key.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RecordIdFromTableKeyParams {
    /// Table name.
    pub table: String,
    /// String key.
    pub key: String,
}

/// Parameters for creating a record ID from a table name and numeric key.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RecordIdFromTableIdParams {
    /// Table name.
    pub table: String,
    /// Numeric key.
    pub id: i64,
}

/// Parameters for parsing a simple `table:key` record ID string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RecordIdParseSimpleParams {
    /// Record ID string, e.g. `person:john`.
    pub s: String,
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "record_id__from_table_key",
    description = "Create a SurrealDB RecordId from a table name and a string key."
)]
#[instrument]
async fn record_id_from_table_key(
    p: RecordIdFromTableKeyParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let r = RecordId::from(surrealdb_types::RecordId::new(p.table, p.key));
    json_result(&r)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "record_id__from_table_id",
    description = "Create a SurrealDB RecordId from a table name and a numeric key."
)]
#[instrument]
async fn record_id_from_table_id(
    p: RecordIdFromTableIdParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let r = RecordId::from(surrealdb_types::RecordId::new(
        p.table,
        surrealdb_types::RecordIdKey::Number(p.id),
    ));
    json_result(&r)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "record_id__parse_simple",
    description = "Parse a simple `table:key` record ID string. Returns null if parsing fails."
)]
#[instrument]
async fn record_id_parse_simple(
    p: RecordIdParseSimpleParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let r = surrealdb_types::RecordId::parse_simple(&p.s)
        .ok()
        .map(RecordId::from);
    json_result(&r)
}

// ── Number constructors ───────────────────────────────────────────────────────

/// Parameters for [`Number::from_int`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NumberFromIntParams {
    /// 64-bit integer value.
    pub v: i64,
}

/// Parameters for [`Number::from_float`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NumberFromFloatParams {
    /// 64-bit float value.
    pub v: f64,
}

/// Parameters for creating a Number from a decimal string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NumberFromDecimalStrParams {
    /// Decimal string, e.g. `"3.14"`.
    pub s: String,
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "number__from_int",
    description = "Create a SurrealDB Number from a 64-bit integer."
)]
#[instrument]
async fn number_from_int(p: NumberFromIntParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let n = Number::from(surrealdb_types::Number::from_int(p.v));
    json_result(&n)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "number__from_float",
    description = "Create a SurrealDB Number from a 64-bit float."
)]
#[instrument]
async fn number_from_float(
    p: NumberFromFloatParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let n = Number::from(surrealdb_types::Number::from_float(p.v));
    json_result(&n)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "number__from_decimal_str",
    description = "Create a SurrealDB Number from a decimal string (e.g. \"3.14\"). Returns null if unparseable."
)]
#[instrument]
async fn number_from_decimal_str(
    p: NumberFromDecimalStrParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let n =
        p.s.parse::<rust_decimal::Decimal>()
            .ok()
            .map(|d| Number::from(surrealdb_types::Number::from_decimal(d)));
    json_result(&n)
}

// ── Kind constructors ─────────────────────────────────────────────────────────

/// Parameters for [`Kind::either`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KindEitherParams {
    /// List of kinds to combine.
    pub kinds: Vec<Kind>,
}

/// Parameters for [`Kind::option`].
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KindOptionParams {
    /// Kind to wrap as optional.
    pub kind: Kind,
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "kind__either",
    description = "Build a SurrealDB Kind::Either from a list of kinds, deduplicating and simplifying."
)]
#[instrument]
async fn kind_either(p: KindEitherParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let inner: Vec<surrealdb_types::Kind> = p
        .kinds
        .into_iter()
        .map(surrealdb_types::Kind::from)
        .collect();
    let k = Kind::from(surrealdb_types::Kind::either(inner));
    json_result(&k)
}

#[elicit_tool(
    plugin = "surreal_constructors",
    name = "kind__option",
    description = "Wrap a SurrealDB Kind as optional (shorthand for Either([None, kind]))."
)]
#[instrument]
async fn kind_option(p: KindOptionParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let inner = surrealdb_types::Kind::from(p.kind);
    let k = Kind::from(surrealdb_types::Kind::option(inner));
    json_result(&k)
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing static constructors for all `surrealdb-types` wrappers.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "surreal_constructors")]
pub struct SurrealConstructorsPlugin;
