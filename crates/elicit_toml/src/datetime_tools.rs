//! Tools for constructing and inspecting `toml_datetime` types:
//! `Date`, `Time`, `Offset`, and `Datetime`.

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::plugin::{err_text, ok_json, ok_text};

// ── Date: new ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DateNewParams {
    /// Year (e.g. 2024).
    pub year: u16,
    /// Month 1–12.
    pub month: u8,
    /// Day 1–31.
    pub day: u8,
}

/// Construct a `toml_datetime::Date` and return it as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__date_new",
    description = "Construct a toml_datetime::Date from year, month, day and return as JSON."
)]
async fn date_new(p: DateNewParams) -> Result<CallToolResult, ErrorData> {
    let d = toml_datetime::Date {
        year: p.year,
        month: p.month,
        day: p.day,
    };
    ok_json(&d)
}

// ── Time: new ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeNewParams {
    /// Hour 0–23.
    pub hour: u8,
    /// Minute 0–59.
    pub minute: u8,
    /// Second 0–60 (allows leap second 60).
    pub second: u8,
    /// Nanosecond 0–999_999_999.
    #[serde(default)]
    pub nanosecond: u32,
}

/// Construct a `toml_datetime::Time` and return it as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__time_new",
    description = "Construct a toml_datetime::Time from hour, minute, second, nanosecond and return as JSON."
)]
async fn time_new(p: TimeNewParams) -> Result<CallToolResult, ErrorData> {
    let t = toml_datetime::Time {
        hour: p.hour,
        minute: p.minute,
        second: Some(p.second),
        nanosecond: Some(p.nanosecond),
    };
    ok_json(&t)
}

// ── Offset: UTC ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OffsetUtcParams {}

/// Construct the UTC `toml_datetime::Offset::Z` and return it as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__offset_utc",
    description = "Return the UTC toml_datetime::Offset (Offset::Z) as JSON."
)]
async fn offset_utc(_p: OffsetUtcParams) -> Result<CallToolResult, ErrorData> {
    ok_json(&elicitation::TomlOffset::Z)
}

// ── Offset: custom ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OffsetCustomParams {
    /// Hours component, -23 to +23.
    pub hours: i8,
    /// Minutes component, 0–59.
    pub minutes: u8,
}

/// Construct a custom `toml_datetime::Offset` and return it as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__offset_custom",
    description = "Construct a custom toml_datetime::Offset (e.g. +05:30) from hours and minutes and return as JSON."
)]
async fn offset_custom(p: OffsetCustomParams) -> Result<CallToolResult, ErrorData> {
    ok_json(&elicitation::TomlOffset::Custom {
        hours: p.hours,
        minutes: p.minutes,
    })
}

// ── Datetime: offset datetime ────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeOffsetParams {
    /// Year (e.g. 2024).
    pub year: u16,
    /// Month 1–12.
    pub month: u8,
    /// Day 1–31.
    pub day: u8,
    /// Hour 0–23.
    pub hour: u8,
    /// Minute 0–59.
    pub minute: u8,
    /// Second 0–60.
    pub second: u8,
    /// Nanosecond 0–999_999_999.
    #[serde(default)]
    pub nanosecond: u32,
    /// UTC offset: "Z" for UTC, or "+HH:MM" / "-HH:MM".
    #[serde(default = "default_utc")]
    pub offset: String,
}

fn default_utc() -> String {
    "Z".to_string()
}

/// Construct an offset `toml_datetime::Datetime` (date + time + UTC offset) and return as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__offset_datetime",
    description = "Construct an offset toml_datetime::Datetime (date + time + UTC or custom offset) and return as JSON."
)]
async fn offset_datetime(p: DatetimeOffsetParams) -> Result<CallToolResult, ErrorData> {
    let offset = parse_offset_str(&p.offset)?;
    let dt = toml_datetime::Datetime {
        date: Some(toml_datetime::Date {
            year: p.year,
            month: p.month,
            day: p.day,
        }),
        time: Some(toml_datetime::Time {
            hour: p.hour,
            minute: p.minute,
            second: Some(p.second),
            nanosecond: Some(p.nanosecond),
        }),
        offset: Some(offset),
    };
    ok_json(&dt)
}

// ── Datetime: local datetime ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeLocalParams {
    /// Year (e.g. 2024).
    pub year: u16,
    /// Month 1–12.
    pub month: u8,
    /// Day 1–31.
    pub day: u8,
    /// Hour 0–23.
    pub hour: u8,
    /// Minute 0–59.
    pub minute: u8,
    /// Second 0–60.
    pub second: u8,
    /// Nanosecond 0–999_999_999.
    #[serde(default)]
    pub nanosecond: u32,
}

/// Construct a local `toml_datetime::Datetime` (date + time, no offset) and return as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__local_datetime",
    description = "Construct a local toml_datetime::Datetime (date + time, no UTC offset) and return as JSON."
)]
async fn local_datetime(p: DatetimeLocalParams) -> Result<CallToolResult, ErrorData> {
    let dt = toml_datetime::Datetime {
        date: Some(toml_datetime::Date {
            year: p.year,
            month: p.month,
            day: p.day,
        }),
        time: Some(toml_datetime::Time {
            hour: p.hour,
            minute: p.minute,
            second: Some(p.second),
            nanosecond: Some(p.nanosecond),
        }),
        offset: None,
    };
    ok_json(&dt)
}

// ── Datetime: local date ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeDateOnlyParams {
    /// Year (e.g. 2024).
    pub year: u16,
    /// Month 1–12.
    pub month: u8,
    /// Day 1–31.
    pub day: u8,
}

/// Construct a local-date `toml_datetime::Datetime` (date only, no time or offset) and return as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__local_date",
    description = "Construct a local-date toml_datetime::Datetime (date only) and return as JSON."
)]
async fn local_date(p: DatetimeDateOnlyParams) -> Result<CallToolResult, ErrorData> {
    let dt = toml_datetime::Datetime {
        date: Some(toml_datetime::Date {
            year: p.year,
            month: p.month,
            day: p.day,
        }),
        time: None,
        offset: None,
    };
    ok_json(&dt)
}

// ── Datetime: local time ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeTimeOnlyParams {
    /// Hour 0–23.
    pub hour: u8,
    /// Minute 0–59.
    pub minute: u8,
    /// Second 0–60.
    pub second: u8,
    /// Nanosecond 0–999_999_999.
    #[serde(default)]
    pub nanosecond: u32,
}

/// Construct a local-time `toml_datetime::Datetime` (time only, no date or offset) and return as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__local_time",
    description = "Construct a local-time toml_datetime::Datetime (time only) and return as JSON."
)]
async fn local_time(p: DatetimeTimeOnlyParams) -> Result<CallToolResult, ErrorData> {
    let dt = toml_datetime::Datetime {
        date: None,
        time: Some(toml_datetime::Time {
            hour: p.hour,
            minute: p.minute,
            second: Some(p.second),
            nanosecond: Some(p.nanosecond),
        }),
        offset: None,
    };
    ok_json(&dt)
}

// ── Datetime: parse from string ───────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeParseParams {
    /// TOML datetime string, e.g. `"2024-01-15T12:00:00Z"` or `"2024-01-15"`.
    pub datetime_str: String,
}

/// Parse a TOML datetime string into a `toml_datetime::Datetime` and return it as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__parse",
    description = "Parse a TOML datetime string (e.g. '2024-01-15T12:00:00Z') into a toml_datetime::Datetime."
)]
async fn datetime_parse(p: DatetimeParseParams) -> Result<CallToolResult, ErrorData> {
    // Parse via a synthetic TOML document (toml_datetime implements FromStr)
    let wrapper = format!("__v = {}", p.datetime_str);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    match doc.get("__v").and_then(|i| i.as_value()).and_then(|v| {
        if let toml_edit::Value::Datetime(f) = v {
            Some(*f.value())
        } else {
            None
        }
    }) {
        Some(dt) => ok_json(&dt),
        None => err_text(format!(
            "'{}' is not a valid TOML datetime string",
            p.datetime_str
        )),
    }
}

// ── Datetime: to TOML string ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeToStringParams {
    /// JSON-serialized `toml_datetime::Datetime`.
    pub datetime_json: String,
}

/// Serialize a JSON-encoded `toml_datetime::Datetime` back to a TOML datetime string.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__to_string",
    description = "Serialize a JSON-encoded toml_datetime::Datetime to its TOML text representation."
)]
async fn datetime_to_string(p: DatetimeToStringParams) -> Result<CallToolResult, ErrorData> {
    let dt: toml_datetime::Datetime = serde_json::from_str(&p.datetime_json)
        .map_err(|e| ErrorData::invalid_params(format!("JSON parse error: {}", e), None))?;
    ok_text(dt.to_string())
}

// ── Datetime: variants reference ─────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DatetimeVariantsParams {}

/// Return a reference table of all TOML datetime variants.
#[elicit_tool(
    plugin = "toml",
    name = "toml__datetime__variants_reference",
    description = "Return a reference table describing the four TOML datetime variants and example strings."
)]
async fn datetime_variants_reference(
    _p: DatetimeVariantsParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(
        "TOML datetime variants (toml_datetime::Datetime):\n\
         \n\
         1. Offset Date-Time (date + time + offset):\n\
            Example: 1979-05-27T07:32:00Z\n\
            Example: 1979-05-27T00:32:00-07:00\n\
            Example: 1979-05-27T00:32:00.999999999Z\n\
         \n\
         2. Local Date-Time (date + time, no offset):\n\
            Example: 1979-05-27T07:32:00\n\
            Example: 1979-05-27T00:32:00.999999\n\
         \n\
         3. Local Date (date only):\n\
            Example: 1979-05-27\n\
         \n\
         4. Local Time (time only):\n\
            Example: 07:32:00\n\
            Example: 00:32:00.999999999\n\
         \n\
         Datetime { date: Option<Date>, time: Option<Time>, offset: Option<Offset> }\n\
         Date   { year: u16, month: u8, day: u8 }\n\
         Time   { hour: u8, minute: u8, second: u8, nanosecond: u32 }\n\
         Offset::Z | Offset::Custom { hours: i8, minutes: u8 }",
    )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_offset_str(s: &str) -> Result<toml_datetime::Offset, ErrorData> {
    if s.eq_ignore_ascii_case("z") || s == "+00:00" || s == "-00:00" {
        return Ok(toml_datetime::Offset::Z);
    }
    // Expect ±HH:MM
    let (sign, rest) = if let Some(r) = s.strip_prefix('+') {
        (1i8, r)
    } else if let Some(r) = s.strip_prefix('-') {
        (-1i8, r)
    } else {
        return Err(ErrorData::invalid_params(
            format!(
                "invalid offset '{}': expected 'Z', '+HH:MM', or '-HH:MM'",
                s
            ),
            None,
        ));
    };
    let parts: Vec<&str> = rest.split(':').collect();
    if parts.len() != 2 {
        return Err(ErrorData::invalid_params(
            format!("invalid offset '{}': expected HH:MM after sign", s),
            None,
        ));
    }
    let hours: i8 = parts[0]
        .parse::<u8>()
        .map(|h| sign * h as i8)
        .map_err(|_| ErrorData::invalid_params(format!("invalid hours in offset '{}'", s), None))?;
    let minutes: u8 = parts[1].parse().map_err(|_| {
        ErrorData::invalid_params(format!("invalid minutes in offset '{}'", s), None)
    })?;
    Ok(toml_datetime::Offset::Custom {
        minutes: hours as i16 * 60 + minutes as i16,
    })
}
