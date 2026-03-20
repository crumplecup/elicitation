//! [`ElicitSpec`](crate::ElicitSpec) implementations for sqlx type elicitation.
//!
//! Available with the `sqlx-types` feature.
//!
//! Complements the [`ElicitIntrospect`](crate::ElicitIntrospect) impls in
//! `primitives/sqlx_types/` — those describe *structure* (pattern, variants),
//! these describe *contracts and usage* browsable by agents via `describe_type`.

#[cfg(feature = "sqlx-types")]
mod sqlx_impls {
    use crate::{
        ColumnDescriptor, ColumnValue, ElicitSpec, RowData, SpecCategoryBuilder, SpecEntryBuilder,
        SqlTypeKind, TypeSpec, TypeSpecBuilder, TypeSpecInventoryKey,
    };
    use sqlx::any::{AnyQueryResult, AnyTypeInfo, AnyTypeInfoKind};
    use sqlx::error::ErrorKind;

    // -------------------------------------------------------------------------
    // Macro: impl_select_spec!
    //
    // Generates ElicitSpec for a sqlx Select enum. Produces a TypeSpec with:
    //   - "variants" category listing each label and its description
    //   - "source" category noting this is a sqlx type
    // -------------------------------------------------------------------------

    macro_rules! impl_select_spec {
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
                                .description("sqlx v0.8 — async SQL toolkit".to_string())
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

    // -------------------------------------------------------------------------
    // Macro: impl_builder_spec!
    //
    // Generates ElicitSpec for a sqlx Survey/builder type.
    // -------------------------------------------------------------------------

    macro_rules! impl_builder_spec {
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
                                .description("sqlx v0.8 — async SQL toolkit".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — fill in each field".to_string())
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
    }

    // -------------------------------------------------------------------------
    // ErrorKind — database constraint violation category
    // -------------------------------------------------------------------------

    impl_select_spec!(
        type    = ErrorKind,
        name    = "sqlx::error::ErrorKind",
        summary = "Database constraint violation category returned by DatabaseError::kind().",
        variants = [
            ("UniqueViolation",     "A unique or primary key constraint was violated"),
            ("ForeignKeyViolation", "A foreign key constraint was violated"),
            ("NotNullViolation",    "A NOT NULL constraint was violated"),
            ("CheckViolation",      "A CHECK constraint was violated"),
            ("Other",               "An unmapped database error not matching the above categories"),
        ]
    );

    // -------------------------------------------------------------------------
    // AnyTypeInfoKind — SQL column type category
    // -------------------------------------------------------------------------

    impl_select_spec!(
        type    = AnyTypeInfoKind,
        name    = "sqlx::any::AnyTypeInfoKind",
        summary = "SQL column type category used by the Any database driver.",
        variants = [
            ("Null",     "SQL NULL type — column has no type or the value is null"),
            ("Bool",     "BOOLEAN — true/false value"),
            ("SmallInt", "SMALLINT — 16-bit integer (-32768 to 32767)"),
            ("Integer",  "INTEGER — 32-bit integer"),
            ("BigInt",   "BIGINT — 64-bit integer"),
            ("Real",     "REAL / FLOAT4 — 32-bit floating point"),
            ("Double",   "DOUBLE PRECISION / FLOAT8 — 64-bit floating point"),
            ("Text",     "TEXT / VARCHAR — variable-length character string"),
            ("Blob",     "BLOB / BYTEA — binary data"),
        ]
    );

    // -------------------------------------------------------------------------
    // SqlTypeKind — serializable column type category (our own type)
    // -------------------------------------------------------------------------

    impl_select_spec!(
        type    = SqlTypeKind,
        name    = "elicitation::SqlTypeKind",
        summary = "Serializable SQL column type category that can cross the MCP boundary. \
                   Mirrors sqlx::any::AnyTypeInfoKind with Serialize/Deserialize/JsonSchema.",
        variants = [
            ("Null",     "SQL NULL type"),
            ("Bool",     "BOOLEAN — true/false"),
            ("SmallInt", "SMALLINT — 16-bit integer"),
            ("Integer",  "INTEGER — 32-bit integer"),
            ("BigInt",   "BIGINT — 64-bit integer"),
            ("Real",     "REAL — 32-bit float"),
            ("Double",   "DOUBLE PRECISION — 64-bit float"),
            ("Text",     "TEXT — variable-length string"),
            ("Blob",     "BLOB — binary data"),
        ]
    );

    // -------------------------------------------------------------------------
    // ColumnValue — serializable SQL value
    // -------------------------------------------------------------------------

    impl_select_spec!(
        type    = ColumnValue,
        name    = "elicitation::ColumnValue",
        summary = "Serializable SQL column value that can cross the MCP boundary. \
                   Mirrors sqlx::any::AnyValueKind with owned types and serde support.",
        variants = [
            ("Null",     "SQL NULL — no value"),
            ("Bool",     "Boolean — true or false"),
            ("SmallInt", "16-bit integer (i16)"),
            ("Integer",  "32-bit integer (i32)"),
            ("BigInt",   "64-bit integer (i64)"),
            ("Real",     "32-bit float (f32)"),
            ("Double",   "64-bit float (f64)"),
            ("Text",     "UTF-8 string (String)"),
            ("Blob",     "Binary data (Vec<u8>), base64-encoded in JSON"),
        ]
    );

    // -------------------------------------------------------------------------
    // AnyTypeInfo — SQL type info with kind
    // -------------------------------------------------------------------------

    impl_builder_spec!(
        type    = AnyTypeInfo,
        name    = "sqlx::any::AnyTypeInfo",
        summary = "SQL column type information from the Any database driver. \
                   Contains a single AnyTypeInfoKind field.",
        fields  = [
            ("kind", "AnyTypeInfoKind — the SQL type category (Null, Bool, SmallInt, Integer, BigInt, Real, Double, Text, Blob)"),
        ]
    );

    // -------------------------------------------------------------------------
    // AnyQueryResult — query execution statistics
    // -------------------------------------------------------------------------

    impl_builder_spec!(
        type    = AnyQueryResult,
        name    = "sqlx::any::AnyQueryResult",
        summary = "Result statistics from an executed SQL statement: rows affected and optional last insert ID.",
        fields  = [
            ("rows_affected",  "u64 — number of rows inserted, updated, or deleted"),
            ("last_insert_id", "Option<i64> — the ROWID/OID of the last inserted row, if supported by the database"),
        ]
    );

    // -------------------------------------------------------------------------
    // ColumnDescriptor — serializable column metadata
    // -------------------------------------------------------------------------

    impl_builder_spec!(
        type    = ColumnDescriptor,
        name    = "elicitation::ColumnDescriptor",
        summary = "Serializable SQL column metadata: ordinal position, name, and type kind.",
        fields  = [
            ("ordinal",   "usize — zero-based column position in the result set"),
            ("name",      "String — column name as returned by the database"),
            ("type_kind", "SqlTypeKind — SQL type category for this column"),
        ]
    );

    // -------------------------------------------------------------------------
    // RowData — serializable SQL row
    // -------------------------------------------------------------------------

    impl_builder_spec!(
        type    = RowData,
        name    = "elicitation::RowData",
        summary = "Serializable SQL row: a list of (column_name, ColumnValue) pairs. \
                   Used to transport row data across the MCP boundary.",
        fields  = [
            ("columns", "Vec<ColumnEntry> — all column name/value pairs in this row"),
        ]
    );
}
