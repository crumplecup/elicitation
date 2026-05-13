//! [`ElicitComplete`] implementations for csv primitive types.
//!
//! Available with the `csv-types` feature.

#[cfg(feature = "csv-types")]
mod csv_impls {
    use crate::{
        CsvByteRecord, CsvErrorKind, CsvPosition, CsvQuoteStyle, CsvStringRecord, CsvTerminator,
        CsvTrim, ElicitComplete, ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    // -------------------------------------------------------------------------
    // Macro: impl_csv_select_spec!
    //
    // Generates ElicitSpec + inventory::submit! + ElicitComplete for a csv
    // Select enum, listing each variant from Select::labels().
    // -------------------------------------------------------------------------

    macro_rules! impl_csv_select_spec {
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
                                .description("csv v1 — fast, flexible CSV reader and writer".to_string())
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
    // Macro: impl_csv_survey_spec!
    //
    // Generates ElicitSpec + inventory::submit! + ElicitComplete for a csv
    // Survey (struct or newtype) type.
    // -------------------------------------------------------------------------

    macro_rules! impl_csv_survey_spec {
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
                                .description("csv v1 — fast, flexible CSV reader and writer".to_string())
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
    // CsvPosition — byte/line/record position in a CSV stream
    // =========================================================================

    impl_csv_survey_spec!(
        type    = CsvPosition,
        name    = "csv::Position",
        summary = "A position within a CSV stream: byte offset, line number, and record index. All indices are zero-based.",
        fields = [
            ("byte",   "Byte offset from the start of the CSV data"),
            ("line",   "Line number within the stream (zero-based)"),
            ("record", "Record index within the stream (zero-based, excludes header)"),
        ]
    );

    // =========================================================================
    // CsvQuoteStyle — quoting behaviour when writing CSV fields
    // =========================================================================

    impl_csv_select_spec!(
        type    = CsvQuoteStyle,
        name    = "csv::QuoteStyle",
        summary = "Controls when fields are quoted when writing CSV output.",
        variants = [
            ("Always",     "Quote every field regardless of its content"),
            ("Necessary",  "Quote only when the field contains a delimiter, quote char, or record terminator"),
            ("NonNumeric", "Quote all fields that are not an integer or float"),
            ("Never",      "Never quote; the writer returns an error if quoting would be required"),
        ]
    );

    // =========================================================================
    // CsvTrim — whitespace trimming when reading CSV fields
    // =========================================================================

    impl_csv_select_spec!(
        type    = CsvTrim,
        name    = "csv::Trim",
        summary = "Controls how leading/trailing whitespace is trimmed when reading CSV data.",
        variants = [
            ("All",     "Trim whitespace from both field values and header names"),
            ("Fields",  "Trim whitespace from field values only (not header names)"),
            ("Headers", "Trim whitespace from header names only (not field values)"),
            ("None",    "Do not trim any whitespace (the default)"),
        ]
    );

    // =========================================================================
    // CsvTerminator — record terminator used when reading/writing CSV
    // =========================================================================

    impl_csv_select_spec!(
        type    = CsvTerminator,
        name    = "csv::Terminator",
        summary = "The byte sequence used to terminate CSV records.",
        variants = [
            ("Crlf",    "Use \\r\\n as the record terminator (RFC 4180 compliant)"),
            ("Any",     "Accept \\r, \\n, or \\r\\n as record terminators when reading"),
            ("AnyByte", "Use a specific byte value as the record terminator"),
        ]
    );

    // =========================================================================
    // CsvStringRecord — one CSV record as UTF-8 string fields
    // =========================================================================

    impl_csv_survey_spec!(
        type    = CsvStringRecord,
        name    = "csv::StringRecord",
        summary = "A single CSV record represented as a sequence of UTF-8 string fields.",
        fields = [
            ("fields", "The string fields of this record; each element is one CSV column value"),
        ]
    );

    // =========================================================================
    // CsvByteRecord — one CSV record as raw byte fields
    // =========================================================================

    impl_csv_survey_spec!(
        type    = CsvByteRecord,
        name    = "csv::ByteRecord",
        summary = "A single CSV record represented as a sequence of raw byte fields. Use base64 for binary data.",
        fields = [
            ("fields", "The raw byte fields of this record; each element is one CSV column value"),
        ]
    );

    // =========================================================================
    // CsvErrorKind — category of a CSV processing error
    // =========================================================================

    impl ElicitSpec for CsvErrorKind {
        fn type_spec() -> TypeSpec {
            let variants = SpecCategoryBuilder::default()
                .name("variants".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("Io".to_string())
                        .description("An I/O error occurred while reading or writing".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("UnequalLengths".to_string())
                        .description(
                            "A record had an unexpected number of fields; \
                             carries expected_len, pos (optional byte offset), and actual len"
                                .to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("Utf8".to_string())
                        .description(
                            "UTF-8 validation failed on a field; carries the zero-based field index"
                                .to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("Deserialize".to_string())
                        .description(
                            "A Serde deserialization error; carries a human-readable message"
                                .to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("Other".to_string())
                        .description("Any other error kind not covered by the variants above".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            let source = SpecCategoryBuilder::default()
                .name("source".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("crate".to_string())
                        .description("csv v1 — fast, flexible CSV reader and writer".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("pattern".to_string())
                        .description(
                            "Select — choose one variant, then provide its fields".to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("csv::ErrorKind".to_string())
                .summary(
                    "The category of a CSV processing error, mirroring csv::ErrorKind \
                     for MCP-boundary reporting."
                        .to_string(),
                )
                .categories(vec![variants, source])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "csv::ErrorKind",
        <CsvErrorKind as ElicitSpec>::type_spec,
        std::any::TypeId::of::<CsvErrorKind>
    ));

    impl ElicitComplete for CsvErrorKind {}
}
