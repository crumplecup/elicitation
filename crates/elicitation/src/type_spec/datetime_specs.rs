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
