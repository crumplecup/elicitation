//! `PolarsExprPlugin` — MCP tools for polars Expr composition.
//!
//! Expressions are stored server-side in a UUID-keyed registry alongside their
//! Rust source code. Agents compose expressions by chaining tool calls; each
//! produces a new UUID handle. The `emit` tool returns the Rust source string
//! for any stored expression.
//!
//! # Tool namespace: `polars_expr__*`

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{Elicit, PluginContext, PolarsDType, VerifiedWorkflow};
use futures::future::BoxFuture;
use polars::prelude::{DataType, Expr, SortOptions, TimeUnit, col, lit};
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

// ── Shared registry type ──────────────────────────────────────────────────────

/// Shared registry of polars `Expr` values and their Rust source strings.
///
/// Both `PolarsExprPlugin` and `PolarsDataFramePlugin` hold a clone of this
/// `Arc` so that DataFrame operations can look up stored expressions.
pub type SharedExprRegistry = Arc<Mutex<HashMap<Uuid, (Expr, String)>>>;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a polars `Expr` was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct PolarsExprCreated;

impl Prop for PolarsExprCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_polars_expr_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "polars expr created");
            }
        }
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_polars_expr_created(ok: bool) -> (result: bool)
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
            pub fn verify_polars_expr_created_contract() -> bool { true }
        }
    }
}

impl VerifiedWorkflow for PolarsExprCreated {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `polars_expr__*` tool calls.
pub struct PolarsExprCtx {
    registry: SharedExprRegistry,
}

impl PolarsExprCtx {
    fn new(registry: SharedExprRegistry) -> Self {
        Self { registry }
    }
}

impl PluginContext for PolarsExprCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Map a `PolarsDType` to a `polars::prelude::DataType`.
pub fn polars_dtype(d: &PolarsDType) -> DataType {
    match d {
        PolarsDType::Boolean => DataType::Boolean,
        PolarsDType::Int32 => DataType::Int32,
        PolarsDType::Int64 => DataType::Int64,
        PolarsDType::Float32 => DataType::Float32,
        PolarsDType::Float64 => DataType::Float64,
        PolarsDType::Utf8 => DataType::String,
        PolarsDType::Date => DataType::Date,
        PolarsDType::Datetime => DataType::Datetime(TimeUnit::Microseconds, None),
        PolarsDType::Duration => DataType::Duration(TimeUnit::Microseconds),
        PolarsDType::List => DataType::List(Box::new(DataType::Unknown(Default::default()))),
        PolarsDType::Struct => DataType::Struct(vec![]),
    }
}

fn dtype_to_code(d: &PolarsDType) -> &'static str {
    match d {
        PolarsDType::Boolean => "DataType::Boolean",
        PolarsDType::Int32 => "DataType::Int32",
        PolarsDType::Int64 => "DataType::Int64",
        PolarsDType::Float32 => "DataType::Float32",
        PolarsDType::Float64 => "DataType::Float64",
        PolarsDType::Utf8 => "DataType::String",
        PolarsDType::Date => "DataType::Date",
        PolarsDType::Datetime => "DataType::Datetime(TimeUnit::Microseconds, None)",
        PolarsDType::Duration => "DataType::Duration(TimeUnit::Microseconds)",
        PolarsDType::List => "DataType::List(Box::new(DataType::Unknown(Default::default())))",
        PolarsDType::Struct => "DataType::Struct(vec![])",
    }
}

async fn store_expr(registry: &SharedExprRegistry, expr: Expr, code: String) -> Uuid {
    let id = Uuid::new_v4();
    registry.lock().await.insert(id, (expr, code));
    id
}

async fn get_pair(
    registry: &SharedExprRegistry,
    id_str: &str,
) -> Result<(Expr, String), ErrorData> {
    let id: Uuid = id_str
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {id_str}")))?;
    registry
        .lock()
        .await
        .get(&id)
        .cloned()
        .ok_or_else(|| json_err(format!("expr_id not found: {id}")))
}

#[derive(Serialize)]
struct ExprIdResult {
    expr_id: String,
}

// ── Param structs (unique per tool) ──────────────────────────────────────────

/// Parameters for `polars_expr__col`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ColParams {
    /// Column name.
    pub name: String,
}

/// Parameters for `polars_expr__lit_int`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LitIntParams {
    /// Integer literal value.
    pub value: i64,
}

/// Parameters for `polars_expr__lit_float`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LitFloatParams {
    /// Float literal value.
    pub value: f64,
}

/// Parameters for `polars_expr__lit_str`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LitStrParams {
    /// String literal value.
    pub value: String,
}

/// Parameters for `polars_expr__lit_bool`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LitBoolParams {
    /// Boolean literal value.
    pub value: bool,
}

/// Parameters for `polars_expr__all_columns`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AllColumnsParams {}

/// Parameters for `polars_expr__first_col`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FirstColParams {}

/// Parameters for `polars_expr__last_col`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LastColParams {}

/// Parameters for `polars_expr__count_all`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CountAllParams {}

/// Parameters for `polars_expr__eq`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EqParams {
    /// UUID of the left operand.
    pub left_id: String,
    /// UUID of the right operand.
    pub right_id: String,
}

/// Parameters for `polars_expr__neq`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct NeqParams {
    /// UUID of the left operand.
    pub left_id: String,
    /// UUID of the right operand.
    pub right_id: String,
}

/// Parameters for `polars_expr__gt`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GtParams {
    /// UUID of the left operand.
    pub left_id: String,
    /// UUID of the right operand.
    pub right_id: String,
}

/// Parameters for `polars_expr__lt`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LtParams {
    /// UUID of the left operand.
    pub left_id: String,
    /// UUID of the right operand.
    pub right_id: String,
}

/// Parameters for `polars_expr__gte`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GteParams {
    /// UUID of the left operand.
    pub left_id: String,
    /// UUID of the right operand.
    pub right_id: String,
}

/// Parameters for `polars_expr__lte`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LteParams {
    /// UUID of the left operand.
    pub left_id: String,
    /// UUID of the right operand.
    pub right_id: String,
}

/// Parameters for `polars_expr__and`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AndParams {
    /// UUID of the left operand.
    pub left_id: String,
    /// UUID of the right operand.
    pub right_id: String,
}

/// Parameters for `polars_expr__or`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct OrParams {
    /// UUID of the left operand.
    pub left_id: String,
    /// UUID of the right operand.
    pub right_id: String,
}

/// Parameters for `polars_expr__not`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct NotParams {
    /// UUID of the expression to negate.
    pub expr_id: String,
}

/// Parameters for `polars_expr__sum`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SumParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__mean`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MeanParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__min`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MinParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__max`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MaxParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__count`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CountParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__n_unique`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct NUniqueParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__median`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MedianParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__std`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StdParams {
    /// UUID of the expression.
    pub expr_id: String,
    /// Delta degrees of freedom (0 = population, 1 = sample).
    pub ddof: u8,
}

/// Parameters for `polars_expr__var`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct VarParams {
    /// UUID of the expression.
    pub expr_id: String,
    /// Delta degrees of freedom (0 = population, 1 = sample).
    pub ddof: u8,
}

/// Parameters for `polars_expr__first_val`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FirstValParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__last_val`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LastValParams {
    /// UUID of the expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__str_contains`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrContainsParams {
    /// UUID of the input expression.
    pub expr_id: String,
    /// Pattern to search for.
    pub pattern: String,
}

/// Parameters for `polars_expr__str_starts_with`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrStartsWithParams {
    /// UUID of the input expression.
    pub expr_id: String,
    /// Prefix to match.
    pub pattern: String,
}

/// Parameters for `polars_expr__str_ends_with`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrEndsWithParams {
    /// UUID of the input expression.
    pub expr_id: String,
    /// Suffix to match.
    pub pattern: String,
}

/// Parameters for `polars_expr__str_to_lowercase`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrToLowercaseParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__str_to_uppercase`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrToUppercaseParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__str_replace`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StrReplaceParams {
    /// UUID of the input expression.
    pub expr_id: String,
    /// Pattern to replace.
    pub pattern: String,
    /// Replacement string.
    pub replacement: String,
}

/// Parameters for `polars_expr__dt_year`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DtYearParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__dt_month`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DtMonthParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__dt_day`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DtDayParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__dt_hour`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DtHourParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__alias`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AliasParams {
    /// UUID of the input expression.
    pub expr_id: String,
    /// New column name alias.
    pub name: String,
}

/// Parameters for `polars_expr__cast`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CastParams {
    /// UUID of the input expression.
    pub expr_id: String,
    /// Target data type.
    pub dtype: PolarsDType,
}

/// Parameters for `polars_expr__fill_null_with_zero`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FillNullWithZeroParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__sort_expr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SortExprParams {
    /// UUID of the input expression.
    pub expr_id: String,
    /// Whether to sort descending.
    pub descending: bool,
}

/// Parameters for `polars_expr__is_null`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IsNullParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__is_not_null`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IsNotNullParams {
    /// UUID of the input expression.
    pub expr_id: String,
}

/// Parameters for `polars_expr__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExprDescribeParams {
    /// UUID of the expression to describe.
    pub expr_id: String,
}

/// Parameters for `polars_expr__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExprEmitParams {
    /// UUID of the expression to emit.
    pub expr_id: String,
}

/// Parameters for `polars_expr__list`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExprListParams {}

// ── Tool functions ─────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__col",
    description = "Create a column reference expression: col(\"name\"). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_col(ctx: Arc<PolarsExprCtx>, p: ColParams) -> Result<CallToolResult, ErrorData> {
    let code = format!("col(\"{}\")", p.name);
    let expr = col(p.name.as_str());
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__lit_int",
    description = "Create an integer literal expression: lit(value). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_lit_int(
    ctx: Arc<PolarsExprCtx>,
    p: LitIntParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!("lit({}i64)", p.value);
    let expr = lit(p.value);
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__lit_float",
    description = "Create a float literal expression: lit(value). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_lit_float(
    ctx: Arc<PolarsExprCtx>,
    p: LitFloatParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!("lit({}f64)", p.value);
    let expr = lit(p.value);
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__lit_str",
    description = "Create a string literal expression: lit(\"value\"). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_lit_str(
    ctx: Arc<PolarsExprCtx>,
    p: LitStrParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!("lit(\"{}\")", p.value);
    let expr = lit(p.value.as_str());
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__lit_bool",
    description = "Create a boolean literal expression: lit(value). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_lit_bool(
    ctx: Arc<PolarsExprCtx>,
    p: LitBoolParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!("lit({})", p.value);
    let expr = lit(p.value);
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__all_columns",
    description = "Create an all-columns wildcard expression: polars::prelude::all(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_all_columns(
    ctx: Arc<PolarsExprCtx>,
    _p: AllColumnsParams,
) -> Result<CallToolResult, ErrorData> {
    let code = "polars::prelude::all()".to_string();
    let expr = Expr::from(polars::prelude::all());
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__first_col",
    description = "Create a first-column selector expression: col(\"*\").first(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_first_col(
    ctx: Arc<PolarsExprCtx>,
    _p: FirstColParams,
) -> Result<CallToolResult, ErrorData> {
    let code = "col(\"*\").first()".to_string();
    let expr = col("*").first();
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__last_col",
    description = "Create a last-column selector expression: col(\"*\").last(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_last_col(
    ctx: Arc<PolarsExprCtx>,
    _p: LastColParams,
) -> Result<CallToolResult, ErrorData> {
    let code = "col(\"*\").last()".to_string();
    let expr = col("*").last();
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__count_all",
    description = "Create a count-all expression: polars::prelude::len(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_count_all(
    ctx: Arc<PolarsExprCtx>,
    _p: CountAllParams,
) -> Result<CallToolResult, ErrorData> {
    let code = "polars::prelude::len()".to_string();
    let expr = polars::prelude::len();
    let id = store_expr(&ctx.registry, expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__eq",
    description = "Create an equality comparison: left_expr.eq(right_expr). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_eq(ctx: Arc<PolarsExprCtx>, p: EqParams) -> Result<CallToolResult, ErrorData> {
    let (le, lc) = get_pair(&ctx.registry, &p.left_id).await?;
    let (re, rc) = get_pair(&ctx.registry, &p.right_id).await?;
    let id = store_expr(&ctx.registry, le.eq(re), format!("({lc}).eq({rc})")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__neq",
    description = "Create a not-equal comparison: left_expr.neq(right_expr). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_neq(ctx: Arc<PolarsExprCtx>, p: NeqParams) -> Result<CallToolResult, ErrorData> {
    let (le, lc) = get_pair(&ctx.registry, &p.left_id).await?;
    let (re, rc) = get_pair(&ctx.registry, &p.right_id).await?;
    let id = store_expr(&ctx.registry, le.neq(re), format!("({lc}).neq({rc})")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__gt",
    description = "Create a greater-than comparison: left_expr.gt(right_expr). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_gt(ctx: Arc<PolarsExprCtx>, p: GtParams) -> Result<CallToolResult, ErrorData> {
    let (le, lc) = get_pair(&ctx.registry, &p.left_id).await?;
    let (re, rc) = get_pair(&ctx.registry, &p.right_id).await?;
    let id = store_expr(&ctx.registry, le.gt(re), format!("({lc}).gt({rc})")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__lt",
    description = "Create a less-than comparison: left_expr.lt(right_expr). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_lt(ctx: Arc<PolarsExprCtx>, p: LtParams) -> Result<CallToolResult, ErrorData> {
    let (le, lc) = get_pair(&ctx.registry, &p.left_id).await?;
    let (re, rc) = get_pair(&ctx.registry, &p.right_id).await?;
    let id = store_expr(&ctx.registry, le.lt(re), format!("({lc}).lt({rc})")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__gte",
    description = "Create a greater-or-equal comparison: left_expr.gt_eq(right_expr). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_gte(ctx: Arc<PolarsExprCtx>, p: GteParams) -> Result<CallToolResult, ErrorData> {
    let (le, lc) = get_pair(&ctx.registry, &p.left_id).await?;
    let (re, rc) = get_pair(&ctx.registry, &p.right_id).await?;
    let id = store_expr(&ctx.registry, le.gt_eq(re), format!("({lc}).gt_eq({rc})")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__lte",
    description = "Create a less-or-equal comparison: left_expr.lt_eq(right_expr). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_lte(ctx: Arc<PolarsExprCtx>, p: LteParams) -> Result<CallToolResult, ErrorData> {
    let (le, lc) = get_pair(&ctx.registry, &p.left_id).await?;
    let (re, rc) = get_pair(&ctx.registry, &p.right_id).await?;
    let id = store_expr(&ctx.registry, le.lt_eq(re), format!("({lc}).lt_eq({rc})")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__and",
    description = "Create a logical AND: left_expr.and(right_expr). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_and(ctx: Arc<PolarsExprCtx>, p: AndParams) -> Result<CallToolResult, ErrorData> {
    let (le, lc) = get_pair(&ctx.registry, &p.left_id).await?;
    let (re, rc) = get_pair(&ctx.registry, &p.right_id).await?;
    let id = store_expr(&ctx.registry, le.and(re), format!("({lc}).and({rc})")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__or",
    description = "Create a logical OR: left_expr.or(right_expr). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_or(ctx: Arc<PolarsExprCtx>, p: OrParams) -> Result<CallToolResult, ErrorData> {
    let (le, lc) = get_pair(&ctx.registry, &p.left_id).await?;
    let (re, rc) = get_pair(&ctx.registry, &p.right_id).await?;
    let id = store_expr(&ctx.registry, le.or(re), format!("({lc}).or({rc})")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__not",
    description = "Negate a boolean expression: expr.not(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_not(ctx: Arc<PolarsExprCtx>, p: NotParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.not(), format!("({c}).not()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__sum",
    description = "Aggregate sum: expr.sum(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_sum(ctx: Arc<PolarsExprCtx>, p: SumParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.sum(), format!("({c}).sum()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__mean",
    description = "Aggregate mean: expr.mean(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_mean(ctx: Arc<PolarsExprCtx>, p: MeanParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.mean(), format!("({c}).mean()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__min",
    description = "Aggregate minimum: expr.min(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_min(ctx: Arc<PolarsExprCtx>, p: MinParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.min(), format!("({c}).min()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__max",
    description = "Aggregate maximum: expr.max(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_max(ctx: Arc<PolarsExprCtx>, p: MaxParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.max(), format!("({c}).max()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__count",
    description = "Aggregate count: expr.count(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_count(ctx: Arc<PolarsExprCtx>, p: CountParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.count(), format!("({c}).count()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__n_unique",
    description = "Aggregate unique count: expr.n_unique(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_n_unique(
    ctx: Arc<PolarsExprCtx>,
    p: NUniqueParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.n_unique(), format!("({c}).n_unique()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__median",
    description = "Aggregate median: expr.median(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_median(
    ctx: Arc<PolarsExprCtx>,
    p: MedianParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.median(), format!("({c}).median()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__std",
    description = "Aggregate standard deviation: expr.std(ddof). ddof=1 = sample, ddof=0 = population. Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_std(ctx: Arc<PolarsExprCtx>, p: StdParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(
        &ctx.registry,
        e.std(p.ddof),
        format!("({c}).std({})", p.ddof),
    )
    .await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__var",
    description = "Aggregate variance: expr.var(ddof). ddof=1 = sample, ddof=0 = population. Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_var(ctx: Arc<PolarsExprCtx>, p: VarParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(
        &ctx.registry,
        e.var(p.ddof),
        format!("({c}).var({})", p.ddof),
    )
    .await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__first_val",
    description = "Take the first value of an expression: expr.first(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_first_val(
    ctx: Arc<PolarsExprCtx>,
    p: FirstValParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.first(), format!("({c}).first()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__last_val",
    description = "Take the last value of an expression: expr.last(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_last_val(
    ctx: Arc<PolarsExprCtx>,
    p: LastValParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.last(), format!("({c}).last()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__str_contains",
    description = "String contains pattern: expr.str().contains(lit(pattern), false). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_str_contains(
    ctx: Arc<PolarsExprCtx>,
    p: StrContainsParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let code = format!("({c}).str().contains(lit(\"{}\"), false)", p.pattern);
    let new_expr = e.str().contains(lit(p.pattern.as_str()), false);
    let id = store_expr(&ctx.registry, new_expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__str_starts_with",
    description = "String starts with: expr.str().starts_with(lit(pattern)). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_str_starts_with(
    ctx: Arc<PolarsExprCtx>,
    p: StrStartsWithParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let code = format!("({c}).str().starts_with(lit(\"{}\"))", p.pattern);
    let new_expr = e.str().starts_with(lit(p.pattern.as_str()));
    let id = store_expr(&ctx.registry, new_expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__str_ends_with",
    description = "String ends with: expr.str().ends_with(lit(pattern)). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_str_ends_with(
    ctx: Arc<PolarsExprCtx>,
    p: StrEndsWithParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let code = format!("({c}).str().ends_with(lit(\"{}\"))", p.pattern);
    let new_expr = e.str().ends_with(lit(p.pattern.as_str()));
    let id = store_expr(&ctx.registry, new_expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__str_to_lowercase",
    description = "Convert string to lowercase: expr.str().to_lowercase(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_str_to_lowercase(
    ctx: Arc<PolarsExprCtx>,
    p: StrToLowercaseParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(
        &ctx.registry,
        e.str().to_lowercase(),
        format!("({c}).str().to_lowercase()"),
    )
    .await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__str_to_uppercase",
    description = "Convert string to uppercase: expr.str().to_uppercase(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_str_to_uppercase(
    ctx: Arc<PolarsExprCtx>,
    p: StrToUppercaseParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(
        &ctx.registry,
        e.str().to_uppercase(),
        format!("({c}).str().to_uppercase()"),
    )
    .await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__str_replace",
    description = "Replace pattern in string: expr.str().replace(lit(pat), lit(rep), false). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_str_replace(
    ctx: Arc<PolarsExprCtx>,
    p: StrReplaceParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let code = format!(
        "({c}).str().replace(lit(\"{}\"), lit(\"{}\"), false)",
        p.pattern, p.replacement
    );
    let new_expr = e
        .str()
        .replace(lit(p.pattern.as_str()), lit(p.replacement.as_str()), false);
    let id = store_expr(&ctx.registry, new_expr, code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__dt_year",
    description = "Extract year from datetime: expr.dt().year(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_dt_year(
    ctx: Arc<PolarsExprCtx>,
    p: DtYearParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.dt().year(), format!("({c}).dt().year()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__dt_month",
    description = "Extract month from datetime: expr.dt().month(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_dt_month(
    ctx: Arc<PolarsExprCtx>,
    p: DtMonthParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.dt().month(), format!("({c}).dt().month()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__dt_day",
    description = "Extract day from datetime: expr.dt().day(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_dt_day(ctx: Arc<PolarsExprCtx>, p: DtDayParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.dt().day(), format!("({c}).dt().day()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__dt_hour",
    description = "Extract hour from datetime: expr.dt().hour(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_dt_hour(
    ctx: Arc<PolarsExprCtx>,
    p: DtHourParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.dt().hour(), format!("({c}).dt().hour()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__alias",
    description = "Rename an expression: expr.alias(name). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_alias(ctx: Arc<PolarsExprCtx>, p: AliasParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let code = format!("({c}).alias(\"{}\")", p.name);
    let id = store_expr(&ctx.registry, e.alias(p.name.as_str()), code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__cast",
    description = "Cast expression to a different data type: expr.cast(dtype). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_cast(ctx: Arc<PolarsExprCtx>, p: CastParams) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let dt_code = dtype_to_code(&p.dtype);
    let code = format!("({c}).cast({dt_code})");
    let id = store_expr(&ctx.registry, e.cast(polars_dtype(&p.dtype)), code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__fill_null_with_zero",
    description = "Fill null values with zero: expr.fill_null(lit(0)). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_fill_null_with_zero(
    ctx: Arc<PolarsExprCtx>,
    p: FillNullWithZeroParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(
        &ctx.registry,
        e.fill_null(lit(0i32)),
        format!("({c}).fill_null(lit(0))"),
    )
    .await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__sort_expr",
    description = "Sort an expression: expr.sort(SortOptions { descending, .. }). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_sort_expr(
    ctx: Arc<PolarsExprCtx>,
    p: SortExprParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let code = format!(
        "({c}).sort(SortOptions {{ descending: {}, ..Default::default() }})",
        p.descending
    );
    let opts = SortOptions {
        descending: p.descending,
        ..Default::default()
    };
    let id = store_expr(&ctx.registry, e.sort(opts), code).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__is_null",
    description = "Check for null values: expr.is_null(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_is_null(
    ctx: Arc<PolarsExprCtx>,
    p: IsNullParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(&ctx.registry, e.is_null(), format!("({c}).is_null()")).await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__is_not_null",
    description = "Check for non-null values: expr.is_not_null(). Establishes: PolarsExprCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_is_not_null(
    ctx: Arc<PolarsExprCtx>,
    p: IsNotNullParams,
) -> Result<CallToolResult, ErrorData> {
    let (e, c) = get_pair(&ctx.registry, &p.expr_id).await?;
    let id = store_expr(
        &ctx.registry,
        e.is_not_null(),
        format!("({c}).is_not_null()"),
    )
    .await;
    let _proof: Established<PolarsExprCreated> = Established::assert();
    Ok(json_result(&ExprIdResult {
        expr_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__describe",
    description = "Describe a stored expression by UUID. Returns the Rust code string.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_describe(
    ctx: Arc<PolarsExprCtx>,
    p: ExprDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let (_e, code) = get_pair(&ctx.registry, &p.expr_id).await?;
    #[derive(Serialize)]
    struct Desc {
        expr_id: String,
        code: String,
    }
    Ok(json_result(&Desc {
        expr_id: p.expr_id,
        code,
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__emit",
    description = "Emit the Rust source code for a stored expression. Returns the polars DSL code string.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_emit(
    ctx: Arc<PolarsExprCtx>,
    p: ExprEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let (_e, code) = get_pair(&ctx.registry, &p.expr_id).await?;
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicitation::elicit_tool(
    plugin = "polars_expr",
    name = "polars_expr__list",
    description = "List all stored expression UUIDs and their Rust code.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn expr_list(
    ctx: Arc<PolarsExprCtx>,
    _p: ExprListParams,
) -> Result<CallToolResult, ErrorData> {
    #[derive(Serialize)]
    struct Entry {
        expr_id: String,
        code: String,
    }
    let guard = ctx.registry.lock().await;
    let entries: Vec<Entry> = guard
        .iter()
        .map(|(id, (_, code))| Entry {
            expr_id: id.to_string(),
            code: code.clone(),
        })
        .collect();
    Ok(json_result(&entries))
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing `polars_expr__*` tools for polars Expr composition.
pub struct PolarsExprPlugin(Arc<PolarsExprCtx>);

impl PolarsExprPlugin {
    /// Create a new `PolarsExprPlugin` with an empty registry.
    pub fn new() -> Self {
        let registry: SharedExprRegistry = Arc::new(Mutex::new(HashMap::new()));
        Self(Arc::new(PolarsExprCtx::new(registry)))
    }

    /// Create a plugin from an existing shared registry.
    pub fn with_registry(registry: SharedExprRegistry) -> Self {
        Self(Arc::new(PolarsExprCtx::new(registry)))
    }

    /// Return a clone of the shared expr registry for use by other plugins.
    pub fn registry(&self) -> SharedExprRegistry {
        self.0.registry.clone()
    }

    /// Return the plugin context as a type-erased Arc for tool dispatch in tests.
    pub fn dispatch_ctx(&self) -> Arc<dyn std::any::Any + Send + Sync> {
        self.0.clone() as Arc<dyn std::any::Any + Send + Sync>
    }
}

impl Default for PolarsExprPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for PolarsExprPlugin {
    fn name(&self) -> &'static str {
        "polars_expr"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "polars_expr")
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
            let full_name = if name.starts_with("polars_expr__") {
                name.to_string()
            } else {
                format!("polars_expr__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "polars_expr")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
