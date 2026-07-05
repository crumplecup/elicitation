//! `Utc` — shadow for `chrono::Utc` timezone marker.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::DateTime;

/// Shadow for [`chrono::Utc`].
///
/// Provides UTC datetime construction methods that return [`DateTime`] instead
/// of the raw `chrono::DateTime<chrono::Utc>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Utc;

impl Utc {
    /// Returns the current UTC time.
    #[instrument]
    pub fn now() -> DateTime {
        chrono::Utc::now().into()
    }

    /// Construct a UTC datetime from year, month, day, hour, minute, second.
    ///
    /// Returns `None` if any value is out of range.
    #[instrument]
    pub fn with_ymd_and_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> Option<DateTime> {
        use chrono::TimeZone;
        chrono::Utc
            .with_ymd_and_hms(year, month, day, hour, min, sec)
            .single()
            .map(Into::into)
    }

    /// Construct a UTC datetime from a Unix timestamp (seconds + nanoseconds).
    ///
    /// Returns `None` if the timestamp is out of range.
    #[instrument]
    pub fn timestamp_opt(secs: i64, nanos: u32) -> Option<DateTime> {
        use chrono::TimeZone;
        chrono::Utc
            .timestamp_opt(secs, nanos)
            .single()
            .map(Into::into)
    }

    /// Construct a UTC datetime from a Unix timestamp in milliseconds.
    ///
    /// Returns `None` if the timestamp is out of range.
    #[instrument]
    pub fn timestamp_millis_opt(millis: i64) -> Option<DateTime> {
        chrono::DateTime::from_timestamp_millis(millis).map(Into::into)
    }

    /// Parse a datetime string using the given strftime format, interpreting it as UTC.
    ///
    /// Returns `None` if parsing fails.
    #[instrument]
    pub fn datetime_from_str(s: &str, fmt: &str) -> Option<DateTime> {
        chrono::NaiveDateTime::parse_from_str(s, fmt)
            .ok()
            .map(|ndt| ndt.and_utc().into())
    }

    /// Returns the UTC offset as a fixed-offset string (`"+00:00"`).
    #[instrument]
    pub fn fix() -> String {
        "+00:00".to_string()
    }

    /// Returns a `Utc` instance (the only valid UTC offset is itself).
    #[instrument]
    pub fn from_offset() -> Self {
        Utc
    }

    /// Returns a stable hash of the `Utc` singleton (always the same value).
    #[instrument]
    pub fn hash() -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        chrono::Utc.hash(&mut h);
        h.finish()
    }

    /// Returns the UTC offset that applies to the given local date (always `"+00:00"`).
    #[instrument]
    pub fn offset_from_local_date(_date: crate::NaiveDate) -> String {
        "+00:00".to_string()
    }

    /// Returns the UTC offset that applies to the given local datetime (always `"+00:00"`).
    #[instrument]
    pub fn offset_from_local_datetime(_dt: crate::NaiveDateTime) -> String {
        "+00:00".to_string()
    }

    /// Returns the UTC offset for the given UTC date (always `"+00:00"`).
    #[instrument]
    pub fn offset_from_utc_date(_date: crate::NaiveDate) -> String {
        "+00:00".to_string()
    }

    /// Returns the UTC offset for the given UTC datetime (always `"+00:00"`).
    #[instrument]
    pub fn offset_from_utc_datetime(_dt: crate::NaiveDateTime) -> String {
        "+00:00".to_string()
    }
}

// ── chrono offset/timezone traits ─────────────────────────────────────────────

impl chrono::Offset for Utc {
    fn fix(&self) -> chrono::FixedOffset {
        chrono::Utc.fix()
    }
}

impl chrono::TimeZone for Utc {
    type Offset = Utc;

    fn from_offset(_offset: &Utc) -> Utc {
        Utc
    }

    fn offset_from_local_date(&self, _local: &chrono::NaiveDate) -> chrono::MappedLocalTime<Utc> {
        chrono::MappedLocalTime::Single(Utc)
    }

    fn offset_from_local_datetime(
        &self,
        _local: &chrono::NaiveDateTime,
    ) -> chrono::MappedLocalTime<Utc> {
        chrono::MappedLocalTime::Single(Utc)
    }

    fn offset_from_utc_date(&self, _utc: &chrono::NaiveDate) -> Utc {
        Utc
    }

    fn offset_from_utc_datetime(&self, _utc: &chrono::NaiveDateTime) -> Utc {
        Utc
    }
}

// ── Elicitation framework traits — delegate to `chrono::Utc` core impls ──────

impl elicitation::Prompt for Utc {
    fn prompt() -> Option<&'static str> {
        <chrono::Utc as elicitation::Prompt>::prompt()
    }
}

impl elicitation::Elicitation for Utc {
    type Style = <chrono::Utc as elicitation::Elicitation>::Style;

    async fn elicit<C: elicitation::ElicitCommunicator>(
        communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        chrono::Utc::elicit(communicator).await.map(|_| Utc)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <chrono::Utc as elicitation::Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <chrono::Utc as elicitation::Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <chrono::Utc as elicitation::Elicitation>::creusot_proof()
    }
}

impl elicitation::ElicitIntrospect for Utc {
    fn pattern() -> elicitation::ElicitationPattern {
        <chrono::Utc as elicitation::ElicitIntrospect>::pattern()
    }
    fn metadata() -> elicitation::TypeMetadata {
        <chrono::Utc as elicitation::ElicitIntrospect>::metadata()
    }
}

impl elicitation::ElicitSpec for Utc {
    fn type_spec() -> elicitation::TypeSpec {
        <chrono::Utc as elicitation::ElicitSpec>::type_spec()
    }
}

impl elicitation::ElicitPromptTree for Utc {
    fn prompt_tree() -> elicitation::PromptTree {
        <chrono::Utc as elicitation::ElicitPromptTree>::prompt_tree()
    }
}

mod emit_impls {
    use super::Utc;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Utc {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_chrono::Utc }
        }
        fn type_tokens() -> TokenStream {
            quote::quote! { ::elicit_chrono::Utc }
        }
    }
}

impl elicitation::ElicitComplete for Utc {}
