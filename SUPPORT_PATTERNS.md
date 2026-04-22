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
| Free function, constructor, or StatefulPlugin instance method | **`#[elicit_tool]`** |
| Generic type `Foo<T>` where T is chosen by the caller | **Factory pattern** |
| Tool needs live objects that survive across calls | **StatefulPlugin** |

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
methods with generic type parameters.

```rust
use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

elicit_newtype!(String, as MyString);

#[reflect_methods]
impl MyString {
    pub fn len(&self) -> usize { self.0.len() }
    pub fn contains<P: Pattern>(&self, pat: P) -> bool { self.0.contains(pat) }
    //                ^^^^^^^^^^^^^^^^^^^^ generics supported
}
// Generates: len_tool(), contains_tool(), param structs, inventory registration
```

`#[reflect_methods]` has full AST access via `syn` and handles:
- `&self` and `&mut self` methods
- Consuming (`self`) methods
- Generic type parameters and where clauses
- Async methods

### Decision tree

```
Does the upstream type need JsonSchema/Serialize?
  └─ Yes, it's an opaque handle → elicit_newtype!(T, as W)
  └─ Yes, fields need names      → hand-written trenchcoat struct

Do you need to expose the type's methods as MCP tools?
  └─ Simple, non-generic methods → elicit_newtype_methods! { W => T, fn … }
  └─ Generic methods, complex signatures → #[reflect_methods] on impl W { … }

Is the function a standalone tool, constructor, or StatefulPlugin tool?
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

## Common pitfalls

| Pitfall | Fix |
|---|---|
| `#[derive(Debug)]` fails because upstream type doesn't impl `Debug` | Implement `Debug` manually, show only counts/metadata |
| `concat!()` in `#[elicit_tool(name = …)]` | Proc macros see raw tokens; use literal strings or the factory pattern |
| Calling a trait method without importing the trait | `use redb::ReadableDatabase as _;` etc. — wildcard-import the trait |
| Double-lock deadlock (savepoints then write_txns, or vice versa) | Establish and document a global lock acquisition order; always follow it |
| Storing `&'static str` or `&'static [u8]` in tool params | Borrow from `p.field` at call time; the params outlive the table operation |
| `persistent_savepoint()` returns `u64`, not `Savepoint` | Call `txn.get_persistent_savepoint(id)` immediately to get the `Savepoint` struct |
