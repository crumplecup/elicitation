//! Compile-time compatibility matrix for every third-party type we impl Elicitation on.
//!
//! # Why this exists
//!
//! Implementing `Elicitation` for a third-party type is necessary but not sufficient for
//! MCP tool use. The MCP tool registration layer also requires:
//!
//! - [`schemars::JsonSchema`] — generates the input schema for the tool
//! - [`serde::Serialize`] + [`serde::de::DeserializeOwned`] — encodes/decodes arguments
//!
//! This file tests all three bounds for every supported third-party type by calling a
//! generic helper that requires them. If any type is missing a bound, the *entire file
//! fails to compile*, making the gap impossible to miss.
//!
//! # How to read failures
//!
//! A compile error like:
//! ```text
//! the trait `JsonSchema` is not implemented for `time::OffsetDateTime`
//! ```
//! means that type needs either:
//! - a `schemars` feature flag wired into the elicitation feature (if schemars supports it), OR
//! - a shadow crate (e.g. `elicit_time`) that provides a newtype with `JsonSchema` derived.
//!
//! # Status
//!
//! Types are grouped by their shadow-crate status:
//!
//! | Type                            | serde  | JsonSchema | Status                     |
//! |---------------------------------|--------|------------|----------------------------|
//! | `uuid::Uuid`                    | ✅     | ✅ (via schemars/uuid1)  | ✅ wire feature  |
//! | `url::Url`                      | ✅     | ✅ (via schemars/url2)   | ✅ wire feature  |
//! | `chrono::DateTime<Utc>`         | ✅     | ✅ (via schemars/chrono04)| ✅ wire feature |
//! | `chrono::DateTime<FixedOffset>` | ✅     | ✅ (via schemars/chrono04)| ✅ wire feature |
//! | `chrono::NaiveDateTime`         | ✅     | ✅ (via schemars/chrono04)| ✅ wire feature |
//! | `jiff::Zoned`                   | ✅     | ✅ (via schemars/jiff02)  | ✅ wire feature |
//! | `jiff::Timestamp`               | ✅     | ✅ (via schemars/jiff02)  | ✅ wire feature |
//! | `time::OffsetDateTime`          | ✅     | ❌ not in schemars        | needs elicit_time |
//! | `time::PrimitiveDateTime`       | ✅     | ❌ not in schemars        | needs elicit_time |
//! | `regex::Regex`                  | ❌     | ❌ not in schemars        | needs elicit_regex |
//! | `reqwest::Method`               | ✅     | ❌ not in schemars        | needs newtype in elicit_reqwest |
//! | `reqwest::StatusCode`           | ✅     | ❌ not in schemars        | needs newtype in elicit_reqwest |
//! | `reqwest::Version`              | ❓     | ❌ not in schemars        | needs newtype in elicit_reqwest |
//! | `http::HeaderMap`               | ❌     | ❌ not in schemars        | needs newtype in elicit_reqwest |

use schemars::JsonSchema;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Asserts at compile time that `T` satisfies all three MCP tool bounds.
///
/// If this function compiles, the type can be used as a field in a `#[derive(Elicit)]`
/// struct that is registered as an MCP tool.
fn assert_mcp_compat<T: JsonSchema + Serialize + DeserializeOwned>() {}

// ── uuid ─────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "uuid")]
fn uuid_uuid_is_mcp_compat() {
    assert_mcp_compat::<uuid::Uuid>();
}

// ── url ──────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "url")]
fn url_url_is_mcp_compat() {
    assert_mcp_compat::<url::Url>();
}

// ── chrono ───────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "chrono")]
fn chrono_datetime_utc_is_mcp_compat() {
    assert_mcp_compat::<chrono::DateTime<chrono::Utc>>();
}

#[test]
#[cfg(feature = "chrono")]
fn chrono_datetime_fixed_offset_is_mcp_compat() {
    assert_mcp_compat::<chrono::DateTime<chrono::FixedOffset>>();
}

#[test]
#[cfg(feature = "chrono")]
fn chrono_naive_datetime_is_mcp_compat() {
    assert_mcp_compat::<chrono::NaiveDateTime>();
}

// ── jiff ─────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "jiff")]
fn jiff_zoned_is_mcp_compat() {
    assert_mcp_compat::<jiff::Zoned>();
}

#[test]
#[cfg(feature = "jiff")]
fn jiff_timestamp_is_mcp_compat() {
    assert_mcp_compat::<jiff::Timestamp>();
}

// ── time ─────────────────────────────────────────────────────────────────────
// BLOCKED: schemars has no time support. Needs elicit_time shadow crate.
// Tracked: https://github.com/crumplecup/elicitation/issues (elicit_time)
//
// #[test]
// #[cfg(feature = "time")]
// fn time_offset_datetime_is_mcp_compat() {
//     assert_mcp_compat::<time::OffsetDateTime>();
// }
//
// #[test]
// #[cfg(feature = "time")]
// fn time_primitive_datetime_is_mcp_compat() {
//     assert_mcp_compat::<time::PrimitiveDateTime>();
// }

// ── regex ─────────────────────────────────────────────────────────────────────
// BLOCKED: regex::Regex has no serde or JsonSchema support.
// Needs elicit_regex shadow crate.
//
// #[test]
// #[cfg(feature = "regex")]
// fn regex_regex_is_mcp_compat() {
//     assert_mcp_compat::<regex::Regex>();
// }
