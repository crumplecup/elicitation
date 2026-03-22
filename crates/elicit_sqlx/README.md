# elicit_sqlx

MCP-enabled database workflows built on [`sqlx`], [`elicitation`], and [`rmcp`].

Wraps sqlx's `Any`-driver types as MCP tools and fragment emitters, giving an
agent a complete vocabulary for constructing verified database workflows without
writing any Rust by hand.

---

## What this crate provides

Four complementary layers:

1. **Newtype wrappers** — `serde` + `JsonSchema`-enabled newtypes for sqlx types
   that lack those impls, so database values can cross the MCP boundary
2. **Any-driver runtime tools** — five `SqlxPlugin` tools that execute SQL
   against a real database via the `Any` driver and return structured results
3. **Driver-specific plugins** — three plugins (`SqlxPgPlugin`, `SqlxSqlitePlugin`,
   `SqlxMySqlPlugin`) that expose stateful, UUID-keyed connection pools and
   transactions for Postgres, SQLite, and MySQL respectively
4. **Fragment tools** — four `SqlxFragPlugin` tools that emit Rust source
   wrapping sqlx's compile-time macros (`query!`, `query_as!`, `query_scalar!`,
   `migrate!`)

---

## Quick start

```toml
[dependencies]
elicit_sqlx = { version = "0.8" }
rmcp = { version = "0.1", features = ["server"] }
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use elicitation::PluginRegistry;
use elicit_sqlx::{SqlxPlugin, SqlxFragPlugin, SqlxPgPlugin, SqlxSqlitePlugin};

let registry = PluginRegistry::new()
    .register("sqlx",      SqlxPlugin::default())
    .register("sqlx_frag", SqlxFragPlugin)
    .register("pg",        SqlxPgPlugin::new())
    .register("sqlite",    SqlxSqlitePlugin::new());

// Any-driver tools: sqlx__fetch_all, sqlx_frag__query, …
// Postgres tools:   pg__connect, pg__begin, pg__tx_execute, pg__commit, …
// SQLite tools:     sqlite__connect, sqlite__execute, sqlite__fetch_all, …
```

---

## How `elicit_sqlx` satisfies the shadow crate motivation

The shadow crate motivation is stated in `SHADOW_CRATE_MOTIVATION.md`. The core
idea is:

> *Define a vocabulary of atomic, verified operations — let an agent compose
> them into tool chains — the tool chain **is** the method.*

Database access is one of the hardest domains to verify after the fact: queries
are strings, result shapes are dynamic, and the type system of the application
rarely reflects the schema it reads from.  `elicit_sqlx` attacks this from the
vocabulary end: by making every sqlx operation an explicitly typed, MCP-callable
tool with a machine-checkable contract, agent-composed database workflows
inherit those contracts structurally rather than requiring per-method proofs.

### The `Any` driver as the verification bridge

sqlx is generic over database backends (`Postgres`, `Sqlite`, `MySql`, …).
Its compile-time generics are powerful but opaque at the MCP boundary — you
cannot pass `sqlx::Row<Postgres>` over JSON.  sqlx solves this with the **Any
driver**: a runtime-erased backend that unifies all supported databases behind
a single concrete type family.

| sqlx Any type | Role in elicit_sqlx |
|---|---|
| `sqlx::any::AnyRow` | Query result transport across the MCP boundary |
| `sqlx::any::AnyColumn` | Column metadata (name, ordinal, type) |
| `sqlx::any::AnyTypeInfo` | SQL type descriptor |
| `sqlx::any::AnyQueryResult` | DML result (rows affected, last insert ID) |
| `sqlx::any::AnyPool` | Connection pool held in `SqlxContext` |
| `sqlx::Error` | Error introspection |

The `Any` types are the bridge: they are concrete enough to serialize and schema-
describe, yet backend-agnostic enough to work with any sqlx-supported database.
An agent that learns the `Any` vocabulary has a verified dialect that works with
Postgres, SQLite, and MySQL without any driver-specific knowledge.

### `SqlTypeKind` — the type vocabulary for SQL

The key connection between the `Any` type family and formal verification is
`AnyTypeInfoKind`, sqlx's enum of known SQL type variants.  We surfaced this as
`SqlTypeKind` in the `elicitation` crate (a `Select` enum with `Elicitation`
derived), giving it three things sqlx itself does not provide:

- `JsonSchema` — so it can appear in MCP tool parameter and return schemas
- `serde` — so values can cross the MCP boundary
- `Kani`/`Creusot`/`Verus` proofs — so an agent can reason about which type
  kinds are valid, how many variants exist, and which labels round-trip

When an agent calls `any_column__type_kind()` and receives `SqlTypeKind::Text`,
it is receiving a formally verified, schema-described value — not a raw string
— that it can use to drive the next step in its tool chain.

---

## Type choices: what we shadowed and why

### `AnyRow` — the central result type

`sqlx::any::AnyRow` is not `Clone`, not `Serialize`, and not `JsonSchema`.
It also holds internal `Arc` state that must survive cloning across async
boundaries.  We could not use `elicit_newtype!` here, so `AnyRow` is hand-
written with:

- `Arc<sqlx::any::AnyRow>` as the inner field (cheap clone across tool calls)
- Manual `JsonSchema` and `Debug` impls
- `#[reflect_methods]` on six methods: `columns()`, `len()`, `is_empty()`,
  `column_names()`, `columns_as_descriptors()`, and `to_row_data()`

The key method is `to_row_data()`, which materialises the row into a
`RowData` — a fully serializable owned snapshot that crosses the MCP boundary
cleanly.  This is the exit point from sqlx's world into the agent's world.

`RowData` holds `Vec<ColumnEntry>`, and each `ColumnEntry` carries a `ColumnValue`
— our own enum that mirrors `AnyValueKind`:

```
AnyValueKind (sqlx, non_exhaustive, non-Serialize)
    ↓  decode_val()
ColumnValue (ours, Serialize + JsonSchema + verified)
    ↓  ColumnEntry
RowData (fully serializable row snapshot)
```

`decode_val()` matches directly on `AnyValueKind` variants rather than going
through the `Decode` trait.  This avoids type-dispatch overhead and correctly
handles null values, which sqlx encodes as `AnyValueKind::Null(kind)` (a null
with a type annotation) rather than a failed decode.  The match arm is marked
with a wildcard fallback because `AnyValueKind` is `#[non_exhaustive]` — new
sqlx variants fall through to `ColumnValue::Null` rather than breaking builds.

### `AnyColumn` — column metadata

`sqlx_core::any::AnyColumn` is the type backing column descriptors in every
row.  It implements `sqlx::Column` and `sqlx::TypeInfo` but not `serde` or
`JsonSchema`.  We wrap it with `elicit_newtype!` and expose four methods:
`ordinal()`, `name()`, `type_kind()`, and `type_name()`.

One note on the import path: `AnyColumn` is internally a `sqlx_core` type
re-exported through several levels of sqlx's module hierarchy.  We import it
directly from `sqlx_core::any::AnyColumn` to avoid ambiguity with the outer
`sqlx::Column` trait of the same name.

### `AnyTypeInfo` — SQL type descriptor

Wraps `sqlx::any::AnyTypeInfo` with `elicit_newtype!` and exposes `kind()`,
`name()`, and `is_null()`.  The `kind()` method returns a `SqlTypeKind`,
completing the chain from raw database type information to a formally verified
Select enum value.

### `AnyQueryResult` — DML results

`sqlx::any::AnyQueryResult` holds two public fields: `rows_affected: u64`
and `last_insert_id: Option<i64>`.  It does not implement `Serialize`, so we
wrap it with `elicit_newtype!` for method reflection and add `QueryResultData`
— a plain serializable struct carrying those same two fields — as the MCP-
boundary transport type.

### `SqlxError` — error introspection

`sqlx::Error` is a large non-serializable enum.  We wrap it with
`elicit_newtype!` and expose two methods: `kind_label()` (returns a string
label for the variant, e.g. `"Database"`, `"RowNotFound"`) and `message()`
(full display string).  This lets agents diagnose errors without receiving an
opaque error blob.

### `SqlxContext` — connection pool carrier

`SqlxContext` wraps `AnyPool` and implements `PluginContext`.  It is the
connection-pool carrier in stateful workflows where the same pool should persist
across multiple tool calls.  The top-level `connect()` function installs the
Any driver and produces a context from a database URL.

---

## Driver-specific plugins

`Pool<Db>` and `Connection<Db>` are generic over the driver type, which makes
them opaque to MCP tool schemas.  The solution is monomorphization: rather than
trying to express the generic, we shadow the concrete type aliases that actually
appear in user code — `PgPool`, `SqlitePool`, `MySqlPool` — and generate a full
stateful plugin for each one via the `impl_driver_plugin!` macro.

Each driver plugin (`SqlxPgPlugin`, `SqlxSqlitePlugin`, `SqlxMySqlPlugin`) holds:

- `Arc<Mutex<HashMap<Uuid, Pool>>>` — named persistent pools
- `Arc<Mutex<HashMap<Uuid, Transaction<'static, Db>>>>` — open transactions

Pools are `Clone` (Arc-backed), so the lock is released before any async call.
Transactions are `!Clone`, so they are removed from the registry, used, and
re-inserted — ensuring the lock is never held across an `await` point.

### The 14-tool surface

All tool names follow the pattern `{prefix}__{verb}`, where the prefix is chosen
at registration time (e.g. `pg`, `sqlite`, `mysql`).

| Category | Tool | Description |
|---|---|---|
| Pool | `*__connect` | Establish pool from URL; returns `ConnectResult { pool_id }` |
| Pool | `*__disconnect` | Remove and close a named pool |
| Pool | `*__pool_stats` | Returns `{ size, num_idle, is_closed }` |
| Direct | `*__execute` | Execute non-returning SQL; returns `QueryResultData` |
| Direct | `*__fetch_all` | SELECT returning all rows as `Vec<RowData>` |
| Direct | `*__fetch_one` | SELECT returning first row; errors if none |
| Direct | `*__fetch_optional` | SELECT returning first row or `null` |
| Transaction | `*__begin` | Begin transaction; returns `BeginResult { tx_id }` |
| Transaction | `*__commit` | Commit and remove a transaction |
| Transaction | `*__rollback` | Roll back and remove a transaction |
| Tx SQL | `*__tx_execute` | Execute non-returning SQL inside a transaction |
| Tx SQL | `*__tx_fetch_all` | SELECT inside a transaction |
| Tx SQL | `*__tx_fetch_one` | SELECT one row inside a transaction |
| Tx SQL | `*__tx_fetch_optional` | SELECT optional row inside a transaction |

### Why not the `Any` driver here?

`SqlxPlugin` uses `AnyPool`, which works with any backend but requires
`sqlx::any::install_default_drivers()` at runtime and routes through a dynamic
dispatch layer.  Driver-specific plugins bypass that layer, using the native
`PgPool`/`SqlitePool`/`MySqlPool` API directly.  This also gives access to
driver-specific result data — for example:

| Driver | `last_insert_id` |
|---|---|
| Postgres | None (use `RETURNING id` in your SQL) |
| SQLite | `last_insert_rowid() → i64` |
| MySQL | `last_insert_id() → u64` (cast to i64) |

Each driver's row decoder dispatches on `TypeInfo::name()` strings appropriate
for that engine rather than the `AnyValueKind` enum.

### `DriverKind` — the verification bridge for drivers

The `DriverKind` Select enum in the `elicitation` crate (under the `sqlx-types`
feature) names the three supported drivers with full Kani, Creusot, and Verus
coverage:

```rust
use elicitation::DriverKind;

let k = DriverKind::Postgres;
assert_eq!(k.url_scheme(), "postgres");
assert_eq!(k.feature_name(), "postgres");
```

Formally verified properties:
- Label count equals option count (3 variants, de-trusted in Creusot)
- Known labels round-trip through `from_label()`
- Unknown labels return `None`

---

## What we chose not to shadow, and why

### Generic `Pool<Db>` and `Connection<Db>` as open generics

The generic forms (`Pool<Postgres>`, `Pool<Sqlite>`, etc.) cannot appear in MCP
tool schemas without a concrete type argument — and that argument belongs to the
user's binary, invisible to the MCP server.  We solve this through two paths:

- **`AnyPool`** (`SqlxPlugin`) — erases the driver at runtime; one tool set
  covers all backends.
- **Monomorphized plugins** (`SqlxPgPlugin`, `SqlxSqlitePlugin`, `SqlxMySqlPlugin`)
  — each shadows the concrete type alias for one backend, giving full stateful
  pool and transaction support with driver-specific fidelity.

The open generic `Pool<DB>` itself is never surfaced at the MCP boundary.

### `Query<'q, DB, Args>` and the typed query family

sqlx's `query()`, `query_as()`, and `query_scalar()` return builder types
generic over lifetime, database, and argument list.  These cannot be serialized
or passed as tool arguments.  We solve this in two complementary ways:

1. **Runtime tools** (`SqlxPlugin`) — accept a raw `{ database_url, sql }` pair
   and execute immediately, returning serializable results.  The builder state
   lives only in the tool implementation, never at the MCP boundary.
2. **Fragment tools** (`SqlxFragPlugin`) — emit Rust source code wrapping the
   compile-time macros (`query!`, `query_as!`, etc.) for inclusion in user
   binaries.  This is the correct solution for code paths that need compile-time
   query verification; the agent composes the fragment and hands it to
   `std__assemble`.

### `sqlx::Row` as a trait object

The `Row` trait is object-unsafe (it is generic over the column accessor), so
`Box<dyn Row>` does not exist.  We work entirely with the concrete `AnyRow`
type, which is what all `Any`-driver queries return.

### `Migrate` and `Migrator`

sqlx's migration types do their real work at compile time (via `migrate!`).
We expose migration as a fragment tool (`sqlx__migrate`) that emits the correct
`sqlx::migrate!` invocation for assembly into a binary.  Runtime-only migration
workflows can also use the `execute` tool directly.

---

## Newtype wrappers

| Wrapper | Inner type | Construction | Notes |
|---|---|---|---|
| [`AnyRow`] | `sqlx::any::AnyRow` | Hand-written (`Arc`) | Non-Clone inner; `to_row_data()` is the MCP exit |
| [`AnyColumn`] | `sqlx_core::any::AnyColumn` | `elicit_newtype!` | Imported from `sqlx_core` directly |
| [`AnyTypeInfo`] | `sqlx::any::AnyTypeInfo` | `elicit_newtype!` | `kind()` returns `SqlTypeKind` |
| [`AnyQueryResult`] | `sqlx::any::AnyQueryResult` | `elicit_newtype!` | `to_result_data()` for MCP transport |
| [`SqlxError`] | `sqlx::Error` | `elicit_newtype!` | `kind_label()` + `message()` for diagnosis |

Transport types (fully serializable, live at the MCP boundary):

| Type | Description |
|---|---|
| [`RowData`] | Owned row snapshot: `Vec<ColumnEntry>` |
| [`ColumnEntry`] | Column name + `ColumnValue` |
| [`ColumnValue`] | Serializable mirror of `AnyValueKind` |
| [`ColumnDescriptor`] | Column name, ordinal, and `SqlTypeKind` |
| [`QueryResultData`] | `rows_affected` + `last_insert_id` |

Select enums (formally verified via Kani/Creusot/Verus):

| Enum | Source | Variants |
|---|---|---|
| [`SqlTypeKind`] | `elicitation::SqlTypeKind` | Maps `AnyTypeInfoKind` |
| [`ErrorKind`] (in `elicitation`) | `sqlx::error::ErrorKind` | Maps sqlx error categories |
| [`DriverKind`] (in `elicitation`) | `elicitation::DriverKind` | Postgres, Sqlite, MySql |

---

## Any-driver runtime tools (`SqlxPlugin`)

All five tools accept `{ database_url, sql? }` and open a short-lived pool
per call — no persistent connection state is required.  These use the `Any`
driver, so one tool set works against Postgres, SQLite, and MySQL alike.

| Tool | Description |
|---|---|
| `sqlx__connect_check` | Verify a URL is reachable |
| `sqlx__execute` | Execute a non-returning statement; returns `QueryResultData` |
| `sqlx__fetch_all` | SELECT returning all rows as `Vec<RowData>` |
| `sqlx__fetch_one` | SELECT returning first row; errors if none found |
| `sqlx__fetch_optional` | SELECT returning first row or `null` |

For persistent pools, transaction support, or driver-specific result data, use
the [driver-specific plugins](#driver-specific-plugins) instead.

---

## Fragment tools (`SqlxFragPlugin`)

Fragment tools emit Rust source code for sqlx compile-time macros.  They do
not execute SQL — they return source fragments for the agent to assemble into
a binary via `std__assemble`.

| Tool | Emits | Notes |
|---|---|---|
| `sqlx_frag__query` | `sqlx::query!(sql, params…)` | Requires `DATABASE_URL` at compile time |
| `sqlx_frag__query_as` | `sqlx::query_as!(Type, sql, params…)` | Target type must impl `sqlx::FromRow` |
| `sqlx_frag__query_scalar` | `sqlx::query_scalar!(sql, params…)` | Returns a single scalar |
| `sqlx_frag__migrate` | `sqlx::migrate!("path")` + `.run(&pool)` | Path is relative to crate root |

The fragment pattern handles the fundamental tension in sqlx's design: its
most powerful feature (compile-time query verification against a live schema)
requires the user's binary to be compiled with `DATABASE_URL` set — something
that cannot happen inside the MCP server process.  Fragment emission sidesteps
this: the agent composes the verified macro call, and the user's binary carries
out the compile-time check.

---

## Formal verification

All sqlx `Select` enum types in the `elicitation` crate have Kani, Creusot, and
Verus proofs covering:

- **Label count** — `labels().len() == options().len()` (de-trusted in Creusot
  via `extern_spec!` axioms in `elicitation_creusot/extern_specs.rs`)
- **Roundtrip** — every label returned by `labels()` is accepted by `from_label()`
- **Rejection** — `from_label("__unknown__")` returns `None`

The `ColumnValue` and `RowData` transport types have Verus proofs on their
constructors, and `AnyQueryResult`'s field accessors are covered by Kani
harnesses.  See `FORMAL_VERIFICATION_LEGOS.md` for the compositional proof
strategy.
