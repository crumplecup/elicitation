# elicit_surrealdb

MCP tools for SurrealDB 3.x — SurrealQL DDL/DML authoring and Rust SDK code generation.

`elicit_surrealdb` is part of the [elicitation](../../README.md) workspace. It follows the same
shadow/trenchcoat/factory patterns used throughout the codebase to bring the entire SurrealDB 3.x
API surface within reach of an AI model operating over the Model Context Protocol.

---

## Coverage at a Glance

| Plugin | Tool prefix | Tools | Coverage category |
|--------|-------------|------:|-------------------|
| `SurrealSchemaPlugin` | `surreal_schema__*` | 16 | DDL (DEFINE / REMOVE) |
| `SurrealCrudPlugin` | `surreal_crud__*` | 16 | DML — SurrealQL + Rust SDK |
| `SurrealConnectionPlugin` | `surreal_connection__*` | 10 | Connection + auth boilerplate |
| `SurrealSelectPlugin` | `surreal_select__*` | 14 | Stateful SELECT query builder |
| `SurrealTransactionPlugin` | `surreal_txn__*` | 6 | Stateful transaction builder |
| **Total** | | **62** | |

Shadow / value types covered:

| Type | Module | Notes |
|------|--------|-------|
| `Value` | `elicitation::surreal_types` | Full variant enum |
| `RecordId` | `elicitation::surreal_types` | `{table, id}` JSON shape |
| `Number` | `elicitation::surreal_types` | `Int / Float / Decimal` |
| `Geometry` | `elicitation::surreal_types` | GeoJSON-compatible 7-variant enum |
| `Datetime` | `elicitation::surreal_types` | ISO 8601 string newtype |
| `Duration` | `elicitation::surreal_types` | SurrealDB duration string newtype |
| `Kind` | `elicitation::surreal_types` | Schema type declaration enum |
| `Root / Namespace / Database / Token` | `auth` | Connection credential structs |
| `Config / Capabilities / PlannerStrategy / ExperimentalFeature` | `config` | Connection config |

---

## Why `elicit_surrealdb`?

SurrealDB is a multi-model database with its own query language (SurrealQL), a Rust SDK, and rich
type system. Writing correct SurrealQL DDL — especially `DEFINE TABLE`, `DEFINE FIELD`, `DEFINE
ACCESS`, or vector index declarations — requires exact syntax knowledge that AI models
frequently mis-recall or hallucinate. The connection and authentication boilerplate for the Rust
SDK is similarly repetitive and error-prone.

`elicit_surrealdb` solves this by providing **62 MCP tools** that accept well-typed, schema-validated
JSON parameters and produce exact, copy-paste-ready SurrealQL strings and Rust SDK snippets. The
model never writes SurrealQL syntax directly; it calls tools that know the grammar.

A key design decision: `elicit_surrealdb` has **zero dependency on `surrealdb` or
`surrealdb-types`** at compile time. It is pure code generation. This avoids a transitive
`geo = "^0.32"` version conflict in `surrealdb-types 3.x` and keeps compile times short. The
shadow types in `crates/elicitation/src/primitives/surreal_types/` are standalone structs built
from the public SurrealDB documentation — no upstream `From` impls required.

---

## Coverage Strategy

### 1. Phase 2 — Primitive shadow types (`elicitation` feature `surreal-types`)

The trenchcoat pattern wraps SurrealDB's value types so they satisfy `ElicitComplete` (which
requires `schemars::JsonSchema` — a trait the upstream crate does not implement). Each shadow type:

- mirrors the upstream type's public API shape faithfully
- derives `Serialize + Deserialize + JsonSchema` so it can cross the MCP boundary
- lives in `crates/elicitation/src/primitives/surreal_types/`

### 2. Phase 3A — Stateless DDL plugins

`SurrealSchemaPlugin` and `SurrealCrudPlugin` are stateless: each tool is a pure function that
maps typed parameters to a SurrealQL string. They use `#[derive(ElicitPlugin)]` and the
`#[elicit_tool(plugin = "…", name = "…", description = "…")]` attribute macro.

**Schema tools (16)** cover every `DEFINE` statement a typical SurrealDB application needs:

```
DEFINE NAMESPACE, DATABASE, TABLE, FIELD, INDEX, EVENT, FUNCTION,
PARAM, ANALYZER, ACCESS (JWT), ACCESS (RECORD), USER, and
REMOVE TABLE/FIELD/INDEX, INFO FOR DB
```

Field definitions support the full `FLEXIBLE TYPE kind ASSERT expr DEFAULT expr VALUE expr READONLY`
modifier surface. Index tools support `UNIQUE`, full-text `SEARCH … BM25 … HIGHLIGHTS`, MTREE, and
HNSW vector indexes. Access tools cover both JWT and RECORD (signup/signin/session) patterns.

**CRUD tools (16)** cover every DML operation in two flavors — raw SurrealQL string emission and
Rust SDK snippet generation:

```
SELECT / CREATE / INSERT / UPDATE / UPSERT / DELETE / MERGE / PATCH / RELATE  (SurrealQL)
select / create / insert / update / delete / query / live               (Rust SDK)
```

### 3. Phase 3B — Connection/auth plugin

`SurrealConnectionPlugin` (10 tools) generates the boilerplate that starts every SurrealDB Rust
program: choose a transport (`Ws`, `Http`, `Mem`, `SurrealKv`), sign in, select namespace and
database. A `full_setup` tool emits all three steps in one call.

Connection config (`Config`, `Capabilities`, `PlannerStrategy`, `ExperimentalFeature`) is captured
as shadow structs with `ToCodeLiteral` so they can appear as nested fields in parameter structs.

### 4. Phase 3C — Stateful workflow plugins

Two plugins accumulate state across tool calls, matching Bevy's `BevyAppPlugin` pattern:

**`SurrealSelectPlugin`** (14 tools) provides an incremental SELECT query builder. The model starts
a descriptor with `surreal_select__start`, progressively adds WHERE conditions, FETCH fields,
ORDER BY terms, GROUP BY, LIMIT, START, SPLIT AT, and VERSION clauses via separate tool calls,
and finally calls `surreal_select__emit_surreal` or `surreal_select__emit_rust` to extract the
finished query. Each descriptor is UUID-keyed so multiple queries can be built in parallel.

**`SurrealTransactionPlugin`** (6 tools) builds `BEGIN TRANSACTION … COMMIT/CANCEL TRANSACTION`
blocks incrementally. The model starts a descriptor, appends SurrealQL statements one by one,
and emits either the SurrealQL block or a Rust SDK `db.begin() / transaction.query(…) / commit()`
chain.

### 5. Trait factory

`prime_surreal_value::<T>()` in `trait_factories.rs` registers any user type implementing the
`SurrealValue` serialization trait into the dynamic tool registry, following the same factory
pattern used for rstar and other generic-parameter surfaces.

---

## Tool Reference

### `SurrealSchemaPlugin` (prefix `surreal_schema__`)

| Tool | Emits |
|------|-------|
| `define_namespace` | `DEFINE NAMESPACE [IF NOT EXISTS] name;` |
| `define_database` | `DEFINE DATABASE … [CHANGEFEED duration];` |
| `define_table` | `DEFINE TABLE … [SCHEMAFULL\|SCHEMALESS] [DROP] [CHANGEFEED] PERMISSIONS …;` |
| `define_field` | `DEFINE FIELD … ON TABLE … [FLEXIBLE] TYPE kind [ASSERT] [DEFAULT] [VALUE] [READONLY];` |
| `define_index` | `DEFINE INDEX … ON TABLE … FIELDS … [UNIQUE\|SEARCH\|MTREE\|HNSW];` |
| `define_event` | `DEFINE EVENT … ON TABLE … WHEN expr THEN expr;` |
| `define_function` | `DEFINE FUNCTION fn::name($args) { body } RETURNS type;` |
| `define_param` | `DEFINE PARAM $name VALUE value;` |
| `define_analyzer` | `DEFINE ANALYZER … TOKENIZERS … FILTERS …;` |
| `define_access_jwt` | `DEFINE ACCESS … TYPE JWT ALGORITHM … KEY …;` |
| `define_access_record` | `DEFINE ACCESS … TYPE RECORD SIGNUP … SIGNIN … SESSION duration;` |
| `define_user` | `DEFINE USER … ON … PASSWORD … ROLES …;` |
| `remove_table` | `REMOVE TABLE [IF EXISTS] name;` |
| `remove_field` | `REMOVE FIELD [IF EXISTS] name ON TABLE name;` |
| `remove_index` | `REMOVE INDEX [IF EXISTS] name ON TABLE name;` |
| `info_for_db` | `INFO FOR DB;` |

### `SurrealCrudPlugin` (prefix `surreal_crud__`)

| Tool | Emits |
|------|-------|
| `select_raw` | `SELECT projections FROM target [WHERE] [FETCH] [ORDER BY] [LIMIT] [START];` |
| `create_raw` | `CREATE target SET\|CONTENT …;` |
| `insert_raw` | `INSERT INTO table (fields) VALUES (values);` |
| `update_raw` | `UPDATE target SET … [WHERE …];` |
| `upsert_raw` | `UPSERT target CONTENT …;` |
| `delete_raw` | `DELETE target [WHERE …];` |
| `merge_raw` | `UPDATE target MERGE {…};` |
| `patch_raw` | `UPDATE target PATCH […];` |
| `relate_raw` | `RELATE from->relation->to [SET …];` |
| `select_rust` | `let result: Vec<T> = db.select("…").await?;` |
| `create_rust` | `db.create(…).content(…).await?` |
| `insert_rust` | `db.insert("…").content(…).await?` |
| `update_rust` | `db.update(…).merge(…).await?` |
| `delete_rust` | `db.delete(…).await?` |
| `query_rust` | `db.query("…").bind(…).await?` with multi-bind support |
| `live_rust` | `LIVE SELECT` + `stream::<Notification<T>>` async stream |

### `SurrealConnectionPlugin` (prefix `surreal_connection__`)

| Tool | Emits |
|------|-------|
| `ws_client` | `Surreal::new::<Ws>("host:port").await?` |
| `http_client` | `Surreal::new::<Http>("url").await?` |
| `memory_client` | `Surreal::new::<Mem>(()).await?` |
| `surrealkv_client` | `Surreal::new::<SurrealKv>("path").await?` |
| `signin_root` | `db.signin(Root { … }).await?` |
| `signin_ns` | `db.signin(Namespace { … }).await?` |
| `signin_db` | `db.signin(Database { … }).await?` |
| `signin_record` | `db.signin(Record { access: "…", … }).await?` |
| `use_ns_db` | `db.use_ns("…").use_db("…").await?` |
| `full_setup` | Complete connection + signin + use_ns/db boilerplate |

### `SurrealSelectPlugin` (prefix `surreal_select__`)

| Tool | Purpose |
|------|---------|
| `start` | Begin a new SELECT descriptor; returns UUID |
| `set_projections` | Override the SELECT field list |
| `set_from` | Set the FROM target |
| `add_where` | Append a WHERE condition (AND-combined) |
| `add_fetch` | Append a FETCH field for graph traversal |
| `set_order_by` | Set ORDER BY terms with direction/collate/numeric options |
| `set_group_by` | Set GROUP BY fields |
| `set_limit` | Set LIMIT count |
| `set_start` | Set START offset for pagination |
| `set_split` | Set SPLIT AT field |
| `set_version` | Set VERSION datetime for temporal queries |
| `inspect` | Return descriptor state as JSON |
| `emit_surreal` | Emit final SurrealQL SELECT string |
| `emit_rust` | Emit `db.query("SELECT …").await?` Rust snippet |

### `SurrealTransactionPlugin` (prefix `surreal_txn__`)

| Tool | Purpose |
|------|---------|
| `start` | Begin a new transaction descriptor; returns UUID |
| `add_statement` | Append a SurrealQL statement |
| `inspect` | Return accumulated statements as JSON |
| `emit_commit` | Emit `BEGIN … COMMIT TRANSACTION` block |
| `emit_cancel` | Emit `BEGIN … CANCEL TRANSACTION` block |
| `emit_rust` | Emit Rust SDK `db.begin() / query / commit` chain |

---

## Example Session

```
→ surreal_connection__full_setup
  { transport: "ws", address: "localhost:8000",
    namespace: "myapp", database: "prod",
    username: "root", password: "secret" }
← use surrealdb::{Surreal, engine::remote::ws::Ws};
  use surrealdb::opt::auth::Root;
  let db = Surreal::new::<Ws>("localhost:8000").await?;
  db.signin(Root { username: "root", password: "secret" }).await?;
  db.use_ns("myapp").use_db("prod").await?;

→ surreal_schema__define_table
  { name: "user", type_: "schemafull", if_not_exists: true }
← DEFINE TABLE IF NOT EXISTS user SCHEMAFULL;

→ surreal_schema__define_field
  { name: "email", table: "user", type_str: "string",
    assert_expr: "$value != NONE AND is::email($value)" }
← DEFINE FIELD email ON TABLE user TYPE string ASSERT $value != NONE AND is::email($value);

→ surreal_schema__define_index
  { name: "unique_email", table: "user", fields: ["email"],
    kind: "unique" }
← DEFINE INDEX unique_email ON TABLE user FIELDS email UNIQUE;

→ surreal_select__start
  { projections: "id, email, created_at", from: "user" }
← "550e8400-e29b-41d4-a716-446655440000"

→ surreal_select__add_where
  { id: "550e8400-…", condition: "active = true" }

→ surreal_select__set_limit
  { id: "550e8400-…", limit: 25 }

→ surreal_select__emit_surreal
  { id: "550e8400-…" }
← SELECT id, email, created_at FROM user WHERE active = true LIMIT 25;
```

---

## Intentional Exclusions

| Excluded | Reason |
|----------|--------|
| `Surreal<C>` runtime connection | Runtime handle; connection is generated, not instantiated over MCP |
| `surrealdb_core::*` internals | Not part of the public SDK surface |
| SurrealML (`surrealml-core`) | Separate optional feature; defer to a future `elicit_surrealml` |
| `kv-rocksdb`, `kv-tikv` backends | Heavy optional deps; covered by code-gen snippets only |

---

## Design Notes

### No upstream dependency

`elicit_surrealdb` deliberately does not depend on `surrealdb` or `surrealdb-types`. This avoids
a transitive version conflict (`surrealdb-types 3.x` pins `geo = "^0.32"` while the workspace
uses `geo = "^0.33"`). The shadow types are standalone and the code generation output is just
`String` — no upstream types are instantiated at runtime.

### Unique params structs per tool

The `#[elicit_tool]` macro generates an `EmitCode` impl for the params struct type. If two tools
share a params struct, the compiler rejects both impls as conflicting. Every tool in this crate
therefore uses its own uniquely-named params struct, even when the fields are identical (e.g.,
the four `*IdParams` structs in `select_plugin.rs` and `txn_plugin.rs`).

### Stateful plugins and UUID-keyed descriptors

`SurrealSelectPlugin` and `SurrealTransactionPlugin` follow the `StatefulPlugin` trait pattern
established by `BevyAppPlugin`. Each holds an `Arc<Ctx>` where `Ctx` wraps a
`Mutex<HashMap<Uuid, Descriptor>>`. UUIDs are returned from `start` tools and passed back in
all subsequent calls, allowing the model to work on multiple independent queries or transactions
simultaneously within the same session.
