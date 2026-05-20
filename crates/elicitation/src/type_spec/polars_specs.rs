//! [`ElicitSpec`](crate::ElicitSpec) and [`ElicitComplete`](crate::ElicitComplete)
//! implementations for polars pipeline types.
//!
//! Available with the `polars-types` feature.

#[cfg(feature = "polars-types")]
mod polars_impls {
    use crate::{
        ElicitComplete, ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    macro_rules! polars_select_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            variants = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let variants = SpecCategoryBuilder::default()
                        .name("variants".to_string())
                        .entries(vec![$(
                            SpecEntryBuilder::default()
                                .label($label.to_string())
                                .description($desc.to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        )+])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants])
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

    macro_rules! polars_survey_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(vec![$(
                            SpecEntryBuilder::default()
                                .label($label.to_string())
                                .description($desc.to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        )+])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![fields])
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

    polars_select_spec!(
        type    = crate::PolarsJoinType,
        name    = "elicitation::PolarsJoinType",
        summary = "Join strategy for Polars DataFrames.",
        variants = [
            ("inner", "Keep only rows with matching keys in both frames"),
            ("left",  "Keep all rows from the left frame; fill right misses with null"),
            ("right", "Keep all rows from the right frame; fill left misses with null"),
            ("full",  "Keep all rows from both frames; fill misses with null"),
            ("semi",  "Keep left rows that have at least one match in right"),
            ("anti",  "Keep left rows that have no match in right"),
            ("cross", "Cartesian product of both frames"),
        ]
    );

    polars_select_spec!(
        type    = crate::PolarsDType,
        name    = "elicitation::PolarsDType",
        summary = "Polars column data type.",
        variants = [
            ("boolean",  "Boolean (true/false)"),
            ("uint8",    "Unsigned 8-bit integer"),
            ("uint16",   "Unsigned 16-bit integer"),
            ("uint32",   "Unsigned 32-bit integer"),
            ("uint64",   "Unsigned 64-bit integer"),
            ("int8",     "Signed 8-bit integer"),
            ("int16",    "Signed 16-bit integer"),
            ("int32",    "Signed 32-bit integer"),
            ("int64",    "Signed 64-bit integer"),
            ("float32",  "32-bit floating point"),
            ("float64",  "64-bit floating point"),
        ]
    );

    polars_select_spec!(
        type    = crate::PolarsPipelineOp,
        name    = "elicitation::PolarsPipelineOp",
        summary = "A single Polars LazyFrame operation in a pipeline.",
        variants = [
            ("select",      "Select columns by name"),
            ("filter",      "Filter rows by a boolean expression string"),
            ("with_column", "Add or replace a column with an expression string"),
            ("rename",      "Rename columns (from/to name pairs)"),
            ("drop",        "Drop columns by name"),
            ("sort",        "Sort by columns with optional descending flags"),
            ("limit",       "Limit to N rows"),
            ("group_by",    "Group by columns and aggregate"),
            ("join",        "Join with another named DataFrame"),
            ("collect",     "Collect lazy frame to eager DataFrame"),
            ("write_csv",   "Write result to a CSV file path"),
            ("write_json",  "Write result to a JSON file path"),
            ("read_csv",    "Read a CSV file into the pipeline"),
            ("read_json",   "Read a JSON file into the pipeline"),
        ]
    );

    polars_survey_spec!(
        type    = crate::PolarsPipelineStep,
        name    = "elicitation::PolarsPipelineStep",
        summary = "A single named step in a Polars pipeline.",
        fields  = [
            ("step_id", "Uuid — unique identifier for this step"),
            ("op",      "PolarsPipelineOp — the operation to perform"),
        ]
    );

    polars_survey_spec!(
        type    = crate::PolarsPipelineDescriptor,
        name    = "elicitation::PolarsPipelineDescriptor",
        summary = "Descriptor for a complete Polars LazyFrame pipeline.",
        fields  = [
            ("pipeline_id", "Uuid — unique identifier for the pipeline"),
            ("name",        "String — human-readable pipeline name"),
            ("steps",       "Vec<PolarsPipelineStep> — ordered list of pipeline steps"),
        ]
    );
}
