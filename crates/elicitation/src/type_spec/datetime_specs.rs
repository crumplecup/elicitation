//! [`ElicitSpec`](crate::ElicitSpec) implementations for datetime contract types.
//!
//! Available with the `chrono` or `jiff` features.

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
macro_rules! impl_datetime_spec {
    (
        type     = $ty:ty,
        name     = $name:literal,
        summary  = $summary:literal,
        requires = [($req_label:literal, $req_desc:literal, $req_expr:literal)],
        related  = $related_type:literal $(,)?
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let requires = SpecCategoryBuilder::default()
                    .name("requires".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label($req_label.to_string())
                            .description($req_desc.to_string())
                            .expression(Some($req_expr.to_string()))
                            .build()
                            .expect("valid SpecEntry"),
                    ])
                    .build()
                    .expect("valid SpecCategory");
                let related = SpecCategoryBuilder::default()
                    .name("related".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label("base_type".to_string())
                            .description(format!("Wraps a {}", $related_type))
                            .expression(None)
                            .build()
                            .expect("valid SpecEntry"),
                    ])
                    .build()
                    .expect("valid SpecCategory");
                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![requires, related])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new(
            $name,
            <$ty as ElicitSpec>::type_spec,
            std::any::TypeId::of::<$ty>
        ));
    };
}

// ── chrono ────────────────────────────────────────────────────────────────────

#[cfg(feature = "chrono")]
mod chrono_specs {
    use super::*;
    use crate::verification::types::{DateTimeUtcAfter, DateTimeUtcBefore, NaiveDateTimeAfter};

    impl_datetime_spec!(
        type     = DateTimeUtcAfter,
        name     = "DateTimeUtcAfter",
        summary  = "A chrono DateTime<Utc> guaranteed to be strictly after a given threshold.",
        requires = [("after", "Timestamp must be strictly greater than the threshold.", "value > threshold")],
        related  = "chrono::DateTime<Utc>",
    );

    impl_datetime_spec!(
        type     = DateTimeUtcBefore,
        name     = "DateTimeUtcBefore",
        summary  = "A chrono DateTime<Utc> guaranteed to be strictly before a given threshold.",
        requires = [("before", "Timestamp must be strictly less than the threshold.", "value < threshold")],
        related  = "chrono::DateTime<Utc>",
    );

    impl_datetime_spec!(
        type     = NaiveDateTimeAfter,
        name     = "NaiveDateTimeAfter",
        summary  = "A chrono NaiveDateTime guaranteed to be strictly after a given threshold.",
        requires = [("after", "Timestamp must be strictly greater than the threshold.", "value > threshold")],
        related  = "chrono::NaiveDateTime",
    );

    #[cfg(not(kani))]
    impl crate::ElicitComplete for DateTimeUtcAfter {}

    impl crate::ElicitSpec for chrono::Weekday {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("chrono::Weekday".to_string())
                .summary("ISO day of the week (Mon–Sun).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "chrono::Weekday",
        <chrono::Weekday as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<chrono::Weekday>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for chrono::Weekday {}

    impl crate::ElicitSpec for chrono::NaiveDate {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("chrono::NaiveDate".to_string())
                .summary("A calendar date without time or timezone (YYYY-MM-DD).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "chrono::NaiveDate",
        <chrono::NaiveDate as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<chrono::NaiveDate>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for chrono::NaiveDate {}

    impl crate::ElicitSpec for chrono::NaiveTime {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("chrono::NaiveTime".to_string())
                .summary("A time of day without date or timezone (HH:MM:SS).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "chrono::NaiveTime",
        <chrono::NaiveTime as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<chrono::NaiveTime>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for chrono::NaiveTime {}

    impl crate::ElicitSpec for chrono::NaiveDateTime {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("chrono::NaiveDateTime".to_string())
                .summary("A combined date and time without timezone (ISO 8601 local).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "chrono::NaiveDateTime",
        <chrono::NaiveDateTime as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<chrono::NaiveDateTime>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for chrono::NaiveDateTime {}

    impl crate::ElicitSpec for chrono::DateTime<chrono::Utc> {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("chrono::DateTime<Utc>".to_string())
                .summary("A UTC datetime with timezone (RFC 3339 / ISO 8601).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "chrono::DateTime<Utc>",
        <chrono::DateTime<chrono::Utc> as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<chrono::DateTime<chrono::Utc>>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for chrono::DateTime<chrono::Utc> {}

    impl crate::ElicitSpec for chrono::DateTime<chrono::FixedOffset> {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("chrono::DateTime<FixedOffset>".to_string())
                .summary("A datetime with a fixed UTC offset (RFC 3339 / ISO 8601).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "chrono::DateTime<FixedOffset>",
        <chrono::DateTime<chrono::FixedOffset> as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<chrono::DateTime<chrono::FixedOffset>>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for chrono::DateTime<chrono::FixedOffset> {}

    impl crate::ElicitSpec for chrono::TimeDelta {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("chrono::TimeDelta".to_string())
                .summary(
                    "A signed duration (seconds + nanoseconds). Also aliased as chrono::Duration."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "chrono::TimeDelta",
        <chrono::TimeDelta as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<chrono::TimeDelta>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for chrono::TimeDelta {}

    impl crate::ElicitSpec for chrono::Month {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("chrono::Month".to_string())
                .summary("A calendar month (January–December).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "chrono::Month",
        <chrono::Month as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<chrono::Month>
    ));

    impl crate::ElicitSpec for crate::MonthSelect {
        fn type_spec() -> crate::TypeSpec {
            <chrono::Month as crate::ElicitSpec>::type_spec()
        }
    }

    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "MonthSelect",
        <crate::MonthSelect as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<crate::MonthSelect>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for crate::MonthSelect {}

    // ── Batch 1: pure Select enums ────────────────────────────────────────────

    macro_rules! chrono_select_spec {
        ($raw:ty, $raw_name:literal, $wrapper:ty, $wrapper_name:literal, $summary:literal) => {
            impl crate::ElicitSpec for $raw {
                fn type_spec() -> crate::TypeSpec {
                    crate::TypeSpecBuilder::default()
                        .type_name($raw_name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![])
                        .build()
                        .expect("valid TypeSpec")
                }
            }
            inventory::submit!(crate::TypeSpecInventoryKey::new(
                $raw_name,
                <$raw as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$raw>
            ));

            impl crate::ElicitSpec for $wrapper {
                fn type_spec() -> crate::TypeSpec {
                    <$raw as crate::ElicitSpec>::type_spec()
                }
            }
            inventory::submit!(crate::TypeSpecInventoryKey::new(
                $wrapper_name,
                <$wrapper as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$wrapper>
            ));

            #[cfg(not(kani))]
            impl crate::ElicitComplete for $wrapper {}
        };
    }

    chrono_select_spec!(
        chrono::RoundingError,
        "chrono::RoundingError",
        crate::RoundingErrorSelect,
        "RoundingErrorSelect",
        "A chrono rounding error indicating why a duration-rounding operation failed."
    );

    chrono_select_spec!(
        chrono::format::Colons,
        "chrono::format::Colons",
        crate::ColonsSelect,
        "ColonsSelect",
        "Separator style between hours and minutes in a UTC offset string."
    );

    chrono_select_spec!(
        chrono::format::Pad,
        "chrono::format::Pad",
        crate::PadSelect,
        "PadSelect",
        "Padding style applied to numeric formatting items (None, Zero, Space)."
    );

    chrono_select_spec!(
        chrono::format::OffsetPrecision,
        "chrono::format::OffsetPrecision",
        crate::OffsetPrecisionSelect,
        "OffsetPrecisionSelect",
        "Precision of a UTC offset field (hours only, hours+minutes, or full h:m:s)."
    );

    chrono_select_spec!(
        chrono::format::ParseErrorKind,
        "chrono::format::ParseErrorKind",
        crate::ParseErrorKindSelect,
        "ParseErrorKindSelect",
        "The category of a chrono datetime parse failure."
    );

    chrono_select_spec!(
        chrono::SecondsFormat,
        "chrono::SecondsFormat",
        crate::SecondsFormatSelect,
        "SecondsFormatSelect",
        "Sub-second precision used when formatting a datetime as RFC 3339."
    );

    chrono_select_spec!(
        chrono::format::Numeric,
        "chrono::format::Numeric",
        crate::NumericSelect,
        "NumericSelect",
        "A numeric strftime-style formatting item (year, month, day, hour, etc.)."
    );

    chrono_select_spec!(
        chrono::format::Fixed,
        "chrono::format::Fixed",
        crate::FixedSelect,
        "FixedSelect",
        "A fixed-format strftime-style formatting item (month name, timezone offset, RFC 3339, etc.)."
    );

    // ── Batch 3: primitive wrappers ───────────────────────────────────────────

    macro_rules! chrono_wrap_spec {
        ($raw:ty, $raw_name:literal, $wrapper:ty, $wrapper_name:literal, $summary:literal) => {
            impl crate::ElicitSpec for $raw {
                fn type_spec() -> crate::TypeSpec {
                    crate::TypeSpecBuilder::default()
                        .type_name($raw_name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![])
                        .build()
                        .expect("valid TypeSpec")
                }
            }
            inventory::submit!(crate::TypeSpecInventoryKey::new(
                $raw_name,
                <$raw as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$raw>
            ));

            impl crate::ElicitSpec for $wrapper {
                fn type_spec() -> crate::TypeSpec {
                    <$raw as crate::ElicitSpec>::type_spec()
                }
            }
            inventory::submit!(crate::TypeSpecInventoryKey::new(
                $wrapper_name,
                <$wrapper as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$wrapper>
            ));

            #[cfg(not(kani))]
            impl crate::ElicitComplete for $wrapper {}
        };
    }

    chrono_wrap_spec!(
        chrono::Days,
        "chrono::Days",
        crate::DaysWrap,
        "DaysWrap",
        "A count of whole days used in calendar arithmetic."
    );
    chrono_wrap_spec!(
        chrono::Months,
        "chrono::Months",
        crate::MonthsWrap,
        "MonthsWrap",
        "A count of whole months used in calendar arithmetic."
    );
    chrono_wrap_spec!(
        chrono::Utc,
        "chrono::Utc",
        crate::UtcWrap,
        "UtcWrap",
        "The UTC timezone marker — a unit struct with a single possible value."
    );
    chrono_wrap_spec!(
        chrono::Local,
        "chrono::Local",
        crate::LocalWrap,
        "LocalWrap",
        "The local timezone marker — resolves to the system timezone at runtime."
    );

    // ── Batch 4: struct types ─────────────────────────────────────────────────

    chrono_wrap_spec!(
        chrono::FixedOffset,
        "chrono::FixedOffset",
        crate::FixedOffsetWrap,
        "FixedOffsetWrap",
        "A fixed UTC offset (seconds east, range -86399..=86399)."
    );
    chrono_wrap_spec!(
        chrono::IsoWeek,
        "chrono::IsoWeek",
        crate::IsoWeekWrap,
        "IsoWeekWrap",
        "An ISO 8601 week (year + week number, formatted YYYY-Www)."
    );
    chrono_wrap_spec!(
        chrono::NaiveWeek,
        "chrono::NaiveWeek",
        crate::NaiveWeekWrap,
        "NaiveWeekWrap",
        "A naive week defined by a reference date and its starting weekday."
    );
    chrono_wrap_spec!(
        chrono::WeekdaySet,
        "chrono::WeekdaySet",
        crate::WeekdaySetWrap,
        "WeekdaySetWrap",
        "A compact bitmask representing a subset of the seven weekdays."
    );
    chrono_wrap_spec!(
        chrono::format::OffsetFormat,
        "chrono::format::OffsetFormat",
        crate::OffsetFormatWrap,
        "OffsetFormatWrap",
        "UTC offset formatting specification (precision, separator, padding)."
    );

    // ── Batch 5: error types ──────────────────────────────────────────────────

    chrono_wrap_spec!(
        chrono::OutOfRange,
        "chrono::OutOfRange",
        crate::OutOfRangeWrap,
        "OutOfRangeWrap",
        "An out-of-range error from chrono (single-value type)."
    );
    chrono_wrap_spec!(
        chrono::OutOfRangeError,
        "chrono::OutOfRangeError",
        crate::OutOfRangeErrorWrap,
        "OutOfRangeErrorWrap",
        "A TimeDelta-to-std::time::Duration conversion error (single-value type)."
    );
    chrono_wrap_spec!(
        chrono::ParseError,
        "chrono::ParseError",
        crate::ParseErrorWrap,
        "ParseErrorWrap",
        "A chrono parse error carrying the error kind (OutOfRange, Invalid, TooShort, etc.)."
    );
    chrono_wrap_spec!(
        chrono::ParseMonthError,
        "chrono::ParseMonthError",
        crate::ParseMonthErrorWrap,
        "ParseMonthErrorWrap",
        "A month parse error from an invalid month string (single-value type)."
    );
    chrono_wrap_spec!(
        chrono::ParseWeekdayError,
        "chrono::ParseWeekdayError",
        crate::ParseWeekdayErrorWrap,
        "ParseWeekdayErrorWrap",
        "A weekday parse error from an invalid weekday string (single-value type)."
    );
    chrono_wrap_spec!(
        chrono::format::InternalNumeric,
        "chrono::format::InternalNumeric",
        crate::InternalNumericWrap,
        "InternalNumericWrap",
        "An uninhabited internal-only type; no value can be constructed or elicited."
    );
    chrono_wrap_spec!(
        chrono::format::InternalFixed,
        "chrono::format::InternalFixed",
        crate::InternalFixedWrap,
        "InternalFixedWrap",
        "One of 4 internal fixed-format variants (TimezoneOffsetPermissive, Nanosecond*NoDot)."
    );
    chrono_wrap_spec!(
        chrono::format::Parsed,
        "chrono::format::Parsed",
        crate::ParsedWrap,
        "ParsedWrap",
        "A 21-field optional survey of parsed date/time components."
    );
    chrono_wrap_spec!(
        chrono::naive::NaiveDateDaysIterator,
        "chrono::NaiveDateDaysIterator",
        crate::NaiveDateDaysIteratorWrap,
        "NaiveDateDaysIteratorWrap",
        "Iterator over consecutive NaiveDates with a step of one day."
    );
    chrono_wrap_spec!(
        chrono::naive::NaiveDateWeeksIterator,
        "chrono::NaiveDateWeeksIterator",
        crate::NaiveDateWeeksIteratorWrap,
        "NaiveDateWeeksIteratorWrap",
        "Iterator over consecutive NaiveDates with a step of seven days."
    );

    // LocalResult<T> is generic, so TypeId requires a concrete T — cannot use
    // inventory::submit! here. Callers that need a concrete TypeSpec look up the
    // inner type and wrap it. No ElicitComplete impl either (foreign generic).
    impl<T: crate::ElicitSpec> crate::ElicitSpec for chrono::LocalResult<T> {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "chrono::LocalResult",
                "Result of a local-to-UTC conversion: None (no matching instant), \
                 Single (exactly one), or Ambiguous (two candidates near a DST gap/fold).",
                vec![crate::SpecCategory::new(
                    "variants",
                    vec![
                        crate::SpecEntry::new(
                            "None",
                            "No instant in UTC corresponds to the local time (e.g. within a DST gap).",
                        ),
                        crate::SpecEntry::new(
                            "Single",
                            "Exactly one UTC instant corresponds to the local time (the common case).",
                        ),
                        crate::SpecEntry::new(
                            "Ambiguous",
                            "Two UTC instants correspond to the local time (clock was set back — a DST fold).",
                        ),
                    ],
                )],
            )
        }
    }

    // OwnedItem and OwnedStrftimeItems are self-contained trenchcoats — there is
    // no separate "raw" type to register alongside them, so we register only once.
    impl crate::ElicitSpec for crate::OwnedItem {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("OwnedItem".to_string())
                .summary(
                    "An owned format item for chrono::format::Item<'_> with no lifetime parameter."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }
    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "OwnedItem",
        <crate::OwnedItem as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<crate::OwnedItem>
    ));

    impl crate::ElicitSpec for crate::OwnedStrftimeItems {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpecBuilder::default()
                .type_name("OwnedStrftimeItems".to_string())
                .summary(
                    "Owned sequence of format items collected from a StrftimeItems iterator."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }
    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "OwnedStrftimeItems",
        <crate::OwnedStrftimeItems as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<crate::OwnedStrftimeItems>
    ));
}

// ── jiff ──────────────────────────────────────────────────────────────────────

#[cfg(feature = "jiff")]
mod jiff_specs {
    use super::*;
    use crate::verification::types::{TimestampAfter, TimestampBefore};

    impl_datetime_spec!(
        type     = TimestampAfter,
        name     = "TimestampAfter",
        summary  = "A jiff Timestamp guaranteed to be strictly after a given threshold.",
        requires = [("after", "Timestamp must be strictly greater than the threshold.", "value > threshold")],
        related  = "jiff::Timestamp",
    );

    impl_datetime_spec!(
        type     = TimestampBefore,
        name     = "TimestampBefore",
        summary  = "A jiff Timestamp guaranteed to be strictly before a given threshold.",
        requires = [("before", "Timestamp must be strictly less than the threshold.", "value < threshold")],
        related  = "jiff::Timestamp",
    );

    impl crate::ElicitSpec for jiff::Timestamp {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("jiff::Timestamp".to_string())
                .summary("A nanosecond-precision Unix timestamp (UTC).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "jiff::Timestamp",
        <jiff::Timestamp as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<jiff::Timestamp>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for jiff::Timestamp {}

    impl crate::ElicitSpec for jiff::Zoned {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("jiff::Zoned".to_string())
                .summary("A nanosecond-precision datetime with a time zone.".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "jiff::Zoned",
        <jiff::Zoned as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<jiff::Zoned>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for jiff::Zoned {}

    impl crate::ElicitSpec for jiff::civil::DateTime {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("jiff::civil::DateTime".to_string())
                .summary(
                    "A naive datetime without a time zone (calendar date + wall clock time)."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "jiff::civil::DateTime",
        <jiff::civil::DateTime as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<jiff::civil::DateTime>
    ));

    #[cfg(not(kani))]
    impl crate::ElicitComplete for jiff::civil::DateTime {}
}

// ── time ──────────────────────────────────────────────────────────────────────

#[cfg(feature = "time")]
mod time_specs {
    use super::*;
    use crate::verification::types::{OffsetDateTimeAfter, OffsetDateTimeBefore};

    impl_datetime_spec!(
        type     = OffsetDateTimeAfter,
        name     = "OffsetDateTimeAfter",
        summary  = "A time OffsetDateTime guaranteed to be strictly after a given threshold.",
        requires = [("after", "Value must be strictly after the threshold.", "value > threshold")],
        related  = "time::OffsetDateTime",
    );

    impl_datetime_spec!(
        type     = OffsetDateTimeBefore,
        name     = "OffsetDateTimeBefore",
        summary  = "A time OffsetDateTime guaranteed to be strictly before a given threshold.",
        requires = [("before", "Value must be strictly before the threshold.", "value < threshold")],
        related  = "time::OffsetDateTime",
    );

    impl crate::ElicitSpec for time::OffsetDateTime {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::OffsetDateTime".to_string())
                .summary("An RFC 3339 datetime with timezone offset.".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::OffsetDateTime",
        <time::OffsetDateTime as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::OffsetDateTime>
    ));

    impl crate::ElicitSpec for time::PrimitiveDateTime {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::PrimitiveDateTime".to_string())
                .summary("An ISO 8601 local datetime without a timezone offset.".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::PrimitiveDateTime",
        <time::PrimitiveDateTime as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::PrimitiveDateTime>
    ));

    impl crate::ElicitSpec for time::Time {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::Time".to_string())
                .summary("A wall clock time of day.".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::Time",
        <time::Time as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::Time>
    ));
}
