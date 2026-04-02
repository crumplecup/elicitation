//! `DateTimeFixed` — elicitation-enabled wrapper around `chrono::DateTime<chrono::FixedOffset>`.

use std::sync::Arc;

use chrono::{Datelike, Timelike};
use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Elicitation-enabled wrapper around `chrono::DateTime<chrono::FixedOffset>`.
///
/// Serializes to/from RFC 3339 strings (e.g. `"2024-01-15T12:30:00+05:30"`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DateTimeFixed(pub Arc<chrono::DateTime<chrono::FixedOffset>>);

impl JsonSchema for DateTimeFixed {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DateTimeFixed".into()
    }

    fn json_schema(schema_gen: &mut SchemaGenerator) -> Schema {
        <chrono::DateTime<chrono::FixedOffset> as JsonSchema>::json_schema(schema_gen)
    }
}

impl std::ops::Deref for DateTimeFixed {
    type Target = chrono::DateTime<chrono::FixedOffset>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<chrono::DateTime<chrono::FixedOffset>> for DateTimeFixed {
    fn as_ref(&self) -> &chrono::DateTime<chrono::FixedOffset> {
        &self.0
    }
}

impl From<chrono::DateTime<chrono::FixedOffset>> for DateTimeFixed {
    fn from(inner: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Self(Arc::new(inner))
    }
}

impl From<Arc<chrono::DateTime<chrono::FixedOffset>>> for DateTimeFixed {
    fn from(arc: Arc<chrono::DateTime<chrono::FixedOffset>>) -> Self {
        Self(arc)
    }
}

#[reflect_methods]
impl DateTimeFixed {
    /// Returns the year.
    #[instrument(skip(self))]
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Returns the month (1 = January … 12 = December).
    #[instrument(skip(self))]
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Returns the day of the month (1–31).
    #[instrument(skip(self))]
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// Returns the hour (0–23).
    #[instrument(skip(self))]
    pub fn hour(&self) -> u32 {
        self.0.hour()
    }

    /// Returns the minute (0–59).
    #[instrument(skip(self))]
    pub fn minute(&self) -> u32 {
        self.0.minute()
    }

    /// Returns the second (0–59).
    #[instrument(skip(self))]
    pub fn second(&self) -> u32 {
        self.0.second()
    }

    /// Returns the nanosecond component (0–999_999_999).
    #[instrument(skip(self))]
    pub fn nanosecond(&self) -> u32 {
        self.0.nanosecond()
    }

    /// Returns the Unix timestamp in whole seconds.
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    /// Returns the UTC offset in seconds east of UTC (negative = west).
    #[instrument(skip(self))]
    pub fn offset_seconds(&self) -> i32 {
        use chrono::Offset;
        self.0.offset().fix().local_minus_utc()
    }

    /// Returns the day of the year (1–366).
    #[instrument(skip(self))]
    pub fn ordinal(&self) -> u32 {
        self.0.ordinal()
    }

    /// Returns the weekday name (e.g. `"Monday"`).
    #[instrument(skip(self))]
    pub fn weekday(&self) -> String {
        self.0.weekday().to_string()
    }

    /// Returns the RFC 3339 string representation.
    #[instrument(skip(self))]
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}

impl DateTimeFixed {
    /// Parse an RFC 3339 string with a fixed offset. Returns `None` if invalid.
    pub fn parse(s: &str) -> Option<Self> {
        chrono::DateTime::parse_from_rfc3339(s).ok().map(Into::into)
    }
}

// ── Elicitation framework traits ─────────────────────────────────────────────

impl elicitation::Prompt for DateTimeFixed {
    fn prompt() -> Option<&'static str> {
        Some(
            "Enter a datetime with timezone offset in RFC 3339 format (e.g. 2024-01-15T12:30:00+05:30):",
        )
    }
}

impl elicitation::Elicitation for DateTimeFixed {
    type Style = ();

    async fn elicit<C: elicitation::ElicitCommunicator>(
        communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        let response = communicator
            .send_prompt("Enter a datetime with timezone offset in RFC 3339 format (e.g. 2024-01-15T12:30:00+05:30):")
            .await?;
        let inner = response
            .parse::<chrono::DateTime<chrono::FixedOffset>>()
            .map_err(|e| {
                elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(format!(
                    "Invalid fixed-offset datetime: {e}"
                )))
            })?;
        Ok(Self(Arc::new(inner)))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::kani_trusted_opaque("DateTimeFixed")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::verus_trusted_opaque("DateTimeFixed")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::creusot_trusted_opaque("DateTimeFixed")
    }
}

impl elicitation::ElicitIntrospect for DateTimeFixed {
    fn pattern() -> elicitation::ElicitationPattern {
        elicitation::ElicitationPattern::Primitive
    }
    fn metadata() -> elicitation::TypeMetadata {
        elicitation::TypeMetadata {
            type_name: "DateTimeFixed",
            description: <Self as elicitation::Prompt>::prompt(),
            details: elicitation::PatternDetails::Primitive,
        }
    }
}

impl elicitation::ElicitPromptTree for DateTimeFixed {
    fn prompt_tree() -> elicitation::PromptTree {
        elicitation::PromptTree::Leaf {
            prompt: "DateTimeFixed".to_string(),
            type_name: "DateTimeFixed".to_string(),
        }
    }
}

impl elicitation::ElicitSpec for DateTimeFixed {
    fn type_spec() -> elicitation::TypeSpec {
        elicitation::TypeSpecBuilder::default()
            .type_name("DateTimeFixed".to_string())
            .summary("Datetime with fixed timezone offset in RFC 3339 format.".to_string())
            .build()
            .expect("valid TypeSpec")
    }
}

mod emit_impls {
    use super::DateTimeFixed;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DateTimeFixed {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.to_rfc3339();
            quote::quote! {
                ::elicit_chrono::DateTimeFixed::from(
                    #s.parse::<::chrono::DateTime<::chrono::FixedOffset>>().expect("valid fixed datetime")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for DateTimeFixed {}
