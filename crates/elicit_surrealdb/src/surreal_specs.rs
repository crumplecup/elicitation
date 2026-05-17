//! [`ElicitSpec`](elicitation::ElicitSpec) stubs for SurrealDB bridge types.
//!
//! Available with the `surreal-types` feature.

mod surreal_impls {
    use crate::primitives::surreal_types::{
        Datetime, Duration, Geometry, GeometryKind, Kind, Number, PatchOp, RecordId, Table, Value,
    };
    use elicitation::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_surreal_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            pattern = $pattern:literal,
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
                                .description("surrealdb-types v3 — SurrealDB value types".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description($pattern.to_string())
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
        };

        (
            type     = $ty:ty,
            name     = $name:literal,
            summary  = $summary:literal,
            pattern  = $pattern:literal,
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
                                .description("surrealdb-types v3 — SurrealDB value types".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description($pattern.to_string())
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

    impl_surreal_spec!(
        type    = Value,
        name    = "surreal::Value",
        summary = "A SurrealDB value that can represent any storable data type.",
        pattern = "Select — choose one variant from the list, then supply its payload",
        variants = [
            ("None",     "The absence of a value (NONE in SurrealQL)"),
            ("Null",     "A null value (NULL in SurrealQL)"),
            ("Bool",     "A boolean true or false"),
            ("Number",   "An integer, float, or decimal number"),
            ("String",   "A UTF-8 string"),
            ("Bytes",    "Raw bytes (base64-encoded for transport)"),
            ("Duration", "A duration span, e.g. 1h30m"),
            ("Datetime", "A UTC datetime, ISO 8601 format"),
            ("Uuid",     "A UUID, as a string"),
            ("Geometry", "A geometric shape (point, line, polygon, …)"),
            ("Table",    "A table name used as a value"),
            ("RecordId", "A record identifier table:key"),
            ("Array",    "An array of values"),
            ("Object",   "A key-value object"),
        ]
    );

    impl_surreal_spec!(
        type    = RecordId,
        name    = "surreal::RecordId",
        summary = "A SurrealDB record identifier: a (table, key) pair.",
        pattern = "Survey — fill table (string) and key (JSON value) fields",
        fields  = [
            ("table", "The table name the record belongs to"),
            ("key",   "The key uniquely identifying the record within the table (string, int, UUID, or object)"),
        ]
    );

    impl_surreal_spec!(
        type    = Number,
        name    = "surreal::Number",
        summary = "A SurrealDB numeric value — integer, float, or decimal.",
        pattern = "Select — choose Int, Float, or Decimal variant",
        variants = [
            ("Int",     "A 64-bit signed integer"),
            ("Float",   "A 64-bit floating-point number"),
            ("Decimal", "An arbitrary-precision decimal, provided as a string"),
        ]
    );

    impl_surreal_spec!(
        type    = Geometry,
        name    = "surreal::Geometry",
        summary = "A SurrealDB geometry value — a GeoJSON-compatible spatial shape.",
        pattern = "Select — choose geometry variant, then provide coordinate arrays",
        variants = [
            ("Point",        "A single 2D point [longitude, latitude]"),
            ("Line",         "An ordered sequence of coordinate pairs"),
            ("Polygon",      "An outer ring plus optional holes"),
            ("MultiPoint",   "Multiple points"),
            ("MultiLine",    "Multiple line strings"),
            ("MultiPolygon", "Multiple polygons"),
            ("Collection",   "A collection of geometry values"),
        ]
    );

    impl_surreal_spec!(
        type    = Datetime,
        name    = "surreal::Datetime",
        summary = "A SurrealDB datetime — a UTC point in time as an ISO 8601 string.",
        pattern = "Survey — provide `value` as an ISO 8601 string, e.g. 2024-01-15T10:30:00Z",
        fields  = [
            ("value", "ISO 8601 datetime in UTC, e.g. \"2024-01-15T10:30:00Z\""),
        ]
    );

    impl_surreal_spec!(
        type    = Duration,
        name    = "surreal::Duration",
        summary = "A SurrealDB duration — a time span in SurrealDB notation.",
        pattern = "Survey — provide `value` as a SurrealDB duration string, e.g. 1y2w3d4h5m6s",
        fields  = [
            ("value", "SurrealDB duration string, e.g. \"1h30m\" or \"500ms\""),
        ]
    );

    impl_surreal_spec!(
        type    = Kind,
        name    = "surreal::Kind",
        summary = "A SurrealDB field type kind for DEFINE FIELD … TYPE declarations.",
        pattern = "Select — choose the type kind variant, with optional params for parameterised kinds",
        variants = [
            ("Any",           "The most permissive type — any value"),
            ("None",          "No value"),
            ("Null",          "Null value"),
            ("Bool",          "Boolean"),
            ("Bytes",         "Raw bytes"),
            ("Datetime",      "Datetime"),
            ("Decimal",       "Arbitrary-precision decimal"),
            ("Duration",      "Duration span"),
            ("Float",         "64-bit float"),
            ("Int",           "64-bit signed integer"),
            ("Number",        "Any numeric type"),
            ("Object",        "JSON object"),
            ("String",        "UTF-8 string"),
            ("Uuid",          "UUID"),
            ("Regex",         "Regular expression"),
            ("Range",         "A range of values"),
            ("Table",         "A table type, optionally restricted to named tables"),
            ("Record",        "A record reference, optionally restricted to named tables"),
            ("Geometry",      "A geometry type, optionally restricted to named geometry kinds"),
            ("Either",        "Any of several types"),
            ("Set",           "A typed set with optional maximum size"),
            ("Array",         "A typed array with optional maximum length"),
            ("File",          "A file reference, optionally restricted to named buckets"),
            ("LiteralString", "A literal string constant"),
            ("LiteralInt",    "A literal integer constant"),
            ("LiteralFloat",  "A literal float constant"),
            ("LiteralBool",   "A literal boolean constant"),
        ]
    );

    impl_surreal_spec!(
        type    = Table,
        name    = "surreal::Table",
        summary = "A SurrealDB table name.",
        pattern = "Survey — provide `name` as a string (alphanumeric + underscore)",
        fields  = [
            ("name", "The table name string"),
        ]
    );

    impl_surreal_spec!(
        type    = PatchOp,
        name    = "surreal::PatchOp",
        summary = "A JSON Patch operation for SurrealDB UPDATE … PATCH statements.",
        pattern = "Select — choose the operation variant",
        variants = [
            ("Add",       "Add a value at a JSON pointer path"),
            ("Remove",    "Remove the value at a JSON pointer path"),
            ("Replace",   "Replace the value at a JSON pointer path"),
            ("Change",    "Apply a string diff at a JSON pointer path"),
            ("Copy",      "Copy a value from one path to another"),
            ("Move",      "Move a value from one path to another"),
            ("Test",      "Test that a value equals an expected value"),
            ("Increment", "Increment the numeric value at a path"),
            ("Decrement", "Decrement the numeric value at a path"),
        ]
    );

    impl_surreal_spec!(
        type     = GeometryKind,
        name     = "surreal::GeometryKind",
        summary  = "A SurrealDB geometry type variant for use in DEFINE FIELD TYPE geometry(…).",
        pattern  = "Select — choose the geometry kind variant",
        variants = [
            ("point",        "Point geometry"),
            ("line",         "Line geometry"),
            ("polygon",      "Polygon geometry"),
            ("multipoint",   "MultiPoint geometry"),
            ("multiline",    "MultiLine geometry"),
            ("multipolygon", "MultiPolygon geometry"),
            ("collection",   "Collection of geometries"),
            ("feature",      "Any geometry type"),
        ]
    );

    impl elicitation::ElicitComplete for Datetime {}
    impl elicitation::ElicitComplete for Duration {}
    impl elicitation::ElicitComplete for Table {}
    impl elicitation::ElicitComplete for RecordId {}
    impl elicitation::ElicitComplete for Number {}
    impl elicitation::ElicitComplete for PatchOp {}
    impl elicitation::ElicitComplete for Geometry {}
    impl elicitation::ElicitComplete for GeometryKind {}
    impl elicitation::ElicitComplete for Kind {}
    impl elicitation::ElicitComplete for Value {}
}
