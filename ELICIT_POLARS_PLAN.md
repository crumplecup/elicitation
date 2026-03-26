# elicit_polars — Complete DataFrame Library Harvesting Plan

> **Completionist mandate:** Expose the entire polars DataFrame/LazyFrame API as MCP tools.
> **Three-pronged approach:** Runtime tools (DataFrame handles) + Fragment tools (code generation) + Dual-mode tools (both).
> **Polars advantage:** ~80% of API is JSON-serializable (better than most Rust libraries).

---

## Executive Summary

**Scope:** Polars 0.53+ complete public API
**Strategy:** Harvest 100% using Runtime + Fragment + Dual patterns
**Estimated tools:** 600-800 MCP tools
**Key advantage:** Polars was designed for serialization - Expr is a serializable AST, DataFrame operations are data-driven.

---

## The Three Patterns Applied to Polars

### Pattern 1: Runtime Tools (UUID Registry)

**What works at runtime:**
- DataFrame/LazyFrame handles (UUID-keyed)
- All operations with serializable params (select, filter, join, group_by)
- Expr AST composition (fully serializable)
- I/O operations (CSV, Parquet, JSON, IPC)
- SQL interface (string → LazyFrame)

**Example:**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "filter")]
async fn df_filter(
    ctx: Arc<PluginContext>,
    params: DfFilterParams,
) -> Result<CallToolResult, ErrorData> {
    let dfs = ctx.dataframes.lock().unwrap();
    let df = dfs.get(&params.handle_id)?;

    // params.predicate is serialized Expr
    let filtered = df.filter(&params.predicate)?;

    let new_id = Uuid::new_v4();
    drop(dfs);
    ctx.dataframes.lock().unwrap().insert(new_id, filtered);

    Ok(CallToolResult::success(json!({ "df_id": new_id })))
}
```

### Pattern 2: Fragment Tools (Code Generation)

**What becomes fragments:**
- DataFrame chain operations (emit "df.select().filter().group_by()")
- LazyFrame query builders (emit logical plan code)
- Expr composition (emit "col(a).gt(lit(5)).and(col(b).eq(lit('foo')))")
- Complete data pipelines

**Example:**
```rust
#[elicit_tool(
    plugin = "polars_fragments",
    name = "emit_filter",
    description = "Emit polars filter code: df.filter(predicate)",
    emit = Auto
)]
async fn emit_filter(p: EmitFilterParams) -> Result<CallToolResult, ErrorData> {
    // Emits: .filter(col("age").gt(lit(18)))
    let code = format!(".filter({})", emit_expr(&p.predicate));
    Ok(CallToolResult::success(Content::text(code)))
}
```

### Pattern 3: Dual-Mode Tools (Runtime + Code Generation)

**Operations that do both:**
- All DataFrame operations have runtime execution AND code generation
- Agent chooses: execute now (get result handle) or emit code (get TokenStream)
- Same tool, two outputs via `emit = Auto`

**Example:**
```rust
#[elicit_tool(
    plugin = "polars_dataframe",
    name = "select",
    description = "Select columns from DataFrame",
    emit = Auto  // Generates both runtime AND code emit
)]
async fn df_select(p: DfSelectParams) -> Result<CallToolResult, ErrorData> {
    // Runtime execution:
    let df = get_dataframe(&p.df_id)?;
    let selected = df.select(&p.exprs)?;
    let new_id = register_dataframe(selected);
    Ok(CallToolResult::success(json!({ "df_id": new_id })))
}

// Auto-generated CustomEmit impl:
impl elicitation::emit_code::CustomEmit<DfSelectParams> for DfSelectEmit {
    fn emit_code(params: &DfSelectParams) -> TokenStream {
        let exprs = params.exprs.iter().map(emit_expr).collect::<Vec<_>>();
        quote! {
            .select(&[ #(#exprs),* ])
        }
    }
}
```

---

## Architecture: Single Shadow Crate

### elicit_polars

**Purpose:** Complete polars API exposure
**Patterns:** All three (Runtime + Fragment + Dual)

**Module structure:**
```
crates/elicit_polars/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── dataframe.rs          // DataFrame runtime plugin
    ├── lazyframe.rs          // LazyFrame runtime plugin
    ├── expr.rs               // Expr composition (runtime + fragment)
    ├── series.rs             // Series operations
    ├── io.rs                 // CSV/Parquet/JSON/IPC I/O
    ├── sql.rs                // SQL interface plugin
    ├── types.rs              // DataType, Schema, Field mirrors
    ├── fragments/
    │   ├── mod.rs
    │   ├── dataframe.rs      // DataFrame code emission
    │   ├── lazyframe.rs      // LazyFrame code emission
    │   ├── expr.rs           // Expr code emission
    │   └── pipeline.rs       // Complete pipeline assembly
    └── workflow.rs           // Workflow tools with propositions
```

---

## Phase 1: DataFrame Core (Runtime + Dual)

### 1.1 DataFrame Runtime Plugin

**UUID registry:**
```rust
pub struct PolarsDataFramePlugin {
    dataframes: Arc<Mutex<HashMap<Uuid, DataFrame>>>,
}

#[derive(ElicitPlugin)]
#[plugin(name = "polars_dataframe")]
impl PolarsDataFramePlugin { /* tools */ }
```

### 1.2 All DataFrame Operations (Dual-Mode)

**Construction:**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "new", emit = Auto)]
async fn df_new(p: DfNewParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "from_rows", emit = Auto)]
async fn df_from_rows(p: DfFromRowsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "read_csv", emit = Auto)]
async fn df_read_csv(p: ReadCsvParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "read_parquet", emit = Auto)]
async fn df_read_parquet(p: ReadParquetParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "read_json", emit = Auto)]
async fn df_read_json(p: ReadJsonParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "read_ipc", emit = Auto)]
async fn df_read_ipc(p: ReadIpcParams) -> Result<CallToolResult, ErrorData>
```

**Selection & Projection (~15 tools):**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "select", emit = Auto)]
async fn df_select(p: DfSelectParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "select_columns", emit = Auto)]
async fn df_select_columns(p: DfSelectColumnsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "with_column", emit = Auto)]
async fn df_with_column(p: DfWithColumnParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "with_columns", emit = Auto)]
async fn df_with_columns(p: DfWithColumnsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "drop", emit = Auto)]
async fn df_drop(p: DfDropParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "rename", emit = Auto)]
async fn df_rename(p: DfRenameParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "get_column", emit = Auto)]
async fn df_get_column(p: DfGetColumnParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "get_columns", emit = Auto)]
async fn df_get_columns(p: DfGetColumnsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "column", emit = Auto)]
async fn df_column(p: DfColumnParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "columns", emit = Auto)]
async fn df_columns(p: DfColumnsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "dtypes", emit = Auto)]
async fn df_dtypes(p: DfDtypesParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "schema", emit = Auto)]
async fn df_schema(p: DfSchemaParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "fields", emit = Auto)]
async fn df_fields(p: DfFieldsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "width", emit = Auto)]
async fn df_width(p: DfWidthParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "height", emit = Auto)]
async fn df_height(p: DfHeightParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "shape", emit = Auto)]
async fn df_shape(p: DfShapeParams) -> Result<CallToolResult, ErrorData>
```

**Filtering (~10 tools):**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "filter", emit = Auto)]
async fn df_filter(p: DfFilterParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "slice", emit = Auto)]
async fn df_slice(p: DfSliceParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "head", emit = Auto)]
async fn df_head(p: DfHeadParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "tail", emit = Auto)]
async fn df_tail(p: DfTailParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "sample_n", emit = Auto)]
async fn df_sample_n(p: DfSampleNParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "sample_frac", emit = Auto)]
async fn df_sample_frac(p: DfSampleFracParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "drop_nulls", emit = Auto)]
async fn df_drop_nulls(p: DfDropNullsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "fill_null", emit = Auto)]
async fn df_fill_null(p: DfFillNullParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "fill_nan", emit = Auto)]
async fn df_fill_nan(p: DfFillNanParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "unique", emit = Auto)]
async fn df_unique(p: DfUniqueParams) -> Result<CallToolResult, ErrorData>
```

**Sorting (~5 tools):**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "sort", emit = Auto)]
async fn df_sort(p: DfSortParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "sort_in_place", emit = Auto)]
async fn df_sort_in_place(p: DfSortInPlaceParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "reverse", emit = Auto)]
async fn df_reverse(p: DfReverseParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "top_k", emit = Auto)]
async fn df_top_k(p: DfTopKParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "bottom_k", emit = Auto)]
async fn df_bottom_k(p: DfBottomKParams) -> Result<CallToolResult, ErrorData>
```

**Joining (~10 tools):**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "join", emit = Auto)]
async fn df_join(p: DfJoinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "left_join", emit = Auto)]
async fn df_left_join(p: DfLeftJoinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "inner_join", emit = Auto)]
async fn df_inner_join(p: DfInnerJoinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "outer_join", emit = Auto)]
async fn df_outer_join(p: DfOuterJoinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "cross_join", emit = Auto)]
async fn df_cross_join(p: DfCrossJoinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "semi_join", emit = Auto)]
async fn df_semi_join(p: DfSemiJoinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "anti_join", emit = Auto)]
async fn df_anti_join(p: DfAntiJoinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "hstack", emit = Auto)]
async fn df_hstack(p: DfHstackParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "vstack", emit = Auto)]
async fn df_vstack(p: DfVstackParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "extend", emit = Auto)]
async fn df_extend(p: DfExtendParams) -> Result<CallToolResult, ErrorData>
```

**Aggregation & GroupBy (~15 tools):**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "group_by", emit = Auto)]
async fn df_group_by(p: DfGroupByParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "rolling", emit = Auto)]
async fn df_rolling(p: DfRollingParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "group_by_dynamic", emit = Auto)]
async fn df_group_by_dynamic(p: DfGroupByDynamicParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "sum", emit = Auto)]
async fn df_sum(p: DfSumParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "mean", emit = Auto)]
async fn df_mean(p: DfMeanParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "median", emit = Auto)]
async fn df_median(p: DfMedianParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "std", emit = Auto)]
async fn df_std(p: DfStdParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "var", emit = Auto)]
async fn df_var(p: DfVarParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "min", emit = Auto)]
async fn df_min(p: DfMinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "max", emit = Auto)]
async fn df_max(p: DfMaxParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "quantile", emit = Auto)]
async fn df_quantile(p: DfQuantileParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "null_count", emit = Auto)]
async fn df_null_count(p: DfNullCountParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "n_unique", emit = Auto)]
async fn df_n_unique(p: DfNUniqueParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "describe", emit = Auto)]
async fn df_describe(p: DfDescribeParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "pivot", emit = Auto)]
async fn df_pivot(p: DfPivotParams) -> Result<CallToolResult, ErrorData>
```

**Reshaping (~10 tools):**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "melt", emit = Auto)]
async fn df_melt(p: DfMeltParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "unpivot", emit = Auto)]
async fn df_unpivot(p: DfUnpivotParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "transpose", emit = Auto)]
async fn df_transpose(p: DfTransposeParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "explode", emit = Auto)]
async fn df_explode(p: DfExplodeParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "unnest", emit = Auto)]
async fn df_unnest(p: DfUnnestParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "to_dummies", emit = Auto)]
async fn df_to_dummies(p: DfToDummiesParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "partition_by", emit = Auto)]
async fn df_partition_by(p: DfPartitionByParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "shift", emit = Auto)]
async fn df_shift(p: DfShiftParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "shift_and_fill", emit = Auto)]
async fn df_shift_and_fill(p: DfShiftAndFillParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "rolling_map", emit = Auto)]
async fn df_rolling_map(p: DfRollingMapParams) -> Result<CallToolResult, ErrorData>
```

**I/O (~10 tools):**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "write_csv", emit = Auto)]
async fn df_write_csv(p: WriteCsvParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "write_parquet", emit = Auto)]
async fn df_write_parquet(p: WriteParquetParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "write_json", emit = Auto)]
async fn df_write_json(p: WriteJsonParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "write_ipc", emit = Auto)]
async fn df_write_ipc(p: WriteIpcParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "write_avro", emit = Auto)]
async fn df_write_avro(p: WriteAvroParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "to_json", emit = Auto)]
async fn df_to_json(p: DfToJsonParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "to_csv", emit = Auto)]
async fn df_to_csv(p: DfToCsvParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "to_struct", emit = Auto)]
async fn df_to_struct(p: DfToStructParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "to_dicts", emit = Auto)]
async fn df_to_dicts(p: DfToDictsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "to_rows", emit = Auto)]
async fn df_to_rows(p: DfToRowsParams) -> Result<CallToolResult, ErrorData>
```

**Metadata & Inspection (~10 tools):**
```rust
#[elicit_tool(plugin = "polars_dataframe", name = "is_empty", emit = Auto)]
async fn df_is_empty(p: DfIsEmptyParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "estimated_size", emit = Auto)]
async fn df_estimated_size(p: DfEstimatedSizeParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "n_chunks", emit = Auto)]
async fn df_n_chunks(p: DfNChunksParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "rechunk", emit = Auto)]
async fn df_rechunk(p: DfRechunkParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "align_chunks", emit = Auto)]
async fn df_align_chunks(p: DfAlignChunksParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "clone", emit = Auto)]
async fn df_clone(p: DfCloneParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "equals", emit = Auto)]
async fn df_equals(p: DfEqualsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "hash_rows", emit = Auto)]
async fn df_hash_rows(p: DfHashRowsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "apply", emit = Auto)]
async fn df_apply(p: DfApplyParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_dataframe", name = "with_row_index", emit = Auto)]
async fn df_with_row_index(p: DfWithRowIndexParams) -> Result<CallToolResult, ErrorData>
```

**Total DataFrame tools:** ~100

---

## Phase 2: LazyFrame Query Builder (Runtime + Dual)

### 2.1 LazyFrame Runtime Plugin

**UUID registry:**
```rust
pub struct PolarsLazyFramePlugin {
    lazyframes: Arc<Mutex<HashMap<Uuid, LazyFrame>>>,
}
```

### 2.2 All LazyFrame Operations (Dual-Mode)

**Scan operations (~10 tools):**
```rust
#[elicit_tool(plugin = "polars_lazy", name = "scan_csv", emit = Auto)]
async fn lf_scan_csv(p: ScanCsvParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "scan_parquet", emit = Auto)]
async fn lf_scan_parquet(p: ScanParquetParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "scan_ipc", emit = Auto)]
async fn lf_scan_ipc(p: ScanIpcParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "scan_ndjson", emit = Auto)]
async fn lf_scan_ndjson(p: ScanNdjsonParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "from_dataframe", emit = Auto)]
async fn lf_from_dataframe(p: LfFromDataframeParams) -> Result<CallToolResult, ErrorData>

// ... 5 more scan variants
```

**Query building (~40 tools):**
```rust
#[elicit_tool(plugin = "polars_lazy", name = "select", emit = Auto)]
async fn lf_select(p: LfSelectParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "with_column", emit = Auto)]
async fn lf_with_column(p: LfWithColumnParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "with_columns", emit = Auto)]
async fn lf_with_columns(p: LfWithColumnsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "filter", emit = Auto)]
async fn lf_filter(p: LfFilterParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "group_by", emit = Auto)]
async fn lf_group_by(p: LfGroupByParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "group_by_dynamic", emit = Auto)]
async fn lf_group_by_dynamic(p: LfGroupByDynamicParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "rolling", emit = Auto)]
async fn lf_rolling(p: LfRollingParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "join", emit = Auto)]
async fn lf_join(p: LfJoinParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "sort", emit = Auto)]
async fn lf_sort(p: LfSortParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "limit", emit = Auto)]
async fn lf_limit(p: LfLimitParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "slice", emit = Auto)]
async fn lf_slice(p: LfSliceParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "unique", emit = Auto)]
async fn lf_unique(p: LfUniqueParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "drop_nulls", emit = Auto)]
async fn lf_drop_nulls(p: LfDropNullsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "explode", emit = Auto)]
async fn lf_explode(p: LfExplodeParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "melt", emit = Auto)]
async fn lf_melt(p: LfMeltParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "rename", emit = Auto)]
async fn lf_rename(p: LfRenameParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "drop", emit = Auto)]
async fn lf_drop(p: LfDropParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "cast", emit = Auto)]
async fn lf_cast(p: LfCastParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "with_streaming", emit = Auto)]
async fn lf_with_streaming(p: LfWithStreamingParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "with_context", emit = Auto)]
async fn lf_with_context(p: LfWithContextParams) -> Result<CallToolResult, ErrorData>

// ... 20 more query building operations
```

**Execution & optimization (~10 tools):**
```rust
#[elicit_tool(plugin = "polars_lazy", name = "collect", emit = Auto)]
async fn lf_collect(p: LfCollectParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "fetch", emit = Auto)]
async fn lf_fetch(p: LfFetchParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "collect_async", emit = Auto)]
async fn lf_collect_async(p: LfCollectAsyncParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "sink_parquet", emit = Auto)]
async fn lf_sink_parquet(p: LfSinkParquetParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "sink_ipc", emit = Auto)]
async fn lf_sink_ipc(p: LfSinkIpcParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "sink_csv", emit = Auto)]
async fn lf_sink_csv(p: LfSinkCsvParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "explain", emit = Auto)]
async fn lf_explain(p: LfExplainParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "describe_plan", emit = Auto)]
async fn lf_describe_plan(p: LfDescribePlanParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "describe_optimized_plan", emit = Auto)]
async fn lf_describe_optimized_plan(p: LfDescribeOptimizedPlanParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_lazy", name = "cache", emit = Auto)]
async fn lf_cache(p: LfCacheParams) -> Result<CallToolResult, ErrorData>
```

**Total LazyFrame tools:** ~60

---

## Phase 3: Expr Composition (Runtime + Dual)

### 3.1 Expr is Serializable AST

**Key advantage:** Polars Expr derives `Serialize`/`Deserialize` out of the box!

```json
{
  "BinaryExpr": {
    "left": { "Column": "price" },
    "op": "Gt",
    "right": { "Literal": { "Int64": 100 } }
  }
}
```

### 3.2 All Expr Operations (Dual-Mode)

**Construction (~10 tools):**
```rust
#[elicit_tool(plugin = "polars_expr", name = "col", emit = Auto)]
async fn expr_col(p: ExprColParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "lit", emit = Auto)]
async fn expr_lit(p: ExprLitParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "cols", emit = Auto)]
async fn expr_cols(p: ExprColsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "all", emit = Auto)]
async fn expr_all(p: ExprAllParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "dtype_cols", emit = Auto)]
async fn expr_dtype_cols(p: ExprDtypeColsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "when", emit = Auto)]
async fn expr_when(p: ExprWhenParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "concat_list", emit = Auto)]
async fn expr_concat_list(p: ExprConcatListParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "concat_str", emit = Auto)]
async fn expr_concat_str(p: ExprConcatStrParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "format", emit = Auto)]
async fn expr_format(p: ExprFormatParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "nth", emit = Auto)]
async fn expr_nth(p: ExprNthParams) -> Result<CallToolResult, ErrorData>
```

**Binary operations (~20 tools):**
```rust
#[elicit_tool(plugin = "polars_expr", name = "add", emit = Auto)]
async fn expr_add(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "sub", emit = Auto)]
async fn expr_sub(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "mul", emit = Auto)]
async fn expr_mul(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "div", emit = Auto)]
async fn expr_div(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "floor_div", emit = Auto)]
async fn expr_floor_div(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "modulo", emit = Auto)]
async fn expr_modulo(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "pow", emit = Auto)]
async fn expr_pow(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "eq", emit = Auto)]
async fn expr_eq(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "neq", emit = Auto)]
async fn expr_neq(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "lt", emit = Auto)]
async fn expr_lt(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "gt", emit = Auto)]
async fn expr_gt(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "lt_eq", emit = Auto)]
async fn expr_lt_eq(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "gt_eq", emit = Auto)]
async fn expr_gt_eq(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "and", emit = Auto)]
async fn expr_and(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "or", emit = Auto)]
async fn expr_or(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "xor", emit = Auto)]
async fn expr_xor(p: ExprBinaryParams) -> Result<CallToolResult, ErrorData>

// ... bitwise operations
```

**Aggregations (~25 tools):**
```rust
#[elicit_tool(plugin = "polars_expr", name = "sum", emit = Auto)]
async fn expr_sum(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "mean", emit = Auto)]
async fn expr_mean(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "median", emit = Auto)]
async fn expr_median(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "min", emit = Auto)]
async fn expr_min(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "max", emit = Auto)]
async fn expr_max(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "std", emit = Auto)]
async fn expr_std(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "var", emit = Auto)]
async fn expr_var(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "first", emit = Auto)]
async fn expr_first(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "last", emit = Auto)]
async fn expr_last(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "count", emit = Auto)]
async fn expr_count(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "n_unique", emit = Auto)]
async fn expr_n_unique(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "null_count", emit = Auto)]
async fn expr_null_count(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "arg_min", emit = Auto)]
async fn expr_arg_min(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "arg_max", emit = Auto)]
async fn expr_arg_max(p: ExprAggParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "quantile", emit = Auto)]
async fn expr_quantile(p: ExprQuantileParams) -> Result<CallToolResult, ErrorData>

// ... 10 more aggregations
```

**String operations (~40 tools):**
```rust
#[elicit_tool(plugin = "polars_expr", name = "str_len", emit = Auto)]
async fn expr_str_len(p: ExprStrParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_contains", emit = Auto)]
async fn expr_str_contains(p: ExprStrContainsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_starts_with", emit = Auto)]
async fn expr_str_starts_with(p: ExprStrStartsWithParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_ends_with", emit = Auto)]
async fn expr_str_ends_with(p: ExprStrEndsWithParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_replace", emit = Auto)]
async fn expr_str_replace(p: ExprStrReplaceParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_replace_all", emit = Auto)]
async fn expr_str_replace_all(p: ExprStrReplaceAllParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_to_lowercase", emit = Auto)]
async fn expr_str_to_lowercase(p: ExprStrParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_to_uppercase", emit = Auto)]
async fn expr_str_to_uppercase(p: ExprStrParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_strip", emit = Auto)]
async fn expr_str_strip(p: ExprStrStripParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_slice", emit = Auto)]
async fn expr_str_slice(p: ExprStrSliceParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "str_split", emit = Auto)]
async fn expr_str_split(p: ExprStrSplitParams) -> Result<CallToolResult, ErrorData>

// ... 30 more string operations
```

**Temporal operations (~30 tools):**
```rust
#[elicit_tool(plugin = "polars_expr", name = "dt_year", emit = Auto)]
async fn expr_dt_year(p: ExprDtParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "dt_month", emit = Auto)]
async fn expr_dt_month(p: ExprDtParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "dt_day", emit = Auto)]
async fn expr_dt_day(p: ExprDtParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "dt_hour", emit = Auto)]
async fn expr_dt_hour(p: ExprDtParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "dt_strftime", emit = Auto)]
async fn expr_dt_strftime(p: ExprDtStrftimeParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "dt_timestamp", emit = Auto)]
async fn expr_dt_timestamp(p: ExprDtParams) -> Result<CallToolResult, ErrorData>

// ... 24 more temporal operations
```

**List operations (~20 tools):**
```rust
#[elicit_tool(plugin = "polars_expr", name = "list_len", emit = Auto)]
async fn expr_list_len(p: ExprListParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "list_get", emit = Auto)]
async fn expr_list_get(p: ExprListGetParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "list_sum", emit = Auto)]
async fn expr_list_sum(p: ExprListParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "list_mean", emit = Auto)]
async fn expr_list_mean(p: ExprListParams) -> Result<CallToolResult, ErrorData>

// ... 16 more list operations
```

**Mathematical (~30 tools):**
```rust
#[elicit_tool(plugin = "polars_expr", name = "abs", emit = Auto)]
async fn expr_abs(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "sqrt", emit = Auto)]
async fn expr_sqrt(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "cbrt", emit = Auto)]
async fn expr_cbrt(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "sin", emit = Auto)]
async fn expr_sin(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "cos", emit = Auto)]
async fn expr_cos(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "tan", emit = Auto)]
async fn expr_tan(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "arcsin", emit = Auto)]
async fn expr_arcsin(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "arccos", emit = Auto)]
async fn expr_arccos(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "arctan", emit = Auto)]
async fn expr_arctan(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "log", emit = Auto)]
async fn expr_log(p: ExprLogParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "log10", emit = Auto)]
async fn expr_log10(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "exp", emit = Auto)]
async fn expr_exp(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "ceil", emit = Auto)]
async fn expr_ceil(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "floor", emit = Auto)]
async fn expr_floor(p: ExprMathParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "round", emit = Auto)]
async fn expr_round(p: ExprRoundParams) -> Result<CallToolResult, ErrorData>

// ... 15 more math operations
```

**Other transformations (~40 tools):**
```rust
#[elicit_tool(plugin = "polars_expr", name = "alias", emit = Auto)]
async fn expr_alias(p: ExprAliasParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "cast", emit = Auto)]
async fn expr_cast(p: ExprCastParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "sort", emit = Auto)]
async fn expr_sort(p: ExprSortParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "is_null", emit = Auto)]
async fn expr_is_null(p: ExprIsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "is_not_null", emit = Auto)]
async fn expr_is_not_null(p: ExprIsParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "fill_null", emit = Auto)]
async fn expr_fill_null(p: ExprFillNullParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "forward_fill", emit = Auto)]
async fn expr_forward_fill(p: ExprFillParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "backward_fill", emit = Auto)]
async fn expr_backward_fill(p: ExprFillParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "interpolate", emit = Auto)]
async fn expr_interpolate(p: ExprInterpolateParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "cumsum", emit = Auto)]
async fn expr_cumsum(p: ExprCumParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "cummin", emit = Auto)]
async fn expr_cummin(p: ExprCumParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "cummax", emit = Auto)]
async fn expr_cummax(p: ExprCumParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "cumprod", emit = Auto)]
async fn expr_cumprod(p: ExprCumParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "rank", emit = Auto)]
async fn expr_rank(p: ExprRankParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "diff", emit = Auto)]
async fn expr_diff(p: ExprDiffParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "shift", emit = Auto)]
async fn expr_shift(p: ExprShiftParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "clip", emit = Auto)]
async fn expr_clip(p: ExprClipParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_expr", name = "over", emit = Auto)]
async fn expr_over(p: ExprOverParams) -> Result<CallToolResult, ErrorData>

// ... 22 more transformations
```

**Total Expr tools:** ~250

---

## Phase 4: SQL Interface (Runtime Only)

### 4.1 SQL Plugin

```rust
pub struct PolarsSqlPlugin {
    contexts: Arc<Mutex<HashMap<Uuid, SQLContext>>>,
}

#[elicit_tool(plugin = "polars_sql", name = "context_new")]
async fn sql_context_new() -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_sql", name = "register_df")]
async fn sql_register_df(p: SqlRegisterParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_sql", name = "register_many")]
async fn sql_register_many(p: SqlRegisterManyParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_sql", name = "execute")]
async fn sql_execute(p: SqlExecuteParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_sql", name = "execute_to_df")]
async fn sql_execute_to_df(p: SqlExecuteParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_sql", name = "get_tables")]
async fn sql_get_tables(p: SqlGetTablesParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_sql", name = "unregister")]
async fn sql_unregister(p: SqlUnregisterParams) -> Result<CallToolResult, ErrorData>
```

**Total SQL tools:** ~10

---

## Phase 5: Series Operations (Runtime + Dual)

### 5.1 Series Tools (~50 tools)

```rust
#[elicit_tool(plugin = "polars_series", name = "new", emit = Auto)]
async fn series_new(p: SeriesNewParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_series", name = "name", emit = Auto)]
async fn series_name(p: SeriesNameParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_series", name = "rename", emit = Auto)]
async fn series_rename(p: SeriesRenameParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_series", name = "dtype", emit = Auto)]
async fn series_dtype(p: SeriesDtypeParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_series", name = "len", emit = Auto)]
async fn series_len(p: SeriesLenParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_series", name = "null_count", emit = Auto)]
async fn series_null_count(p: SeriesNullCountParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_series", name = "sum", emit = Auto)]
async fn series_sum(p: SeriesSumParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "polars_series", name = "mean", emit = Auto)]
async fn series_mean(p: SeriesMeanParams) -> Result<CallToolResult, ErrorData>

// ... 42 more Series operations
```

**Total Series tools:** ~50

---

## Phase 6: Fragment Tools (Code Generation Only)

### 6.1 Pipeline Assembly

**Emit complete data pipeline:**
```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AssemblePipelineParams {
    pub package_name: String,
    pub input_file: String,
    pub input_format: String,  // "csv", "parquet", etc.
    pub operations: Vec<PipelineOperation>,
    pub output_file: String,
    pub output_format: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PipelineOperation {
    pub op_type: String,  // "select", "filter", "group_by", etc.
    pub params: serde_json::Value,
}

#[elicit_tool(
    plugin = "polars_fragments",
    name = "assemble_pipeline",
    description = "Generate complete polars data pipeline with main.rs and Cargo.toml",
    emit = Auto
)]
async fn assemble_pipeline(p: AssemblePipelineParams) -> Result<CallToolResult, ErrorData> {
    let main_rs = generate_pipeline_main(&p);
    let cargo_toml = generate_pipeline_cargo(&p);

    Ok(CallToolResult::success(json!({
        "main_rs": main_rs,
        "cargo_toml": cargo_toml
    })))
}

fn generate_pipeline_main(p: &AssemblePipelineParams) -> String {
    format!(r#"
use polars::prelude::*;

fn main() -> PolarsResult<()> {{
    let df = {}?;

    let result = df
        {};

    {}

    Ok(())
}}
"#,
        generate_input_code(&p.input_file, &p.input_format),
        generate_operations_chain(&p.operations),
        generate_output_code(&p.output_file, &p.output_format)
    )
}
```

**Example output:**
```rust
// Generated main.rs
use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let df = CsvReader::from_path("data.csv")?
        .has_header(true)
        .finish()?;

    let result = df
        .lazy()
        .filter(col("age").gt(lit(18)))
        .select(&[col("name"), col("email"), col("age")])
        .group_by(&[col("country")])
        .agg(&[col("age").mean().alias("avg_age")])
        .sort("avg_age", SortOptions::default())
        .collect()?;

    result.write_parquet("output.parquet")?;

    Ok(())
}
```

**Total Fragment-only tools:** ~30

---

## Estimated Tool Count

| Category | Runtime | Dual-Mode | Fragment | Total |
|---|---|---|---|---|
| DataFrame operations | 0 | 100 | 0 | 100 |
| LazyFrame operations | 0 | 60 | 0 | 60 |
| Expr operations | 0 | 250 | 0 | 250 |
| Series operations | 0 | 50 | 0 | 50 |
| SQL interface | 10 | 0 | 0 | 10 |
| I/O operations | 0 | 20 | 0 | 20 |
| Types/Schema | 30 | 0 | 0 | 30 |
| Pipeline assembly | 0 | 0 | 30 | 30 |
| Workflow tools | 20 | 0 | 0 | 20 |
| **Total** | **60** | **480** | **30** | **570** |

**Dual-mode breakdown:**
- Each dual-mode tool generates BOTH a runtime handler AND a CustomEmit impl
- Actual implementation count: 570 runtime tools + 510 emit impls = 1,080 implementations
- Exposed to agents as: 570 MCP tools (agent chooses runtime vs emit via context)

---

## Implementation Timeline

**Week 1:** DataFrame operations (50 tools)
**Week 2:** DataFrame operations (remaining 50 tools)
**Week 3:** LazyFrame operations (60 tools)
**Week 4:** Expr construction + binary ops (80 tools)
**Week 5:** Expr aggregations + string ops (80 tools)
**Week 6:** Expr temporal + list ops (50 tools)
**Week 7:** Expr math + transformations (40 tools)
**Week 8:** Series operations (50 tools)
**Week 9:** SQL interface + I/O (30 tools)
**Week 10:** Fragment assembly + testing (30 tools)

**Total:** 10 weeks for complete implementation

---

## Success Criteria

1. ✅ 100% of polars public API exposed (DataFrame, LazyFrame, Expr, Series, SQL)
2. ✅ All operations are dual-mode (runtime + code generation)
3. ✅ Expr serialization works perfectly (test with complex nested expressions)
4. ✅ Agent can build complete data pipelines from scratch
5. ✅ All 570 tools registered and tested
6. ✅ Runtime execution matches generated code output (equivalence testing)
7. ✅ Comprehensive documentation with 30+ examples

---

## Key Innovations

1. **Expr AST Advantage:** Polars Expr is already serializable - no custom encoding needed
2. **Triple Harvest with Dual-Mode:** Most tools are dual-mode (both runtime + emit)
3. **Code Recovery:** Agents execute at runtime, recover source for deployment
4. **SQL Escape Hatch:** High-level interface for complex queries
5. **Zero Compromise:** 100% of serializable API exposed, closures skipped (only ~10% of API)
