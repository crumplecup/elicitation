# `elicit_surrealdb` — SurrealDB 3.x MCP Coverage Plan

## Why SurrealDB?

SurrealDB is a scalable, multi-model document-graph database with its own query
language, SurrealQL. It supports records, relations, schema-full and schema-less
tables, vector indexes, geospatial data, live queries, and full ACID transactions
— all addressable through a single Rust SDK.

For an AI agent, SurrealDB authoring has two distinct dimensions:

1. **SurrealQL strings** — DDL (`DEFINE TABLE`, `DEFINE FIELD`, `DEFINE INDEX`,
   `DEFINE ACCESS`, …) and DML (`SELECT`, `CREATE`, `RELATE`, `UPDATE`,
   `DELETE`, `BEGIN TRANSACTION`, …).
2. **Rust SDK setup code** — connection, authentication, `db.query()` /
   `db.select()` / `db.create()` chains.

Both outputs are equally valuable and naturally map to the elicitation plugin
patterns already established in `elicit_sqlx`.

---

## Design Principles

| Principle | Rationale |
|-----------|-----------|
| SurrealQL strings are the primary output | Agents need to emit DDL/DML directly, not just Rust code |
| Value types cross the MCP boundary as JSON parameters | `RecordId`, `Number`, `Geometry`, `Kind` need `JsonSchema` |
| Trenchcoat pattern for upstream types | `surrealdb-types` lacks `schemars::JsonSchema`; newtypes provide it |
| Factory pattern for `Record<P>` | Generic auth credential with `P: SurrealValue` |
| Stateful plugins for incremental query building | SELECT with WHERE/ORDER/LIMIT accumulation and transaction blocks |
| Fragment/descriptor plugins for DDL | Each `DEFINE …` statement is a pure function of its parameters |

---

## Phase 1 — Workspace Configuration

### `Cargo.toml` workspace additions

```toml
# [workspace.members]
"crates/elicit_surrealdb",

# [workspace.dependencies]
surrealdb = { version = "3", default-features = false, features = [
    "kv-mem", "protocol-ws", "rustls"
] }
surrealdb-types = "3"

# [workspace.dependencies] — crate alias
elicit_surrealdb = { path = "crates/elicit_surrealdb", version = "0.10.0" }
```

The `kv-mem` feature enables an in-process embedded engine for testing without
an external SurrealDB server. `protocol-ws` gives WebSocket client support.

---

## Phase 2 — Primitive Types in `crates/elicitation/`

Feature flag: `surreal-types` in `crates/elicitation/Cargo.toml`.

### Why Phase 2?

The `surrealdb-types` crate provides Rust types (`Value`, `RecordId`, `Number`,
`Geometry`, `Kind`, …) that have `serde::{Serialize, Deserialize}` but lack
`schemars::JsonSchema`. The trenchcoat pattern wraps each in a local newtype
that adds `JsonSchema` and the full `ElicitComplete` trait stack, making the
types crossable over the MCP JSON boundary as tool parameters.

### New files: `crates/elicitation/src/primitives/surreal_types/`

| File | Exposed Type | Notes |
|------|-------------|-------|
| `mod.rs` | module re-exports | |
| `db_value.rs` | `Value` | Shadow of `Value` enum. Variants: `None`, `Null`, `Bool`, `Int`, `Float`, `Decimal`, `String`, `Bytes`, `Datetime`, `Duration`, `Uuid`, `Array`, `Object`, `Geometry`, `RecordId` |
| `record_id.rs` | `RecordId` | `{ table: String, id: serde_json::Value }` newtype; `Into<surrealdb_types::RecordId>` |
| `number.rs` | `Number` | `Int(i64)` / `Float(f64)` / `Decimal(String)` enum; `Into<surrealdb_types::Number>` |
| `geometry.rs` | `Geometry` | Shadow of `Geometry` enum, GeoJSON-compatible; bridges to `geo::*` types via `From` impl. Variants: `Point`, `Line`, `Polygon`, `MultiPoint`, `MultiLine`, `MultiPolygon`, `Collection` |
| `datetime.rs` | `Datetime` | ISO 8601 string newtype; `Into<surrealdb_types::Datetime>` |
| `duration.rs` | `Duration` | SurrealDB duration string (`1y2w3d4h5m6s`) newtype with `Display`/`FromStr` |
| `kind.rs` | `Kind` | Full shadow of `Kind` enum for use in `DEFINE FIELD TYPE` authoring; includes `Array`, `Set`, `Either`, `Record`, `Geometry`, `Literal`, `Function`, `File` variants |
| `table.rs` | `Table` | String newtype for table names; validated (alphanumeric + `_`) |
| `patch_op.rs` | `PatchOp` | Shadow of `PatchOp` enum (`Add`, `Remove`, `Replace`, `Change`, `Copy`, `Move`, `Test`, `Increment`, `Decrement`) for JSON Patch support |

### TypeSpec: `crates/elicitation/src/type_spec/surreal_specs.rs`

`ElicitSpec` (Kani proof harness stubs) for each Phase 2 type, following the
`kani::assume(true)` placeholder pattern established in existing specs.

### Module wiring

```rust
// crates/elicitation/src/primitives/mod.rs
#[cfg(feature = "surreal-types")]
pub mod surreal_types;

// crates/elicitation/src/type_spec/mod.rs
#[cfg(feature = "surreal-types")]
pub mod surreal_specs;

// crates/elicitation/src/lib.rs
#[cfg(feature = "surreal-types")]
pub use primitives::surreal_types::*;
```

---

## Phase 3 — `crates/elicit_surrealdb/`

### Crate structure

```
crates/elicit_surrealdb/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs               # mod + pub use only; #![forbid(unsafe_code)]
    ├── types.rs             # re-export Phase 2 primitives + reflect_methods
    ├── auth.rs              # shadow auth credential types
    ├── config.rs            # shadow Config / Capabilities / PlannerStrategy
    ├── schema_plugin.rs     # SurrealSchemaPlugin — DDL authoring
    ├── crud_plugin.rs       # SurrealCrudPlugin — DML + Rust SDK snippets
    ├── connection_plugin.rs # SurrealConnectionPlugin — connection setup code
    ├── select_plugin.rs     # SurrealSelectPlugin — stateful SELECT builder
    ├── txn_plugin.rs        # SurrealTransactionPlugin — transaction block
    └── trait_factories.rs   # SurrealValue trait factory
```

### `Cargo.toml` dependencies

```toml
[dependencies]
elicitation = { workspace = true, features = ["surreal-types", "emit"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
surrealdb = { workspace = true }
surrealdb-types = { workspace = true }
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde.workspace = true
serde_json.workspace = true
proc-macro2 = { workspace = true }
quote = { workspace = true }
tracing.workspace = true
derive_more = { workspace = true }
derive-getters = { workspace = true }
```

---

### 3.1 Shadow types

#### `auth.rs` — Credential types

| Elicit type | Mirrors | Fields |
|-------------|---------|--------|
| `Root` | `opt::auth::Root` | `username: String`, `password: String` |
| `Namespace` | `opt::auth::Namespace` | `namespace`, `username`, `password` |
| `Database` | `opt::auth::Database` | `namespace`, `database`, `username`, `password` |
| `Token` | `opt::auth::Token` | `token: String` |

`Record<P>` (generic) is handled by the connection plugin via a factory tool
that takes `params: serde_json::Value` and emits `db.signin(Record { … })` code
with the appropriate generic bound, rather than as a Phase 2 type.

#### `config.rs` — Connection configuration

| Elicit type | Mirrors | Notes |
|-------------|---------|-------|
| `Config` | `opt::Config` | Carries `query_timeout`, `transaction_timeout`, builder flag booleans |
| `Capabilities` | `opt::Capabilities` | Allow/deny lists for functions, scripting, network, guest access |
| `PlannerStrategy` | `opt::PlannerStrategy` | `BestEffort`, `ComputeOnly`, `AllReadOnly` |
| `ExperimentalFeature` | `opt::ExperimentalFeature` | `Files`, `Surrealism` |

---

### 3.2 `SurrealSchemaPlugin` — DDL generation (fragment/descriptor, Phase 3E)

The highest-value plugin. Emits SurrealQL DDL strings. Each tool is a pure
function of its parameters — no accumulated state required.

**Namespace / database:**

| Tool name | Output |
|-----------|--------|
| `surreal_schema__define_namespace` | `DEFINE NAMESPACE [IF NOT EXISTS] name [COMMENT '…']` |
| `surreal_schema__define_database` | `DEFINE DATABASE name [CHANGEFEED duration] [COMMENT '…']` |
| `surreal_schema__remove_namespace` | `REMOVE NAMESPACE name` |
| `surreal_schema__remove_database` | `REMOVE DATABASE name` |

**Tables:**

| Tool name | Output |
|-----------|--------|
| `surreal_schema__define_table` | `DEFINE TABLE name [SCHEMAFULL\|SCHEMALESS] [DROP] [CHANGEFEED] [PERMISSIONS …]` |
| `surreal_schema__define_table_as_select` | `DEFINE TABLE name AS SELECT … FROM …` (pre-computed table) |
| `surreal_schema__remove_table` | `REMOVE TABLE name` |

**Fields:**

| Tool name | Output |
|-----------|--------|
| `surreal_schema__define_field` | `DEFINE FIELD name ON TABLE name [FLEXIBLE] TYPE kind [ASSERT expr] [DEFAULT expr] [VALUE expr] [READONLY]` |
| `surreal_schema__define_field_reference` | `DEFINE FIELD name ON TABLE name TYPE record(table) REFERENCE [ON DELETE cascade\|reject\|ignore]` |
| `surreal_schema__remove_field` | `REMOVE FIELD name ON TABLE name` |

**Indexes:**

| Tool name | Output |
|-----------|--------|
| `surreal_schema__define_index_unique` | `DEFINE INDEX name ON TABLE name FIELDS … UNIQUE` |
| `surreal_schema__define_index_search` | `DEFINE INDEX name ON TABLE name FIELDS … SEARCH ANALYZER name [BM25\|HIGHLIGHTS]` |
| `surreal_schema__define_index_mtree` | `DEFINE INDEX name ON TABLE name FIELDS … MTREE DIMENSION n [DIST euclidean\|cosine\|manhattan\|minkowski]` |
| `surreal_schema__define_index_hnsw` | `DEFINE INDEX name ON TABLE name FIELDS … HNSW DIMENSION n [EFC n] [M n]` |
| `surreal_schema__remove_index` | `REMOVE INDEX name ON TABLE name` |

**Events:**

| Tool name | Output |
|-----------|--------|
| `surreal_schema__define_event` | `DEFINE EVENT name ON TABLE name WHEN expr THEN expr` |
| `surreal_schema__remove_event` | `REMOVE EVENT name ON TABLE name` |

**Functions, params, analyzers:**

| Tool name | Output |
|-----------|--------|
| `surreal_schema__define_function` | `DEFINE FUNCTION fn::name($arg: Type, …) { body } RETURNS Type` |
| `surreal_schema__define_param` | `DEFINE PARAM $name VALUE value` |
| `surreal_schema__define_analyzer` | `DEFINE ANALYZER name TOKENIZERS … FILTERS …` |
| `surreal_schema__remove_function` | `REMOVE FUNCTION fn::name` |
| `surreal_schema__remove_param` | `REMOVE PARAM $name` |
| `surreal_schema__remove_analyzer` | `REMOVE ANALYZER name` |

**Access control:**

| Tool name | Output |
|-----------|--------|
| `surreal_schema__define_access_jwt` | `DEFINE ACCESS name ON [ROOT\|NS\|DB] TYPE JWT ALGORITHM HS512 KEY '…' [SESSION duration]` |
| `surreal_schema__define_access_record` | `DEFINE ACCESS name ON DB TYPE RECORD SIGNUP (…) SIGNIN (…) SESSION duration` |
| `surreal_schema__define_user` | `DEFINE USER name ON [ROOT\|NS\|DB] PASSWORD '…' ROLES [owner\|editor\|viewer]` |
| `surreal_schema__remove_access` | `REMOVE ACCESS name ON [ROOT\|NS\|DB]` |
| `surreal_schema__remove_user` | `REMOVE USER name ON [ROOT\|NS\|DB]` |

**Introspection:**

| Tool name | Output |
|-----------|--------|
| `surreal_schema__info_for_root` | `INFO FOR ROOT` |
| `surreal_schema__info_for_ns` | `INFO FOR NS` |
| `surreal_schema__info_for_db` | `INFO FOR DB` |
| `surreal_schema__info_for_table` | `INFO FOR TABLE name` |

---

### 3.3 `SurrealCrudPlugin` — DML + SDK code (fragment, Phase 3E)

Emits both SurrealQL statement strings and Rust SDK call snippets. The caller
controls which form they want via tool selection.

**SurrealQL DML string emitters:**

| Tool name | Output |
|-----------|--------|
| `surreal_crud__select_raw` | `SELECT projections FROM target [WHERE …] [LIMIT n] [START n]` |
| `surreal_crud__create_raw` | `CREATE table:id SET field = val, …` |
| `surreal_crud__insert_raw` | `INSERT INTO table { field: val, … }` |
| `surreal_crud__insert_relation_raw` | `INSERT RELATION INTO table (in, id, out) VALUES (…)` |
| `surreal_crud__update_raw` | `UPDATE table:id SET field = val` |
| `surreal_crud__upsert_raw` | `UPSERT table:id CONTENT { … }` |
| `surreal_crud__delete_raw` | `DELETE table:id [WHERE …]` |
| `surreal_crud__merge_raw` | `UPDATE table:id MERGE { … }` |
| `surreal_crud__patch_raw` | `UPDATE table:id PATCH [PatchOp, …]` |
| `surreal_crud__relate_raw` | `RELATE from->edge:id->to [SET …]` |
| `surreal_crud__live_raw` | `LIVE SELECT * FROM table [WHERE …]` |
| `surreal_crud__kill_raw` | `KILL $live_id` |

**Rust SDK snippet emitters:**

| Tool name | Output |
|-----------|--------|
| `surreal_crud__select_rust` | `db.select("table").await?` |
| `surreal_crud__select_record_rust` | `db.select(("table", "id")).await?` |
| `surreal_crud__query_rust` | `db.query("SurrealQL").bind(("var", value)).await?` |
| `surreal_crud__create_rust` | `db.create(("table", id)).content(value).await?` |
| `surreal_crud__insert_rust` | `db.insert("table").content(vec![…]).await?` |
| `surreal_crud__update_merge_rust` | `db.update(("table","id")).merge(value).await?` |
| `surreal_crud__update_content_rust` | `db.update(("table","id")).content(value).await?` |
| `surreal_crud__upsert_rust` | `db.upsert(("table","id")).content(value).await?` |
| `surreal_crud__delete_rust` | `db.delete(("table","id")).await?` |
| `surreal_crud__patch_rust` | `db.update(("table","id")).patch(PatchOp::…).await?` |
| `surreal_crud__live_rust` | `let stream = db.query("LIVE SELECT * FROM table").await?; …` |
| `surreal_crud__run_rust` | `db.run("fn::name").args((…,)).await?` |
| `surreal_crud__object_macro_rust` | `object!({ field: value })` macro usage snippet |
| `surreal_crud__array_macro_rust` | `array![val1, val2, …]` macro usage snippet |

---

### 3.4 `SurrealConnectionPlugin` — Connection setup (fragment, Phase 3E)

Emits Rust SDK boilerplate for establishing a connection.

| Tool name | Output |
|-----------|--------|
| `surreal_connection__ws_client` | `let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;` |
| `surreal_connection__http_client` | `let db = Surreal::new::<Http>("http://127.0.0.1:8000").await?;` |
| `surreal_connection__memory_client` | `let db = Surreal::new::<Mem>(()).await?;` (embedded, no server) |
| `surreal_connection__surrealkv_client` | `let db = Surreal::new::<SurrealKv>("path/to/db").await?;` (embedded, persistent) |
| `surreal_connection__signin_root` | `db.signin(Root { username: "…", password: "…" }).await?;` |
| `surreal_connection__signin_namespace` | `db.signin(Namespace { ns: "…", … }).await?;` |
| `surreal_connection__signin_database` | `db.signin(Database { ns: "…", db: "…", … }).await?;` |
| `surreal_connection__signin_record` | `db.signin(Record { access: "…", ns: "…", db: "…", params: … }).await?;` |
| `surreal_connection__signup_record` | `db.signup(Record { … }).await?;` |
| `surreal_connection__authenticate` | `db.authenticate(Token("…")).await?;` |
| `surreal_connection__invalidate` | `db.invalidate().await?;` |
| `surreal_connection__use_ns_db` | `db.use_ns("ns").use_db("db").await?;` |
| `surreal_connection__set_var` | `db.set("var", value).await?;` |
| `surreal_connection__unset_var` | `db.unset("var").await?;` |
| `surreal_connection__export` | `db.export("backup.surql").await?;` |
| `surreal_connection__import` | `db.import("backup.surql").await?;` |
| `surreal_connection__version` | `db.version().await?;` |
| `surreal_connection__health` | `db.health().await?;` |
| `surreal_connection__with_config` | Full `Config::new().query_timeout(…).transaction_timeout(…).user(…).capabilities(…)` snippet |
| `surreal_connection__full_setup` | Complete end-to-end: new client + config + signin + use_ns_db boilerplate |

---

### 3.5 `SurrealSelectPlugin` — Stateful SELECT builder (Phase 3C)

An MCP-side descriptor accumulates SELECT clause components across tool calls
before emitting the final SurrealQL string. This mirrors the `BevyRenderWorkflowPlugin`
pattern from `elicit_bevy`.

**State struct** (`SelectDescriptor`): `projections`, `target`, `where_clauses: Vec<String>`,
`fetch_fields: Vec<String>`, `order_by: Vec<(String, bool)>`, `group_by: Vec<String>`,
`limit: Option<u64>`, `start: Option<u64>`, `split_at: Option<String>`,
`version: Option<String>`, `with_no_index: bool`, `explain: bool`

| Tool name | Effect |
|-----------|--------|
| `surreal_select__start` | Create new descriptor (returns session ID) |
| `surreal_select__set_projections` | Set `SELECT field1, field2` or `SELECT *` |
| `surreal_select__set_from` | Set `FROM target` — table name, record ID, or value subquery |
| `surreal_select__add_where` | Append `AND condition` to WHERE clause |
| `surreal_select__add_fetch` | Append `FETCH field` (resolves record links) |
| `surreal_select__add_order_by` | Append `ORDER BY field [ASC\|DESC] [NUMERIC\|COLLATE]` |
| `surreal_select__add_group_by` | Append `GROUP BY field` |
| `surreal_select__set_limit` | Set `LIMIT count` |
| `surreal_select__set_start` | Set `START offset` (pagination) |
| `surreal_select__set_split` | Set `SPLIT AT field` |
| `surreal_select__set_version` | Set `VERSION datetime` (temporal) |
| `surreal_select__set_explain` | Add `EXPLAIN [FULL]` |
| `surreal_select__inspect` | Return current descriptor as JSON |
| `surreal_select__emit_surreal` | Emit final SurrealQL `SELECT …` string |
| `surreal_select__emit_rust` | Emit `db.query("SELECT …").await?` Rust snippet |
| `surreal_select__discard` | Drop descriptor |

---

### 3.6 `SurrealTransactionPlugin` — Transaction block (Phase 3C)

Accumulates SurrealQL statements and emits a `BEGIN … COMMIT/CANCEL` block.

**State struct** (`TxnDescriptor`): `statements: Vec<String>`, `let_bindings: Vec<(String, String)>`

| Tool name | Effect |
|-----------|--------|
| `surreal_txn__start` | Create new transaction descriptor |
| `surreal_txn__add_statement` | Append any SurrealQL statement |
| `surreal_txn__add_let` | Append `LET $var = expr` binding |
| `surreal_txn__add_return` | Append `RETURN expr` |
| `surreal_txn__inspect` | Show accumulated statements as JSON |
| `surreal_txn__emit_commit` | Emit `BEGIN TRANSACTION; … COMMIT TRANSACTION;` |
| `surreal_txn__emit_cancel` | Emit `BEGIN TRANSACTION; … CANCEL TRANSACTION;` |
| `surreal_txn__emit_rust` | Emit `let txn = db.begin().await?; … txn.commit().await?;` snippet |
| `surreal_txn__discard` | Drop descriptor |

---

### 3.7 `trait_factories.rs` — Trait factory

`SurrealValue` (the SDK trait for types that can cross the SurrealDB wire) maps to
the factory pattern already used for `Asset` in `elicit_bevy`. A factory
registration macro generates the necessary conversion glue for user types:

```rust
// Example usage in the crate
prime_surreal_value::<MyCustomType>();
```

The factory provides tools for registering user types and emitting
`impl SurrealValue for MyType` boilerplate code.

---

## Phase 4 — Kani Verification (`crates/elicitation_kani/`)

New file: `src/surreal_types.rs`

| Proof harness | Property |
|---------------|----------|
| `prop_encode_decode_record_id` | `RecordId` roundtrip via `serde_json` |
| `prop_encode_decode_number` | `Number` roundtrip for all three variants |
| `prop_encode_decode_kind` | `Kind` roundtrip for flat and nested variants |
| `prop_encode_decode_db_value` | `Value` roundtrip for representative variants |
| `prop_surreal_duration_parse` | `Duration` FromStr/Display round-trips valid strings |
| `prop_patch_op_roundtrip` | `PatchOp` serde roundtrip |

---

## Phase 5 — Creusot (deferred)

Creusot proofs for property verification of the schema and query emitters
(e.g., well-formedness of emitted SurrealQL) are deferred to a future iteration.

---

## Coverage Summary

| SurrealDB API category | Coverage | Mechanism |
|------------------------|----------|-----------|
| `surrealdb::types::Value` | Full shadow | Phase 2 `Value` |
| `surrealdb::types::RecordId` | Full newtype | Phase 2 `RecordId` |
| `surrealdb::types::Number` | Full shadow | Phase 2 `Number` |
| `surrealdb::types::Geometry` | Full shadow | Phase 2 `Geometry` |
| `surrealdb::types::Datetime` | Newtype | Phase 2 `Datetime` |
| `surrealdb::types::Duration` | Newtype | Phase 2 `Duration` |
| `surrealdb::types::Kind` | Full shadow | Phase 2 `Kind` |
| `surrealdb::types::PatchOp` | Full shadow | Phase 2 `PatchOp` |
| Auth types (Root/NS/DB/Token) | Shadow structs | `auth.rs` |
| `Record<P>` (generic auth) | Factory tool | `connection_plugin.rs` |
| `Config` / `Capabilities` | Shadow | `config.rs` |
| `Surreal<C>` query methods | Rust snippets | `SurrealCrudPlugin` |
| `Surreal<C>` CRUD methods | Rust snippets | `SurrealCrudPlugin` |
| `Surreal<C>` connection/auth | Rust snippets | `SurrealConnectionPlugin` |
| `Surreal<C>` transaction | Stateful plugin | `SurrealTransactionPlugin` |
| `Surreal<C>` live queries | Rust snippets | `SurrealCrudPlugin` |
| SurrealQL DDL (DEFINE/REMOVE) | SurrealQL strings | `SurrealSchemaPlugin` (32+ tools) |
| SurrealQL SELECT builder | Stateful plugin | `SurrealSelectPlugin` (16 tools) |
| SurrealQL DML fragments | SurrealQL strings | `SurrealCrudPlugin` (12+ tools) |
| `object!` / `array!` macros | Code snippet tools | `SurrealCrudPlugin` |
| `SurrealValue` trait | Factory | `trait_factories.rs` |

**Total plugin tools (estimated): ~95 tools across 5 plugins**

---

## Intentional Exclusions

| Excluded surface | Reason |
|-----------------|--------|
| `Surreal<C>` itself as a runtime type | Connection is generated, not instantiated over MCP |
| `surrealdb_core::*` (internal) | Not part of the stable public SDK |
| SurrealML (`surrealml-core`) | Separate optional feature; defer to `elicit_surrealml` |
| Surrealism framework | Experimental; not stable in SurrealDB 3.0.x |
| `kv-rocksdb` / `kv-tikv` compile-time deps | Heavy; covered by code-gen snippets, not runtime |
| Creusot proofs | Phase 5, deferred |

---

## Implementation Order

1. `Cargo.toml` workspace additions (Phase 1)
2. `crates/elicitation/src/primitives/surreal_types/` + feature flag (Phase 2)
3. `crates/elicitation/src/type_spec/surreal_specs.rs` (Phase 2)
4. `crates/elicit_surrealdb/Cargo.toml` + `src/lib.rs` skeleton
5. `src/types.rs` + `src/auth.rs` + `src/config.rs`
6. `src/schema_plugin.rs` — `SurrealSchemaPlugin`
7. `src/crud_plugin.rs` — `SurrealCrudPlugin`
8. `src/connection_plugin.rs` — `SurrealConnectionPlugin`
9. `src/select_plugin.rs` — `SurrealSelectPlugin` (stateful)
10. `src/txn_plugin.rs` — `SurrealTransactionPlugin` (stateful)
11. `src/trait_factories.rs`
12. `crates/elicitation_kani/src/surreal_types.rs` (Phase 4)
13. `crates/elicit_surrealdb/README.md`
14. Wire workspace integration test in `elicit_server`
