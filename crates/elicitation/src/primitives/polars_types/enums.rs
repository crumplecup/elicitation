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

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};

// --- PolarsJoinType ----------------------------------------------------------

impl Prompt for PolarsJoinType {
    fn prompt() -> Option<&'static str> {
        Some("Choose the join strategy:")
    }
}

impl Select for PolarsJoinType {
    fn options() -> Vec<Self> {
        vec![
            PolarsJoinType::Inner, PolarsJoinType::Left, PolarsJoinType::Right,
            PolarsJoinType::Full, PolarsJoinType::Cross, PolarsJoinType::Semi,
            PolarsJoinType::Anti,
        ]
    }
    fn labels() -> Vec<String> {
        Self::options().iter()
            .map(|v| serde_json::to_string(v).unwrap().trim_matches('"').to_string())
            .collect()
    }
    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(PolarsJoinType => PolarsJoinTypeStyle);

impl Elicitation for PolarsJoinType {
    type Style = PolarsJoinTypeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PolarsJoinType");
        let params = mcp::select_params(Self::prompt().unwrap_or("Choose join type:"), &Self::labels());
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select()).with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid PolarsJoinType: {label}"))))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum("elicitation::PolarsJoinType", "inner")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum("elicitation::PolarsJoinType")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum("elicitation::PolarsJoinType")
    }
}

impl ElicitIntrospect for PolarsJoinType {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::PolarsJoinType",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] }).collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for PolarsJoinType {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: "PolarsJoinType".to_string(),
            type_name: "PolarsJoinType".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

// --- PolarsDType -------------------------------------------------------------

impl Prompt for PolarsDType {
    fn prompt() -> Option<&'static str> {
        Some("Choose the Polars column data type:")
    }
}

impl Select for PolarsDType {
    fn options() -> Vec<Self> {
        vec![
            PolarsDType::Boolean, PolarsDType::Int32, PolarsDType::Int64,
            PolarsDType::Float32, PolarsDType::Float64, PolarsDType::Utf8,
            PolarsDType::Date, PolarsDType::Datetime, PolarsDType::Duration,
            PolarsDType::List, PolarsDType::Struct,
        ]
    }
    fn labels() -> Vec<String> {
        Self::options().iter()
            .map(|v| serde_json::to_string(v).unwrap().trim_matches('"').to_string())
            .collect()
    }
    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(PolarsDType => PolarsDTypeStyle);

impl Elicitation for PolarsDType {
    type Style = PolarsDTypeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PolarsDType");
        let params = mcp::select_params(Self::prompt().unwrap_or("Choose dtype:"), &Self::labels());
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select()).with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid PolarsDType: {label}"))))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum("elicitation::PolarsDType", "boolean")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum("elicitation::PolarsDType")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum("elicitation::PolarsDType")
    }
}

impl ElicitIntrospect for PolarsDType {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::PolarsDType",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] }).collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for PolarsDType {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: "PolarsDType".to_string(),
            type_name: "PolarsDType".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

// --- PolarsPipelineOp --------------------------------------------------------

impl Prompt for PolarsPipelineOp {
    fn prompt() -> Option<&'static str> {
        Some("Choose a LazyFrame pipeline operation:")
    }
}

impl Select for PolarsPipelineOp {
    fn options() -> Vec<Self> {
        vec![
            PolarsPipelineOp::ReadCsv { path: String::new(), has_header: true },
            PolarsPipelineOp::ReadParquet { path: String::new() },
            PolarsPipelineOp::ReadJson { path: String::new() },
            PolarsPipelineOp::Filter { predicate: String::new() },
            PolarsPipelineOp::Select { columns: vec![] },
            PolarsPipelineOp::WithColumns { exprs: vec![] },
            PolarsPipelineOp::GroupByAgg { by: vec![], agg: vec![] },
            PolarsPipelineOp::Join { right_path: String::new(), left_on: vec![], right_on: vec![], how: String::new() },
            PolarsPipelineOp::Sort { by: vec![], descending: vec![] },
            PolarsPipelineOp::Unique { subset: None },
            PolarsPipelineOp::DropNulls { subset: None },
            PolarsPipelineOp::WriteCsv { path: String::new() },
            PolarsPipelineOp::WriteParquet { path: String::new() },
            PolarsPipelineOp::WriteJson { path: String::new() },
        ]
    }
    fn labels() -> Vec<String> {
        vec![
            "read_csv".to_string(), "read_parquet".to_string(), "read_json".to_string(),
            "filter".to_string(), "select".to_string(), "with_columns".to_string(),
            "group_by_agg".to_string(), "join".to_string(), "sort".to_string(),
            "unique".to_string(), "drop_nulls".to_string(),
            "write_csv".to_string(), "write_parquet".to_string(), "write_json".to_string(),
        ]
    }
    fn from_label(label: &str) -> Option<Self> {
        Self::options().into_iter().zip(Self::labels())
            .find(|(_, l)| l == label)
            .map(|(v, _)| v)
    }
}

crate::default_style!(PolarsPipelineOp => PolarsPipelineOpStyle);

impl Elicitation for PolarsPipelineOp {
    type Style = PolarsPipelineOpStyle;

    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            tracing::debug!("Eliciting PolarsPipelineOp variant");
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Choose operation:"),
                &Self::labels(),
            );
            let result = communicator.call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select()).with_arguments(params),
            ).await?;
            let label = mcp::parse_string(mcp::extract_value(result)?)?;
            tracing::debug!(variant = %label, "PolarsPipelineOp variant selected");
            match label.as_str() {
                "read_csv" => {
                    let path = String::elicit(communicator).await?;
                    let has_header = bool::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::ReadCsv { path, has_header })
                }
                "read_parquet" => {
                    let path = String::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::ReadParquet { path })
                }
                "read_json" => {
                    let path = String::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::ReadJson { path })
                }
                "filter" => {
                    let predicate = String::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::Filter { predicate })
                }
                "select" => {
                    let columns = Vec::<String>::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::Select { columns })
                }
                "with_columns" => {
                    let exprs = Vec::<String>::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::WithColumns { exprs })
                }
                "group_by_agg" => {
                    let by = Vec::<String>::elicit(communicator).await?;
                    let agg = Vec::<String>::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::GroupByAgg { by, agg })
                }
                "join" => {
                    let right_path = String::elicit(communicator).await?;
                    let left_on = Vec::<String>::elicit(communicator).await?;
                    let right_on = Vec::<String>::elicit(communicator).await?;
                    let how = String::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::Join { right_path, left_on, right_on, how })
                }
                "sort" => {
                    let by = Vec::<String>::elicit(communicator).await?;
                    let descending = Vec::<bool>::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::Sort { by, descending })
                }
                "unique" => {
                    let subset = Option::<Vec<String>>::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::Unique { subset })
                }
                "drop_nulls" => {
                    let subset = Option::<Vec<String>>::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::DropNulls { subset })
                }
                "write_csv" => {
                    let path = String::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::WriteCsv { path })
                }
                "write_parquet" => {
                    let path = String::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::WriteParquet { path })
                }
                "write_json" => {
                    let path = String::elicit(communicator).await?;
                    Ok(PolarsPipelineOp::WriteJson { path })
                }
                _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Invalid PolarsPipelineOp variant: {label}"
                )))),
            }
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for PolarsPipelineOp {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::PolarsPipelineOp",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] }).collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for PolarsPipelineOp {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().map(|s| s.to_string()).unwrap_or_default(),
            type_name: "PolarsPipelineOp".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}
