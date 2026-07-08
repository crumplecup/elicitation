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

    // LocalResult<T> / LocalResultWrap<T> are generic — cannot use
    // inventory::submit! here (TypeId requires a concrete type).
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

    // LocalResultWrap<T> — trenchcoat that can be ElicitComplete where T: ElicitComplete.
    // Same spec content as chrono::LocalResult<T> since it mirrors the same semantics.
    impl<T: crate::ElicitSpec> crate::ElicitSpec for crate::LocalResultWrap<T> {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "LocalResultWrap",
                "Serialisable mirror of chrono::LocalResult<T>: the result of a \
                 local-to-UTC conversion (None, Single, or Ambiguous).",
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
    use crate::verification::types::{
        ComponentRangeWrap, ConversionRangeWrap, DateWrap, DifferentVariantWrap, DurationWrap,
        FmtCalendarYearCenturyExtendedRangeWrap, FmtCalendarYearCenturyStandardRangeWrap,
        FmtCalendarYearFullExtendedRangeWrap, FmtCalendarYearFullStandardRangeWrap,
        FmtCalendarYearLastTwoWrap, FmtDayWrap, FmtEndWrap, FmtHour12Wrap, FmtHour24Wrap,
        FmtIgnoreWrap, FmtIsoYearCenturyExtendedRangeWrap, FmtIsoYearCenturyStandardRangeWrap,
        FmtIsoYearFullExtendedRangeWrap, FmtIsoYearFullStandardRangeWrap, FmtIsoYearLastTwoWrap,
        FmtMinuteWrap, FmtMonthLongWrap, FmtMonthNumericalWrap, FmtMonthShortWrap,
        FmtOffsetHourWrap, FmtOffsetMinuteWrap, FmtOffsetSecondWrap, FmtOrdinalWrap,
        FmtPaddingWrap, FmtPeriodWrap, FmtSecondWrap, FmtSubsecondDigitsWrap, FmtSubsecondWrap,
        FmtTrailingInputWrap, FmtUnixTimestampMicrosecondWrap, FmtUnixTimestampMillisecondWrap,
        FmtUnixTimestampNanosecondWrap, FmtUnixTimestampSecondWrap, FmtWeekNumberIsoWrap,
        FmtWeekNumberMondayWrap, FmtWeekNumberSundayWrap, FmtWeekdayLongWrap, FmtWeekdayMondayWrap,
        FmtWeekdayShortWrap, FmtWeekdaySundayWrap, MonthWrap, OffsetDateTimeAfter,
        WellKnownRfc2822Wrap,
        OffsetDateTimeBefore, OffsetDateTimeWrap, PrimitiveDateTimeWrap, TimeWrap, UtcDateTimeWrap,
        UtcOffsetWrap, WeekdayWrap,
    };

    impl crate::ElicitSpec for PrimitiveDateTimeWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("PrimitiveDateTimeWrap".to_string())
                .summary(
                    "Trenchcoat for time::PrimitiveDateTime — an ISO 8601 local datetime without a timezone offset."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "PrimitiveDateTimeWrap",
        <PrimitiveDateTimeWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<PrimitiveDateTimeWrap>
    ));

    impl crate::ElicitSpec for OffsetDateTimeWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("OffsetDateTimeWrap".to_string())
                .summary(
                    "Trenchcoat for time::OffsetDateTime — an RFC 3339 datetime with timezone offset."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "OffsetDateTimeWrap",
        <OffsetDateTimeWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<OffsetDateTimeWrap>
    ));

    impl crate::ElicitSpec for TimeWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("TimeWrap".to_string())
                .summary(
                    "Trenchcoat for time::Time — a wall-clock time of day (HH:MM:SS).".to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "TimeWrap",
        <TimeWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<TimeWrap>
    ));

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

    impl crate::ElicitSpec for time::Date {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::Date".to_string())
                .summary("An ISO 8601 calendar date (year, month, day).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::Date",
        <time::Date as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::Date>
    ));

    impl crate::ElicitSpec for DateWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("DateWrap".to_string())
                .summary(
                    "Trenchcoat for time::Date — an ISO 8601 calendar date (YYYY-MM-DD)."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "DateWrap",
        <DateWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<DateWrap>
    ));

    impl crate::ElicitSpec for time::Duration {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::Duration".to_string())
                .summary(
                    "A signed duration (seconds + subsecond nanoseconds), unlike std::time::Duration."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::Duration",
        <time::Duration as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::Duration>
    ));

    impl crate::ElicitSpec for DurationWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("DurationWrap".to_string())
                .summary(
                    "Trenchcoat for time::Duration — a signed duration with nanosecond precision."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "DurationWrap",
        <DurationWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<DurationWrap>
    ));

    impl crate::ElicitSpec for time::Month {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::Month".to_string())
                .summary("A calendar month (January through December).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::Month",
        <time::Month as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::Month>
    ));

    impl crate::ElicitSpec for MonthWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("MonthWrap".to_string())
                .summary(
                    "Trenchcoat for time::Month — a calendar month (January through December)."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "MonthWrap",
        <MonthWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<MonthWrap>
    ));

    impl crate::ElicitSpec for time::Weekday {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::Weekday".to_string())
                .summary("A day of the week (Monday through Sunday).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::Weekday",
        <time::Weekday as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::Weekday>
    ));

    impl crate::ElicitSpec for WeekdayWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("WeekdayWrap".to_string())
                .summary(
                    "Trenchcoat for time::Weekday — a day of the week (Monday through Sunday)."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "WeekdayWrap",
        <WeekdayWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<WeekdayWrap>
    ));

    impl crate::ElicitSpec for time::UtcDateTime {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::UtcDateTime".to_string())
                .summary("A datetime anchored to UTC (no offset stored).".to_string())
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::UtcDateTime",
        <time::UtcDateTime as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::UtcDateTime>
    ));

    impl crate::ElicitSpec for UtcDateTimeWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("UtcDateTimeWrap".to_string())
                .summary(
                    "Trenchcoat for time::UtcDateTime — a UTC datetime with ISO 8601 schema."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "UtcDateTimeWrap",
        <UtcDateTimeWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<UtcDateTimeWrap>
    ));

    impl crate::ElicitSpec for time::UtcOffset {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::UtcOffset".to_string())
                .summary(
                    "A UTC offset stored as whole seconds (e.g., +3600 for +01:00).".to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::UtcOffset",
        <time::UtcOffset as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::UtcOffset>
    ));

    impl crate::ElicitSpec for UtcOffsetWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("UtcOffsetWrap".to_string())
                .summary(
                    "Trenchcoat for time::UtcOffset — a UTC offset as ±HH:MM:SS string."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "UtcOffsetWrap",
        <UtcOffsetWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<UtcOffsetWrap>
    ));

    impl crate::ElicitSpec for time::error::ComponentRange {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::error::ComponentRange".to_string())
                .summary(
                    "A time component range error — identifies which component was out of range."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::error::ComponentRange",
        <time::error::ComponentRange as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::error::ComponentRange>
    ));

    impl crate::ElicitSpec for ComponentRangeWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("ComponentRangeWrap".to_string())
                .summary(
                    "Trenchcoat for time::error::ComponentRange — serializes as the component name string."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "ComponentRangeWrap",
        <ComponentRangeWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<ComponentRangeWrap>
    ));

    impl crate::ElicitSpec for time::error::ConversionRange {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::error::ConversionRange".to_string())
                .summary(
                    "A time conversion range error — produced when a std::time::Duration exceeds time::Duration's range (single-value type)."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::error::ConversionRange",
        <time::error::ConversionRange as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::error::ConversionRange>
    ));

    impl crate::ElicitSpec for ConversionRangeWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("ConversionRangeWrap".to_string())
                .summary(
                    "Trenchcoat for time::error::ConversionRange — serializes as JSON null (single-value unit type)."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "ConversionRangeWrap",
        <ConversionRangeWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<ConversionRangeWrap>
    ));

    impl crate::ElicitSpec for time::error::DifferentVariant {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("time::error::DifferentVariant".to_string())
                .summary(
                    "A format-description variant mismatch error — produced when a FormatDescription method requires a specific variant but finds a different one (single-value type)."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::error::DifferentVariant",
        <time::error::DifferentVariant as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::error::DifferentVariant>
    ));

    impl crate::ElicitSpec for DifferentVariantWrap {
        fn type_spec() -> crate::TypeSpec {
            TypeSpecBuilder::default()
                .type_name("DifferentVariantWrap".to_string())
                .summary(
                    "Trenchcoat for time::error::DifferentVariant — serializes as JSON null (single-value unit type)."
                        .to_string(),
                )
                .categories(vec![])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "DifferentVariantWrap",
        <DifferentVariantWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<DifferentVariantWrap>
    ));

    impl crate::ElicitSpec for time::format_description::modifier::End {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::End",
                "End-of-input component — single value (trailing_input field is pub(crate)).",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::End",
        <time::format_description::modifier::End as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::End>
    ));

    impl crate::ElicitSpec for FmtEndWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "FmtEndWrap",
                "Trenchcoat for time::format_description::modifier::End — serializes as JSON null.",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtEndWrap",
        <FmtEndWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtEndWrap>
    ));

    impl crate::ElicitSpec for time::format_description::modifier::TrailingInput {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::TrailingInput",
                "Whether trailing input after the declared end is permitted.",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::TrailingInput",
        <time::format_description::modifier::TrailingInput as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::TrailingInput>
    ));

    impl crate::ElicitSpec for FmtTrailingInputWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "FmtTrailingInputWrap",
                "Trenchcoat for time::format_description::modifier::TrailingInput — serializes as \"Prohibit\" or \"Discard\".",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtTrailingInputWrap",
        <FmtTrailingInputWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtTrailingInputWrap>
    ));

    impl crate::ElicitSpec for time::format_description::modifier::Day {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::Day",
                "Day-of-month component modifier — controls the padding type.",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::Day",
        <time::format_description::modifier::Day as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::Day>
    ));

    impl crate::ElicitSpec for FmtDayWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "FmtDayWrap",
                "Trenchcoat for time::format_description::modifier::Day — serializes as {\"padding\": \"Space\"|\"Zero\"|\"None\"}.",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtDayWrap",
        <FmtDayWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtDayWrap>
    ));

    impl crate::ElicitSpec for time::format_description::modifier::Padding {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::Padding",
                "Padding strategy for a format description field — Space, Zero, or None.",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::Padding",
        <time::format_description::modifier::Padding as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::Padding>
    ));

    impl crate::ElicitSpec for FmtPaddingWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "FmtPaddingWrap",
                "Trenchcoat for time::format_description::modifier::Padding — serializes as a JSON string (\"Space\", \"Zero\", or \"None\").",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtPaddingWrap",
        <FmtPaddingWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtPaddingWrap>
    ));

    impl crate::ElicitSpec for time::format_description::modifier::Ignore {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::Ignore",
                "Ignore a non-zero count of bytes in format descriptions.",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::Ignore",
        <time::format_description::modifier::Ignore as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::Ignore>
    ));

    impl crate::ElicitSpec for FmtIgnoreWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "FmtIgnoreWrap",
                "Trenchcoat for time::format_description::modifier::Ignore — serializes as {\"count\": u16} (non-zero).",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtIgnoreWrap",
        <FmtIgnoreWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtIgnoreWrap>
    ));

    impl crate::ElicitSpec for time::format_description::modifier::Minute {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::Minute",
                "Minute modifier with padding configuration.",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::Minute",
        <time::format_description::modifier::Minute as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::Minute>
    ));

    impl crate::ElicitSpec for FmtMinuteWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "FmtMinuteWrap",
                "Trenchcoat for time::format_description::modifier::Minute — serializes as {\"padding\": \"Space\"|\"Zero\"|\"None\"}.",
                vec![],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtMinuteWrap",
        <FmtMinuteWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtMinuteWrap>
    ));

    // ── padding-only modifiers (Ordinal, Second, OffsetMinute, OffsetSecond,
    //    WeekNumberIso, WeekNumberSunday, WeekNumberMonday) ──────────────────

    macro_rules! impl_padding_modifier_spec {
        ($ty:ty, $name:literal, $summary:literal, $wrap:ty, $wrap_name:literal, $wrap_summary:literal) => {
            impl crate::ElicitSpec for $ty {
                fn type_spec() -> crate::TypeSpec {
                    crate::TypeSpec::new($name, $summary, vec![])
                }
            }
            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
            impl crate::ElicitSpec for $wrap {
                fn type_spec() -> crate::TypeSpec {
                    crate::TypeSpec::new($wrap_name, $wrap_summary, vec![])
                }
            }
            inventory::submit!(TypeSpecInventoryKey::new(
                $wrap_name,
                <$wrap as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$wrap>
            ));
        };
    }

    impl_padding_modifier_spec!(
        time::format_description::modifier::Ordinal,
        "time::format_description::modifier::Ordinal",
        "Ordinal day-of-year (1–366) with padding.",
        FmtOrdinalWrap,
        "FmtOrdinalWrap",
        "Trenchcoat for modifier::Ordinal — serializes as {\"padding\": ...}."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::Second,
        "time::format_description::modifier::Second",
        "Second within the minute (0–59) with padding.",
        FmtSecondWrap,
        "FmtSecondWrap",
        "Trenchcoat for modifier::Second — serializes as {\"padding\": ...}."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::OffsetMinute,
        "time::format_description::modifier::OffsetMinute",
        "UTC offset minute component with padding.",
        FmtOffsetMinuteWrap,
        "FmtOffsetMinuteWrap",
        "Trenchcoat for modifier::OffsetMinute — serializes as {\"padding\": ...}."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::OffsetSecond,
        "time::format_description::modifier::OffsetSecond",
        "UTC offset second component with padding.",
        FmtOffsetSecondWrap,
        "FmtOffsetSecondWrap",
        "Trenchcoat for modifier::OffsetSecond — serializes as {\"padding\": ...}."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::WeekNumberIso,
        "time::format_description::modifier::WeekNumberIso",
        "ISO week number with padding.",
        FmtWeekNumberIsoWrap,
        "FmtWeekNumberIsoWrap",
        "Trenchcoat for modifier::WeekNumberIso — serializes as {\"padding\": ...}."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::WeekNumberSunday,
        "time::format_description::modifier::WeekNumberSunday",
        "Sunday-based week number with padding.",
        FmtWeekNumberSundayWrap,
        "FmtWeekNumberSundayWrap",
        "Trenchcoat for modifier::WeekNumberSunday — serializes as {\"padding\": ...}."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::WeekNumberMonday,
        "time::format_description::modifier::WeekNumberMonday",
        "Monday-based week number with padding.",
        FmtWeekNumberMondayWrap,
        "FmtWeekNumberMondayWrap",
        "Trenchcoat for modifier::WeekNumberMonday — serializes as {\"padding\": ...}."
    );

    // ── Opaque-padding modifier specs ─────────────────────────────────────────
    // Raw types emit default() in ToCodeLiteral; wrappers emit faithfully.

    impl_padding_modifier_spec!(
        time::format_description::modifier::Hour12,
        "time::format_description::modifier::Hour12",
        "12-hour clock hour with padding.",
        FmtHour12Wrap,
        "FmtHour12Wrap",
        "Trenchcoat for modifier::Hour12 — stores Padding for faithful code generation."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::Hour24,
        "time::format_description::modifier::Hour24",
        "24-hour clock hour with padding.",
        FmtHour24Wrap,
        "FmtHour24Wrap",
        "Trenchcoat for modifier::Hour24 — stores Padding for faithful code generation."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::MonthNumerical,
        "time::format_description::modifier::MonthNumerical",
        "Numerical month with padding.",
        FmtMonthNumericalWrap,
        "FmtMonthNumericalWrap",
        "Trenchcoat for modifier::MonthNumerical — stores Padding for faithful code generation."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::CalendarYearLastTwo,
        "time::format_description::modifier::CalendarYearLastTwo",
        "Calendar year last two digits with padding.",
        FmtCalendarYearLastTwoWrap,
        "FmtCalendarYearLastTwoWrap",
        "Trenchcoat for modifier::CalendarYearLastTwo — stores Padding for faithful code generation."
    );
    impl_padding_modifier_spec!(
        time::format_description::modifier::IsoYearLastTwo,
        "time::format_description::modifier::IsoYearLastTwo",
        "ISO year last two digits with padding.",
        FmtIsoYearLastTwoWrap,
        "FmtIsoYearLastTwoWrap",
        "Trenchcoat for modifier::IsoYearLastTwo — stores Padding for faithful code generation."
    );

    // ── Bool-field modifier specs ─────────────────────────────────────────────

    macro_rules! impl_bool_modifier_spec {
        ($ty:ty, $name:literal, $summary:literal, $wrap:ty, $wrap_name:literal, $wrap_summary:literal) => {
            impl crate::ElicitSpec for $ty {
                fn type_spec() -> crate::TypeSpec {
                    crate::TypeSpec::new($name, $summary, vec![])
                }
            }
            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
            impl crate::ElicitSpec for $wrap {
                fn type_spec() -> crate::TypeSpec {
                    crate::TypeSpec::new($wrap_name, $wrap_summary, vec![])
                }
            }
            inventory::submit!(TypeSpecInventoryKey::new(
                $wrap_name,
                <$wrap as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$wrap>
            ));
        };
    }

    impl_bool_modifier_spec!(
        time::format_description::modifier::MonthShort,
        "time::format_description::modifier::MonthShort",
        "Abbreviated month name; case_sensitive controls parsing.",
        FmtMonthShortWrap,
        "FmtMonthShortWrap",
        "Trenchcoat for modifier::MonthShort — stores case_sensitive bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::MonthLong,
        "time::format_description::modifier::MonthLong",
        "Full month name; case_sensitive controls parsing.",
        FmtMonthLongWrap,
        "FmtMonthLongWrap",
        "Trenchcoat for modifier::MonthLong — stores case_sensitive bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::WeekdayShort,
        "time::format_description::modifier::WeekdayShort",
        "Abbreviated weekday name; case_sensitive controls parsing.",
        FmtWeekdayShortWrap,
        "FmtWeekdayShortWrap",
        "Trenchcoat for modifier::WeekdayShort — stores case_sensitive bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::WeekdayLong,
        "time::format_description::modifier::WeekdayLong",
        "Full weekday name; case_sensitive controls parsing.",
        FmtWeekdayLongWrap,
        "FmtWeekdayLongWrap",
        "Trenchcoat for modifier::WeekdayLong — stores case_sensitive bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::WeekdaySunday,
        "time::format_description::modifier::WeekdaySunday",
        "Sunday-based weekday index; one_indexed controls 0- vs 1-based output.",
        FmtWeekdaySundayWrap,
        "FmtWeekdaySundayWrap",
        "Trenchcoat for modifier::WeekdaySunday — stores one_indexed bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::WeekdayMonday,
        "time::format_description::modifier::WeekdayMonday",
        "Monday-based weekday index; one_indexed controls 0- vs 1-based output.",
        FmtWeekdayMondayWrap,
        "FmtWeekdayMondayWrap",
        "Trenchcoat for modifier::WeekdayMonday — stores one_indexed bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::UnixTimestampSecond,
        "time::format_description::modifier::UnixTimestampSecond",
        "Unix timestamp at second precision; sign_is_mandatory controls + sign.",
        FmtUnixTimestampSecondWrap,
        "FmtUnixTimestampSecondWrap",
        "Trenchcoat for modifier::UnixTimestampSecond — stores sign_is_mandatory bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::UnixTimestampMillisecond,
        "time::format_description::modifier::UnixTimestampMillisecond",
        "Unix timestamp at millisecond precision; sign_is_mandatory controls + sign.",
        FmtUnixTimestampMillisecondWrap,
        "FmtUnixTimestampMillisecondWrap",
        "Trenchcoat for modifier::UnixTimestampMillisecond — stores sign_is_mandatory bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::UnixTimestampMicrosecond,
        "time::format_description::modifier::UnixTimestampMicrosecond",
        "Unix timestamp at microsecond precision; sign_is_mandatory controls + sign.",
        FmtUnixTimestampMicrosecondWrap,
        "FmtUnixTimestampMicrosecondWrap",
        "Trenchcoat for modifier::UnixTimestampMicrosecond — stores sign_is_mandatory bool."
    );
    impl_bool_modifier_spec!(
        time::format_description::modifier::UnixTimestampNanosecond,
        "time::format_description::modifier::UnixTimestampNanosecond",
        "Unix timestamp at nanosecond precision; sign_is_mandatory controls + sign.",
        FmtUnixTimestampNanosecondWrap,
        "FmtUnixTimestampNanosecondWrap",
        "Trenchcoat for modifier::UnixTimestampNanosecond — stores sign_is_mandatory bool."
    );

    // ── Padding+sign modifier specs ───────────────────────────────────────────

    macro_rules! impl_padding_sign_modifier_spec {
        ($ty:ty, $name:literal, $summary:literal, $wrap:ty, $wrap_name:literal, $wrap_summary:literal) => {
            impl crate::ElicitSpec for $ty {
                fn type_spec() -> crate::TypeSpec {
                    crate::TypeSpec::new($name, $summary, vec![])
                }
            }
            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
            impl crate::ElicitSpec for $wrap {
                fn type_spec() -> crate::TypeSpec {
                    crate::TypeSpec::new($wrap_name, $wrap_summary, vec![])
                }
            }
            inventory::submit!(TypeSpecInventoryKey::new(
                $wrap_name,
                <$wrap as crate::ElicitSpec>::type_spec,
                std::any::TypeId::of::<$wrap>
            ));
        };
    }

    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::CalendarYearFullExtendedRange,
        "time::format_description::modifier::CalendarYearFullExtendedRange",
        "Calendar year (full digits, extended range) with padding and sign.",
        FmtCalendarYearFullExtendedRangeWrap,
        "FmtCalendarYearFullExtendedRangeWrap",
        "Trenchcoat for modifier::CalendarYearFullExtendedRange."
    );
    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::CalendarYearFullStandardRange,
        "time::format_description::modifier::CalendarYearFullStandardRange",
        "Calendar year (full digits, standard range) with padding and sign.",
        FmtCalendarYearFullStandardRangeWrap,
        "FmtCalendarYearFullStandardRangeWrap",
        "Trenchcoat for modifier::CalendarYearFullStandardRange."
    );
    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::CalendarYearCenturyExtendedRange,
        "time::format_description::modifier::CalendarYearCenturyExtendedRange",
        "Calendar year century (extended range) with padding and sign.",
        FmtCalendarYearCenturyExtendedRangeWrap,
        "FmtCalendarYearCenturyExtendedRangeWrap",
        "Trenchcoat for modifier::CalendarYearCenturyExtendedRange."
    );
    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::CalendarYearCenturyStandardRange,
        "time::format_description::modifier::CalendarYearCenturyStandardRange",
        "Calendar year century (standard range) with padding and sign.",
        FmtCalendarYearCenturyStandardRangeWrap,
        "FmtCalendarYearCenturyStandardRangeWrap",
        "Trenchcoat for modifier::CalendarYearCenturyStandardRange."
    );
    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::IsoYearFullExtendedRange,
        "time::format_description::modifier::IsoYearFullExtendedRange",
        "ISO year (full digits, extended range) with padding and sign.",
        FmtIsoYearFullExtendedRangeWrap,
        "FmtIsoYearFullExtendedRangeWrap",
        "Trenchcoat for modifier::IsoYearFullExtendedRange."
    );
    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::IsoYearFullStandardRange,
        "time::format_description::modifier::IsoYearFullStandardRange",
        "ISO year (full digits, standard range) with padding and sign.",
        FmtIsoYearFullStandardRangeWrap,
        "FmtIsoYearFullStandardRangeWrap",
        "Trenchcoat for modifier::IsoYearFullStandardRange."
    );
    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::IsoYearCenturyExtendedRange,
        "time::format_description::modifier::IsoYearCenturyExtendedRange",
        "ISO year century (extended range) with padding and sign.",
        FmtIsoYearCenturyExtendedRangeWrap,
        "FmtIsoYearCenturyExtendedRangeWrap",
        "Trenchcoat for modifier::IsoYearCenturyExtendedRange."
    );
    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::IsoYearCenturyStandardRange,
        "time::format_description::modifier::IsoYearCenturyStandardRange",
        "ISO year century (standard range) with padding and sign.",
        FmtIsoYearCenturyStandardRangeWrap,
        "FmtIsoYearCenturyStandardRangeWrap",
        "Trenchcoat for modifier::IsoYearCenturyStandardRange."
    );

    // ── OffsetHour, Period, Subsecond(Digits) specs ───────────────────────────

    impl_padding_sign_modifier_spec!(
        time::format_description::modifier::OffsetHour,
        "time::format_description::modifier::OffsetHour",
        "UTC offset hour with padding and mandatory-sign flag.",
        FmtOffsetHourWrap,
        "FmtOffsetHourWrap",
        "Trenchcoat for modifier::OffsetHour."
    );

    impl crate::ElicitSpec for time::format_description::modifier::Period {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::Period",
                "AM/PM period with is_uppercase and case_sensitive flags.",
                vec![],
            )
        }
    }
    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::Period",
        <time::format_description::modifier::Period as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::Period>
    ));
    impl crate::ElicitSpec for FmtPeriodWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new("FmtPeriodWrap", "Trenchcoat for modifier::Period.", vec![])
        }
    }
    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtPeriodWrap",
        <FmtPeriodWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtPeriodWrap>
    ));

    impl crate::ElicitSpec for time::format_description::modifier::SubsecondDigits {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::SubsecondDigits",
                "Number of subsecond digits to display (OneOrMore, One…Nine).",
                vec![],
            )
        }
    }
    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::SubsecondDigits",
        <time::format_description::modifier::SubsecondDigits as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::SubsecondDigits>
    ));
    impl crate::ElicitSpec for FmtSubsecondDigitsWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "FmtSubsecondDigitsWrap",
                "Trenchcoat for modifier::SubsecondDigits.",
                vec![],
            )
        }
    }
    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtSubsecondDigitsWrap",
        <FmtSubsecondDigitsWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtSubsecondDigitsWrap>
    ));

    impl crate::ElicitSpec for time::format_description::modifier::Subsecond {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::modifier::Subsecond",
                "Subsecond component with configurable digit count.",
                vec![],
            )
        }
    }
    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::modifier::Subsecond",
        <time::format_description::modifier::Subsecond as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::modifier::Subsecond>
    ));
    impl crate::ElicitSpec for FmtSubsecondWrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "FmtSubsecondWrap",
                "Trenchcoat for modifier::Subsecond.",
                vec![],
            )
        }
    }
    inventory::submit!(TypeSpecInventoryKey::new(
        "FmtSubsecondWrap",
        <FmtSubsecondWrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<FmtSubsecondWrap>
    ));

    // ── well_known::Rfc2822 ───────────────────────────────────────────────────

    impl crate::ElicitSpec for time::format_description::well_known::Rfc2822 {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "time::format_description::well_known::Rfc2822",
                "RFC 2822 email date-time format — unit struct, no configuration.",
                vec![],
            )
        }
    }
    inventory::submit!(TypeSpecInventoryKey::new(
        "time::format_description::well_known::Rfc2822",
        <time::format_description::well_known::Rfc2822 as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<time::format_description::well_known::Rfc2822>
    ));

    impl crate::ElicitSpec for WellKnownRfc2822Wrap {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "WellKnownRfc2822Wrap",
                "Trenchcoat for well_known::Rfc2822 — serializes as {}.",
                vec![],
            )
        }
    }
    inventory::submit!(TypeSpecInventoryKey::new(
        "WellKnownRfc2822Wrap",
        <WellKnownRfc2822Wrap as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<WellKnownRfc2822Wrap>
    ));
}
