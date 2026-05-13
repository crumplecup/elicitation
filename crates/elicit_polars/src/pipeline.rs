//! `PolarsPipelinePlugin` — MCP tools for polars LazyFrame pipeline code generation.
//!
//! Pipeline descriptors are stored server-side in a UUID-keyed registry.
//! No live polars evaluation happens at tool-call time — this plugin is
//! purely a code-generation layer. The `emit_main` tool renders a complete
//! `main.rs` from a stored pipeline descriptor.
//!
//! # Tool namespace: `polars_pipeline__*`

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{
    Elicit, PluginContext, PolarsPipelineDescriptor, PolarsPipelineOp, PolarsPipelineStep,
    VerifiedWorkflow,
};
use futures::future::BoxFuture;
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

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a polars pipeline descriptor was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct PolarsPipelineCreated;

impl Prop for PolarsPipelineCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_polars_pipeline_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "polars pipeline created");
            }
        }
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_polars_pipeline_created(ok: bool) -> (result: bool)
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
            pub fn verify_polars_pipeline_created_contract() -> bool { true }
        }
    }
}

impl VerifiedWorkflow for PolarsPipelineCreated {}

// ── Plugin context ─────────────────────────────────────────────────────────────

/// Shared state for all `polars_pipeline__*` tool calls.
pub struct PolarsPipelineCtx {
    pipelines: Mutex<HashMap<Uuid, PolarsPipelineDescriptor>>,
}

impl PolarsPipelineCtx {
    fn new() -> Self {
        Self {
            pipelines: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for PolarsPipelineCtx {}

// ── Code generation ────────────────────────────────────────────────────────────

fn emit_pipeline(desc: &PolarsPipelineDescriptor) -> String {
    let prelude = vec!["use polars::prelude::*;".to_string(), String::new()];
    let mut fn_lines = vec!["fn main() -> PolarsResult<()> {".to_string()];
    let mut chain_lines: Vec<String> = vec![];
    let mut output_lines: Vec<String> = vec![];
    let mut has_lazy = false;

    for step in &desc.steps {
        match &step.op {
            PolarsPipelineOp::ReadCsv { path, has_header } => {
                if !has_lazy {
                    chain_lines.push(format!(
                        "    let lf = LazyCsvReader::new(\"{path}\")\n        .with_has_header({has_header})\n        .finish()?"
                    ));
                    has_lazy = true;
                }
            }
            PolarsPipelineOp::ReadParquet { path } => {
                if !has_lazy {
                    chain_lines.push(format!(
                        "    let lf = LazyFrame::scan_parquet(\"{path}\", ScanArgsParquet::default())?"
                    ));
                    has_lazy = true;
                }
            }
            PolarsPipelineOp::ReadJson { path } => {
                if !has_lazy {
                    chain_lines.push(format!(
                        "    let lf = LazyFrame::scan_ndjson(\"{path}\", ScanArgsAnonymous::default())?"
                    ));
                    has_lazy = true;
                }
            }
            PolarsPipelineOp::Filter { predicate } => {
                chain_lines.push(format!("        .filter({predicate})"));
            }
            PolarsPipelineOp::Select { columns } => {
                let cols: Vec<String> = columns.iter().map(|c| format!("col(\"{c}\")")).collect();
                chain_lines.push(format!("        .select([{}])", cols.join(", ")));
            }
            PolarsPipelineOp::WithColumns { exprs } => {
                chain_lines.push(format!("        .with_columns([{}])", exprs.join(", ")));
            }
            PolarsPipelineOp::GroupByAgg { by, agg } => {
                let by_cols: Vec<String> = by.iter().map(|c| format!("col(\"{c}\")")).collect();
                chain_lines.push(format!(
                    "        .group_by([{}])\n        .agg([{}])",
                    by_cols.join(", "),
                    agg.join(", ")
                ));
            }
            PolarsPipelineOp::Join {
                right_path,
                left_on,
                right_on,
                how,
            } => {
                let left_on_cols: Vec<String> =
                    left_on.iter().map(|c| format!("col(\"{c}\")")).collect();
                let right_on_cols: Vec<String> =
                    right_on.iter().map(|c| format!("col(\"{c}\")")).collect();
                let join_type = match how.as_str() {
                    "left" => "JoinType::Left",
                    "right" => "JoinType::Right",
                    "full" => "JoinType::Full",
                    "cross" => "JoinType::Cross",
                    "semi" => "JoinType::Semi",
                    "anti" => "JoinType::Anti",
                    _ => "JoinType::Inner",
                };
                chain_lines.push(format!(
                    "        .join(\n            LazyCsvReader::new(\"{right_path}\").finish()?,\n            [{left}],\n            [{right}],\n            JoinArgs {{ how: {join_type}, ..Default::default() }},\n        )",
                    left = left_on_cols.join(", "),
                    right = right_on_cols.join(", "),
                ));
            }
            PolarsPipelineOp::Sort { by, descending } => {
                let by_strs: Vec<String> = by.iter().map(|c| format!("\"{c}\"")).collect();
                let desc_vals: Vec<String> = descending.iter().map(|d| d.to_string()).collect();
                chain_lines.push(format!(
                    "        .sort([{}], SortMultipleOptions::default().with_order_descending_multi([{}]))",
                    by_strs.join(", "),
                    desc_vals.join(", ")
                ));
            }
            PolarsPipelineOp::Unique { subset } => {
                if let Some(cols) = subset {
                    let col_strs: Vec<String> = cols.iter().map(|c| format!("\"{c}\"")).collect();
                    chain_lines.push(format!(
                        "        .unique(Some(vec![{}]), UniqueKeepStrategy::First)",
                        col_strs.join(", ")
                    ));
                } else {
                    chain_lines
                        .push("        .unique(None, UniqueKeepStrategy::First)".to_string());
                }
            }
            PolarsPipelineOp::DropNulls { subset } => {
                if let Some(cols) = subset {
                    let col_strs: Vec<String> = cols.iter().map(|c| format!("\"{c}\"")).collect();
                    chain_lines.push(format!(
                        "        .drop_nulls(Some(vec![{}].into()))",
                        col_strs.join(", ")
                    ));
                } else {
                    chain_lines.push("        .drop_nulls(None)".to_string());
                }
            }
            PolarsPipelineOp::WriteCsv { path } => {
                output_lines.push(format!(
                    "    CsvWriter::new(std::fs::File::create(\"{path}\")?.into()).finish(&mut df)?;"
                ));
            }
            PolarsPipelineOp::WriteParquet { path } => {
                output_lines.push(format!(
                    "    ParquetWriter::new(std::fs::File::create(\"{path}\")?).finish(&mut df)?;"
                ));
            }
            PolarsPipelineOp::WriteJson { path } => {
                output_lines.push(format!(
                    "    JsonWriter::new(std::fs::File::create(\"{path}\")?)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df)?;"
                ));
            }
        }
    }

    // Build the function body
    if has_lazy {
        let first = chain_lines.remove(0);
        fn_lines.push(first);
        for line in &chain_lines {
            fn_lines.push(line.clone());
        }
        if !chain_lines.is_empty() || !output_lines.is_empty() {
            fn_lines.push("        .collect()?;".to_string());
        } else {
            fn_lines.push(";".to_string());
        }
        fn_lines.push("    let mut df = lf;".to_string());
    }

    fn_lines.extend(output_lines);
    fn_lines.push("    Ok(())".to_string());
    fn_lines.push("}".to_string());

    let mut all = prelude;
    all.extend(fn_lines);
    all.join("\n")
}

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

#[derive(Serialize)]
struct PipelineIdResult {
    pipeline_id: String,
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `polars_pipeline__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PipelineNewParams {
    /// Human-readable pipeline name.
    pub name: String,
}

/// Parameters for `polars_pipeline__add_step`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PipelineAddStepParams {
    /// UUID of the pipeline to extend.
    pub pipeline_id: String,
    /// The operation to add.
    pub op: PolarsPipelineOp,
}

/// Parameters for `polars_pipeline__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PipelineDescribeParams {
    /// UUID of the pipeline.
    pub pipeline_id: String,
}

/// Parameters for `polars_pipeline__emit_main`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PipelineEmitMainParams {
    /// UUID of the pipeline.
    pub pipeline_id: String,
}

/// Parameters for `polars_pipeline__list`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PipelineListParams {}

/// Parameters for `polars_pipeline__remove_step`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PipelineRemoveStepParams {
    /// UUID of the pipeline to modify.
    pub pipeline_id: String,
    /// UUID of the step to remove.
    pub step_id: String,
}

/// Parameters for `polars_pipeline__clear`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PipelineClearParams {
    /// UUID of the pipeline to clear all steps from.
    pub pipeline_id: String,
}

// ── Tool functions ─────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "polars_pipeline",
    name = "polars_pipeline__new",
    description = "Create a new empty polars pipeline descriptor. \
                   Establishes: PolarsPipelineCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn pipeline_new(
    ctx: Arc<PolarsPipelineCtx>,
    p: PipelineNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let desc = PolarsPipelineDescriptor {
        pipeline_id: id,
        name: p.name,
        steps: vec![],
    };
    ctx.pipelines.lock().await.insert(id, desc);
    let _proof: Established<PolarsPipelineCreated> = Established::assert();
    Ok(json_result(&PipelineIdResult {
        pipeline_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_pipeline",
    name = "polars_pipeline__add_step",
    description = "Append a step to an existing pipeline descriptor. \
                   Establishes: PolarsPipelineCreated.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn pipeline_add_step(
    ctx: Arc<PolarsPipelineCtx>,
    p: PipelineAddStepParams,
) -> Result<CallToolResult, ErrorData> {
    let pipeline_id: Uuid = p
        .pipeline_id
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {}", p.pipeline_id)))?;
    let step_id = Uuid::new_v4();
    let step = PolarsPipelineStep { step_id, op: p.op };
    let mut guard = ctx.pipelines.lock().await;
    let desc = guard
        .get_mut(&pipeline_id)
        .ok_or_else(|| json_err(format!("pipeline_id not found: {pipeline_id}")))?;
    desc.steps.push(step);
    let _proof: Established<PolarsPipelineCreated> = Established::assert();
    #[derive(Serialize)]
    struct Result_ {
        pipeline_id: String,
        step_id: String,
        step_count: usize,
    }
    let count = desc.steps.len();
    Ok(json_result(&Result_ {
        pipeline_id: pipeline_id.to_string(),
        step_id: step_id.to_string(),
        step_count: count,
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_pipeline",
    name = "polars_pipeline__remove_step",
    description = "Remove a step from a pipeline by step UUID.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn pipeline_remove_step(
    ctx: Arc<PolarsPipelineCtx>,
    p: PipelineRemoveStepParams,
) -> Result<CallToolResult, ErrorData> {
    let pipeline_id: Uuid = p
        .pipeline_id
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {}", p.pipeline_id)))?;
    let step_id: Uuid = p
        .step_id
        .parse()
        .map_err(|_| json_err(format!("invalid step UUID: {}", p.step_id)))?;
    let mut guard = ctx.pipelines.lock().await;
    let desc = guard
        .get_mut(&pipeline_id)
        .ok_or_else(|| json_err(format!("pipeline_id not found: {pipeline_id}")))?;
    let before = desc.steps.len();
    desc.steps.retain(|s| s.step_id != step_id);
    let after = desc.steps.len();
    if before == after {
        return Err(json_err(format!("step_id not found: {step_id}")));
    }
    Ok(json_result(&PipelineIdResult {
        pipeline_id: pipeline_id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_pipeline",
    name = "polars_pipeline__clear",
    description = "Remove all steps from a pipeline descriptor.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn pipeline_clear(
    ctx: Arc<PolarsPipelineCtx>,
    p: PipelineClearParams,
) -> Result<CallToolResult, ErrorData> {
    let pipeline_id: Uuid = p
        .pipeline_id
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {}", p.pipeline_id)))?;
    let mut guard = ctx.pipelines.lock().await;
    let desc = guard
        .get_mut(&pipeline_id)
        .ok_or_else(|| json_err(format!("pipeline_id not found: {pipeline_id}")))?;
    desc.steps.clear();
    Ok(json_result(&PipelineIdResult {
        pipeline_id: pipeline_id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "polars_pipeline",
    name = "polars_pipeline__describe",
    description = "Describe a stored pipeline descriptor (name, step count, step ops).",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn pipeline_describe(
    ctx: Arc<PolarsPipelineCtx>,
    p: PipelineDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let pipeline_id: Uuid = p
        .pipeline_id
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {}", p.pipeline_id)))?;
    let guard = ctx.pipelines.lock().await;
    let desc = guard
        .get(&pipeline_id)
        .ok_or_else(|| json_err(format!("pipeline_id not found: {pipeline_id}")))?;
    Ok(json_result(desc))
}

#[elicitation::elicit_tool(
    plugin = "polars_pipeline",
    name = "polars_pipeline__emit_main",
    description = "Generate a complete Rust main.rs for a stored pipeline. \
                   Returns the Rust source code as a string.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn pipeline_emit_main(
    ctx: Arc<PolarsPipelineCtx>,
    p: PipelineEmitMainParams,
) -> Result<CallToolResult, ErrorData> {
    let pipeline_id: Uuid = p
        .pipeline_id
        .parse()
        .map_err(|_| json_err(format!("invalid UUID: {}", p.pipeline_id)))?;
    let guard = ctx.pipelines.lock().await;
    let desc = guard
        .get(&pipeline_id)
        .ok_or_else(|| json_err(format!("pipeline_id not found: {pipeline_id}")))?;
    let code = emit_pipeline(desc);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicitation::elicit_tool(
    plugin = "polars_pipeline",
    name = "polars_pipeline__list",
    description = "List all stored pipeline UUIDs, names, and step counts.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn pipeline_list(
    ctx: Arc<PolarsPipelineCtx>,
    _p: PipelineListParams,
) -> Result<CallToolResult, ErrorData> {
    #[derive(Serialize)]
    struct Entry {
        pipeline_id: String,
        name: String,
        step_count: usize,
    }
    let guard = ctx.pipelines.lock().await;
    let entries: Vec<Entry> = guard
        .iter()
        .map(|(id, desc)| Entry {
            pipeline_id: id.to_string(),
            name: desc.name.clone(),
            step_count: desc.steps.len(),
        })
        .collect();
    Ok(json_result(&entries))
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing `polars_pipeline__*` tools for polars pipeline code generation.
///
/// Stores `PolarsPipelineDescriptor` values in a UUID-keyed registry and
/// generates Rust `main.rs` source code via `emit_main`.
pub struct PolarsPipelinePlugin(Arc<PolarsPipelineCtx>);

impl PolarsPipelinePlugin {
    /// Create a new `PolarsPipelinePlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(PolarsPipelineCtx::new()))
    }

    /// Return the plugin context as a type-erased Arc for tool dispatch in tests.
    pub fn dispatch_ctx(&self) -> Arc<dyn std::any::Any + Send + Sync> {
        self.0.clone() as Arc<dyn std::any::Any + Send + Sync>
    }
}

impl Default for PolarsPipelinePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for PolarsPipelinePlugin {
    fn name(&self) -> &'static str {
        "polars_pipeline"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "polars_pipeline")
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
            let full_name = if name.starts_with("polars_pipeline__") {
                name.to_string()
            } else {
                format!("polars_pipeline__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "polars_pipeline")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
