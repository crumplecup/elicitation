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
                .summary("A signed duration (seconds + nanoseconds). Also aliased as chrono::Duration.".to_string())
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
}
