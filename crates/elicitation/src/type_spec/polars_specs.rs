//! [`ElicitSpec`](crate::ElicitSpec) impls for polars descriptor types.
//!
//! Available with the `polars-types` feature.

#[cfg(feature = "polars-types")]
mod polars_impls {
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_polars_enum_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            variants = [ $( ($label:literal, $desc:literal) ),+ $(,)? ]
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
                                .description("polars 0.53 — DataFrame library based on Apache Arrow".to_string())
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
        };
    }

    impl_polars_enum_spec!(
        type    = crate::PolarsJoinType,
        name    = "PolarsJoinType",
        summary = "Polars join strategy (inner, left, right, full, cross, semi, anti).",
        variants = [
            ("inner", "Keep only rows with matches in both frames."),
            ("left",  "Keep all left rows, NULLs for unmatched right."),
            ("right", "Keep all right rows, NULLs for unmatched left."),
            ("full",  "Keep all rows from both frames (FULL OUTER JOIN)."),
            ("cross", "Cartesian product of both frames."),
            ("semi",  "Keep left rows that have a match in right."),
            ("anti",  "Keep left rows that do NOT have a match in right."),
        ]
    );

    impl_polars_enum_spec!(
        type    = crate::PolarsDType,
        name    = "PolarsDType",
        summary = "Common polars column data types.",
        variants = [
            ("boolean",  "Boolean true/false column."),
            ("int32",    "32-bit signed integer."),
            ("int64",    "64-bit signed integer."),
            ("float32",  "32-bit floating point."),
            ("float64",  "64-bit floating point."),
            ("utf8",     "UTF-8 string column."),
            ("date",     "Calendar date (no time)."),
            ("datetime", "Datetime with optional timezone."),
            ("duration", "Duration (time delta)."),
            ("list",     "List column (each cell is a list)."),
            ("struct",   "Struct column (each cell is a named record)."),
        ]
    );
}
