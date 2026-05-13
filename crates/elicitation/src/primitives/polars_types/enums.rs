//! Polars enum types for join strategies, data types, and pipeline operations.
//!
//! Available with the `polars-types` feature.

use elicitation_derive::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Polars join strategies.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(rename_all = "snake_case")]
pub enum PolarsJoinType {
    /// INNER JOIN — keep only rows with matches in both frames.
    Inner,
    /// LEFT JOIN — keep all left rows, NULLs for unmatched right.
    Left,
    /// RIGHT JOIN — keep all right rows, NULLs for unmatched left.
    Right,
    /// FULL OUTER JOIN — keep all rows from both frames.
    Full,
    /// CROSS JOIN — cartesian product.
    Cross,
    /// SEMI JOIN — keep left rows that have a match in right.
    Semi,
    /// ANTI JOIN — keep left rows that do NOT have a match in right.
    Anti,
}

/// Common polars data types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(rename_all = "snake_case")]
pub enum PolarsDType {
    /// Boolean column.
    Boolean,
    /// 32-bit signed integer.
    Int32,
    /// 64-bit signed integer.
    Int64,
    /// 32-bit float.
    Float32,
    /// 64-bit float.
    Float64,
    /// UTF-8 string.
    Utf8,
    /// Calendar date.
    Date,
    /// Datetime with optional timezone.
    Datetime,
    /// Duration (time delta).
    Duration,
    /// List column.
    List,
    /// Struct column.
    Struct,
}

/// A single step in a polars LazyFrame pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PolarsPipelineOp {
    /// Read a CSV file.
    ReadCsv {
        /// Path to the CSV file.
        path: String,
        /// Whether the CSV has a header row.
        has_header: bool,
    },
    /// Read a Parquet file.
    ReadParquet {
        /// Path to the Parquet file.
        path: String,
    },
    /// Read a JSON/NDJSON file.
    ReadJson {
        /// Path to the JSON file.
        path: String,
    },
    /// Filter rows by a predicate expression.
    Filter {
        /// Rust Expr code (e.g. `col("age").gt(lit(18))`).
        predicate: String,
    },
    /// Select columns by name.
    Select {
        /// Column names to keep.
        columns: Vec<String>,
    },
    /// Add or replace columns with computed expressions.
    WithColumns {
        /// Rust Expr code strings (e.g. `[col("a").alias("b")]`).
        exprs: Vec<String>,
    },
    /// Group rows and aggregate.
    GroupByAgg {
        /// Column names to group by.
        by: Vec<String>,
        /// Rust Expr code strings for aggregations.
        agg: Vec<String>,
    },
    /// Join with another file.
    Join {
        /// Path to the right-hand file (CSV).
        right_path: String,
        /// Left-side join key columns.
        left_on: Vec<String>,
        /// Right-side join key columns.
        right_on: Vec<String>,
        /// Join strategy (inner, left, full, etc.).
        how: String,
    },
    /// Sort rows.
    Sort {
        /// Column names to sort by.
        by: Vec<String>,
        /// Whether each column sorts descending.
        descending: Vec<bool>,
    },
    /// Drop duplicate rows.
    Unique {
        /// Subset of columns to deduplicate on (None = all columns).
        subset: Option<Vec<String>>,
    },
    /// Drop rows with null values.
    DropNulls {
        /// Subset of columns to check (None = all columns).
        subset: Option<Vec<String>>,
    },
    /// Write output to CSV.
    WriteCsv {
        /// Destination path.
        path: String,
    },
    /// Write output to Parquet.
    WriteParquet {
        /// Destination path.
        path: String,
    },
    /// Write output to JSON/NDJSON.
    WriteJson {
        /// Destination path.
        path: String,
    },
}
