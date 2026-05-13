//! [`ElicitComplete`] implementations for TOML primitive types.
//!
//! Available with the `toml-types` feature.

#[cfg(feature = "toml-types")]
mod toml_impls {
    use crate::{
        ElicitComplete, ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TomlDate, TomlDatetime,
        TomlDeError, TomlOffset, TomlSerError, TomlTime, TomlValue, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    // -------------------------------------------------------------------------
    // Macro: impl_toml_select_spec!
    //
    // Generates ElicitSpec + inventory::submit! + ElicitComplete for a TOML
    // Select enum, listing each variant from Select::labels().
    // -------------------------------------------------------------------------

    macro_rules! impl_toml_select_spec {
        (
            type     = $ty:ty,
            name     = $name:literal,
            summary  = $summary:literal,
            source   = $source:literal,
            variants = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let variants = SpecCategoryBuilder::default()
                        .name("variants".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description($source.to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Select — choose one variant from the list".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitComplete for $ty {}
        };
    }

    // -------------------------------------------------------------------------
    // Macro: impl_toml_survey_spec!
    //
    // Generates ElicitSpec + inventory::submit! + ElicitComplete for a TOML
    // Survey (struct) type.
    // -------------------------------------------------------------------------

    macro_rules! impl_toml_survey_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            source  = $source:literal,
            fields  = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description($source.to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — structured value elicited field by field".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![fields, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitComplete for $ty {}
        };
    }

    // =========================================================================
    // TomlDate — local date: year, month, day
    // =========================================================================

    impl_toml_survey_spec!(
        type    = TomlDate,
        name    = "toml_datetime::Date",
        summary = "A TOML local date value: year, month, and day.",
        source  = "toml_datetime v1 — TOML date/time value types",
        fields = [
            ("year",  "Gregorian year (u16)"),
            ("month", "Month of the year (u8, 1–12)"),
            ("day",   "Day of the month (u8, 1–31)"),
        ]
    );

    // =========================================================================
    // TomlTime — local time: hour, minute, second, nanosecond
    // =========================================================================

    impl_toml_survey_spec!(
        type    = TomlTime,
        name    = "toml_datetime::Time",
        summary = "A TOML local time value: hour, minute, second, and sub-second nanoseconds.",
        source  = "toml_datetime v1 — TOML date/time value types",
        fields = [
            ("hour",       "Hour of the day (u8, 0–23)"),
            ("minute",     "Minute of the hour (u8, 0–59)"),
            ("second",     "Second of the minute (u8, 0–60, allowing leap seconds)"),
            ("nanosecond", "Sub-second nanoseconds (u32, 0–999_999_999)"),
        ]
    );

    // =========================================================================
    // TomlOffset — UTC offset: Z or Custom { hours, minutes }
    // =========================================================================

    impl_toml_select_spec!(
        type     = TomlOffset,
        name     = "toml_datetime::Offset",
        summary  = "A TOML UTC offset: either Z (UTC) or a custom +/- hours/minutes offset.",
        source   = "toml_datetime v1 — TOML date/time value types",
        variants = [
            ("Z",      "UTC (zero offset)"),
            ("Custom", "Custom offset: hours (i8, −23..=23) and minutes (u8, 0–59)"),
        ]
    );

    // =========================================================================
    // TomlDatetime — four calendar/clock variants
    // =========================================================================

    impl_toml_select_spec!(
        type     = TomlDatetime,
        name     = "toml_datetime::Datetime",
        summary  = "A TOML datetime: one of four variants — offset date-time, local date-time, local date, or local time.",
        source   = "toml_datetime v1 — TOML date/time value types",
        variants = [
            ("OffsetDateTime", "date + time + UTC offset (e.g. 1979-05-27T07:32:00Z)"),
            ("LocalDateTime",  "date + time, no offset (e.g. 1979-05-27T07:32:00)"),
            ("LocalDate",      "date only (e.g. 1979-05-27)"),
            ("LocalTime",      "time only (e.g. 07:32:00)"),
        ]
    );

    // =========================================================================
    // TomlValue — 7-variant recursive value tree
    // =========================================================================

    impl_toml_select_spec!(
        type     = TomlValue,
        name     = "toml::Value",
        summary  = "A TOML value: one of seven recursive variants mirroring toml::Value.",
        source   = "toml v1 — TOML serde layer",
        variants = [
            ("String",   "A UTF-8 string value"),
            ("Integer",  "A 64-bit signed integer"),
            ("Float",    "A 64-bit IEEE 754 float"),
            ("Boolean",  "A boolean"),
            ("Datetime", "A TOML datetime (TomlDatetime)"),
            ("Array",    "An array of TOML values (Vec<TomlValue>)"),
            ("Table",    "A TOML table as ordered key-value pairs (Vec<(String, TomlValue)>)"),
        ]
    );

    // =========================================================================
    // TomlDeError — deserialization error message
    // =========================================================================

    impl_toml_survey_spec!(
        type    = TomlDeError,
        name    = "toml::de::Error",
        summary = "A TOML deserialization error, represented as a human-readable message string.",
        source  = "toml v1 — TOML serde layer",
        fields = [
            ("message", "Human-readable error message from the TOML deserializer"),
        ]
    );

    // =========================================================================
    // TomlSerError — serialization error message
    // =========================================================================

    impl_toml_survey_spec!(
        type    = TomlSerError,
        name    = "toml::ser::Error",
        summary = "A TOML serialization error, represented as a human-readable message string.",
        source  = "toml v1 — TOML serde layer",
        fields = [
            ("message", "Human-readable error message from the TOML serializer"),
        ]
    );
}
