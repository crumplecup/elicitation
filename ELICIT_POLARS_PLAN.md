# elicit_polars — Implementation Plan

> **Premise:** Expose polars' serializable DataFrame/LazyFrame operations as MCP tools.
> Pragmatic approach: **~70-80% of the API is JSON-serializable**, while **~20-30% requires closures** and cannot cross the MCP boundary.

---

## The Polars Advantage

Unlike tokio (where futures/closures dominate the API), polars was **designed for serialization**:

| Category | Examples | Viable as MCP tool? |
|---|---|---|
| DataFrame operations | `select`, `filter`, `join`, `group_by` | ✅ All serializable |
| LazyFrame query building | `scan_csv`, `filter`, `with_column` | ✅ Builds logical plan |
| Expr DSL | `col("a").gt(lit(5))`, aggregations | ✅ `#[derive(Serialize)]` |
| I/O operations | CSV, Parquet, JSON, IPC | ✅ File paths + options structs |
| SQL interface | `ctx.execute("SELECT ...")` | ✅ String → LazyFrame |
| Built-in functions | ~200 functions (sum, mean, etc.) | ✅ Enum-based dispatch |
| Custom UDFs | `expr.map(\|s\| custom(s))` | ❌ Closures |
| Object columns | `df.with_object_column::<T>()` | ❌ Requires trait impl |

**Key Insight:** Polars' `Expr` type is a **serializable AST**. Agents can build complex queries by composing JSON-serializable expressions.

---

## What Actually Makes Sense to Elicit

### Tier 1: DataFrame Handle Registry

**Stateful plugin** managing DataFrame/LazyFrame handles via UUID:

```rust
pub struct PolarsDataPlugin {
    dataframes: Arc<Mutex<HashMap<Uuid, DataFrame>>>,
    lazyframes: Arc<Mutex<HashMap<Uuid, LazyFrame>>>,
}
```

**Operations:**
- Create: `df_new`, `df_from_json`, `df_read_csv`, etc.
- Transform: `df_select`, `df_filter`, `df_sort`, etc.
- Query: `df_shape`, `df_schema`, `df_describe`, `df_to_json`
- Join/Combine: `df_join`, `df_hstack`, `df_vstack`
- Export: `df_write_csv`, `df_write_parquet`, etc.

### Tier 2: LazyFrame Query Builder

**Lazy operations build logical plans without executing:**

```rust
// Agent workflow:
1. lf_scan_csv(path) → uuid_1
2. lf_filter(uuid_1, predicate_expr) → uuid_2
3. lf_group_by(uuid_2, by_exprs, agg_exprs) → uuid_3
4. lf_collect(uuid_3) → df_uuid
```

**All operations return new LazyFrame UUIDs** (immutable transformations).

### Tier 3: Expr Composition Tools

**Expressions are serializable ASTs:**

```json
// col("price") > lit(100)
{
  "BinaryExpr": {
    "left": { "Column": "price" },
    "op": "Gt",
    "right": { "Literal": { "Int64": 100 } }
  }
}

// col("revenue").sum().alias("total")
{
  "Alias": [
    {
      "Agg": {
        "Sum": [{ "Column": "revenue" }]
      }
    },
    "total"
  ]
}
```

**Tools:**
- `expr_col(name)` → Expr
- `expr_lit(value)` → Expr
- `expr_binary(left, op, right)` → Expr
- `expr_agg(input, agg_type)` → Expr (sum, mean, min, max, etc.)
- `expr_function(name, args)` → Expr (dispatches to ~200 built-ins)

### Tier 4: SQL Interface

**Highest-level abstraction:**

```rust
pub struct PolarsSqlPlugin {
    contexts: Arc<Mutex<HashMap<Uuid, SQLContext>>>,
}

// Tools:
sql_context_new() → uuid
sql_register_dataframe(ctx_uuid, df_uuid, table_name)
sql_execute(ctx_uuid, query_string) → lf_uuid or df_uuid
```

**Agent can write SQL queries, get back LazyFrame handles.**

---

## Phase Breakdown

### Phase 1: DataFrame Core (Eager Operations)

**Crate structure:**
```
crates/elicit_polars/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── dataframe.rs          // DataFrame handle plugin
    ├── expr.rs               // Expr serialization helpers
    ├── io.rs                 // CSV/Parquet I/O
    └── types.rs              // DataType, Schema mirrors
```

**Cargo.toml essentials:**
```toml
[dependencies]
polars = { version = "0.53", features = [
    "lazy",
    "dtype-full",
    "serde",
    "csv",
    "json",
    "parquet",
    "ipc",
    "describe",
    "strings",
    "temporal",
] }
elicitation = { workspace = true }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
```

**PolarsDataFramePlugin:**
```rust
#[derive(Clone, ElicitPlugin)]
pub struct PolarsDataFramePlugin(Arc<PluginContext>);

struct PluginContext {
    dataframes: Mutex<HashMap<Uuid, DataFrame>>,
}

#[elicit_tool(plugin = "polars_dataframe", name = "create_from_dict")]
async fn df_create_from_dict(
    ctx: Arc<PluginContext>,
    params: DfCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let df = DataFrame::new(params.columns)?;
    let id = Uuid::new_v4();
    ctx.dataframes.lock().unwrap().insert(id, df);
    Ok(CallToolResult::success(json!({ "handle_id": id })))
}

#[elicit_tool(plugin = "polars_dataframe", name = "select")]
async fn df_select(
    ctx: Arc<PluginContext>,
    params: DfSelectParams,
) -> Result<CallToolResult, ErrorData> {
    let dfs = ctx.dataframes.lock().unwrap();
    let df = dfs.get(&params.handle_id)
        .ok_or_else(|| ErrorData::new("DataFrame not found"))?;

    let result = df.select(params.columns)?;

    let new_id = Uuid::new_v4();
    drop(dfs); // Release lock
    ctx.dataframes.lock().unwrap().insert(new_id, result);

    Ok(CallToolResult::success(json!({ "handle_id": new_id })))
}

#[elicit_tool(plugin = "polars_dataframe", name = "to_json")]
async fn df_to_json(
    ctx: Arc<PluginContext>,
    params: DfToJsonParams,
) -> Result<CallToolResult, ErrorData> {
    let dfs = ctx.dataframes.lock().unwrap();
    let df = dfs.get(&params.handle_id)
        .ok_or_else(|| ErrorData::new("DataFrame not found"))?;

    let json = serde_json::to_value(df)?;
    Ok(CallToolResult::success(json))
}
```

**Operations (Phase 1):**
| Tool | Params | Returns | Contract |
|---|---|---|---|
| `df_create_from_dict` | `columns: HashMap<String, Vec<Value>>` | `{ handle_id }` | `DataFrameCreated` |
| `df_read_csv` | `path, options: CsvReadOptions` | `{ handle_id }` | `DataFrameCreated` |
| `df_read_parquet` | `path, options` | `{ handle_id }` | `DataFrameCreated` |
| `df_select` | `handle_id, columns: Vec<String>` | `{ handle_id }` | — |
| `df_filter` | `handle_id, mask_handle_id` | `{ handle_id }` | — |
| `df_slice` | `handle_id, offset, length` | `{ handle_id }` | — |
| `df_head` | `handle_id, n` | `{ handle_id }` | — |
| `df_tail` | `handle_id, n` | `{ handle_id }` | — |
| `df_sort` | `handle_id, by: Vec<String>, descending` | `{ handle_id }` | — |
| `df_reverse` | `handle_id` | `{ handle_id }` | — |
| `df_join` | `left_id, right_id, on, how: JoinType` | `{ handle_id }` | — |
| `df_hstack` | `handle_id, other_id` | `{ handle_id }` | — |
| `df_vstack` | `handle_id, other_id` | `{ handle_id }` | — |
| `df_drop` | `handle_id, columns` | `{ handle_id }` | — |
| `df_drop_nulls` | `handle_id, subset` | `{ handle_id }` | — |
| `df_shape` | `handle_id` | `{ rows, cols }` | — |
| `df_schema` | `handle_id` | `{ fields: [{ name, dtype }] }` | — |
| `df_describe` | `handle_id` | `{ handle_id }` (summary stats) | — |
| `df_to_json` | `handle_id` | `{ data: [...] }` | — |
| `df_write_csv` | `handle_id, path, options` | `{ bytes_written }` | — |
| `df_write_parquet` | `handle_id, path, options` | `{ bytes_written }` | — |

---

### Phase 2: LazyFrame Query Builder

**PolarsLazyFramePlugin:**
```rust
#[derive(Clone, ElicitPlugin)]
pub struct PolarsLazyFramePlugin(Arc<PluginContext>);

struct PluginContext {
    lazyframes: Mutex<HashMap<Uuid, LazyFrame>>,
}

#[elicit_tool(plugin = "polars_lazy", name = "scan_csv")]
async fn lf_scan_csv(
    ctx: Arc<PluginContext>,
    params: LfScanCsvParams,
) -> Result<CallToolResult, ErrorData> {
    let lf = LazyFrame::scan_csv(
        params.path,
        params.options.unwrap_or_default()
    )?;

    let id = Uuid::new_v4();
    ctx.lazyframes.lock().unwrap().insert(id, lf);
    Ok(CallToolResult::success(json!({ "handle_id": id })))
}

#[elicit_tool(plugin = "polars_lazy", name = "select")]
async fn lf_select(
    ctx: Arc<PluginContext>,
    params: LfSelectParams,
) -> Result<CallToolResult, ErrorData> {
    let lfs = ctx.lazyframes.lock().unwrap();
    let lf = lfs.get(&params.handle_id)
        .ok_or_else(|| ErrorData::new("LazyFrame not found"))?;

    // params.exprs is Vec<Expr> deserialized from JSON
    let result = lf.clone().select(&params.exprs);

    let new_id = Uuid::new_v4();
    drop(lfs);
    ctx.lazyframes.lock().unwrap().insert(new_id, result);

    Ok(CallToolResult::success(json!({ "handle_id": new_id })))
}

#[elicit_tool(plugin = "polars_lazy", name = "collect")]
async fn lf_collect(
    ctx: Arc<PluginContext>,
    params: LfCollectParams,
) -> Result<CallToolResult, ErrorData> {
    let lfs = ctx.lazyframes.lock().unwrap();
    let lf = lfs.get(&params.handle_id)
        .ok_or_else(|| ErrorData::new("LazyFrame not found"))?;

    let df = lf.clone().collect()?;
    drop(lfs);

    // Register in DataFrame plugin
    let df_plugin = /* get DF plugin from context */;
    let df_id = Uuid::new_v4();
    df_plugin.dataframes.lock().unwrap().insert(df_id, df);

    Ok(CallToolResult::success(json!({
        "handle_id": df_id,
        "type": "dataframe"
    })))
}
```

**Operations (Phase 2):**
| Tool | Params | Returns |
|---|---|---|
| `lf_from_df` | `df_handle_id` | `{ handle_id }` |
| `lf_scan_csv` | `path, options` | `{ handle_id }` |
| `lf_scan_parquet` | `path, options` | `{ handle_id }` |
| `lf_scan_ipc` | `path, options` | `{ handle_id }` |
| `lf_select` | `handle_id, exprs: Vec<Expr>` | `{ handle_id }` |
| `lf_with_column` | `handle_id, expr: Expr` | `{ handle_id }` |
| `lf_with_columns` | `handle_id, exprs: Vec<Expr>` | `{ handle_id }` |
| `lf_filter` | `handle_id, predicate: Expr` | `{ handle_id }` |
| `lf_group_by` | `handle_id, by: Vec<Expr>, agg: Vec<Expr>` | `{ handle_id }` |
| `lf_join` | `left_id, right_id, left_on, right_on, how` | `{ handle_id }` |
| `lf_sort` | `handle_id, by: Vec<Expr>, descending` | `{ handle_id }` |
| `lf_limit` | `handle_id, n` | `{ handle_id }` |
| `lf_slice` | `handle_id, offset, length` | `{ handle_id }` |
| `lf_unique` | `handle_id, subset, keep` | `{ handle_id }` |
| `lf_drop_nulls` | `handle_id, subset` | `{ handle_id }` |
| `lf_explode` | `handle_id, columns` | `{ handle_id }` |
| `lf_with_streaming` | `handle_id, enabled: bool` | `{ handle_id }` |
| `lf_explain` | `handle_id, optimized: bool` | `{ plan: String }` |
| `lf_collect` | `handle_id` | `{ handle_id }` (DataFrame) |

---

### Phase 3: Expression Builder

**Expr Serialization:**

Polars' `Expr` already derives `Serialize`/`Deserialize`. We just need helper functions for construction:

```rust
#[elicit_tool(plugin = "polars_expr", name = "col")]
async fn expr_col(params: ExprColParams) -> Result<CallToolResult, ErrorData> {
    let expr = col(&params.name);
    let json = serde_json::to_value(&expr)?;
    Ok(CallToolResult::success(json))
}

#[elicit_tool(plugin = "polars_expr", name = "lit")]
async fn expr_lit(params: ExprLitParams) -> Result<CallToolResult, ErrorData> {
    let expr = lit(params.value); // value is serde_json::Value
    let json = serde_json::to_value(&expr)?;
    Ok(CallToolResult::success(json))
}

#[elicit_tool(plugin = "polars_expr", name = "binary_op")]
async fn expr_binary_op(params: ExprBinaryParams) -> Result<CallToolResult, ErrorData> {
    let left: Expr = serde_json::from_value(params.left)?;
    let right: Expr = serde_json::from_value(params.right)?;

    let expr = match params.op.as_str() {
        "add" => left + right,
        "sub" => left - right,
        "mul" => left * right,
        "div" => left / right,
        "gt" => left.gt(right),
        "lt" => left.lt(right),
        "eq" => left.eq(right),
        "and" => left.and(right),
        "or" => left.or(right),
        _ => return Err(ErrorData::new("Unknown operator")),
    };

    let json = serde_json::to_value(&expr)?;
    Ok(CallToolResult::success(json))
}

#[elicit_tool(plugin = "polars_expr", name = "agg")]
async fn expr_agg(params: ExprAggParams) -> Result<CallToolResult, ErrorData> {
    let input: Expr = serde_json::from_value(params.input)?;

    let expr = match params.agg_type.as_str() {
        "sum" => input.sum(),
        "mean" => input.mean(),
        "median" => input.median(),
        "min" => input.min(),
        "max" => input.max(),
        "std" => input.std(params.ddof.unwrap_or(1)),
        "var" => input.var(params.ddof.unwrap_or(1)),
        "count" => input.count(),
        "first" => input.first(),
        "last" => input.last(),
        "n_unique" => input.n_unique(),
        _ => return Err(ErrorData::new("Unknown aggregation")),
    };

    let json = serde_json::to_value(&expr)?;
    Ok(CallToolResult::success(json))
}

#[elicit_tool(plugin = "polars_expr", name = "alias")]
async fn expr_alias(params: ExprAliasParams) -> Result<CallToolResult, ErrorData> {
    let expr: Expr = serde_json::from_value(params.expr)?;
    let aliased = expr.alias(&params.name);
    let json = serde_json::to_value(&aliased)?;
    Ok(CallToolResult::success(json))
}

#[elicit_tool(plugin = "polars_expr", name = "when_then_otherwise")]
async fn expr_when_then_otherwise(
    params: ExprConditionalParams
) -> Result<CallToolResult, ErrorData> {
    let condition: Expr = serde_json::from_value(params.condition)?;
    let then_val: Expr = serde_json::from_value(params.then_value)?;
    let otherwise_val: Expr = serde_json::from_value(params.otherwise_value)?;

    let expr = when(condition).then(then_val).otherwise(otherwise_val);
    let json = serde_json::to_value(&expr)?;
    Ok(CallToolResult::success(json))
}
```

**Expression Namespace Tools:**
- String ops: `expr.str().len()`, `expr.str().contains()`, etc.
- Temporal ops: `expr.dt().year()`, `expr.dt().month()`, etc.
- List ops: `expr.list().len()`, `expr.list().get()`, etc.
- Math ops: `expr.abs()`, `expr.sqrt()`, `expr.round()`, etc.

Each namespace gets its own set of tools.

---

### Phase 4: SQL Interface

**PolarsSqlPlugin:**
```rust
#[derive(Clone, ElicitPlugin)]
pub struct PolarsSqlPlugin(Arc<PluginContext>);

struct PluginContext {
    contexts: Mutex<HashMap<Uuid, SQLContext>>,
}

#[elicit_tool(plugin = "polars_sql", name = "context_new")]
async fn sql_context_new(
    ctx: Arc<PluginContext>,
    _: EmptyParams,
) -> Result<CallToolResult, ErrorData> {
    let sql_ctx = SQLContext::new();
    let id = Uuid::new_v4();
    ctx.contexts.lock().unwrap().insert(id, sql_ctx);
    Ok(CallToolResult::success(json!({ "context_id": id })))
}

#[elicit_tool(plugin = "polars_sql", name = "register_df")]
async fn sql_register_df(
    ctx: Arc<PluginContext>,
    params: SqlRegisterParams,
) -> Result<CallToolResult, ErrorData> {
    let mut contexts = ctx.contexts.lock().unwrap();
    let sql_ctx = contexts.get_mut(&params.context_id)
        .ok_or_else(|| ErrorData::new("SQL context not found"))?;

    // Get DataFrame from DF plugin
    let df = /* fetch from DF plugin via params.df_handle_id */;

    sql_ctx.register(&params.table_name, df.lazy());

    Ok(CallToolResult::success(json!({ "registered": true })))
}

#[elicit_tool(plugin = "polars_sql", name = "execute")]
async fn sql_execute(
    ctx: Arc<PluginContext>,
    params: SqlExecuteParams,
) -> Result<CallToolResult, ErrorData> {
    let contexts = ctx.contexts.lock().unwrap();
    let sql_ctx = contexts.get(&params.context_id)
        .ok_or_else(|| ErrorData::new("SQL context not found"))?;

    let lf = sql_ctx.execute(&params.query)?;

    // Register LazyFrame in LF plugin
    let lf_id = Uuid::new_v4();
    // ... insert into LF plugin

    Ok(CallToolResult::success(json!({
        "handle_id": lf_id,
        "type": "lazyframe"
    })))
}
```

**SQL Tools:**
| Tool | Params | Returns |
|---|---|---|
| `sql_context_new` | — | `{ context_id }` |
| `sql_register_df` | `context_id, df_handle_id, table_name` | `{ registered }` |
| `sql_execute` | `context_id, query: String` | `{ handle_id }` (LazyFrame) |

---

## Phase 5: Data Types & Schema

**Mirror polars DataType as serializable enum:**

```rust
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum DataTypeKind {
    // Numeric
    Int8, Int16, Int32, Int64, Int128,
    UInt8, UInt16, UInt32, UInt64, UInt128,
    Float32, Float64,

    // Boolean
    Boolean,

    // String
    String,
    Binary,

    // Temporal
    Date,
    Datetime { time_unit: TimeUnitKind, time_zone: Option<String> },
    Duration { time_unit: TimeUnitKind },
    Time,

    // Nested
    List(Box<DataTypeKind>),
    Struct(Vec<FieldKind>),

    // Special
    Null,
}

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum TimeUnitKind {
    Nanoseconds,
    Microseconds,
    Milliseconds,
}

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub struct FieldKind {
    pub name: String,
    pub dtype: DataTypeKind,
}
```

**Tools:**
| Tool | Params | Returns |
|---|---|---|
| `df_dtypes` | `handle_id` | `{ dtypes: [DataTypeKind] }` |
| `df_schema` | `handle_id` | `{ fields: [FieldKind] }` |

---

## What We're NOT Exposing

### 1. Closure-Based Operations
**Cannot cross MCP boundary:**
```rust
// ❌ Not exposed
df.apply("col", |series| series.pow(2))
expr.map(|column| custom_transform(column), output_type)
lf.map(|df| custom_function(df))
```

**Alternative:** Use built-in functions via `expr_function` tool with ~200 function names.

### 2. Object Columns
**Requires Rust trait impls:**
```rust
// ❌ Not exposed
df.with_object_column::<MyCustomType>()
```

**Alternative:** Use Struct columns or serialize to Binary/String.

### 3. FFI Plugins
**Dynamic library loading:**
```rust
// ❌ Not exposed
expr.plugin(library_path, symbol, args)
```

### 4. Direct Memory Access
**Unsafe/internal:**
```rust
// ❌ Not exposed
df.chunks_mut()
unsafe { series.get_unchecked(idx) }
```

### 5. Iterators with Lifetimes
**Non-'static references:**
```rust
// ❌ Not exposed
series.iter() // returns SeriesIter<'a>
```

**Alternative:** Export to JSON/Arrow IPC, iterate on client side.

---

## Propositions (Contract System)

```rust
pub struct DataFrameCreated;     // df_create_*, df_read_*
pub struct DataFrameWritten;     // df_write_*
pub struct LazyFrameCreated;     // lf_*, sql_execute
pub struct QueryCollected;       // lf_collect
pub struct SqlContextCreated;    // sql_context_new
pub struct TableRegistered;      // sql_register_df
```

---

## Implementation Order

### Phase 1: DataFrame Core (Week 1-2)
1. **Crate scaffold** - `Cargo.toml`, feature selection
2. **DataType mirrors** - `types.rs` with Elicit derives
3. **DataFrame plugin** - UUID registry, basic ops
4. **I/O operations** - CSV/Parquet read/write
5. **Tests** - Round-trip serialization, basic transforms

### Phase 2: LazyFrame Builder (Week 2-3)
1. **LazyFrame plugin** - UUID registry
2. **Scan operations** - `lf_scan_csv`, `lf_scan_parquet`
3. **Transform operations** - select, filter, with_column
4. **Group/Join operations** - group_by, join variants
5. **Collect/Explain** - lf_collect, lf_explain
6. **Tests** - Query building, optimization flags

### Phase 3: Expression DSL (Week 3-4)
1. **Basic constructors** - col, lit
2. **Binary operations** - arithmetic, comparison, logic
3. **Aggregations** - sum, mean, min, max, etc.
4. **Conditional** - when/then/otherwise
5. **String namespace** - str() methods
6. **Temporal namespace** - dt() methods
7. **Tests** - Expression composition, serialization

### Phase 4: SQL Interface (Week 4)
1. **SQL plugin** - Context registry
2. **Register/Execute** - table registration, query execution
3. **Integration tests** - SQL → LazyFrame → DataFrame

### Phase 5: Integration & Documentation (Week 5)
1. **Cross-plugin coordination** - DF ↔ LF ↔ SQL
2. **End-to-end workflows** - Complete data pipelines
3. **Performance tests** - Large dataset handling
4. **Documentation** - README, examples, cookbook
5. **Wire into elicit_server emit chain**

---

## Dependencies & Feature Selection

**Cargo.toml:**
```toml
[dependencies]
polars = { version = "0.53", features = [
    # Core
    "lazy",           # LazyFrame
    "dtype-full",     # All data types
    "serde",          # DataFrame/Series serialization
    "serde-lazy",     # LazyFrame plan serialization

    # I/O
    "csv",
    "json",
    "parquet",
    "ipc",

    # SQL
    "sql",

    # Operations
    "strings",        # String methods
    "temporal",       # Date/time operations
    "describe",       # df.describe()
    "pivot",
    "rank",
    "diff",
    "cum_agg",        # Cumulative aggregations
    "rolling_window",

    # Joins
    "cross_join",
    "asof_join",
    "semi_anti_join",

    # Performance
    "performant",
    "streaming",
] }

elicitation = { workspace = true, features = ["emit"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
tokio = { workspace = true }
```

**Features to AVOID:**
- `object` - Requires closures
- `ffi_plugin` - FFI not viable over JSON
- `python` - Python integration not needed

---

## Example Agent Workflow

**Scenario:** Load CSV, filter, aggregate, export

```json
// 1. Load CSV lazily
{
  "tool": "lf_scan_csv",
  "params": {
    "path": "/data/sales.csv",
    "options": {
      "has_header": true,
      "infer_schema_length": 1000
    }
  }
}
// → { "handle_id": "lf_uuid_1" }

// 2. Build filter expression: revenue > 1000
{
  "tool": "expr_binary_op",
  "params": {
    "left": { "Column": "revenue" },
    "op": "gt",
    "right": { "Literal": { "Int64": 1000 } }
  }
}
// → { "expr": { "BinaryExpr": {...} } }

// 3. Apply filter
{
  "tool": "lf_filter",
  "params": {
    "handle_id": "lf_uuid_1",
    "predicate": { "BinaryExpr": {...} }
  }
}
// → { "handle_id": "lf_uuid_2" }

// 4. Build aggregation: sum(revenue).alias("total")
{
  "tool": "expr_agg",
  "params": {
    "input": { "Column": "revenue" },
    "agg_type": "sum"
  }
}
// → { "expr": { "Agg": { "Sum": [...] } } }

{
  "tool": "expr_alias",
  "params": {
    "expr": { "Agg": {...} },
    "name": "total"
  }
}
// → { "expr": { "Alias": [...] } }

// 5. Group by category
{
  "tool": "lf_group_by",
  "params": {
    "handle_id": "lf_uuid_2",
    "by": [{ "Column": "category" }],
    "agg": [{ "Alias": [...] }]
  }
}
// → { "handle_id": "lf_uuid_3" }

// 6. Collect to DataFrame
{
  "tool": "lf_collect",
  "params": {
    "handle_id": "lf_uuid_3"
  }
}
// → { "handle_id": "df_uuid_1", "type": "dataframe" }

// 7. Export to Parquet
{
  "tool": "df_write_parquet",
  "params": {
    "handle_id": "df_uuid_1",
    "path": "/output/results.parquet",
    "options": {
      "compression": "snappy"
    }
  }
}
// → { "bytes_written": 12345 }
```

---

## Success Metrics

1. **API Coverage:** 70-80% of polars operations exposed
2. **Serialization:** All Expr, DataFrame, LazyFrame serializable
3. **Performance:** Lazy evaluation preserves polars' zero-copy optimizations
4. **Workflows:** Complete data pipelines buildable via MCP tools
5. **Documentation:** Cookbook with 10+ real-world examples
6. **Testing:** >85% code coverage, integration tests for all plugins

---

## Estimated Effort

- **Phase 1 (DataFrame):** 2 weeks - ~40 tools
- **Phase 2 (LazyFrame):** 1.5 weeks - ~25 tools
- **Phase 3 (Expr DSL):** 1.5 weeks - ~30 tools
- **Phase 4 (SQL):** 0.5 weeks - ~5 tools
- **Phase 5 (Integration):** 1 week - documentation, testing

**Total:** ~6 weeks, ~100 MCP tools

---

## Key Advantages Over Other Shadow Crates

1. **Built-in Serialization:** Polars was designed for serde, unlike tokio
2. **AST-Based Expr:** Expressions are data, not closures
3. **SQL Escape Hatch:** High-level abstraction for complex queries
4. **Lazy Optimization:** Query optimization happens automatically
5. **Arrow Integration:** Efficient data transfer via Arrow IPC format

---

## Conclusion

Polars is **exceptionally well-suited** for MCP integration:
- ✅ ~200 built-in functions eliminate need for custom closures
- ✅ Expr DSL is fully serializable (JSON-based AST)
- ✅ SQL interface provides declarative query building
- ✅ LazyFrame enables optimization without code generation
- ✅ Comprehensive I/O support (CSV, Parquet, JSON, IPC)

**Result:** Agents can build production-grade data pipelines entirely through MCP tools, with the full power of polars' query optimizer and execution engine.
