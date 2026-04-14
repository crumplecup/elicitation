# archive — Feature Parity Plan

> **Goal:** Bring `archive` to feature parity with pgAdmin 4, DBeaver CE,
> TablePlus, and DataGrip across all three frontends (ratatui, browser, egui).
>
> **Constraint:** Every user-visible action must flow through the
> `Established<P>` proof chain. UI layers read descriptors and invoke
> `ArchiveDbBackend` traits; they never issue raw SQL directly.

---

## Competitor Snapshot

| Feature area | pgAdmin 4 | DBeaver CE | TablePlus | DataGrip |
|---|:---:|:---:|:---:|:---:|
| Schema/table tree | ✅ | ✅ | ✅ | ✅ |
| Data grid (row viewer) | ✅ | ✅ | ✅ | ✅ |
| Inline row edit/insert/delete | ✅ | ✅ | ✅ | ✅ |
| SQL editor + run | ✅ | ✅ | ✅ | ✅ |
| Syntax highlighting | ✅ | ✅ | ✅ | ✅ |
| Query history | ✅ | ✅ | ✅ | ✅ |
| Saved queries / snippets | ✅ | ✅ | ✅ | ✅ |
| EXPLAIN plan (text) | ✅ | ✅ | ✅ | ✅ |
| EXPLAIN plan (visual) | ✅ | ✅ | ✅ | ✅ |
| DDL viewer | ✅ | ✅ | ✅ | ✅ |
| FK relationship display | ✅ | ✅ | ✅ | ✅ |
| ERD diagram | ✅ | ✅ | ❌ | ✅ |
| Index details panel | ✅ | ✅ | ✅ | ✅ |
| Column statistics (pg_stats) | ✅ | ✅ | ❌ | ✅ |
| Constraint viewer | ✅ | ✅ | ✅ | ✅ |
| Object search | ✅ | ✅ | ✅ | ✅ |
| CSV / JSON export | ✅ | ✅ | ✅ | ✅ |
| Multi-connection | ✅ | ✅ | ✅ | ✅ |
| Function / procedure browser | ✅ | ✅ | ✅ | ✅ |
| Trigger browser | ✅ | ✅ | ✅ | ✅ |
| Sequence browser | ✅ | ✅ | ❌ | ✅ |
| Extension browser | ✅ | ✅ | ❌ | ✅ |
| Enum / domain browser | ✅ | ✅ | ❌ | ✅ |
| Role / privilege matrix | ✅ | ✅ | ❌ | ✅ |
| Live server monitoring | ✅ | ✅ | ❌ | ❌ |
| Lock wait viewer | ✅ | ✅ | ❌ | ❌ |
| Backup / restore UI | ✅ | ✅ | ❌ | ❌ |
| Replication status | ✅ | ❌ | ❌ | ❌ |
| SSH tunnel | ✅ | ✅ | ✅ | ✅ |
| SSL connection config | ✅ | ✅ | ✅ | ✅ |
| Query plan comparison | ❌ | ✅ | ❌ | ✅ |
| Dark theme | ✅ | ✅ | ✅ | ✅ |

### Current `archive` status

✅ Schema/table tree (all 3 frontends) — ✅ Column metadata — ✅ Index metadata
✅ Basic SQL execute (MCP tool) — ✅ Spatial column detection — ✅ Backend traits
for all operations (not yet surfaced in UI)

**IR bridge contract (complete):** `IrSourced` proof token added to `elicit_ui`.
`ArchiveNavModel::to_verified_tree()` is the sole mint point.  All three frontends
(ratatui, egui, leptos) now gate rendering on `Established<IrSourced>` — divergence
from the AccessKit IR is a compile error.  ~2 700 lines of per-frontend draw code
replaced by bridge calls; overlay state consolidated in `ArchiveNavModel`.

The browser frontend is now a **dynamic axum server** (rebuilds IR per request)
rather than a one-shot static render.  Remaining leptos work is feature parity
for the 7 feature panels (see leptos-* todos below).

---

## Architecture Principles

All phases follow the same composition pattern:

```text
User action (keypress / click)
  │
  ▼
ArchiveNavModel / UI action handler
  │  (calls ArchiveDbBackend trait method)
  ▼
Established<P> proof returned
  │
  ▼
Descriptor type (ElicitComplete)
  │
  ├─ ratatui: TuiNode rendered inline
  ├─ browser: AccessKit IR → Leptos HTML
  └─ egui: rendered in central panel
```

New descriptor types are added to `types.rs` and implement `ElicitComplete`.
New MCP plugins follow the `#[elicit_tool]` + `ElicitPlugin` pattern.
UI changes are made once to `ArchiveNavModel` / layout helpers and propagate
to all three renderers.

---

## Phase 1 — Interactive Data Browsing
>
> Closes every "critical blocking" gap. Brings basic daily-driver usability.

### 1.1  Data Grid Panel

**What:** When a table row is selected (Enter), fetch up to N rows via
`DbQueryExecutor::query_rows` and render them in a paginated grid panel.

**New types:**

```rust
pub struct DataGridDescriptor {
    pub table: TableDescriptor,
    pub rows: DbRows,
    pub page: u32,
    pub page_size: u32,
    pub total_estimate: Option<i64>,
}
```

**New MCP tool:** `archive_browse__preview_table` already exists in
`plugins/query.rs` — wire its output to the central panel instead of
discarding it.

**Frontend work:**

- ratatui: `Table` widget with column widths, scrollable, page up/down
- egui: `egui_extras::TableBuilder` (add `egui_extras` dep with `table` feature)
- browser: `<table>` rendered from AccessKit `Role::Grid` / `Role::Row` nodes

**Key bindings (add to `StatusBarDescriptor::archive_browse`):**

- `PgDn` / `PgUp` — next/previous page
- `g` / `G` — first / last page

---

### 1.2  SQL Editor Pane

**What:** A multi-line text input area the user can type SQL into. `Ctrl+Enter`
or `F5` runs the query; results appear in the data grid panel below.

**New types:**

```rust
pub struct SqlEditorState {
    pub text: String,
    pub cursor_line: u32,
    pub cursor_col: u32,
    pub history: Vec<String>,      // last N queries
    pub history_cursor: Option<usize>,
}
```

**New MCP tool:** (reuse `archive_query__execute`)

**Frontend work:**

- ratatui: `tui-textarea` crate (already used in the ecosystem) or manual
  multi-line `TextArea` widget wrapping a `String` buffer; `Ctrl+Enter` runs
- egui: `egui::TextEdit::multiline` in a resizable panel; capture
  `Modifiers::CTRL + Key::Enter`
- browser: `<textarea>` + JS `keydown` handler for Ctrl+Enter

**Panel layout change:** Split central panel vertically — top half = editor,
bottom half = result grid.  Toggle with `Tab` or a toolbar button.

**Key bindings:**

- `Ctrl+Enter` / `F5` — execute
- `Ctrl+L` — clear editor
- `↑` / `↓` in empty editor — cycle query history

---

### 1.3  Live Tree Refresh

**What:** `r` in the nav panel re-queries the backend and rebuilds
`ArchiveNavModel` without restarting the frontend.

**Change:** `ArchiveNavModel::refresh()` currently sets a flash string. Replace
with an async callback pattern:

- ratatui / egui: `refresh()` signals a `needs_refresh: bool` flag; the event
  loop calls `build_nav_tree(&backend).await` and replaces `model.schemas`
- browser: POST `/api/refresh` which re-runs the SSR pipeline

---

### 1.4  Object Search / Filter

**What:** `/` or `Ctrl+F` opens a search bar above the nav panel. Typing
filters the flat list to matching schema/table names (substring, case-insensitive).

**Change to `ArchiveNavModel`:**

```rust
pub struct ArchiveNavModel {
    // ... existing fields ...
    pub filter: String,          // current filter text
    pub filter_active: bool,     // whether search bar is open
    pub flat_unfiltered: Vec<FlatItem>,   // full flat list
}
```

`rebuild_flat()` applies the filter when `filter_active` and `!filter.is_empty()`.

**Key bindings (add to `StatusBarDescriptor`):**

- `/` — open filter
- `Esc` (in filter) — clear and close filter

---

## Phase 2 — Rich Object Inspection
>
> Surfaces the object metadata already queryable via `ArchiveDbBackend` traits.

### 2.1  DDL Viewer

**What:** Press `d` on any selected table/view to show the `CREATE TABLE` DDL
in the central panel.

**New MCP tool:** `archive_browse__generate_ddl`

**Implementation:** Build DDL string from `TableDescriptor` (column definitions,
PK, FK, NOT NULL, defaults, indexes). For PostgreSQL, can also run
`pg_get_tabledef` or reconstruct from `information_schema`.

**New type:**

```rust
pub struct DdlDescriptor {
    pub schema: String,
    pub object_name: String,
    pub ddl: String,   // CREATE TABLE … AS TEXT
}
```

**Key binding:** `d` — show DDL for selected object

---

### 2.2  Foreign Key Relationships

**What:** Show FK references (both inbound and outbound) in the table detail
panel. Navigate to the referenced table with Enter.

**New type:**

```rust
pub struct ForeignKeyDescriptor {
    pub constraint_name: String,
    pub from_schema: String,
    pub from_table: String,
    pub from_columns: Vec<String>,
    pub to_schema: String,
    pub to_table: String,
    pub to_columns: Vec<String>,
    pub on_delete: FkAction,  // Cascade | SetNull | Restrict | NoAction
    pub on_update: FkAction,
}

pub enum FkAction { Cascade, SetNull, Restrict, NoAction, SetDefault }
```

**`ColumnDescriptor` change:** Replace `is_foreign_key: bool` with
`foreign_key: Option<FkTarget>` where `FkTarget` names the referenced table +
column.

**New MCP tool:** `archive_browse__list_foreign_keys`

**New query:** `information_schema.referential_constraints` +
`key_column_usage`.

---

### 2.3  Constraint Viewer

**What:** Panel tab showing all constraints on a table: PK, FK, UNIQUE, CHECK,
EXCLUSION (PG-specific).

**New type:**

```rust
pub struct ConstraintDescriptor {
    pub name: String,
    pub kind: ConstraintKind,
    pub columns: Vec<String>,
    pub definition: Option<String>,   // CHECK expression etc.
}

pub enum ConstraintKind { PrimaryKey, ForeignKey, Unique, Check, Exclusion }
```

**New MCP tool:** `archive_browse__list_constraints`

---

### 2.4  Index Details in Panel

**What:** Index tab in the table detail panel.  Already queryable via
`DbIndexManager::list_indexes` — just wire to UI.

**Frontend change:** Add "Indexes" tab in central panel alongside "Columns".
Render `Vec<IndexDescriptor>` as a table showing name, columns, type, unique.

---

### 2.5  Column Statistics (PostgreSQL)

**What:** For PostgreSQL, show `pg_stats` data per column: null fraction,
distinct count, most-common values, histogram.

**New type:**

```rust
pub struct ColumnStats {
    pub column_name: String,
    pub null_fraction: f64,
    pub avg_width: i32,
    pub n_distinct: f64,         // negative = fraction of total rows
    pub most_common_vals: Vec<String>,
    pub histogram_bounds: Vec<String>,
    pub correlation: Option<f64>,
}
```

**New MCP tool:** `archive_browse__column_stats`  (queries `pg_stats`)

---

### 2.6  EXPLAIN Plan Viewer

**What:** `Ctrl+E` in the SQL editor runs `EXPLAIN (ANALYZE, FORMAT TEXT)` and
shows the plan in the result panel. A tree-structured text render for ratatui/egui,
a collapsible `<details>` tree for the browser.

**New type:**

```rust
pub struct ExplainNode {
    pub node_type: String,
    pub relation: Option<String>,
    pub startup_cost: f64,
    pub total_cost: f64,
    pub plan_rows: u64,
    pub actual_rows: Option<u64>,
    pub actual_time_ms: Option<f64>,
    pub children: Vec<ExplainNode>,
    pub extra: Vec<(String, String)>,   // filter, index cond, etc.
}
```

**New MCP tool:** `archive_query__explain` (wraps `DbQueryExecutor::explain`)

**Parsing:** Parse `EXPLAIN (FORMAT JSON)` output into `ExplainNode` tree.

**Key binding:** `Ctrl+E` — explain current SQL editor contents

---

## Phase 3 — Power-User Features

### 3.1  Inline Row Edit / Insert / Delete

**What:** In the data grid, press `e` to edit the selected cell in-place, `i`
to insert a new row, `Delete` to mark a row for deletion.  Changes are staged
and committed with `Ctrl+S`, rolled back with `Esc`.

**Architecture:** Uses `DbTransactor::begin` typestate machine:

```text
begin(ReadCommitted) → TxMarker<Open>
  ├─ execute(UPDATE …) → DbExecuteResult
  ├─ execute(INSERT …) → DbExecuteResult
  ├─ execute(DELETE …) → DbExecuteResult
  └─ commit(handle)   → TxMarker<Committed>
              or rollback → TxMarker<RolledBack>
```

**New MCP tool:** `archive_query__edit_row` (wraps execute in transaction)

**Key bindings:**

- `e` — edit selected cell
- `i` — insert new row
- `Delete` — mark row for deletion
- `Ctrl+S` — commit pending changes
- `Esc` — rollback / discard

---

### 3.2  Query History

**What:** `archive` persists the last N queries in a local SQLite file
(`~/.config/archive/history.db`). Navigate with `Ctrl+↑` / `Ctrl+↓` in the
SQL editor.

**Persistence:** `elicit_db` + `elicit_sqlx` can manage the history database
(SQLite). Schema: `(id, timestamp, sql_text, duration_ms, row_count, error)`.

**New type:**

```rust
pub struct QueryHistoryEntry {
    pub id: i64,
    pub executed_at: DateTime<Utc>,
    pub sql: String,
    pub duration_ms: u64,
    pub row_count: Option<u64>,
    pub error: Option<String>,
}
```

---

### 3.3  Saved Queries / Snippets

**What:** Name and persist frequently-used queries. Browse in a sidebar panel.

**Persistence:** Same local SQLite as history. Schema:
`(id, name, description, sql_text, tags)`.

**Key binding:** `Ctrl+Shift+S` — save current query with name prompt.

---

### 3.4  CSV / JSON / TSV Export

**What:** In the data grid, `x` opens an export dialog:

- Format: CSV, JSON array, NDJSON, TSV
- Destination: clipboard, file path, stdout

**New MCP tool:** `archive_query__export` (streams rows from query, formats output)

---

### 3.5  Multi-Connection Management

**What:** Maintain a named list of connection profiles. Switch connections with
`Ctrl+Tab`. Each connection has its own `ArchiveDbBackend` + `ArchiveNavModel`.

**New type:**

```rust
pub struct ConnectionProfile {
    pub name: String,
    pub url_env_key: String,    // env var, never stored raw
    pub backend: BackendKind,
    pub color: Option<String>,  // Catppuccin accent for tab badge
}
```

**Architecture change:** `ArchiveEguiApp` / `TuiApp` hold
`Vec<(ConnectionProfile, ArchiveNavModel)>` with an `active: usize` index.

---

## Phase 4 — Advanced Object Types (PostgreSQL-Specific)

These surface Postgres-specific objects that pgAdmin handles but simpler tools
don't bother with.

### 4.1  Function / Procedure Browser

**New type:** `FunctionDescriptor` (name, schema, return type, argument list,
language, volatility, body preview)

**Nav tree extension:** Add `Functions` node under each schema (collapsed by
default). Expand to list functions. Press Enter to view DDL.

**New MCP tools:** `archive_browse__list_functions`,
`archive_browse__describe_function`

---

### 4.2  Trigger Browser

**New type:** `TriggerDescriptor` (name, table, event, timing, function,
enabled)

**New MCP tools:** `archive_browse__list_triggers`

---

### 4.3  Sequence Browser

**New type:** `SequenceDescriptor` (name, schema, current value, start,
increment, min, max, cycle)

**New MCP tools:** `archive_browse__list_sequences`

---

### 4.4  Type Browser (Enum, Domain, Composite)

**New types:** `EnumDescriptor`, `DomainDescriptor`, `CompositeTypeDescriptor`

**New MCP tools:** `archive_browse__list_types`

---

### 4.5  Extension Browser

**Nav tree extension:** Top-level "Extensions" node under the database.
Uses existing `DbServerAdmin::list_extensions`.

---

## Phase 5 — Monitoring & Administration Dashboard

### 5.1  Live Activity Monitor

**What:** A dedicated "Monitor" panel showing active sessions, long-running
queries, lock waits, cache hit ratio in real time (polling interval: 5s).

**Backend:** All of `DbMonitor`'s methods are already implemented:
`active_sessions`, `slow_queries`, `table_bloat`, `index_usage`, `lock_waits`,
`cache_hit_ratio`.

**Frontend:**  

- ratatui: sparkline widgets for cache hit ratio, table for sessions  
- egui: `egui::plot` (or `egui_plot`) for time-series metrics  
- browser: SSE-driven live update via Axum

---

### 5.2  Role / Privilege Matrix

**What:** Table of roles vs. objects showing GRANT/REVOKE status. Edit cells to
change privileges.

**Backend:** Uses `DbRoleManager::list_roles`, `grant`, `revoke`.

---

### 5.3  Backup / WAL Status

**What:** Show last backup label + time, WAL status. Button to initiate a new
backup.

**Backend:** Uses `DbBackupManager::initiate_backup`, `list_backups`,
`wal_status`.

---

### 5.4  Server Settings Viewer

**What:** Browse `pg_settings` with search. Mark which settings require restart.

**Backend:** Uses `DbServerAdmin::list_settings`.

---

## Phase 6 — ERD Diagram

**What:** A relationship diagram showing tables as boxes with FK arrows between
them. Pan, zoom, click to open table detail.

**Scope decision:** Start with a static SVG export (simpler) before investing in
an interactive layout. Use `petgraph` for the graph + a simple force-directed
layout algorithm.

**Frontend:**

- ratatui: text-art boxes connected by ASCII lines (minimal, opt-in)
- egui: `egui_graphs` or custom `egui::Painter` rendering
- browser: SVG embedded in HTML (clickable `<a>` anchors on table boxes)

**New MCP tool:** `archive_browse__generate_erd` — emits SVG string from FK
graph

---

## Implementation Order and Dependencies

```text
Phase 1.3 (live refresh)           ← no deps, 1 day
Phase 1.4 (object search)          ← no deps, 1 day
Phase 2.4 (index details panel)    ← no deps (data already fetched)
Phase 2.2 (FK descriptors)         ← types change, 2 days
Phase 2.3 (constraint viewer)      ← 1 day
Phase 2.1 (DDL viewer)             ← FK types, 2 days
Phase 1.1 (data grid)              ← 3 days
Phase 1.2 (SQL editor)             ← data grid, 3 days
Phase 2.5 (column stats)           ← PG-only, 1 day
Phase 2.6 (EXPLAIN viewer)         ← SQL editor, 2 days
Phase 3.4 (CSV export)             ← data grid, 1 day
Phase 3.2 (query history)          ← SQL editor, 2 days
Phase 3.3 (saved queries)          ← query history, 1 day
Phase 3.1 (row edit)               ← data grid + transactions, 3 days
Phase 3.5 (multi-connection)       ← nav model refactor, 2 days
Phase 4.1–4.5 (PG object types)    ← nav tree extension, 1 day each
Phase 5.1–5.4 (monitoring)         ← all backend traits exist, 4 days
Phase 6   (ERD)                    ← FK types, petgraph, 5 days
```

---

## Data Model Extensions Summary

| New type | Phase | Affects |
|---|---|---|
| `DataGridDescriptor` | 1.1 | types.rs + all 3 frontends |
| `SqlEditorState` | 1.2 | ratatui/egui/browser |
| `DdlDescriptor` | 2.1 | types.rs |
| `ForeignKeyDescriptor` | 2.2 | types.rs, ColumnDescriptor |
| `ConstraintDescriptor`, `ConstraintKind` | 2.3 | types.rs |
| `ColumnStats` | 2.5 | types.rs |
| `ExplainNode` | 2.6 | types.rs |
| `QueryHistoryEntry` | 3.2 | local SQLite |
| `ConnectionProfile` | 3.5 | nav model |
| `FunctionDescriptor` | 4.1 | types.rs |
| `TriggerDescriptor` | 4.2 | types.rs |
| `SequenceDescriptor` | 4.3 | types.rs |
| `EnumDescriptor` etc. | 4.4 | types.rs |

---

## New MCP Plugins / Tools Summary

| Plugin | New tools |
|---|---|
| `archive_browse` | `generate_ddl`, `list_foreign_keys`, `list_constraints`, `column_stats`, `list_functions`, `describe_function`, `list_triggers`, `list_sequences`, `list_types`, `generate_erd` |
| `archive_query` | `explain`, `edit_row`, `export` |
| `archive_monitor` | (new plugin) `active_sessions`, `slow_queries_dashboard`, `cache_hit_ratio`, `lock_waits_dashboard`, `table_bloat_report`, `index_usage_report` |
| `archive_admin` | (new plugin) `backup_status`, `initiate_backup`, `role_matrix`, `server_settings` |

---

## What Makes `archive` Distinct

Even at full parity with pgAdmin, `archive` has properties its competitors
cannot match:

1. **Proof-carrying operations** — every destructive action (DROP, TRUNCATE,
   DELETE, GRANT) returns `Established<AuditLogged>`. You can't accidentally
   drop a table without the event being recorded in the proof chain.

2. **MCP-first** — every operation is also an MCP tool. An AI agent can browse
   your database schema, run explains, compare query plans, and generate DDL
   just by calling the same tools the UI does. pgAdmin has no equivalent.

3. **Three parallel renderers, one model** — `ArchiveNavModel` and all
   descriptor types are frontend-agnostic. Adding a new object type
   automatically gets ratatui + egui + browser rendering with no extra code
   beyond the three rendering leaves.

4. **Formal verification coverage** — core operations in `elicit_db` /
   `elicit_sqlx` carry Kani / Creusot / Prusti proofs. Production correctness
   guarantees that pgAdmin (Python + Flask) cannot provide.
