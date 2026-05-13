# elicit_polars

MCP shadow crate for [polars](https://pola.rs/) 0.53 — provides ~75 Model
Context Protocol tools across four plugins for DataFrame operations, Expr
composition, pipeline code generation, and SQL execution.

## Plugins

### `PolarsExprPlugin` — `polars_expr__*` (~37 tools)

Composes polars `Expr` values server-side. Every tool stores a real `Expr` and
its Rust source string; `emit` returns the DSL code for downstream code
generation.

| Tool | Params | Description |
|---|---|---|
| `col` | `name` | `col("name")` |
| `lit_int` | `value: i64` | `lit(v)` |
| `lit_float` | `value: f64` | `lit(v)` |
| `lit_str` | `value` | `lit("v")` |
| `lit_bool` | `value` | `lit(true/false)` |
| `all_columns` | — | `polars::prelude::all()` |
| `first_col` | — | `col("*").first()` |
| `last_col` | — | `col("*").last()` |
| `count_all` | — | `polars::prelude::len()` |
| `eq/neq/gt/lt/gte/lte` | `left_id, right_id` | Binary comparisons |
| `and/or` | `left_id, right_id` | Logical ops |
| `not` | `expr_id` | Negation |
| `sum/mean/min/max/count/n_unique/median` | `expr_id` | Aggregations |
| `std/var` | `expr_id, ddof` | Std dev / variance |
| `first_val/last_val` | `expr_id` | First/last value |
| `str_contains` | `expr_id, pattern` | `.str().contains(lit(p), false)` |
| `str_starts_with/ends_with` | `expr_id, pattern` | String prefix/suffix |
| `str_to_lowercase/uppercase` | `expr_id` | Case conversion |
| `str_replace` | `expr_id, pattern, replacement` | String replace |
| `dt_year/month/day/hour` | `expr_id` | Temporal extraction |
| `alias` | `expr_id, name` | `.alias("name")` |
| `cast` | `expr_id, dtype` | `.cast(DataType::...)` |
| `fill_null_with_zero` | `expr_id` | `.fill_null(lit(0))` |
| `sort_expr` | `expr_id, descending` | `.sort(SortOptions)` |
| `is_null/is_not_null` | `expr_id` | Null checks |
| `describe` | `expr_id` | Show stored code |
| `emit` | `expr_id` | Return Rust DSL source string |
| `list` | — | List all stored exprs |

### `PolarsDataFramePlugin` — `polars_df__*` (~23 tools)

Stores live `DataFrame` values by UUID. I/O ops work with files; the shared
`SharedExprRegistry` (from `PolarsExprPlugin`) powers filter, select, etc.

| Tool | Params | Description |
|---|---|---|
| `read_csv` | `path, has_header` | Read CSV to DataFrame |
| `read_parquet` | `path` | Read Parquet |
| `read_json` | `path` | Read JSON/NDJSON |
| `from_json_string` | `json` | Deserialise from JSON string |
| `schema` | `df_id` | Column names and types |
| `shape` | `df_id` | `{rows, cols}` |
| `head` | `df_id, n?` | First N rows as JSON array |
| `to_json_string` | `df_id, n?` | Serialise to JSON |
| `select` | `df_id, columns` | Keep named columns |
| `filter` | `df_id, expr_id` | Filter via stored Expr |
| `with_columns` | `df_id, expr_ids` | Compute new columns |
| `sort` | `df_id, by, descending` | Sort multi-column |
| `group_by_agg` | `df_id, by_ids, agg_ids` | GroupBy + aggregate |
| `join` | `df_id, right_id, left_on_ids, right_on_ids, how` | Join |
| `unique` | `df_id, subset?` | Deduplicate rows |
| `drop_nulls` | `df_id, subset?` | Drop null rows |
| `rename_column` | `df_id, old_name, new_name` | Rename column |
| `drop_column` | `df_id, name` | Drop column |
| `write_csv` | `df_id, path` | Write CSV |
| `write_parquet` | `df_id, path` | Write Parquet |
| `write_json` | `df_id, path` | Write JSON |
| `write_ipc` | `df_id, path` | Write Arrow IPC |
| `list` | — | List stored DataFrame UUIDs |

### `PolarsPipelinePlugin` — `polars_pipeline__*` (~7 tools)

Pure code generation — no live polars data. Stores `PolarsPipelineDescriptor`
values and emits idiomatic Rust `main.rs` source from them.

| Tool | Params | Description |
|---|---|---|
| `new` | `name` | Create pipeline descriptor |
| `add_step` | `pipeline_id, op` | Append a `PolarsPipelineOp` |
| `remove_step` | `pipeline_id, step_id` | Remove a step |
| `clear` | `pipeline_id` | Remove all steps |
| `describe` | `pipeline_id` | Inspect pipeline |
| `emit_main` | `pipeline_id` | Generate `fn main()` Rust source |
| `list` | — | List all pipeline UUIDs |

### `PolarsSqlPlugin` — `polars_sql__*` (~5 tools)

Wraps polars `SQLContext`. The shared `SharedDfRegistry` allows registering
DataFrames created by `PolarsDataFramePlugin` as SQL tables.

| Tool | Params | Description |
|---|---|---|
| `new_context` | — | Create a new SQLContext |
| `register` | `ctx_id, table_name, df_id` | Register a DataFrame as a table |
| `execute` | `ctx_id, query` | Run SQL, store result as DataFrame |
| `describe` | `ctx_id` | List registered table names |
| `list` | — | List all context UUIDs |

## Usage

```rust
use elicit_polars::{PolarsExprPlugin, PolarsDataFramePlugin, PolarsPipelinePlugin, PolarsSqlPlugin};

// Wire up shared registries
let expr_plugin = PolarsExprPlugin::new();
let df_plugin = PolarsDataFramePlugin::new(expr_plugin.registry());
let sql_plugin = PolarsSqlPlugin::new(df_plugin.df_registry());
let pipeline_plugin = PolarsPipelinePlugin::new();
```

Register all four with your MCP server, then agents can compose polars
pipelines through tool calls.

## Architecture

- **Shared Expr registry** (`SharedExprRegistry = Arc<Mutex<HashMap<Uuid, (Expr, String)>>>`):
  Both `PolarsExprPlugin` and `PolarsDataFramePlugin` hold a clone, so DataFrame
  tools can look up stored expressions.
- **Shared DF registry** (`SharedDfRegistry = Arc<Mutex<HashMap<Uuid, DataFrame>>>`):
  Both `PolarsDataFramePlugin` and `PolarsSqlPlugin` hold a clone, so SQL tools
  can access stored DataFrames.
- **Code tracking**: Every `PolarsExprPlugin` tool stores both the live `Expr`
  (for runtime) and a Rust code string (for `emit`). No reflection over the
  Expr AST is needed.
- **Pipeline code gen**: `PolarsPipelinePlugin` is pure descriptor storage with
  an `emit_main` tool that walks `PolarsPipelineOp` variants and produces a
  complete `fn main()`.

## Cargo features

Enable the following polars features in your project's `Cargo.toml`:

```toml
polars = { version = "0.53", features = [
    "lazy", "serde", "serde-lazy",
    "csv", "parquet", "json", "ipc",
    "dtype-full", "strings", "temporal", "sql",
] }
```

The `elicit_polars` crate enables all of the above through its own
`polars` dependency.
