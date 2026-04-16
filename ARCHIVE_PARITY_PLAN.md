# archive — Feature Parity Plan (Phase 7+)

> **Goal:** Bring `archive` to feature parity with pgAdmin 4, DBeaver CE,
> TablePlus, and DataGrip across all three frontends (ratatui, browser, egui).
>
> **Constraint:** Every user-visible action must flow through the
> `Established<P>` proof chain and the AccessKit IR. UI layers read
> `ArchiveDisplay::to_ak_nodes` output; they never produce widgets or HTML
> from raw data. Adding a new feature means: implement `ArchiveDisplay` for
> the type, extend `PanelMode` or the nav tree, wire `build_content_nodes`,
> done — all three frontends pick it up for free.

---

## Current Status (as of Phase 6)

### ✅ Completed phases

| Phase | Feature |
|---|---|
| 1.1 | Data grid with pagination |
| 1.2 | SQL editor + results |
| 1.3 | Live tree refresh (`r`) |
| 1.4 | Nav filter (`/`) |
| 2.1 | DDL viewer (`d`) |
| 2.2 | FK descriptors (`ForeignKeyDescriptor`) |
| 2.5 | Column statistics (`ColumnStats`) |
| 2.6 | EXPLAIN plan viewer |
| 3.1 | Inline row edit/insert/delete |
| 3.2 | Query history |
| 3.3 | Saved queries |
| 3.4 | CSV/JSON export |
| 3.5 | Multi-connection (`Ctrl+Tab`) |
| 4.1 | Function browser (nav tree + MCP) |
| 4.3 | Sequence browser (nav tree + MCP) |
| 4.4 | Type browser — enum, domain, composite (nav tree + MCP) |
| 4.5 | Extension list (in Admin panel) |
| 5.1 | Monitor panel — sessions, roles, cache, backups (`m`) |
| 5.2–5.4 | Admin panel — roles, backups, WAL, extensions, settings (`a`) |
| 6 | ERD diagram — table list + FK edges (`g`) |

### ⚠️ Partial (MCP tools exist, UI not surfaced)

- **2.3 Constraint viewer** — `ConstraintDescriptor` + `ArchiveConstraintPlugin` exist; no panel
- **2.4 Index details panel** — `IndexDescriptor` fetched inside inspect path; never rendered standalone
- **4.2 Trigger browser** — `TriggerDescriptor` + `list_triggers` MCP tool exist; no nav tree node
- **4.5 Extension nav node** — extensions visible in admin panel only; no top-level nav tree node
- **5.1 Monitor depth** — `MonitorSnapshot` only shows sessions/roles/cache/backups; `slow_queries`,
  `lock_waits`, `table_bloat`, `index_usage` are MCP tools but not in the panel UI
- **6 ERD is_fk** — `ErdColumn.is_fk` hardcoded `false`; FK participation not back-propagated from edges

### ❌ Not yet implemented

- Data-grid pagination keybindings (`PgDn`/`PgUp`, first/last page)
- SSE live polling for monitor (browser frontend only)
- Syntax highlighting in SQL editor (all frontends)
- SSH tunnel + SSL cert fields in `ConnectionProfile`
- Visual ERD layout (petgraph + force-directed, SVG browser, egui Painter)
- Query plan comparison (DBeaver/DataGrip feature)
- Theme toggle (dark/light; currently hardcoded Catppuccin Mocha in egui, none in ratatui/browser)

### ❌ `ArchiveDisplay` gap — the core deficit

The `ArchiveDisplay` trait (`to_ak_nodes`) is the contract that lets the IR
pipeline surface any type in all three frontends. Only **5 of ~25 types** in
`types.rs` implement it. Every type without an impl is invisible to the UI and
to `ArchiveDisplayPlugin` (MCP rendering). This is the root cause of most
"partial" items above.

| Type | Has `ArchiveDisplay` |
|---|:---:|
| `ColumnDescriptor` | ✅ |
| `DatabaseDescriptor` | ✅ |
| `QueryResult` | ✅ |
| `SchemaDescriptor` | ✅ |
| `TableDescriptor` | ✅ |
| `ForeignKeyDescriptor` | ❌ |
| `ConstraintDescriptor` | ❌ |
| `DdlDescriptor` | ❌ |
| `TableInspection` | ❌ |
| `IndexDescriptor` | ❌ |
| `ColumnStats` | ❌ |
| `ExplainNode` | ❌ |
| `QueryHistoryEntry` | ❌ |
| `SavedQuery` | ❌ |
| `StagedEdit` / `RowEditState` | ❌ |
| `ConnectionProfile` | ❌ |
| `FunctionDescriptor` | ❌ |
| `TriggerDescriptor` | ❌ |
| `SequenceDescriptor` | ❌ |
| `EnumDescriptor` | ❌ |
| `DomainDescriptor` | ❌ |
| `CompositeTypeDescriptor` | ❌ |
| `MonitorSnapshot` | ❌ |
| `AdminSnapshot` | ❌ |
| `ErdDiagram` / `ErdNode` / `ErdEdge` / `ErdColumn` | ❌ |

---

## Architecture Reminder

The display pipeline is:

```text
Descriptor type
  │
  └── impl ArchiveDisplay { to_ak_nodes(&self, mode, id_base) }
        │
        ▼
  build_content_nodes()   ← inserts nodes into the shared tree map
        │
        ▼
  to_verified_tree()      ← mints Established<IrSourced> proof token
        │
  ┌─────┼──────────────────────────┐
  │     │                          │
ratatui egui                    leptos
bridge  bridge           (Axum server — rebuilds IR per HTTP request)
```

Every new feature follows this flow. Implementing `ArchiveDisplay` is the
first step for any type. Once the impl exists, `build_content_nodes` can
call it and all three frontends render it. However, **Leptos is not free** —
it is a server-side request cycle, not a live event loop. Every new
`PanelMode` variant requires explicit Leptos work:

1. A new `A::Open*` arm in `dispatch_action_on_model()`
2. A new `async fn api_*` handler function
3. A new `GET /api/*` route registered in `build_router()`

ratatui and egui pick up new `PanelMode` arms automatically from
`build_content_nodes` via the shared in-process model. Leptos must be
wired separately for each panel variant.

---

## Phase 7 — `ArchiveDisplay` for all descriptor types

This is the prerequisite for Phases 8–10. Each type gets a new file in
`crates/elicit_server/src/archive/display/` and is registered in `display/mod.rs`.

### Pattern

```rust
// display/index.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum IndexDescriptorMode {
    #[default]
    Row,        // single list item for embedding in a parent list
    Detailed,   // expanded: name + columns + type + unique flag
}

impl ArchiveDisplay for IndexDescriptor {
    type Mode = IndexDescriptorMode;
    fn root_role(mode: &Self::Mode) -> Role { … }
    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) { … }
}
```

### Types and their natural modes

| Type | Modes | Root role |
|---|---|---|
| `ForeignKeyDescriptor` | `Inline` / `Detailed` | `Row` / `Group` |
| `ConstraintDescriptor` | `Inline` / `Detailed` | `Row` / `Group` |
| `DdlDescriptor` | `Block` | `GenericContainer` (preformatted text) |
| `TableInspection` | `FkList` / `ConstraintList` / `IndexList` | `List` |
| `IndexDescriptor` | `Row` / `Detailed` | `Row` / `Group` |
| `ColumnStats` | `Summary` / `Detailed` | `Article` / `Group` |
| `ExplainNode` | `TreeNode` (recursive) | `TreeItem` |
| `QueryHistoryEntry` | `Row` / `Detailed` | `Row` / `Article` |
| `SavedQuery` | `Row` / `Detailed` | `Row` / `Article` |
| `StagedEdit` | `Row` | `Row` |
| `ConnectionProfile` | `Card` / `Row` | `Article` / `Row` |
| `FunctionDescriptor` | `Row` / `Detailed` | `Row` / `Group` |
| `TriggerDescriptor` | `Row` / `Detailed` | `Row` / `Group` |
| `SequenceDescriptor` | `Row` / `Detailed` | `Row` / `Group` |
| `EnumDescriptor` | `Row` / `Detailed` | `Row` / `Group` |
| `DomainDescriptor` | `Row` / `Detailed` | `Row` / `Group` |
| `CompositeTypeDescriptor` | `Row` / `Detailed` | `Row` / `Group` |
| `MonitorSnapshot` | `Dashboard` / `SessionList` | `Group` / `List` |
| `AdminSnapshot` | `RoleList` / `BackupList` / `WalStatus` / `ExtList` / `Settings` | `List` / `Group` |
| `ErdDiagram` | `NodeList` / `EdgeList` / `Visual` | `List` / `Group` / `Figure` |
| `ErdNode` | `TableBox` | `Group` |
| `ErdEdge` | `Row` | `Row` |
| `ErdColumn` | `Row` | `Row` |

### Key design notes

**`ExplainNode` is recursive.** The impl must allocate a block of `id_base`
space large enough to cover the subtree, or use a counter passed by mutable
reference. Recommended: pass a `&mut u64` counter through a private helper:

```rust
fn node_to_ak(node: &ExplainNode, counter: &mut u64) -> (NodeId, Vec<(NodeId, NodeJson)>) { … }
```

**`DdlDescriptor` is verbatim text.** Use `Role::GenericContainer` with the
DDL as the label. Syntax highlighting is a Phase 12 concern — Phase 7 just
gets the text into the tree.

**`MonitorSnapshot` and `AdminSnapshot`** should produce self-contained
subtrees so that `build_content_nodes` can call them directly without
rebuilding the entire panel by hand.

**`ErdDiagram::Visual` mode** is a placeholder in Phase 7 — it emits a
`Role::Figure` with a `description` field containing a text summary. Phase 10
replaces it with actual coordinate data.

### `ArchiveDisplayPlugin` extension

Once impls are added, register them as new MCP display tools:

```rust
archive_display__foreign_keys    → Vec<ForeignKeyDescriptor>
archive_display__constraints     → Vec<ConstraintDescriptor>
archive_display__indexes         → Vec<IndexDescriptor>
archive_display__column_stats    → ColumnStats
archive_display__explain         → ExplainNode (recursive tree)
archive_display__history         → Vec<QueryHistoryEntry>
archive_display__saved           → Vec<SavedQuery>
archive_display__function        → FunctionDescriptor
archive_display__trigger         → TriggerDescriptor
archive_display__sequence        → SequenceDescriptor
archive_display__type            → EnumDescriptor | DomainDescriptor | CompositeTypeDescriptor
archive_display__monitor         → MonitorSnapshot
archive_display__admin           → AdminSnapshot
archive_display__erd             → ErdDiagram
```

---

## Phase 8 — Surface all partial features in the UI

With Phase 7 done, wiring each feature into the UI is a `build_content_nodes`
arm + optional `PanelMode` variant + key binding. Each item below is a
self-contained diff.

### 8.1 — Constraint panel

**Key binding:** `c` — show constraints for selected table
**New `PanelMode`:** `ConstraintPanel { table: TableDescriptor, constraints: Vec<ConstraintDescriptor> }`
**New `FetchRequest`:** `FetchConstraints { schema: String, table: String }`
**New `PanelEvent`:** `ConstraintsReady(Vec<ConstraintDescriptor>)`
**New HTMX route:** `GET /api/constraints?schema=&table=`
**`build_content_nodes` arm:** call `ConstraintDescriptor::to_ak_nodes` per row in a `Role::List`

### 8.2 — Index panel

**Key binding:** `i` — show indexes for selected table (currently `i` = column detail; remap or use `I`)
**New `PanelMode`:** `IndexPanel { table: TableDescriptor, indexes: Vec<IndexDescriptor> }`
**New `FetchRequest`:** `FetchIndexes { schema: String, table: String }`
**New HTMX route:** `GET /api/indexes?schema=&table=`
**`build_content_nodes` arm:** call `IndexDescriptor::to_ak_nodes` per row in a `Role::List`

### 8.3 — Trigger browser in nav tree

`TriggerDescriptor` already fetched by `inspect_table_direct`. The nav tree needs:

- `FlatItem::TriggersGroup(si)` — collapsible group under each schema
- `FlatItem::Trigger(si, ti)` — individual trigger
- `SchemaEntry` extended with `triggers: Vec<TriggerDescriptor>`
- `nav_tree.rs`: query `information_schema.triggers` alongside functions/sequences/types
- `toggle_expand` covers `TriggersGroup` via the same expand-flag pattern as `FunctionsGroup`
- `build_nav_nodes` arm for `Trigger` → calls `TriggerDescriptor::to_ak_nodes(&mode, id_base)`
- **Leptos:** `A::ExpandTriggers` action + `dispatch_action_on_model` arm; no dedicated panel, so no new `/api/` route needed (nav tree is rebuilt on tree-expand events shared across all three frontends)

### 8.4 — Extension nav tree node

Extensions are global (not per-schema). The plan calls for a top-level
"Extensions" node under the database node:

- `FlatItem::ExtensionsGroup` — top-level collapsible
- `FlatItem::Extension(idx)` — one per installed extension
- `DatabaseEntry` extended with `extensions: Vec<(String, String)>` (name, version)
- `nav_tree.rs`: populate from `DbServerAdmin::list_extensions()` in `build_nav_tree`
- `build_nav_nodes` arm: list items using `Role::ListItem`
- **Leptos:** same as 8.3 — nav tree expand is a model-side action; no separate `/api/` route needed since extensions are loaded at tree build time

### 8.5 — Data-grid pagination keybindings

Currently page navigation in `DataGrid` exists in the model
(`page_next`, `page_prev`) but has no key bindings. Add to `ArchiveKeyMap`:

```rust
KeyMapEntry::nav(p(K::PageDown), A::PageNext, "PgDn", "Next page", true),
KeyMapEntry::nav(p(K::PageUp),   A::PagePrev, "PgUp", "Prev page", true),
KeyMapEntry::nav(p(K::Home),     A::PageFirst, "Home", "First page", false),
KeyMapEntry::nav(p(K::End),      A::PageLast,  "End",  "Last page",  false),
```

Add `PageNext`, `PagePrev`, `PageFirst`, `PageLast` to `ArchiveAction`.
All three frontends pick up the bindings automatically via `resolve()`.

### 8.6 — FK is_fk enrichment in ERD

In `nav_tree::fetch_erd`, after building edges, enrich `ErdColumn.is_fk`:

```rust
for edge in &edges {
    if let Some(node) = nodes.iter_mut().find(|n| n.table_name == edge.from_table) {
        if let Some(col) = node.columns.iter_mut().find(|c| c.name == edge.from_col) {
            col.is_fk = true;
        }
    }
}
```

---

## Phase 9 — Monitor panel depth

The `MonitorSnapshot` today only captures sessions, roles, cache hit, and
backups. `ArchiveMonitorPlugin` already has MCP tools for `slow_queries`,
`lock_waits`, `table_bloat`, and `index_usage`. Phase 9 brings these into the
panel UI.

### 9.1 — Extend `MonitorSnapshot`

```rust
pub struct MonitorSnapshot {
    // existing fields …
    pub slow_queries: Vec<DbSlowQuery>,
    pub lock_waits: Vec<DbLockWait>,
    pub table_bloat: Vec<DbTableBloat>,
    pub index_usage: Vec<DbIndexUsage>,
}
```

Add the four sub-types to `types.rs` with `#[derive(Elicit)]` and
`ArchiveDisplay` impls in Phase 7.

### 9.2 — `MonitorTab` enum (mirrors `AdminTab`)

```rust
pub enum MonitorTab {
    Sessions,
    SlowQueries,
    LockWaits,
    TableBloat,
    IndexUsage,
}
```

`MonitorPanel` variant gains `active_tab: MonitorTab`. `[`/`]` cycle tabs
(same key bindings already used for AdminPanel — they're mode-aware).

### 9.3 — Fetch in all three frontends

`FetchMonitor` tasks in ratatui + egui call the four new backend methods.

**Leptos:** `GET /api/monitor` already exists (Phase 5). Extend the handler to call the four
new `ArchiveMonitorPlugin` tools and populate the new `MonitorSnapshot` fields. Add a
`MonitorTab` query param so the response renders the correct tab. No new route needed —
existing route gains tab-aware rendering.

### 9.4 — `build_content_nodes` arm update

The `MonitorPanel` arm calls `MonitorSnapshot::to_ak_nodes` with the active
tab as the mode discriminant, producing a `Role::List` per tab.

---

## Phase 10 — Visual ERD layout

The current ERD is a text IR: a list of table nodes and a list of FK edges.
Phase 10 replaces `ErdDiagram::Visual` mode with a real spatial layout.

### 10.1 — Layout algorithm

Add `petgraph` (already a transitive dep via `elicit_rstar`; confirm direct
dep) to `elicit_server`. Use `petgraph::Graph<ErdNode, ErdEdge>` as the
intermediate representation.

Layout: **Sugiyama/layer-based** (for DAG-like FK graphs) or a simple
**grid layout** (row × col based on table count) as a first pass.
Force-directed is visually nicer but harder to implement stably — defer to a
later polish iteration.

```rust
pub struct ErdLayout {
    pub positions: HashMap<String, (f32, f32)>,  // table_name → (x, y)
    pub diagram: ErdDiagram,
}
```

### 10.2 — Browser frontend: SVG rendering

`ErdDiagram::to_ak_nodes` in `Visual` mode produces:

```text
Role::Figure
  ├─ Role::Group (table box for each ErdNode)
  │    label: "<table_name>\n<col1>: <type>\n…"
  │    value: "x=120,y=340,w=200,h=120"   ← bounding box in description
  └─ Role::Group (edge for each ErdEdge)
       label: "from_table.from_col → to_table.to_col"
       value: "x1=320,y1=380,x2=500,y2=200"  ← line endpoints
```

The Leptos renderer translates `Role::Figure` children with coordinate
`value` fields into SVG `<rect>`, `<text>`, and `<line>` elements. FK edges
get clickable `<a>` anchors that navigate to the target table.

This keeps the SVG in the IR pipeline: the AccessKit tree holds the layout
data; the renderer decides the output format.

### 10.3 — egui frontend: Painter rendering

The egui bridge checks for `Role::Figure` children and uses `egui::Painter`
to draw rectangles and Bézier lines. Pan/zoom via `egui::ScrollArea`. Click
on a table box navigates the nav tree to that table.

### 10.4 — ratatui frontend: text-art layout

ratatui cannot render SVG. The ratatui bridge for `ErdPanel` renders a
simple grid of box-drawing characters:

```
┌──────────────┐     ┌──────────────┐
│ users        │────▶│ orders       │
│ id: int4 PK  │     │ user_id: FK  │
└──────────────┘     └──────────────┘
```

This is best-effort; the text layout uses the same coordinate data from
`ErdLayout::positions`, snapped to character cells.

---

## Phase 11 — SSE live monitor polling

The browser monitor panel currently requires a manual HTMX reload. Phase 11
adds a Server-Sent Events stream so the browser auto-refreshes at a
configurable interval.

### Architecture

```text
GET /api/monitor-stream
  │
  ├─ Axum: returns Response<Body> with Content-Type: text/event-stream
  ├─ tokio: spawns task that loops every 5 s
  │     calls DbMonitor + DbRoleManager + DbBackupManager
  │     emits SSE: "data: <JSON MonitorSnapshot>\n\n"
  └─ Browser: EventSource('/api/monitor-stream')
        onmessage: parse snapshot, call HTMX.trigger("#monitor-panel", "refresh")
```

### Implementation notes

- Use `axum::response::sse::{Event, Sse}` + `tokio_stream::wrappers::IntervalStream`
- `AppState` must carry the DB URL as `Arc<Mutex<Option<String>>>` (already the case)
- The HTMX panel subscribes on open; unsubscribes on panel close (browser JS)
- ratatui / egui: periodic `FetchRequest::FetchMonitor` already triggered from
  a `tokio::time::interval` in the event loop — no change needed

### New route

```
GET /api/monitor-stream    → SSE stream (text/event-stream)
```

---

## Phase 12 — SQL editor syntax highlighting

All three frontends currently show the SQL editor as plain text.

### Browser

Use [CodeMirror 6](https://codemirror.net/). Loaded from a CDN bundle or
vendored JS. The `<textarea>` is replaced with a CodeMirror editor; the
Leptos backend only sees the textarea value on submit, so the IR pipeline is
unaffected. The JS integration is entirely frontend-local.

### egui

Use `egui_code_editor` (crate) or a hand-rolled `egui::TextEdit` with a custom
layouter that tokenises SQL keywords and applies Catppuccin Mocha colours.
The custom layouter approach avoids an additional heavy dependency.

### ratatui

Use `tui-textarea` (already in the ecosystem notes) which supports syntax
highlighting via `ratatui_syntax_highlight` or manual span markup. Minimal:
keyword highlighting only (SELECT, FROM, WHERE, JOIN, etc.) in the accent
colour.

### Design note

Syntax highlighting is **not** part of the IR. The AccessKit tree carries the
raw SQL text. Highlighting is a pure rendering concern applied by each
frontend's text display primitive. No changes to `ArchiveDisplay` or
`build_content_nodes`.

---

## Phase 13 — Connection config (SSH tunnel + SSL)

`ConnectionProfile` today stores `url_env_key: String` only. Production use
requires SSH tunnels (pgAdmin's most-used feature) and SSL certificate paths.

### Extended type

```rust
pub struct ConnectionProfile {
    pub name: String,
    pub url_env_key: String,
    pub backend: BackendKind,
    pub color: Option<String>,

    // SSH tunnel (all optional; if host is set, tunnel is active)
    pub ssh_host: Option<String>,
    pub ssh_port: Option<u16>,
    pub ssh_user: Option<String>,
    pub ssh_key_env: Option<String>,   // env var naming the private key path

    // SSL
    pub ssl_mode: SslMode,
    pub ssl_cert_env: Option<String>,  // env var naming client cert path
    pub ssl_key_env: Option<String>,
    pub ssl_ca_env: Option<String>,

    // Display
    pub color: Option<String>,
}

pub enum SslMode { Disable, Allow, Prefer, Require, VerifyCa, VerifyFull }
```

### Architecture notes

- `ConnectionProfile` never stores raw paths or passwords — only env var
  names. The actual secrets stay in the environment / `.env` file.
- SSH tunnel is established before `AnyPool` creation using `openssh` crate
  (or `ssh2`). The tunnel binds a random local port; the pool connects to
  `localhost:<local_port>`.
- `ArchiveDbBackend::connect_profile(profile)` replaces the current
  `connect(url_str)` for profile-based connections.
- The UI has a connection editor panel (new `PanelMode::ConnectionEditor`)
  gated on a new `ArchiveAction::EditConnection`.

### `ArchiveDisplay` for `ConnectionProfile`

```rust
pub enum ConnectionProfileMode { Card, Row, Editor }
```

`Editor` mode emits a `Role::Form` subtree with labelled `Role::TextInput`
fields for each editable property — the IR-native form pattern.

---

## Phase 14 — Query plan comparison

DataGrip and DBeaver both show side-by-side EXPLAIN plans for two queries.

### Architecture

```rust
pub struct ExplainComparison {
    pub left: ExplainNode,
    pub right: ExplainNode,
    pub label_left: String,
    pub label_right: String,
}
```

`PanelMode::ExplainCompare { left: ExplainNode, right: ExplainNode }` is set
when the user runs the second EXPLAIN (toggled by `Ctrl+Shift+E`).

`ExplainComparison::to_ak_nodes` produces two `Role::Tree` children inside
a `Role::Group`. Each tree is built by `ExplainNode::to_ak_nodes` — the same
impl used for the single-plan view.

### UI behaviour

- First `Ctrl+E` → opens `ExplainPlan` panel (existing)
- Second `Ctrl+E` (with plan already open) → upgrades to `ExplainCompare`,
  showing the new plan on the right and preserving the old on the left
- `Ctrl+Shift+E` → clear comparison, return to single plan

### Diff highlighting

Cost-delta annotations: if `left.total_cost` and `right.total_cost` differ
by > 10%, the changed node label gains a `▲`/`▼` prefix. Implemented in
`ExplainComparison::to_ak_nodes` as label suffix, not as renderer-specific
colour — the IR stays clean; renderers can add colour on top.

---

## Phase 15 — Theme system

Currently: egui hardcodes Catppuccin Mocha; ratatui and browser have no
theme awareness.

### Design

A `ColorTheme` type (already in `elicit_accesskit`) should drive all three
frontends. Extend it with named semantic tokens:

```rust
pub struct ArchiveTheme {
    pub base: catppuccin::Flavour,  // Latte | Frappe | Macchiato | Mocha
}
```

`ArchiveNavModel` carries `pub theme: ArchiveTheme`. All three frontends read
`model.theme` at render time.

- **egui**: `apply_theme(ctx, &model.theme)` — already partially done (hardcoded Mocha)
- **ratatui**: `Style::fg(Color::Rgb(r,g,b))` from theme tokens; applied in
  `TuiAccessKitConverter::convert`
- **browser**: CSS custom properties injected in the `<head>` from the Leptos
  renderer, derived from `model.theme`; no hardcoded hex values in HTML

### Key binding

`T` → cycle theme (Latte → Frappe → Macchiato → Mocha → Latte)
New `ArchiveAction::CycleTheme`.

---

## Implementation order

```text
Phase 7  (ArchiveDisplay impls)     ← foundation for everything
  │
  ├─ Phase 8  (surface partial features)   ← high value, low risk, uses Phase 7
  │     8.1 constraint panel
  │     8.2 index panel
  │     8.3 trigger browser nav node
  │     8.4 extension nav node
  │     8.5 pagination keybindings
  │     8.6 ERD is_fk enrichment
  │
  ├─ Phase 9  (monitor depth)        ← extends existing panel, uses Phase 7
  │
  ├─ Phase 10 (visual ERD)           ← petgraph layout, SVG/Painter/text-art
  │
  ├─ Phase 11 (SSE live monitor)     ← browser only, no IR changes
  │
  ├─ Phase 12 (syntax highlighting)  ← pure rendering, no IR changes
  │
  ├─ Phase 13 (connection config)    ← type + ArchiveDisplay + UI panel
  │
  ├─ Phase 14 (plan comparison)      ← type + ArchiveDisplay + new PanelMode
  │
  └─ Phase 15 (theme system)         ← egui already started, propagate everywhere
```

---

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
