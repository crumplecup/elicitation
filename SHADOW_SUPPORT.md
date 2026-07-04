# Shadow Crate Support Patterns

This document distils the four core implementation patterns used when writing
`elicit_*` shadow crates.  Read `SHADOW_CRATE_MOTIVATION.md` first for *why*
shadow crates exist; read this document for *how* to implement them correctly.

---

## Quick-reference: which tool do I reach for?

| Situation | Pattern / Macro |
|---|---|
| Upstream type lacks `Serialize`/`JsonSchema` (orphan rule) | **Trenchcoat** → `elicit_newtype!` |
| Reflecting a wrapped type's own methods into MCP tools | **`#[reflect_methods]`** (full-featured) or **`elicit_newtype_methods!`** (simple cases) |
| Stateless plugin with multiple tools | **`#[reflect_methods]`** on `impl PluginStruct` |
| Stateful plugin tool (needs `Arc<Ctx>`) | **`#[elicit_tool]`** free function |
| Free function or constructor tool | **`#[elicit_tool]`** |
| Adding comparison/ordering traits to a newtype | **`elicit_newtype_traits!`** |
| Generic type `Foo<T>` where T is chosen by the caller | **Factory pattern** |
| Tool needs live objects that survive across calls | **Stateful plugin** — `(Arc<Ctx>)` newtype + `#[derive(ElicitPlugin)]` |
| Plugin with no inter-call state | **Stateless plugin** — unit struct + `#[derive(ElicitPlugin)]` |
| Adding `ElicitComplete` to a new upstream type | **Compiler-guided** — stub `impl ElicitComplete for X {}`, fix errors |
| Enforcing tool call ordering (parse before inspect, etc.) | **`Prop`/`Established` contracts** |

---

## 1. Trenchcoat Pattern

**Problem:** The orphan rule blocks `impl schemars::JsonSchema for upstream::Type`
(or `Serialize`, `Deserialize`) when neither the trait nor the type are yours.
Without `JsonSchema`, the type cannot cross the MCP boundary at all.

**Solution:** Wrap the upstream type in a local newtype ("trenchcoat").  The
wrapper is yours, so you can implement any trait on it.  `From`/`Into` provide
a lossless bridge back to the original.

### Minimal hand-written trenchcoat

```rust
// crates/elicitation/src/primitives/proj_types/area.rs

#[derive(Debug, Clone, Copy, PartialEq,
         serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ProjArea {
    pub west: f64,
    pub south: f64,
    pub east: f64,
    pub north: f64,
}

// Lossless bridge ↔ upstream type
impl From<proj::Area> for ProjArea {
    fn from(v: proj::Area) -> Self {
        Self { west: v.west, south: v.south, east: v.east, north: v.north }
    }
}
impl From<ProjArea> for proj::Area {
    fn from(v: ProjArea) -> Self {
        Self::new(v.west, v.south, v.east, v.north)
    }
}
```

### Macro-generated trenchcoat (`elicit_newtype!`)

For cases where you need `Deref`/`DerefMut` access and `Arc`-backed cloning:

```rust
use elicitation::elicit_newtype;

// Arc-backed, Clone always works, Deref gives &reqwest::Client
elicit_newtype!(reqwest::Client, as Client);
```

This generates: newtype struct around `Arc<T>`, `From<T>`, `From<Arc<T>>`,
`Deref`, `DerefMut`, conditional forwarding for `PartialEq`/`Hash`/`Display`.

### When to use hand-written vs `elicit_newtype!`

- **Hand-written:** upstream type is `Copy` or has custom field mapping (like
  `ProjArea`, `RstarAabb`), or you need `Serialize`/`JsonSchema` on the struct
  fields themselves.
- **`elicit_newtype!`:** upstream type needs to be opaque and Clone-able, you
  want transparent `Deref` access, and fields don't need individual serialization.

### Key rule

Every trenchcoat must implement `ElicitComplete` (which requires `Debug +
Clone + Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static`).
This is the gate that allows the type to appear in tool parameters and results.

---

## 2. Factory Pattern

**Problem:** A library has a generic type `Tree<T>` or `Table<K, V>`.  You
cannot make a single MCP tool that works for all `T` — the JSON schema for
parameters is different for every concrete instantiation, and `T` may not
be known at crate-compile time.

**Solution:** A *factory* is a struct that, given a concrete `T` chosen by the
caller at runtime, produces a complete set of typed tools specialized to that
`T`.  The factory is registered with `DynamicToolRegistry`, which calls it once
per `T` and installs the resulting tools under a name prefix.

### Shape

```rust
// elicit_rstar/src/factory.rs

pub struct RTreeObjectFactory;   // marker — carries no state

impl RTreeObjectFactory {
    /// Produce tools for a concrete T at registry time.
    pub fn prime<T>(&self, prefix: &str) -> Vec<DynamicToolDescriptor>
    where
        T: ElicitComplete + RTreeObject<Envelope = AABB<[f64; 2]>>,
    {
        // One DynamicToolDescriptor per tool name, each closes over T
        vec![
            descriptor_for_new::<T>(prefix),
            descriptor_for_insert::<T>(prefix),
            // ...
        ]
    }
}
```

### Registration

```rust
use elicitation::DynamicToolRegistry;
use elicit_rstar::{RTreeObjectFactory, prime_rtree_object_tree};
use elicitation::RstarRectangle;

let registry = DynamicToolRegistry::new()
    .with_factory("rtree_rect", prime_rtree_object_tree::<RstarRectangle>);
```

Calling `.instantiate("rtree_rect")` replaces *all* tools under that prefix
with tools freshly specialized to `RstarRectangle`.

### When to hand-expand instead

For a small, fixed number of `(K, V)` combinations (e.g. `elicit_redb` tables
with `u64/u64`, `&str/&str`, `&[u8]/&[u8]`), writing them out explicitly with
`#[elicit_tool]` is simpler and avoids the `DynamicToolRegistry` overhead.
Use the factory when the set of types is open-ended or user-supplied.

### Constraint

`#[elicit_tool(name = concat!(...))` does **not** work — proc macros receive
raw token streams before `concat!` expands.  Tool names must be string literals.
The factory pattern sidesteps this by building tool names programmatically in
Rust at registration time (not in attributes).

---

## 3. Stateful Plugin

**Problem:** Some library APIs are inherently stateful — `Database::open`
returns a `Database` that must be passed to `begin_write`, which returns a
`WriteTransaction` that must be passed to `open_table`, and so on.  MCP tools
are stateless function calls; they cannot hold a `WriteTransaction` between
invocations.

**Solution:** A `StatefulPlugin` carries a shared `Context` (wrapped in
`Arc<Mutex<…>>` maps) that lives for the lifetime of the MCP server session.
Each tool call receives an `Arc<Context>` and looks up live objects by UUID.

### Context structure

```rust
// crates/elicit_redb/src/plugin.rs

pub struct RedbCtx {
    pub(crate) databases:  Mutex<HashMap<Uuid, redb::Database>>,
    pub(crate) write_txns: Mutex<HashMap<Uuid, redb::WriteTransaction>>,
    pub(crate) read_txns:  Mutex<HashMap<Uuid, redb::ReadTransaction>>,
    pub(crate) savepoints: Mutex<HashMap<Uuid, redb::Savepoint>>,
}

impl PluginContext for RedbCtx {}
```

Types that don't implement `Debug` require a manual `impl std::fmt::Debug` —
show counts, not contents:

```rust
impl std::fmt::Debug for RedbCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedbCtx")
            .field("databases", &self.databases.lock().map(|m| m.len()).unwrap_or(0))
            .field("write_txns", &self.write_txns.lock().map(|m| m.len()).unwrap_or(0))
            .finish()
    }
}
```

### StatefulPlugin impl

```rust
pub struct RedbPlugin(Arc<RedbCtx>);

impl StatefulPlugin for RedbPlugin {
    type Context = RedbCtx;

    fn name(&self) -> &'static str { "redb" }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "redb")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "redb")
            .map(|r| (r.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<RedbCtx> { self.0.clone() }
}
```

### Tool functions for a stateful plugin

Tool functions for a `StatefulPlugin` take `Arc<Context>` as first argument
and are annotated with `#[elicit_tool(plugin = "name")]` — exactly like free
functions:

```rust
#[elicit_tool(
    plugin = "redb",
    name = "database_begin_write",
    description = "Begin a write transaction. Returns a txn_id UUID.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn database_begin_write(
    ctx: Arc<RedbCtx>,
    p: DatabaseBeginParams,
) -> Result<CallToolResult, ErrorData> {
    let db_id = parse_uuid(&p.db_id)?;
    let txn = {
        let dbs = ctx.lock_databases()?;
        let db = dbs.get(&db_id)
            .ok_or_else(|| ErrorData::invalid_params("unknown db_id", None))?;
        db.begin_write()
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?
    };
    let txn_id = Uuid::new_v4();
    ctx.lock_write_txns()?.insert(txn_id, txn);
    ok_json(&IdResult { id: txn_id.to_string() })
}
```

### Lock ordering

Always acquire locks in a consistent order to prevent deadlocks.  The
`elicit_redb` convention is: `savepoints` → `write_txns` (when holding both
simultaneously, e.g. `restore_savepoint`).  Document your ordering in the
plugin's module comment.

---

## 4. Macro Cheat-Sheet: which one for each situation

### `#[elicit_tool]` — the universal tool wrapper

**Use for:** free functions, constructors, and instance methods on
`StatefulPlugin` tool modules.

```rust
#[elicit_tool(
    plugin = "redb",          // plugin name (omit for standalone tools)
    name = "database_create", // tool name (no plugin prefix)
    description = "…",
    emit = None               // None = live operation; MyEmitType = code generator
)]
async fn database_create(ctx: Arc<RedbCtx>, p: DatabaseCreateParams)
    -> Result<CallToolResult, ErrorData> { … }
```

- `emit = None` → the function *performs* the operation (shadow crate style)
- `emit = MyType` → the function *emits Rust code* as a string (code-gen style)
- Works on `async fn` and plain `fn`
- Registered via `inventory::submit!` at link time
- **Cannot** use `concat!()` in the `name = …` attribute

### `elicit_newtype!` — newtype generation

**Use for:** wrapping upstream types that need `Serialize`/`JsonSchema` but
you don't need explicit field layout.

```rust
elicit_newtype!(reqwest::Client, as Client);
// ↓ generates Arc-backed newtype with Deref, From impls
```

Variants:

- `elicit_newtype!(T, as W)` — Arc-backed, no serde derives on wrapper
- `elicit_newtype!(T, as W, serde)` — adds Serialize/Deserialize (requires T: Serde)
- `elicit_newtype!(T, as W, schema)` — adds JsonSchema
- `elicit_newtype!(T, as W, serde, schema)` — full ElicitComplete support

### `elicit_newtype_methods!` — newtype + method delegation in one macro

**Use for:** simple types where every method maps directly to the inner type
with no generic parameters.

```rust
elicit_newtype_methods! {
    MyClient => reqwest::Client,
    fn get(url: &str) -> Result<Response, Error>;
    async fn post(url: &str, body: Vec<u8>) -> Result<Response, Error>;
}
// Generates: newtype, delegating methods, params structs, MCP tool wrappers
```

**Limitation:** does not support generic methods.  Use `#[reflect_methods]`
instead.

### `#[reflect_methods]` — full method reflection via proc macro

**Use for:** reflecting a wrapped type's `impl` block into MCP tools, including
methods with generic type parameters.  Also used directly on stateless plugin
structs to register multiple tool methods in one block.

#### What `#[reflect_methods]` generates

Given:

```rust
#[reflect_methods]
impl MyString {
    pub fn repeat(&self, n: usize) -> String { self.0.repeat(n) }
    pub async fn fetch(&self, url: String) -> Result<String, ErrorData> { … }
}
```

The macro outputs **three things** beside the original impl block:

1. **A param struct** per method that has non-`&self` parameters, named
   `{PascalCase(method_name)}Params`:

   ```rust
   // generated — do NOT define this yourself
   #[derive(Debug, Clone, Elicit, JsonSchema, Serialize, Deserialize)]
   pub struct RepeatParams { pub n: usize }

   #[derive(Debug, Clone, Elicit, JsonSchema, Serialize, Deserialize)]
   pub struct FetchParams { pub url: String }
   ```

2. **Wrapper methods** with a `_tool` suffix that accept
   `Parameters<XxxParams>` and return `Result<Json<T>, ErrorData>`:

   ```rust
   // generated
   #[::rmcp::tool(description = "repeat operation")]
   pub fn repeat_tool(&self, params: Parameters<RepeatParams>)
       -> Result<Json<String>, ErrorData> { … }

   #[::rmcp::tool(description = "fetch operation")]
   pub async fn fetch_tool(&self, params: Parameters<FetchParams>)
       -> Result<Json<String>, ErrorData> { … }
   ```

3. Tool descriptions default to `"{snake method name} operation"`.  There is
   **no** mechanism to customise the description via an attribute on the
   original method — put description text in the doc comment instead.

#### Rules when writing methods for `#[reflect_methods]`

- **Individual named parameters only** — not a pre-made params struct.
  The macro reads each parameter's name and type and builds the struct for you.

  ```rust
  // ✅ Individual params — macro generates AssertInRangeParams { datetime, start, end }
  pub async fn assert_in_range(&self, datetime: String, start: String, end: String)

  // ❌ Pre-made struct — macro tries to re-generate AssertInRangeParams, causing
  //    "defined multiple times" compile errors
  pub async fn assert_in_range(&self, p: AssertInRangeParams)
  ```

- **Return `Result<T, ErrorData>` where `T: Serialize`**, not
  `Result<CallToolResult, ErrorData>`.  `CallToolResult` is itself a structured
  MCP response object; wrapping it in `Json<…>` double-nests the content.

  ```rust
  // ✅ Plain return value — macro wraps in Json<String>
  pub async fn parse_datetime(&self, datetime: String) -> Result<String, ErrorData>

  // ❌ CallToolResult — gets wrapped in Json<CallToolResult>, breaking the schema
  pub async fn parse_datetime(&self, datetime: String) -> Result<CallToolResult, ErrorData>
  ```

- **Do not put `#[tool(…)]` on the original method** — the `#[tool]` attribute
  belongs on the generated `_tool` wrapper, not the original method.  If you
  add it to the original, rustc will complain "cannot find attribute `tool` in
  this scope".

- **Do not import `schemars::JsonSchema` or `serde::Deserialize`** just for the
  param structs — the macro generates those derives on your behalf.

#### Using `#[reflect_methods]` on a stateless plugin

A stateless plugin (no shared state between calls) can replace a collection of
`#[elicit_tool]` free functions with a single `#[reflect_methods] impl` block:

```rust
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow};
use elicitation_derive::reflect_methods;
use rmcp::ErrorData;

#[derive(Debug, ElicitPlugin)]
#[plugin(name = "chrono_workflow")]
pub struct ChronoWorkflowPlugin;

#[reflect_methods]
impl ChronoWorkflowPlugin {
    /// Parse an RFC 3339 datetime string. Establishes: DateTimeParsed.
    #[instrument(skip_all)]
    pub async fn parse_datetime(&self, datetime: String) -> Result<String, ErrorData> {
        // … returns a human-readable summary string
    }

    /// Add seconds to a datetime. Establishes: DateTimeParsed(result).
    #[instrument(skip_all)]
    pub async fn add_seconds(&self, datetime: String, seconds: i64) -> Result<String, ErrorData> {
        // …
    }
}
// → generates ParseDatetimeParams, AddSecondsParams, parse_datetime_tool, add_seconds_tool
```

`#[elicit_tool]` free functions are the right choice for **stateful** plugins
where each tool receives `Arc<Ctx>` — `reflect_methods` does not thread context.

#### Param struct naming

`reflect_methods` converts method names with `to_pascal_case` and appends
`Params`.  Each word (split on `_`) gets its first letter uppercased:

| method name       | generated struct name     |
|-------------------|---------------------------|
| `repeat`          | `RepeatParams`            |
| `assert_future`   | `AssertFutureParams`      |
| `parse_datetime`  | `ParseDatetimeParams`     |
| `assert_in_range` | `AssertInRangeParams`     |

Note: multi-word segments like `datetime` → `Datetime` (not `DateTime`).

### `elicit_newtype_traits!` — add comparison traits to newtypes

**Use for:** adding `PartialEq + Eq + Hash + PartialOrd + Ord` to a newtype
generated by `elicit_newtype!` with the `serde` flag.  The `serde` form of
`elicit_newtype!` does not derive these automatically; call
`elicit_newtype_traits!` separately.

```rust
use elicitation::elicit_newtype;

elicit_newtype!(chrono::NaiveDateTime, as NaiveDateTime, serde);
elicitation::elicit_newtype_traits!(NaiveDateTime, chrono::NaiveDateTime, [cmp]);
//                                                                          ^^^
//                                        [cmp] adds PartialEq+Eq+Hash+PartialOrd+Ord
//                                        all delegating through *self.0 (deref through Arc)
```

Shadow types should derive the same traits as their upstream types.  If the
upstream implements `PartialEq + Ord + Hash`, the shadow must too — otherwise
it silently fails to provide the same guarantees.

```rust
// ✅ Shadow matches upstream's guarantees
elicit_newtype_traits!(NaiveDateTime, chrono::NaiveDateTime, [cmp]);

// For types that are manually written (not via elicit_newtype! serde form),
// add the derives directly on the struct:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DateTimeUtc(pub Arc<chrono::DateTime<chrono::Utc>>);
// Arc<T> delegates PartialEq/Ord/Hash to T by value, not by pointer — correct.
```

### Decision tree

```text
Does the upstream type need JsonSchema/Serialize?
  └─ Yes, it's an opaque handle → elicit_newtype!(T, as W)
  └─ Yes, fields need names      → hand-written trenchcoat struct

Does the shadow need PartialEq/Ord/Hash like upstream?
  └─ elicit_newtype! serde form → elicit_newtype_traits!(W, T, [cmp])
  └─ Manually written struct    → #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]

Do you need to expose the type's methods as MCP tools?
  └─ Simple, non-generic methods → elicit_newtype_methods! { W => T, fn … }
  └─ Generic methods, complex signatures → #[reflect_methods] on impl W { … }

Do you need a multi-tool plugin struct?
  └─ No shared state between calls (stateless) → #[reflect_methods] on impl PluginStruct
  └─ Shared handles/transactions between calls (stateful) → #[elicit_tool] free functions
     with Arc<Ctx> first param, + #[derive(ElicitPlugin)] on (Arc<Ctx>) newtype

Is the function a standalone tool, constructor, or stateful plugin tool?
  └─ #[elicit_tool(plugin = "…", name = "…", description = "…", emit = …)]

Is T generic and the concrete type chosen by the caller?
  └─ Factory pattern + DynamicToolRegistry
  └─ Small fixed set of T? → write out each #[elicit_tool] explicitly

Does the API require objects that outlive a single tool call?
  └─ StatefulPlugin with Arc<Mutex<HashMap<Uuid, LiveObject>>>
```

---

## Pattern interactions

These patterns compose naturally:

| Combination | Example |
|---|---|
| Trenchcoat + `#[elicit_tool]` | Wrap `rstar::AABB` as `RstarAabb`, expose `rtree_insert` tool that accepts `RstarAabb` |
| Trenchcoat + Factory | Wrap `proj::Area` as `ProjArea`; factory produces tools per `T: ElicitComplete` using `ProjArea` for coordinates |
| StatefulPlugin + `#[elicit_tool]` | `elicit_redb`: every tool is `#[elicit_tool]`, context holds live `Database`/`WriteTransaction` maps |
| Factory + StatefulPlugin | `elicit_rstar`: `RstarTree<T>` stored in context; factory determines which `T` to use |

---

## 5. Compiler-Guided `ElicitComplete`

**Pattern:** Start by writing empty `impl ElicitComplete for MyType {}` for
every type you want to support.  The compiler then enumerates every missing
supertrait obligation — `ElicitIntrospect`, `ElicitPromptTree`, `ElicitSpec`,
`ToCodeLiteral`, `Elicitation` — in a single error batch.  Work through them
one by one.

### Steps

```rust
// Step 1: stub impls in type_spec/csv_specs.rs (or wherever)
#[cfg(feature = "csv-types")]
mod csv_impls {
    use crate::{CsvPosition, CsvQuoteStyle, ElicitComplete};

    impl ElicitComplete for CsvPosition {}   // ← compiler will list what's missing
    impl ElicitComplete for CsvQuoteStyle {}
}
```

```bash
cargo check -p elicitation --features csv-types
```

The compiler responds with one error per unsatisfied supertrait per type.
Implement them in the order it lists them — typically:

1. `Prompt` (required by `Elicitation`)
2. `crate::default_style!(MyType => MyTypeStyle)` (required by `Elicitation`)
3. `Elicitation` — `type Style`, `async fn elicit`, three proof methods
4. `ElicitIntrospect` — `pattern()` + `metadata()`
5. `ElicitPromptTree` — `prompt_tree()`
6. `ToCodeLiteral` — `to_code_literal()`
7. `ElicitSpec` + `inventory::submit!` in `type_spec/`

### Pattern selection

| Type shape | `ElicitationPattern` | `prompt_tree` variant |
|---|---|---|
| Struct with named fields | `Survey` | `PromptTree::Survey { fields: … }` |
| Enum with unit variants only | `Select` | `PromptTree::Select { options: … }` |
| Enum with data variants | `Select` | `PromptTree::Select { branches: … }` — sub-trees for data variants |

### Feature gate wiring

Add the feature-gated module to `type_spec/mod.rs`:

```rust
#[cfg(feature = "csv-types")]
mod csv_specs;
```

And confirm the feature is declared in `Cargo.toml`:

```toml
[features]
csv-types = ["dep:csv"]
```

---

## 6. `#[derive(ElicitPlugin)]` — Stateless vs Stateful

There are two forms depending on whether the plugin needs to hold live objects
between tool calls.

### Stateless — unit struct

When all tools are pure functions (no handles, builders, or readers to track
between calls), the plugin is a unit struct:

```rust
// elicit_wkb: parsing WKB is stateless — each tool receives its full input
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wkb_reader")]
pub struct WkbReaderPlugin;

impl WkbReaderPlugin {
    pub fn new() -> Self { Self }
}
impl Default for WkbReaderPlugin {
    fn default() -> Self { Self::new() }
}
```

Tools for a stateless plugin take only a params struct — no `ctx` argument:

```rust
#[elicit_tool(plugin = "wkb_reader", name = "read_wkb", description = "…")]
async fn read_wkb_tool(p: ReadWkbParams) -> Result<CallToolResult, ErrorData> { … }
```

### Stateful — `(Arc<Ctx>)` newtype

When tools produce objects that must survive across calls (readers, writers,
transactions, connections…), wrap an `Arc<Ctx>` in the plugin newtype:

```rust
pub struct CsvCtx {
    pub mem_readers: Mutex<HashMap<Uuid, csv::Reader<Cursor<Vec<u8>>>>>,
    pub mem_writers: Mutex<HashMap<Uuid, csv::Writer<Vec<u8>>>>,
    // …
}
impl PluginContext for CsvCtx {}

#[derive(ElicitPlugin)]
#[plugin(name = "csv")]
pub struct CsvPlugin(pub Arc<CsvCtx>);

impl CsvPlugin {
    pub fn new() -> Self { CsvPlugin(Arc::new(CsvCtx::default())) }
}
```

Tools for a stateful plugin take `Arc<Ctx>` as first param named `ctx`:

```rust
#[elicit_tool(plugin = "csv", name = "csv__reader__next_record", description = "…")]
async fn reader_next_record(ctx: Arc<CsvCtx>, p: ReaderNextRecordParams)
    -> Result<CallToolResult, ErrorData> { … }
```

The `#[derive(ElicitPlugin)]` macro detects the `ctx` param by name and routes
context injection automatically.

### The old manual `impl StatefulPlugin` — do not use

Earlier code used `impl StatefulPlugin for … { fn context() … }`.  This is
superseded by `#[derive(ElicitPlugin)]` on the newtype.  Do not write new code
using the old style.

---

## 7. Tool Function Naming

**Rule:** Rust function names must be valid `snake_case`.  The MCP tool name
(which may contain double-underscores) belongs only in the `name = "…"` attribute.

```rust
// ✅ Correct — Rust name is plain snake_case
#[elicit_tool(plugin = "csv", name = "csv__reader__next_record", description = "…")]
async fn reader_next_record(ctx: Arc<CsvCtx>, p: Params)
    -> Result<CallToolResult, ErrorData> { … }

// ❌ Wrong — Rust name has double-underscores → 60+ snake_case warnings
#[elicit_tool(plugin = "csv", name = "csv__reader__next_record", description = "…")]
async fn csv__reader__next_record(ctx: Arc<CsvCtx>, p: Params)
    -> Result<CallToolResult, ErrorData> { … }
```

Convention for naming Rust functions when shadowing a crate:

| MCP tool name | Rust function name |
|---|---|
| `csv__reader_builder__from_path` | `reader_builder_from_path` |
| `redb__database__begin_write` | `database_begin_write` |
| `wkb_reader__read_wkb` | `read_wkb_tool` (suffix `_tool` when clashing with imports) |

---

## 8. `Prop` / `Established` Workflow Contracts

**Problem:** Tool A must be called before tool B (e.g., parse bytes before
inspecting the result), but nothing in the type system enforces this sequence.
Description strings are the only documentation, and LLMs can ignore them.

**Solution:** Use `#[derive(Prop)]` to declare typed propositions, and
`Established<P>::assert()` to witness that a precondition was met.  The type
system prevents constructing `Established<P>` without going through the code
that verifies the proposition.

### Declaring a proposition

```rust
use elicitation::{Prop, VerifiedWorkflow};

/// Proposition: WKB bytes were successfully parsed.
#[derive(Prop)]
pub struct WkbParsed;

impl VerifiedWorkflow for WkbParsed {}
```

### Establishing a proposition in a tool

```rust
use elicitation::contracts::Established;

#[elicit_tool(plugin = "wkb_reader", name = "read_wkb",
              description = "Parse WKB bytes. Establishes: WkbParsed.")]
async fn read_wkb_tool(p: ReadWkbParams) -> Result<CallToolResult, ErrorData> {
    let wkb = read_wkb(&p.bytes.bytes)
        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
    let _proof = Established::<WkbParsed>::assert();  // ← witness: parse succeeded
    json_result(&wkb)
}
```

### Using `proof_credential!` for stronger guarantees

When the proposition requires a runtime check (not just "this code ran"),
use `proof_credential!` to restrict who can mint the proof:

```rust
use elicitation::{proof_credential, contracts::{Established, ProvableFrom}};

#[derive(Prop)]
pub struct ContrastChecked;

proof_credential! {
    pub(crate) NormalContrastVerified => ContrastChecked;
}

// Only code inside this crate can call Established::prove(&NormalContrastVerified)
let proof = Established::prove(&NormalContrastVerified);
```

`Established<P>` is a zero-sized type — no runtime cost.

### When to use contracts

- Multi-step workflows where ordering matters (parse → inspect → transform)
- Tools that produce a resource ID that must be passed to subsequent tools
- Documenting postconditions in a machine-readable way (proofs frameworks)

---

## Common pitfalls

| Pitfall | Fix |
|---|---|
| `#[derive(Debug)]` fails because upstream type doesn't impl `Debug` | Implement `Debug` manually, show only counts/metadata |
| `concat!()` in `#[elicit_tool(name = …)]` | Proc macros see raw tokens; use literal strings or the factory pattern |
| Calling a trait method without importing the trait | `use redb::ReadableDatabase as _;` etc. — wildcard-import the trait |
| Double-lock deadlock (savepoints then write_txns, or vice versa) | Establish and document a global lock acquisition order; always follow it |
| Storing `&'static str` or `&'static [u8]` in tool params | Borrow from `p.field` at call time; the params outlive the table operation |
| `persistent_savepoint()` returns `u64`, not `Savepoint` | Call `txn.get_persistent_savepoint(id)` immediately to get the `Savepoint` struct |
| Two `#[elicit_tool]` functions share the same params struct | Each tool must have a **unique** params struct; sharing one triggers conflicting generated `EmitCode` impls |
| Pre-defining `*Params` structs for `#[reflect_methods]` methods | `reflect_methods` generates `{PascalCase}Params` for you; manually defining the same name causes "defined multiple times" errors |
| Using `Result<CallToolResult, ErrorData>` as `#[reflect_methods]` return type | The macro wraps the `Ok` value in `Json<T>`, so `Json<CallToolResult>` double-nests the response; return `Result<String, ErrorData>` (or any serializable `T`) instead |
| Putting `#[tool(description = "…")]` on a method inside `#[reflect_methods]` | `reflect_methods` does not consume this attribute; rustc sees an unknown `tool` attribute and errors; put the description in the `///` doc comment instead |
| Using a single `p: SomeParams` parameter with `#[reflect_methods]` | The macro generates a new struct from the method's individual parameters; passing a pre-built struct conflicts with the generated one; use individual named params (`datetime: String, seconds: i64`) |
| Shadow type missing comparison traits present on the upstream | Each shadow must provide the same guarantees as upstream; use `elicit_newtype_traits!(W, T, [cmp])` for `elicit_newtype!` newtypes, or `#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]` on hand-written structs |
| Reading the `reflect_methods` source before using it | **Always read `elicitation_derive/src/method_reflection/`** before attempting a `reflect_methods` impl; the macro has clear opinions about param shape and return types that are not obvious from the attribute name alone |
