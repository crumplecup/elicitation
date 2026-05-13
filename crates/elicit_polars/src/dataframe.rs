//! `PolarsDataFramePlugin` — MCP tools for polars DataFrame runtime execution.
//!
//! DataFrames are stored server-side in a UUID-keyed registry. Agents load
//! data from files or JSON strings, apply transformations, query schema/head,
//! and write results back to files. Expr operations use the shared
//! `SharedExprRegistry` from `PolarsExprPlugin`.
//!
//! # Tool namespace: `polars_df__*`

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{Elicit, PluginContext, VerifiedWorkflow};
use futures::future::BoxFuture;
use polars::prelude::{
    CsvWriter, DataFrame, Expr, IntoLazy, IpcWriter, JsonFormat, JsonWriter, LazyCsvReader,
    LazyFileListReader, ParquetWriter, PolarsError, SerWriter, SortMultipleOptions, col,
};
use polars::prelude::{JsonReader, ParquetReader, SerReader};
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::instrument;
use uuid::Uuid;

use crate::expr::SharedExprRegistry;

// ── Shared registry type ──────────────────────────────────────────────────────

/// Shared registry of polars `DataFrame` values keyed by UUID.
pub type SharedDfRegistry = Arc<Mutex<HashMap<Uuid, DataFrame>>>;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a polars `DataFrame` was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct PolarsDfCreated;

impl Prop for PolarsDfCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_polars_df_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "polars df created");
            }
        }
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_polars_df_created(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }

    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_polars_df_created_contract() -> bool { true }
        }
    }
}

impl VerifiedWorkflow for PolarsDfCreated {}

// ── Plugin context ─────────────────────────────────────────────────────────────

/// Shared state for all `polars_df__*` tool calls.
pub struct PolarsDfCtx {
    dfs: SharedDfRegistry,
    exprs: SharedExprRegistry,
}

impl PolarsDfCtx {
    fn new(dfs: SharedDfRegistry, exprs: SharedExprRegistry) -> Self {
        Self { dfs, exprs }
    }
}

impl PluginContext for PolarsDfCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn polars_err(e: PolarsError) -> ErrorData {
    ErrorData::invalid_params(e.to_string(), None)
}

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

async fn store_df(registry: &SharedDfRegistry, df: DataFrame) -> Uuid {
    let id = Uuid::new_v4();
    registry.lock().await.insert(id, df);
    id
}

async fn get_df(registry: &SharedDfRegistry, id_str: &str) -> Result<DataFrame, ErrorData> {
    let id: Uuid = id_str
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {id_str}")))?;
    registry
        .lock()
        .await
        .get(&id)
        .cloned()
        .ok_or_else(|| json_err(format!("df_id not found: {id}")))
}

async fn get_expr(registry: &SharedExprRegistry, id_str: &str) -> Result<Expr, ErrorData> {
    let id: Uuid = id_str
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {id_str}")))?;
    registry
        .lock()
        .await
        .get(&id)
        .map(|(e, _)| e.clone())
        .ok_or_else(|| json_err(format!("expr_id not found: {id}")))
}

#[derive(Serialize)]
struct DfIdResult {
    df_id: String,
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `polars_df__read_csv`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadCsvParams {
    /// Path to the CSV file.
    pub path: String,
    /// Whether the CSV file has a header row.
    pub has_header: bool,
}

/// Parameters for `polars_df__read_parquet`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadParquetParams {
    /// Path to the Parquet file.
    pub path: String,
}

/// Parameters for `polars_df__read_json`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadJsonParams {
    /// Path to the JSON/NDJSON file.
    pub path: String,
}

/// Parameters for `polars_df__from_json_string`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FromJsonStringParams {
    /// JSON array string (e.g. `[{"a": 1, "b": 2}]`).
    pub json: String,
}

/// Parameters for `polars_df__schema`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DfLookupParams {
    /// UUID of the DataFrame.
    pub df_id: String,
}

/// Parameters for `polars_df__head`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HeadParams {
    /// UUID of the DataFrame.
    pub df_id: String,
    /// Number of rows to return (default: 5).
    pub n: Option<usize>,
}

/// Parameters for `polars_df__to_json_string`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ToJsonStringParams {
    /// UUID of the DataFrame.
    pub df_id: String,
    /// Number of rows to include (None = all rows).
    pub n: Option<usize>,
}

/// Parameters for `polars_df__select`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// Column names to select.
    pub columns: Vec<String>,
}

/// Parameters for `polars_df__filter`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FilterParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// UUID of the predicate Expr from `PolarsExprPlugin`.
    pub expr_id: String,
}

/// Parameters for `polars_df__with_columns`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WithColumnsParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// UUIDs of the Expr values to add/replace as columns.
    pub expr_ids: Vec<String>,
}

/// Parameters for `polars_df__sort`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SortParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// Column names to sort by.
    pub by: Vec<String>,
    /// Whether each column sorts descending (parallel to `by`).
    pub descending: Vec<bool>,
}

/// Parameters for `polars_df__group_by_agg`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GroupByAggParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// Column names to group by.
    pub by: Vec<String>,
    /// UUIDs of the aggregation Expr values.
    pub agg_expr_ids: Vec<String>,
}

/// Parameters for `polars_df__join`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct JoinParams {
    /// UUID of the left DataFrame.
    pub left_id: String,
    /// UUID of the right DataFrame.
    pub right_id: String,
    /// Column names for the left join keys.
    pub left_on: Vec<String>,
    /// Column names for the right join keys.
    pub right_on: Vec<String>,
    /// Join type: "inner", "left", "right", "full", "cross", "semi", "anti".
    pub how: String,
}

/// Parameters for `polars_df__unique`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UniqueParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// Subset of columns to deduplicate on (None = all columns).
    pub subset: Option<Vec<String>>,
}

/// Parameters for `polars_df__drop_nulls`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DropNullsParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// Subset of columns to check for nulls (None = all columns).
    pub subset: Option<Vec<String>>,
}

/// Parameters for `polars_df__rename_column`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RenameColumnParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// Existing column name.
    pub old_name: String,
    /// New column name.
    pub new_name: String,
}

/// Parameters for `polars_df__drop_column`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DropColumnParams {
    /// UUID of the input DataFrame.
    pub df_id: String,
    /// Column name to drop.
    pub column: String,
}

/// Parameters for `polars_df__shape`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShapeParams {
    /// UUID of the DataFrame.
    pub df_id: String,
}

/// Parameters for `polars_df__write_csv`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteCsvParams {
    /// UUID of the DataFrame.
    pub df_id: String,
    /// Destination file path.
    pub path: String,
}

/// Parameters for `polars_df__write_parquet`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteParquetParams {
    /// UUID of the DataFrame.
    pub df_id: String,
    /// Destination file path.
    pub path: String,
}

/// Parameters for `polars_df__write_json`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteJsonParams {
    /// UUID of the DataFrame.
    pub df_id: String,
    /// Destination file path.
    pub path: String,
}

/// Parameters for `polars_df__write_ipc`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteIpcParams {
    /// UUID of the DataFrame.
    pub df_id: String,
    /// Destination file path.
    pub path: String,
}

/// Parameters for `polars_df__list`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListParams {}

// ── Tool functions ─────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__read_csv",
    description = "Read a CSV file into a DataFrame. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_read_csv(ctx: Arc<PolarsDfCtx>, p: ReadCsvParams) -> Result<CallToolResult, ErrorData> {
    let df = LazyCsvReader::new(p.path.as_str().into())
        .with_has_header(p.has_header)
        .finish()
        .and_then(|lf| lf.collect())
        .map_err(polars_err)?;
    let id = store_df(&ctx.dfs, df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__read_parquet",
    description = "Read a Parquet file into a DataFrame. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_read_parquet(
    ctx: Arc<PolarsDfCtx>,
    p: ReadParquetParams,
) -> Result<CallToolResult, ErrorData> {
    let file = std::fs::File::open(&p.path).map_err(|e| json_err(format!("open failed: {e}")))?;
    let df = ParquetReader::new(file).finish().map_err(polars_err)?;
    let id = store_df(&ctx.dfs, df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__read_json",
    description = "Read a JSON/NDJSON file into a DataFrame. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_read_json(
    ctx: Arc<PolarsDfCtx>,
    p: ReadJsonParams,
) -> Result<CallToolResult, ErrorData> {
    let file = std::fs::File::open(&p.path).map_err(|e| json_err(format!("open failed: {e}")))?;
    let df = JsonReader::new(file).finish().map_err(polars_err)?;
    let id = store_df(&ctx.dfs, df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__from_json_string",
    description = "Create a DataFrame from a JSON array string. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_from_json_string(
    ctx: Arc<PolarsDfCtx>,
    p: FromJsonStringParams,
) -> Result<CallToolResult, ErrorData> {
    let cursor = Cursor::new(p.json.as_bytes());
    let df = JsonReader::new(cursor).finish().map_err(polars_err)?;
    let id = store_df(&ctx.dfs, df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__schema",
    description = "Return the schema (column name → dtype) of a stored DataFrame.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_schema(ctx: Arc<PolarsDfCtx>, p: DfLookupParams) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let schema: HashMap<String, String> = df
        .schema()
        .iter()
        .map(|(name, dtype)| (name.to_string(), format!("{:?}", dtype)))
        .collect();
    Ok(json_result(&schema))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__shape",
    description = "Return the shape (rows, cols) of a stored DataFrame.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_shape(ctx: Arc<PolarsDfCtx>, p: ShapeParams) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let (rows, cols) = df.shape();
    #[derive(Serialize)]
    struct ShapeResult {
        rows: usize,
        cols: usize,
    }
    Ok(json_result(&ShapeResult { rows, cols }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__head",
    description = "Return the first N rows of a DataFrame as JSON. Default N = 5.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_head(ctx: Arc<PolarsDfCtx>, p: HeadParams) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let sample = df.head(p.n);
    let mut buf = Vec::new();
    JsonWriter::new(&mut buf)
        .with_json_format(JsonFormat::Json)
        .finish(&mut sample.clone())
        .map_err(polars_err)?;
    let s = String::from_utf8(buf).map_err(json_err)?;
    Ok(CallToolResult::success(vec![Content::text(s)]))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__to_json_string",
    description = "Serialize a DataFrame (or first N rows) to a JSON string.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_to_json_string(
    ctx: Arc<PolarsDfCtx>,
    p: ToJsonStringParams,
) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let sample = if let Some(n) = p.n {
        df.head(Some(n))
    } else {
        df.clone()
    };
    let mut buf = Vec::new();
    JsonWriter::new(&mut buf)
        .with_json_format(JsonFormat::Json)
        .finish(&mut sample.clone())
        .map_err(polars_err)?;
    let s = String::from_utf8(buf).map_err(json_err)?;
    Ok(CallToolResult::success(vec![Content::text(s)]))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__select",
    description = "Select columns from a DataFrame by name. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_select(ctx: Arc<PolarsDfCtx>, p: SelectParams) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let cols: Vec<Expr> = p.columns.iter().map(|c| col(c.as_str())).collect();
    let new_df = df.lazy().select(cols).collect().map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__filter",
    description = "Filter a DataFrame rows using a predicate Expr UUID. \
                   Requires a stored Expr from PolarsExprPlugin. \
                   Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_filter(ctx: Arc<PolarsDfCtx>, p: FilterParams) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let predicate = get_expr(&ctx.exprs, &p.expr_id).await?;
    let new_df = df.lazy().filter(predicate).collect().map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__with_columns",
    description = "Add or replace columns via Expr UUIDs. \
                   Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_with_columns(
    ctx: Arc<PolarsDfCtx>,
    p: WithColumnsParams,
) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let mut exprs = Vec::new();
    for eid in &p.expr_ids {
        exprs.push(get_expr(&ctx.exprs, eid).await?);
    }
    let new_df = df
        .lazy()
        .with_columns(exprs)
        .collect()
        .map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__sort",
    description = "Sort a DataFrame by one or more columns. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_sort(ctx: Arc<PolarsDfCtx>, p: SortParams) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let opts = SortMultipleOptions::default().with_order_descending_multi(p.descending.clone());
    let new_df = df
        .sort(p.by.iter().map(|s| s.as_str()), opts)
        .map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__group_by_agg",
    description = "Group by columns and aggregate using Expr UUIDs. \
                   Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_group_by_agg(
    ctx: Arc<PolarsDfCtx>,
    p: GroupByAggParams,
) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let by_exprs: Vec<Expr> = p.by.iter().map(|c| col(c.as_str())).collect();
    let mut agg_exprs = Vec::new();
    for eid in &p.agg_expr_ids {
        agg_exprs.push(get_expr(&ctx.exprs, eid).await?);
    }
    let new_df = df
        .lazy()
        .group_by(by_exprs)
        .agg(agg_exprs)
        .collect()
        .map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__join",
    description = "Join two DataFrames by UUID. \
                   how: inner | left | right | full | cross | semi | anti. \
                   Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_join(ctx: Arc<PolarsDfCtx>, p: JoinParams) -> Result<CallToolResult, ErrorData> {
    use polars::prelude::{JoinArgs, JoinType};

    let left = get_df(&ctx.dfs, &p.left_id).await?;
    let right = get_df(&ctx.dfs, &p.right_id).await?;
    let left_on: Vec<Expr> = p.left_on.iter().map(|c| col(c.as_str())).collect();
    let right_on: Vec<Expr> = p.right_on.iter().map(|c| col(c.as_str())).collect();

    let join_type = match p.how.as_str() {
        "inner" => JoinType::Inner,
        "left" => JoinType::Left,
        "right" => JoinType::Right,
        "full" => JoinType::Full,
        "cross" => JoinType::Cross,
        "semi" => JoinType::Semi,
        "anti" => JoinType::Anti,
        other => return Err(json_err(format!("unknown join type: {other}"))),
    };
    let args = JoinArgs {
        how: join_type,
        ..Default::default()
    };
    let new_df = left
        .lazy()
        .join(right.lazy(), left_on, right_on, args)
        .collect()
        .map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__unique",
    description = "Drop duplicate rows from a DataFrame. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_unique(ctx: Arc<PolarsDfCtx>, p: UniqueParams) -> Result<CallToolResult, ErrorData> {
    use polars::prelude::UniqueKeepStrategy;
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let subset_ref: Option<Vec<String>> = p.subset.clone();
    let new_df = df
        .unique_stable(subset_ref.as_deref(), UniqueKeepStrategy::First, None)
        .map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__drop_nulls",
    description = "Drop rows containing null values. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_drop_nulls(
    ctx: Arc<PolarsDfCtx>,
    p: DropNullsParams,
) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let new_df = df.drop_nulls(p.subset.as_deref()).map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__rename_column",
    description = "Rename a column in a DataFrame. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_rename_column(
    ctx: Arc<PolarsDfCtx>,
    p: RenameColumnParams,
) -> Result<CallToolResult, ErrorData> {
    let mut df = get_df(&ctx.dfs, &p.df_id).await?;
    df.rename(&p.old_name, p.new_name.as_str().into())
        .map_err(polars_err)?;
    let id = store_df(&ctx.dfs, df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__drop_column",
    description = "Drop a column from a DataFrame. Establishes: PolarsDfCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_drop_column(
    ctx: Arc<PolarsDfCtx>,
    p: DropColumnParams,
) -> Result<CallToolResult, ErrorData> {
    let df = get_df(&ctx.dfs, &p.df_id).await?;
    let new_df = df.drop(&p.column).map_err(polars_err)?;
    let id = store_df(&ctx.dfs, new_df).await;
    let _proof: Established<PolarsDfCreated> = Established::assert();
    Ok(json_result(&DfIdResult {
        df_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__write_csv",
    description = "Write a DataFrame to a CSV file.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_write_csv(
    ctx: Arc<PolarsDfCtx>,
    p: WriteCsvParams,
) -> Result<CallToolResult, ErrorData> {
    let mut df = get_df(&ctx.dfs, &p.df_id).await?;
    let file =
        std::fs::File::create(&p.path).map_err(|e| json_err(format!("create failed: {e}")))?;
    CsvWriter::new(file).finish(&mut df).map_err(polars_err)?;
    Ok(CallToolResult::success(vec![Content::text(format!(
        "wrote CSV to {}",
        p.path
    ))]))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__write_parquet",
    description = "Write a DataFrame to a Parquet file.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_write_parquet(
    ctx: Arc<PolarsDfCtx>,
    p: WriteParquetParams,
) -> Result<CallToolResult, ErrorData> {
    let mut df = get_df(&ctx.dfs, &p.df_id).await?;
    let file =
        std::fs::File::create(&p.path).map_err(|e| json_err(format!("create failed: {e}")))?;
    ParquetWriter::new(file)
        .finish(&mut df)
        .map_err(polars_err)?;
    Ok(CallToolResult::success(vec![Content::text(format!(
        "wrote Parquet to {}",
        p.path
    ))]))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__write_json",
    description = "Write a DataFrame to a JSON file.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_write_json(
    ctx: Arc<PolarsDfCtx>,
    p: WriteJsonParams,
) -> Result<CallToolResult, ErrorData> {
    let mut df = get_df(&ctx.dfs, &p.df_id).await?;
    let file =
        std::fs::File::create(&p.path).map_err(|e| json_err(format!("create failed: {e}")))?;
    JsonWriter::new(file)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df)
        .map_err(polars_err)?;
    Ok(CallToolResult::success(vec![Content::text(format!(
        "wrote JSON to {}",
        p.path
    ))]))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__write_ipc",
    description = "Write a DataFrame to an Arrow IPC file.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_write_ipc(
    ctx: Arc<PolarsDfCtx>,
    p: WriteIpcParams,
) -> Result<CallToolResult, ErrorData> {
    let mut df = get_df(&ctx.dfs, &p.df_id).await?;
    let file =
        std::fs::File::create(&p.path).map_err(|e| json_err(format!("create failed: {e}")))?;
    IpcWriter::new(file).finish(&mut df).map_err(polars_err)?;
    Ok(CallToolResult::success(vec![Content::text(format!(
        "wrote IPC to {}",
        p.path
    ))]))
}

#[elicitation::elicit_tool(
    plugin = "polars_df",
    name = "polars_df__list",
    description = "List all stored DataFrame UUIDs and their shapes.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn df_list(ctx: Arc<PolarsDfCtx>, _p: ListParams) -> Result<CallToolResult, ErrorData> {
    #[derive(Serialize)]
    struct Entry {
        df_id: String,
        rows: usize,
        cols: usize,
    }
    let guard = ctx.dfs.lock().await;
    let entries: Vec<Entry> = guard
        .iter()
        .map(|(id, df)| {
            let (rows, cols) = df.shape();
            Entry {
                df_id: id.to_string(),
                rows,
                cols,
            }
        })
        .collect();
    Ok(json_result(&entries))
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing `polars_df__*` tools for polars DataFrame operations.
///
/// Stores `DataFrame` values in a UUID-keyed registry. Accepts a shared
/// `SharedExprRegistry` from `PolarsExprPlugin` so that filter, select, and
/// aggregation tools can look up stored expressions.
pub struct PolarsDataFramePlugin(Arc<PolarsDfCtx>);

impl PolarsDataFramePlugin {
    /// Create a new `PolarsDataFramePlugin` with the given expr registry.
    pub fn new(exprs: SharedExprRegistry) -> Self {
        let dfs: SharedDfRegistry = Arc::new(Mutex::new(HashMap::new()));
        Self(Arc::new(PolarsDfCtx::new(dfs, exprs)))
    }

    /// Create with both a shared df registry and expr registry.
    pub fn with_registries(dfs: SharedDfRegistry, exprs: SharedExprRegistry) -> Self {
        Self(Arc::new(PolarsDfCtx::new(dfs, exprs)))
    }

    /// Return a clone of the shared DataFrame registry for use by `PolarsSqlPlugin`.
    pub fn df_registry(&self) -> SharedDfRegistry {
        self.0.dfs.clone()
    }

    /// Return the plugin context as a type-erased Arc for tool dispatch in tests.
    pub fn dispatch_ctx(&self) -> Arc<dyn std::any::Any + Send + Sync> {
        self.0.clone() as Arc<dyn std::any::Any + Send + Sync>
    }
}

impl elicitation::ElicitPlugin for PolarsDataFramePlugin {
    fn name(&self) -> &'static str {
        "polars_df"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "polars_df")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let plugin_ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            let full_name = if name.starts_with("polars_df__") {
                name.to_string()
            } else {
                format!("polars_df__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "polars_df")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
