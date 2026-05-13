# elicit_polars — DataFrame Library Shadow Crate Plan

> **Architecture:** Three distinct patterns cleanly separated — runtime execution, Expr composition, and pipeline code generation.
> **Scale:** ~75 tools across 4 plugins — realistic and testable.
> **Key advantage:** Polars `Expr` serializes via the `serde-lazy` feature, enabling a true Expr composition registry.

---

## Why the Original Plan Was Wrong

The original 570-tool plan conflated three distinct workflows into a confused "dual-mode" concept:

1. **Runtime execution** — Agent runs actual polars on server, gets live results
2. **Expr composition** — Agent composes Expr ASTs (stored as real polars Expr via serde-lazy)
3. **Code generation** — Agent builds a pipeline descriptor, emits a complete Rust program

These serve different purposes and must remain separate tools. "Dual-mode" and `emit = Auto` do not exist in our system.

---

## Polars Version

**Target:** polars 0.53.0 (current stable)

**Required features:**

- `lazy` — LazyFrame support
- `serde` — DataFrame, Schema, DataType serialization
- `serde-lazy` — **Expr and LazyFrame plan serialization** (the key feature)
- `csv`, `parquet`, `json`, `ipc` — I/O formats
- `sql` — SQLContext interface
- `dtype-full` — full DataType support (Date, Datetime, Duration, etc.)
- `strings`, `temporal` — string/date Expr ops

---

## Architecture: 4 Plugins, ~75 Tools

### Plugin 1: `PolarsExprPlugin` — Expr Composition Registry (~30 tools)

**The crown jewel:** Because `polars::Expr` serializes with `serde-lazy`, we store real `Expr` objects in
`Mutex<HashMap<Uuid, Expr>>`. Tools compose expressions functionally — each tool takes 0-2 `expr_id`
params and produces a new `expr_id`. The `expr__emit` tool serializes any stored Expr back to idiomatic
Rust code.

This is **not** the `body: String` pattern — we store and execute real Expr ASTs.

**Construction tools:**

- `expr__col(name: String)` → col("name")
- `expr__lit_int(value: i64)` → lit(42)
- `expr__lit_float(value: f64)` → lit(3.14)
- `expr__lit_str(value: String)` → lit("hello")
- `expr__lit_bool(value: bool)` → lit(true)
- `expr__all_columns()` → col("*")
- `expr__first_col()` → first()
- `expr__last_col()` → last()
- `expr__count_all()` → len()

**Binary comparison tools** (take `left_id`, `right_id`):

- `expr__eq`, `expr__neq`, `expr__gt`, `expr__lt`, `expr__gte`, `expr__lte`

**Logical combinator tools** (take two expr_ids):

- `expr__and`, `expr__or`, `expr__not(expr_id)`

**Aggregation tools** (take one expr_id):

- `expr__sum`, `expr__mean`, `expr__min`, `expr__max`, `expr__count`, `expr__n_unique`
- `expr__median`, `expr__std`, `expr__var`, `expr__first_val`, `expr__last_val`

**String ops** (take one expr_id + pattern: String):

- `expr__str_contains`, `expr__str_starts_with`, `expr__str_ends_with`
- `expr__str_to_lowercase`, `expr__str_to_uppercase`, `expr__str_replace`

**Temporal ops** (take one expr_id, extract field):

- `expr__dt_year`, `expr__dt_month`, `expr__dt_day`, `expr__dt_hour`

**Modifier tools** (take one expr_id):

- `expr__alias(expr_id, name: String)` — emit .alias("new_name")
- `expr__cast(expr_id, dtype: String)` — emit .cast(DataType::Int32)
- `expr__fill_null(expr_id, fill: String)` — emit .fill_null(0)
- `expr__sort_expr(expr_id, descending: bool)` — emit .sort(options)
- `expr__reverse_expr(expr_id)` — emit .reverse()
- `expr__is_null(expr_id)` — emit .is_null()
- `expr__is_not_null(expr_id)` — emit .is_not_null()

**Meta tools:**

- `expr__describe(expr_id)` → describe stored Expr
- `expr__emit(expr_id)` → emit Expr as Rust code string
- `expr__list()` → list all registered Expr UUIDs

**Proposition:** `ExprCreated { expr_id: Uuid, description: String }`

---

### Plugin 2: `PolarsDataFramePlugin` — Runtime Execution (~28 tools)

Stores actual polars `DataFrame` in `Mutex<HashMap<Uuid, DataFrame>>`. Tools execute real polars
operations at tool-call time and return results as JSON. Analogous to elicit_geo — actual computation
happens on the server.

**I/O tools** (produce df_id):

- `df__read_csv(path: String, has_header: bool, delimiter: Option<String>)` → `{ df_id }`
- `df__read_parquet(path: String)` → `{ df_id }`
- `df__read_json(path: String)` → `{ df_id }`
- `df__read_ipc(path: String)` → `{ df_id }`
- `df__from_json_string(json: String)` → `{ df_id }` (create from inline JSON records)

**Exploration tools** (take df_id, return data):

- `df__schema(df_id)` → `{ columns: [{name, dtype}] }`
- `df__shape(df_id)` → `{ rows, cols }`
- `df__head(df_id, n: usize)` → rows as JSON
- `df__tail(df_id, n: usize)` → rows as JSON
- `df__describe(df_id)` → summary stats as JSON
- `df__null_count(df_id)` → null count per column
- `df__sample(df_id, n: usize, seed: Option<u64>)` → rows as JSON

**Transform tools** (take df_id, produce new df_id):

- `df__select(df_id, expr_ids: Vec<Uuid>)` → new df_id
- `df__filter(df_id, predicate_expr_id: Uuid)` → new df_id
- `df__with_columns(df_id, expr_ids: Vec<Uuid>)` → new df_id
- `df__rename(df_id, old_name: String, new_name: String)` → new df_id
- `df__drop(df_id, column_names: Vec<String>)` → new df_id
- `df__sort(df_id, by: Vec<String>, descending: Vec<bool>)` → new df_id
- `df__unique(df_id, subset: Option<Vec<String>>)` → new df_id
- `df__drop_nulls(df_id, subset: Option<Vec<String>>)` → new df_id

**Join tool:**

- `df__join(left_id, right_id, left_on: Vec<String>, right_on: Vec<String>, how: String)` → new df_id

**Group/aggregate tool:**

- `df__group_by_agg(df_id, by: Vec<String>, agg_expr_ids: Vec<Uuid>)` → new df_id

**Output tools:**

- `df__write_csv(df_id, path: String)` → ok/error
- `df__write_parquet(df_id, path: String)` → ok/error
- `df__to_json_string(df_id, n: Option<usize>)` → JSON string of rows (for agent to read results)

**Propositions:**

- `DataFrameLoaded { df_id: Uuid, source: String, rows: usize, cols: usize }`
- `DataFrameTransformed { df_id: Uuid, op: String, from_id: Uuid }`
- `DataFrameWritten { df_id: Uuid, path: String }`

---

### Plugin 3: `PolarsPipelinePlugin` — Code Generation (~15 tools)

Descriptor-registry pattern (tower/axum style). Builds a LazyFrame pipeline as a series of named
steps, then emits a complete `main.rs` Rust program. No live polars at tool-call time — pure code generation.

**Descriptor types (in `elicitation` crate, `polars-types` feature):**

```rust
pub enum PolarsPipelineOp {
    ReadCsv { path: String, has_header: bool },
    ReadParquet { path: String },
    ReadJson { path: String },
    Filter { predicate: String },           // Rust Expr code (body: String)
    Select { columns: Vec<String> },
    WithColumns { exprs: Vec<String> },     // Rust Expr code per entry
    GroupByAgg { by: Vec<String>, agg: Vec<String> },
    Join { right_path: String, left_on: Vec<String>, right_on: Vec<String>, how: String },
    Sort { by: Vec<String>, descending: Vec<bool> },
    Unique { subset: Option<Vec<String>> },
    DropNulls { subset: Option<Vec<String>> },
    WriteCsv { path: String },
    WriteParquet { path: String },
    WriteJson { path: String },
}
```

**Tools:**

- `pipeline__new(name: String)` → `{ pipeline_id }`
- `pipeline__set_read_csv(pipeline_id, path, has_header)` → ok
- `pipeline__set_read_parquet(pipeline_id, path)` → ok
- `pipeline__add_filter(pipeline_id, predicate: String)` → ok
- `pipeline__add_select(pipeline_id, columns: Vec<String>)` → ok
- `pipeline__add_with_columns(pipeline_id, exprs: Vec<String>)` → ok
- `pipeline__add_group_by_agg(pipeline_id, by, agg_exprs)` → ok
- `pipeline__add_sort(pipeline_id, by, descending)` → ok
- `pipeline__add_unique(pipeline_id, subset)` → ok
- `pipeline__add_drop_nulls(pipeline_id, subset)` → ok
- `pipeline__set_output_csv(pipeline_id, path)` → ok
- `pipeline__set_output_parquet(pipeline_id, path)` → ok
- `pipeline__describe(pipeline_id)` → describe all steps
- `pipeline__emit_main(pipeline_id)` → complete `main.rs` Rust source
- `pipeline__list()` → list all pipelines

**`emit_main` output example:**

```rust
use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let df = LazyFrame::scan_csv("sales.csv", ScanArgsAnonymous::default())?
        .filter(col("region").eq(lit("EMEA")))
        .select([col("name"), col("revenue"), col("region")])
        .group_by([col("region")])
        .agg([col("revenue").sum().alias("total_revenue")])
        .sort(["total_revenue"], SortMultipleOptions::default().with_order_descending(true))
        .collect()?;

    df.write_parquet("emea_summary.parquet", ParquetWriteOptions::default())?;
    Ok(())
}
```

**Proposition:** `PipelineEmitted { pipeline_id: Uuid, steps: usize }`

---

### Plugin 4: `PolarsSqlPlugin` — SQL Interface (~5 tools)

Uses polars `SQLContext` to run SQL strings against registered DataFrames. Purely runtime.

**Tools:**

- `sql__new_context()` → `{ ctx_id }`
- `sql__register(ctx_id, frame_name: String, df_id: Uuid)` → ok
- `sql__execute(ctx_id, query: String)` → `{ df_id }` (registers result as new df)
- `sql__describe(ctx_id)` → list registered frames and their schemas
- `sql__list()` → list all SQL contexts

**Proposition:** `SqlQueryExecuted { ctx_id: Uuid, query: String, result_df_id: Uuid }`

---

## Verified Workflow: `PolarsDataPipeline`

```
ExprCreated -> DataFrameLoaded -> DataFrameTransformed -> DataFrameWritten
                                                       -> PipelineEmitted
                                                       -> SqlQueryExecuted
```

Propositions compose: a verified pipeline is one where data flows from load -> transform -> write,
with each step producing a new proposition.

---

## Elicitation Types (`polars-types` feature)

New feature in `crates/elicitation/`:

```rust
// enums.rs
pub enum PolarsJoinType { Inner, Left, Right, Full, Cross, Semi, Anti }
pub enum PolarsDType { Boolean, Int32, Int64, Float32, Float64, Utf8, Date, Datetime, Duration }
pub enum PolarsPipelineOp { /* as above */ }

// descriptors.rs
pub struct PolarsPipelineStep { pub step_id: Uuid, pub op: PolarsPipelineOp }
pub struct PolarsPipelineDescriptor {
    pub pipeline_id: Uuid,
    pub name: String,
    pub steps: Vec<PolarsPipelineStep>,
}
```

ElicitSpec/ElicitComplete impls: `PolarsJoinType`, `PolarsDType`, `PolarsPipelineDescriptor`.

---

## Implementation Phases

### Phase 1: Elicitation types + PolarsExprPlugin

1. Add `polars-types` feature to `crates/elicitation/Cargo.toml`
2. Add enum/descriptor types in `crates/elicitation/src/primitives/polars_types/`
3. Add ElicitSpec/ElicitComplete in `crates/elicitation/src/type_spec/polars_specs.rs`
4. Create `crates/elicit_polars/` workspace crate (Cargo.toml, lib.rs)
5. Implement `PolarsExprPlugin` (~30 tools)
6. Tests: expr composition, emit

### Phase 2: PolarsDataFramePlugin

1. Implement `PolarsDataFramePlugin` (~28 tools)
2. Tests: read CSV, filter, join, group_by_agg, write

### Phase 3: PolarsPipelinePlugin + PolarsSqlPlugin + README

1. Implement `PolarsPipelinePlugin` (~15 tools)
2. Implement `PolarsSqlPlugin` (~5 tools)
3. Tests: pipeline emit_main, SQL execute
4. Write README
5. Commit and push

---

## File Structure

```
crates/elicit_polars/
+-- Cargo.toml
+-- README.md
+-- src/
    +-- lib.rs
    +-- expr.rs           // PolarsExprPlugin (Expr registry)
    +-- dataframe.rs      // PolarsDataFramePlugin (runtime)
    +-- pipeline.rs       // PolarsPipelinePlugin (descriptor + emit)
    +-- sql.rs            // PolarsSqlPlugin (SQLContext)
tests/
    +-- polars_test.rs

crates/elicitation/src/primitives/polars_types/
    +-- mod.rs
    +-- enums.rs          // PolarsJoinType, PolarsDType
    +-- descriptors.rs    // PolarsPipelineStep, PolarsPipelineDescriptor

crates/elicitation/src/type_spec/
    +-- polars_specs.rs   // ElicitSpec/ElicitComplete impls
```

---

## Cargo.toml for elicit_polars

```toml
[package]
name = "elicit_polars"
version = "0.1.0"
edition = "2021"

[dependencies]
elicitation = { path = "../elicitation", features = ["polars-types"] }
polars = { version = "0.53", features = [
    "lazy",
    "serde",
    "serde-lazy",
    "csv",
    "parquet",
    "json",
    "ipc",
    "sql",
    "dtype-full",
    "strings",
    "temporal",
] }
# ... standard plugin deps (rmcp, tokio, uuid, serde, tracing, etc.)
```

---

## What We Deliberately Omit

- **Series operations** — duplicate what DataFrame + Expr already cover
- **Custom UDFs/apply** — require `Fn` closures; agent uses `body: String` in pipeline steps instead
- **Streaming/batching** — advanced; add in a future elicit_polars_streaming crate
- **600+ individual column-type methods** — covered by Expr composition + SQL instead
- **Direct LazyFrame plugin** — LazyFrame is the internal execution engine; agents compose via the pipeline builder, not raw LazyFrame methods

---

## Why 75 Tools (not 570)

The original plan counted every individual method as a tool. This violates composition principles:

- `df__select` + `expr__col` + `expr__alias` = column renaming (3 tools, infinite combinations)
- vs. `df__select_and_rename` + `df__select_cols_aliased` + ... = 50 brittle single-purpose tools

Compose from small primitives. The agent finds this easier to use than an exhaustive flat API.
