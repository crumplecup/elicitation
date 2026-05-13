# archive

> A pgAdmin-style database browser powered by the `elicit_*` ecosystem.

`archive` is a verified command-line tool for exploring and querying databases.
It ships three parallel display frontends — a crossterm terminal UI, a
Leptos/Axum browser UI, and a native egui window — all driven by the same
`ArchiveNavModel` and rendered through the AccessKit IR pipeline.

Every HTML response and every TUI/egui frame is gated by an
`Established<IrSourced>` proof token minted by `ArchiveNavModel`, guaranteeing
that all three frontends are contractually equivalent and IR-sourced.

Every rendered colour pair is gated by an `Established<WcagContrastMinimumNormalText>`
or `Established<WcagNonTextContrastMinimum>` proof token minted by `PaletteBuilder`,
guaranteeing WCAG 2.2 Level AA compliance. You cannot hold a `Palette` value
without having proven every colour pair passes its contrast threshold.

---

## URL resolution

Every command that connects to a database accepts an **optional** URL
argument.  When omitted, `DATABASE_URL` is used:

```
archive connect [DB_URL]
```

**Priority order:**

1. Explicit positional argument
2. `DATABASE_URL` environment variable

`archive` calls `dotenvy::dotenv()` at startup, so a `.env` file in the
working directory is loaded automatically.  A minimal `.env` looks like:

```dotenv
DATABASE_URL=postgres://user:pass@localhost/mydb
```

Once `.env` is in place, every command works without arguments:

```bash
archive connect           # uses DATABASE_URL from .env
archive list-schemas      # same
archive serve             # same
```

---

## Quick start

### Build

```bash
cargo build -p elicit_server --bin archive
# or install globally
cargo install --path crates/elicit_server --bin archive
```

### Connect to a database

```bash
archive connect postgres://user:pass@localhost/mydb
# PostgreSQL 15.3 on x86_64-pc-linux-gnu
```

Supported URL schemes: `postgres://`, `sqlite:`, `mysql://`

---

## Commands

### `connect [URL]`

Print the server version and confirm the connection is live.

```bash
archive connect                              # uses DATABASE_URL
archive connect postgres://localhost/mydb   # explicit override
```

### `list-schemas [URL]`

List all schema names in the database.

```bash
archive list-schemas
# public
# analytics
# staging
```

### `list-tables [URL] [--schema <S>]`

List tables in a schema (default: `public`).

```bash
archive list-tables --schema analytics
# analytics.events
# analytics.sessions
# analytics.users
```

### `query [URL] --sql <SQL>`

Execute a SQL statement and print each row.

```bash
archive query --sql "SELECT id, name FROM users LIMIT 5"
# id=1 | name="alice"
# id=2 | name="bob"
```

### `serve [URL] --mode <ratatui|browser|egui> [--port <P>]`

Serve the archive UI for a live database.

```bash
# Terminal UI (default mode)
archive serve --mode ratatui

# Browser UI on port 3000 (default)
archive serve --mode browser --port 3000
# Archive browser: http://localhost:3000/

# Native egui window (GPU-accelerated, winit/wgpu)
archive serve --mode egui

# Explicit URL overrides .env
archive serve postgres://localhost/mydb --mode browser
```

**ratatui mode** — opens a crossterm alternate-screen TUI. Press `q` or `Esc`
to exit.

**browser mode** — starts an Axum HTTP server. Open the URL in any browser.
Stop with `Ctrl-C`.

**egui mode** — opens a native OS window rendered with wgpu. Uses the same
keyboard navigation as ratatui. Press `q` or `Esc` to exit.

### `demo [--mode <ratatui|browser|egui>] [--port <P>]`

Try the UI without a live database. Uses synthetic metadata.

```bash
archive demo --mode browser --port 4000
# Archive browser: http://localhost:4000/

archive demo --mode egui
# opens a native window with demo data
```

---

## Keyboard navigation

All keybindings are declared in `ArchiveKeyMap` (the single source of truth)
and flow through `ArchiveNavModel` to every frontend. The IR contract
guarantees that a binding declared once is honoured identically in all three
renderers — no per-frontend key wiring can silently diverge.

### Default navigation mode (all frontends)

| Key                | Action                                                  |
|--------------------|---------------------------------------------------------|
| `↑` / `k`         | Move selection up                                       |
| `↓` / `j`         | Move selection down                                     |
| `Enter`            | Expand / collapse / select item                         |
| `r`                | Refresh nav tree from database                          |
| `?`                | Toggle keybinding help overlay                          |
| `q` / `Esc`        | Quit                                                    |
| `/`                | Open nav filter                                         |
| `s`                | Open SQL editor panel                                   |
| `d`                | Open DDL viewer for selected table                      |
| `e`                | Open EXPLAIN plan for selected table                    |
| `Ctrl+Shift+E`     | Clear explain comparison (return to single plan)        |
| `F2`               | Open saved-query browser                                |
| `m`                | Open / close live monitor panel                         |
| `a`                | Open / close admin panel                                |
| `g`                | Open / close ERD diagram for selected schema            |
| `c`                | Open constraint panel for selected table                |
| `i`                | Open index panel for selected table                     |
| `o`                | Open connection editor for active connection            |
| `[` / `]`          | Cycle admin / monitor panel tabs (prev / next)          |
| `x`                | Open export format picker (DataGrid only)               |
| `PgDn` / `PgUp`    | Next / previous data page (DataGrid only)               |
| `Home` / `End`     | First / last data page (DataGrid only)                  |
| `Ctrl+Tab`         | Cycle to next database connection                       |
| `Ctrl+Shift+Tab`   | Cycle to previous database connection                   |

### Modal key maps

| Mode              | Key       | Action                               |
|-------------------|-----------|--------------------------------------|
| **Filter**        | `Esc`     | Close filter (restore full list)     |
|                   | `Backspace` | Delete last filter character       |
| **Save prompt**   | `Esc`     | Cancel without saving                |
|                   | `Backspace` | Delete last name character         |
|                   | `Enter`   | Confirm and persist query            |
| **Saved browser** | `Esc`/`q` | Close browser                        |
|                   | `↑`/`k`   | Move selection up                    |
|                   | `↓`/`j`   | Move selection down                  |
|                   | `Enter`   | Load selected query into editor      |
|                   | `d`/`Del` | Delete selected query                |
| **Export picker** | `Esc`     | Cancel                               |
|                   | `↑`/`k`   | Move selection up                    |
|                   | `↓`/`j`   | Move selection down                  |
|                   | `Enter`   | Export in selected format            |
| **SQL editor**    | `Ctrl+Enter` | Execute SQL                       |
|                   | `↑` (history) | Navigate to previous history entry |
|                   | `↓` (history) | Navigate to next history entry     |
|                   | `Esc`     | Close SQL editor                     |
|                   | `Ctrl+s`  | Open save-query name prompt          |

---

## Panel modes

`PanelMode` is the central content-area state machine, shared by all three
frontends. Each variant is wired in the AccessKit IR — no frontend may render
a panel that isn't modelled here.

| Variant              | Description                                                | Frontends |
|----------------------|------------------------------------------------------------|-----------|
| `Welcome`            | Welcome message / connection prompt                        | all       |
| `Loading`            | Spinner while async fetch is in flight                     | all       |
| `DataGrid`           | Paginated table data with row editing                      | all       |
| `SqlEditor`          | SQL editor + query result + error display                  | all       |
| `Ddl`                | DDL source for the selected table / view                   | all       |
| `ExplainPlan`        | Visual EXPLAIN plan tree                                   | all       |
| `ExplainCompare`     | Side-by-side EXPLAIN plan comparison (left vs right)       | all       |
| `ColumnDetail`       | Column type + statistics table                             | all       |
| `MonitorPanel`       | Live server activity: sessions, cache, roles, backups      | all       |
| `AdminPanel`         | Admin tabs: Roles · Backups · WAL · Extensions · Settings  | all       |
| `ErdPanel`           | Entity-relationship diagram for the selected schema        | all       |
| `ConstraintPanel`    | Constraints (PK, FK, unique, check) for selected table     | all       |
| `IndexPanel`         | Indexes for the selected table                             | all       |
| `ConnectionEditor`   | Connection profile editor (name, URL, SSH, SSL)            | all       |
| `Error`              | Error message display                                      | all       |
| `HistoryPanel`       | Query history list with one-click load                     | browser   |
| `SavedPanel`         | Saved queries with load / delete                           | browser   |
| `ExportPanel`        | Export format picker (CSV, JSON, Parquet…)                 | browser   |
| `HelpPanel`          | Keybinding reference                                       | browser   |

TUI overlays (help, export picker, save prompt, saved browser) are rendered
as AccessKit `Role::Dialog` nodes appended to the Window root by
`to_verified_tree()`, driven by the same boolean flags used for event routing.

---

## Monitor panel

Press `m` to open the live monitor panel. It queries the database via the
`elicit_db` `DbMonitor`, `DbRoleManager`, and `DbBackupManager` traits and
surfaces:

- **Active sessions** — pid, application, state, duration, query snippet
- **Cache hit ratio** — shared-buffer hit percentage
- **Roles** — all database roles
- **Backups** — recent backup records

In the browser frontend, `GET /api/monitor` fetches a fresh `MonitorSnapshot`
and re-renders the panel via HTMX. The browser also subscribes to
`GET /api/monitor-stream` (a Server-Sent Events stream) that pushes a
refreshed `MonitorSnapshot` every 5 seconds — no manual reload needed.

Use `[` / `]` to switch between monitor tabs (Sessions, Roles, Backups).

---

## Admin panel

Press `a` to open the admin panel. Use `[` / `]` to switch tabs:

| Tab          | Contents                                            |
|--------------|-----------------------------------------------------|
| **Roles**    | All database roles (name, superuser, login flags)   |
| **Backups**  | Backup records with status and path                 |
| **WAL**      | WAL archiving ready / not-ready status              |
| **Extensions** | Installed extensions with version                 |
| **Settings** | Top GUC settings (name, value, description)         |

The admin panel is backed by `elicit_db` traits:
`DbRoleManager`, `DbBackupManager`, `DbServerAdmin`.

In the browser frontend, `GET /api/admin` fetches a fresh `AdminSnapshot` and
re-renders. Tab navigation uses `GET /api/admin-tab-next` /
`GET /api/admin-tab-prev`.

### MCP admin plugin

`ArchiveAdminPlugin` exposes the admin operations as MCP tools
(`archive_admin__*`), each carrying an `Established<P>` proof proposition:

| Tool                 | Proposition          | Description                     |
|----------------------|----------------------|---------------------------------|
| `list_roles`         | `RoleListRead`       | List all database roles         |
| `create_role`        | `RoleCreated`        | Create a new role               |
| `drop_role`          | `RoleDropped`        | Drop an existing role           |
| `grant_privilege`    | `PrivilegeGranted`   | Grant privilege to a role       |
| `revoke_privilege`   | `PrivilegeRevoked`   | Revoke privilege from a role    |
| `initiate_backup`    | `BackupStarted`      | Trigger a database backup       |
| `list_backups`       | `BackupListRead`     | List recent backup records      |
| `verify_backup`      | `BackupVerified`     | Verify a backup by path         |
| `wal_status`         | `WalStatusRead`      | Report WAL archiving status     |
| `server_version`     | `VersionRead`        | Return the server version string|
| `list_extensions`    | `ExtensionListRead`  | List installed extensions       |
| `install_extension`  | `ExtensionInstalled` | Install a named extension       |
| `list_settings`      | `AdminSettingsRead`  | Read top GUC settings           |
| `reload_config`      | `ConfigReloaded`     | Reload server configuration     |

---

## ERD diagram view

Press `g` to open the entity-relationship diagram for the currently selected
schema. The diagram is built from two queries:

1. `DbTableManager::list_tables()` — one `ErdNode` per table, with
   `ErdColumn` children carrying name, type, and PK flag.
2. `information_schema.referential_constraints` — one `ErdEdge` per FK
   relationship, recording `from_table.from_column → to_table.to_column`.

The `ErdPanel` content area renders:

- A header line: `ERD: <schema> — N tables, M foreign keys`
- An expandable tree of tables, each with its column list (PK columns marked)
- A flat list of FK edges: `orders.customer_id → customers.id`

In the browser frontend, `GET /api/erd?schema=<name>` fetches and renders
the diagram via HTMX. The ERD is PostgreSQL-specific (uses
`information_schema`); other backends receive an empty edge set.

---

## Browser API routes

All content-returning routes respond with IR-sourced HTML fragments.
HTMX swaps use `hx-target="#content" hx-swap="outerHTML"` so the `<div
id="content">` wrapper is always present for the next swap.

| Method | Path                    | Description                                                  |
|--------|-------------------------|--------------------------------------------------------------|
| GET    | `/`                     | Full page (IR-sourced)                                       |
| GET    | `/api/nav`              | Nav-tree fragment (filter param)                             |
| GET    | `/api/nav-up`           | Move cursor up, return nav fragment                          |
| GET    | `/api/nav-down`         | Move cursor down, return nav fragment                        |
| GET    | `/api/nav-enter`        | Toggle-expand, return nav fragment                           |
| GET    | `/api/preview`          | Table data grid panel                                        |
| POST   | `/api/sql`              | Execute SQL, return content fragment                         |
| GET    | `/api/inspect`          | Table inspection JSON                                        |
| GET    | `/api/stats`            | Column statistics JSON                                       |
| GET    | `/api/explain`          | EXPLAIN plan panel (with params)                             |
| GET    | `/api/history`          | Query history JSON                                           |
| POST   | `/api/history`          | Append history entry                                         |
| GET    | `/api/saved`            | Saved queries JSON                                           |
| POST   | `/api/saved`            | Save a query                                                 |
| DELETE | `/api/saved/{id}`       | Delete a saved query                                         |
| GET    | `/api/export`           | Download exported data file                                  |
| POST   | `/api/refresh`          | Reload nav tree from DB                                      |
| GET    | `/api/open-sql-editor`  | SQL editor panel fragment                                    |
| GET    | `/api/monitor`          | Fetch live monitor snapshot → MonitorPanel                   |
| GET    | `/api/monitor-stream`   | SSE stream — emits `monitor` events every 5 s                |
| GET    | `/api/admin`            | Fetch admin snapshot → AdminPanel                            |
| GET    | `/api/admin-tab-next`   | Cycle admin tab forward → AdminPanel                         |
| GET    | `/api/admin-tab-prev`   | Cycle admin tab backward → AdminPanel                        |
| GET    | `/api/erd`              | ERD diagram for `?schema=` → ErdPanel                        |
| GET    | `/api/constraints`      | Constraint panel for `?schema=&table=` → ConstraintPanel     |
| GET    | `/api/indexes`          | Index panel for `?schema=&table=` → IndexPanel               |
| GET    | `/api/open-help`        | Help / keybindings panel fragment                            |
| GET    | `/api/history-panel`    | History browser panel fragment                               |
| GET    | `/api/saved-panel`      | Saved queries panel fragment                                 |
| GET    | `/api/export-panel`     | Export picker panel fragment                                 |
| GET    | `/api/ddl-panel`        | DDL viewer panel fragment                                    |
| GET    | `/api/explain-panel`    | EXPLAIN plan panel fragment                                  |
| GET    | `/api/col-detail-panel` | Column detail + stats panel fragment                         |
| GET    | `/api/load-history`     | Load history entry into SQL editor                           |
| GET    | `/api/load-saved`       | Load saved query into SQL editor                             |
| POST   | `/api/switch-connection`| Switch active database connection                            |
| POST   | `/api/action`           | Dispatch any `ArchiveAction` by name                         |

---

## SQL syntax highlighting

All three frontends highlight SQL in the editor and query result panels. The
tokeniser lives in `elicit_accesskit::sql` and is shared — keyword sets and
token boundaries are identical across renderers.

Token categories are mapped through the `SemanticRole` palette:

| Token kind       | `SemanticRole` | WCAG threshold |
|------------------|----------------|----------------|
| Keywords         | `Keyword`      | 4.5 : 1        |
| String literals  | `StringLit`    | 4.5 : 1        |
| Comments         | `Comment`      | 4.5 : 1        |
| Numbers          | `Number`       | 4.5 : 1        |
| Plain text       | `Text`         | 4.5 : 1        |

All syntax colours are contrast-proven; no special exemption for "decorative"
code tokens.

---

## EXPLAIN plan comparison

Press `e` once to open an EXPLAIN plan. Press `e` again while a plan is
already open to enter side-by-side comparison mode (`ExplainCompare`). The
previous plan moves to the left pane; the new plan appears on the right.

Press `Ctrl+Shift+E` to clear the comparison and return to a single plan view.

Cost-delta annotations: if `left.total_cost` and `right.total_cost` differ by
more than 10%, the higher-cost node label gains a `▲` prefix in the IR — all
three renderers display it without extra work.

---

## WCAG palette system

Every rendered colour pair is a **proof-carrying value**. `Palette` can only
be constructed by `PaletteBuilder::build()` (returns `Result`) or
`build_adjusted()` (auto-corrects non-compliant colours, panics only if the
heuristic cannot converge). Holding a `Palette` *is* the proof of compliance.

### Semantic roles

| Role         | Used for                                    | Threshold  |
|--------------|---------------------------------------------|------------|
| `Background` | Page / panel background                     | —          |
| `Surface`    | Card / sidebar surfaces                     | —          |
| `Text`       | Body text (vs Background **and** Surface)   | 4.5 : 1    |
| `DimText`    | Captions, placeholder text                  | 4.5 : 1    |
| `Accent`     | Focus rings, active indicators (non-text)   | 3.0 : 1    |
| `Error`      | Error messages, badges (non-text)           | 3.0 : 1    |
| `Keyword`    | SQL keyword highlighting                    | 4.5 : 1    |
| `StringLit`  | SQL string literal highlighting             | 4.5 : 1    |
| `Comment`    | SQL comment highlighting                    | 4.5 : 1    |
| `Number`     | SQL number highlighting                     | 4.5 : 1    |

Code is text. Every syntax-highlighting colour must pass the 4.5 : 1
normal-text threshold — no exemptions.

### Pre-built palettes

Four Catppuccin-based palettes are available from `elicit_ui::palettes`:

| Function       | Base theme                       |
|----------------|----------------------------------|
| `mocha()`      | Catppuccin Mocha (dark)          |
| `macchiato()`  | Catppuccin Macchiato (dark)      |
| `frappe()`     | Catppuccin Frappé (medium)       |
| `latte()`      | Catppuccin Latte (light)         |

Each is initialised once via `OnceLock` using `build_adjusted()`. Known
Catppuccin colours that fail contrast (e.g. Latte green ~3.0 : 1, Latte peach
~2.68 : 1, all four Overlay1 comment colours ~2.3–2.9 : 1) are auto-corrected
to the nearest compliant value; the upstream palette values are a Catppuccin
problem, not ours.

### Proof credentials

```rust
// In elicit_ui::proof_credentials:
proof_credential!(PaletteNormalTextVerified => WcagContrastMinimumNormalText);
proof_credential!(PaletteNonTextVerified    => WcagNonTextContrastMinimum);
```

Both credentials are `pub(crate)` — external code cannot forge a `Palette`.

### Dependency topology

`elicit_ui` depends on `elicit_accesskit`. Therefore `elicit_accesskit` cannot
depend on `elicit_ui` (cycle). Palette-aware colour resolution lives in the
crates that depend on both:

- `elicit_egui` — `sql_layout_job(sql, palette)`
- `elicit_ratatui` — `sql_highlight_rich(sql, palette)`
- `elicit_server` — `apply_theme(ctx, palette)` / `wrap_page(body, palette)`

`elicit_accesskit::sql` is colour-agnostic; callers map `SqlTokenKind →
SemanticRole` themselves.

---

Set `RUST_LOG` to control log output:

```bash
RUST_LOG=archive=debug archive serve postgres://localhost/mydb --mode browser
RUST_LOG=info          archive list-schemas postgres://localhost/mydb
```

---

## How it works

Every step is a **verified tool-call composition**:

```text
ArchiveDbBackend::connect(url)          → ConnectionEstablished proof
  │
  ├─ DbServerAdmin::server_version()
  ├─ DbSchemaManager::list_schemas()
  ├─ DbTableManager::list_tables()
  ├─ DbMonitor::active_sessions()
  ├─ DbRoleManager::list_roles()
  ├─ DbBackupManager::list_backups()
  │
  ▼
NavTree / ArchiveNavModel               → shared flat-list nav state
  │                                       (cursor, PanelMode, keybindings)
  │
  ▼  ArchiveNavModel::to_verified_tree()
     ├─ Window → [Toolbar, Main[Nav+Content], StatusBar, (overlay?)]
     │    All structure defined in AccessKit IR (no hardcoded HTML/widgets)
     └─ Returns (VerifiedTree, Established<IrSourced>) proof token
          │
          ├─ ratatui path:
          │    TuiAccessKitConverter::convert(tree) → ratatui widgets
          │    crossterm alternate-screen TUI
          │
          ├─ browser path:
          │    to_content_tree() / to_nav_tree() for fragment responses
          │    LeptosRenderer::render(&tree) → HTML fragment
          │    axum::Router → live HTMX-powered server
          │    All HTMX panel swaps outerHTML → IR fragment always present
          │
          └─ egui path:
               EguiAccessKitConverter::convert(tree) → egui widgets
               native OS window (winit 0.30 + egui 0.34 + wgpu)
```

### IR contract

The `Established<IrSourced>` proof token, returned alongside every
`VerifiedTree`, is the compile-time guarantee that all rendered output
originates from the AccessKit IR. No frontend may produce HTML or widgets
without going through `ArchiveNavModel`'s tree-building methods. This keeps
all three frontends contractually equivalent.

### Palette contract

The `Palette` type, returned only by `PaletteBuilder::build()` or
`build_adjusted()`, is the compile-time guarantee that every rendered colour
pair passes WCAG 2.2 Level AA contrast. Frontends receive `&'static Palette`
from `elicit_ui::palettes` and pass it to every rendering call — they cannot
construct one without the proofs. Holding the value is the proof.

### Keybinding IR faithfulness

`ArchiveKeyMap` is the **single source of truth** for all key bindings.  It
feeds:

1. `ArchiveKeyMap::resolve()` — runtime dispatch in every frontend
2. `ArchiveKeyMap::to_status_bar()` → `StatusBarDescriptor` chips in the IR
3. `ArchiveKeyMap::to_js_listener()` → the browser JS keyboard handler

Adding a binding in one place automatically propagates to all three surfaces.
The compiler enforces exhaustive handling: every `ArchiveAction` variant must
be matched in every frontend's dispatch function.

### PanelMode

`PanelMode` is the central content-area state machine. Adding a new variant
requires implementing it in `build_content_nodes()` (the IR builder) and all
three frontend event/fetch dispatch functions — the compiler enforces
completeness at every step. There are currently 19 variants (see the
[Panel modes](#panel-modes) table above).

---

## Verified State Machines

`archive` is structured top-to-bottom as **four interconnected Verified State
Machines (VSMs)**. Every observable application state — connection lifecycle,
navigation tree, modal overlays, and content panel — is modelled as a VSM.

Contracts flow through the entire display pipeline: the `Established<Invariant>`
proof token that guards each transition is the same token required by
`ArchiveNavModel::to_verified_tree()`, which is the same token required by all
three frontends. You cannot display a state you cannot prove is consistent.

### The four machines

| Machine | State type | Invariant | Transitions |
|---|---|---|---|
| `ArchiveConnectionMachine` | `ArchiveConnectionState` | `ArchiveConnectionConsistent` | 7 |
| `ArchiveNavMachine` | `ArchiveNavState` | `ArchiveNavConsistent` | 9 |
| `ArchiveOverlayMachine` | `ArchiveOverlayState` | `ArchiveOverlayConsistent` | 11 |
| `ArchivePanelMachine` | `ArchivePanelState` | `ArchivePanelConsistent` | 23 |

### Connection machine state diagram

```text
Disconnected ──begin_connect_{sql,kv}──► Connecting
     ▲                                        │
     │              ┌─────────────────────────┤ finish_connect_sql
     │              │                         │ finish_connect_kv
     │              ▼                         ▼
     │         SqlConnected             KvConnected
     │              │                         │
     │              └──────── reconnect ──────► Reconnecting
     │                                        │
     │               ┌────────────────────────┤ finish_connect_{sql,kv}
     │               │                        │ connection_error
     └───disconnect───┘              ConnectionError
```

### Declaring a machine

```rust
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [
    begin_connect_sql, begin_connect_kv,
    finish_connect_sql, finish_connect_kv,
    disconnect, reconnect, connection_error,
])]
pub struct ArchiveConnectionMachine;
```

`#[derive(VerifiedStateMachine)]` generates the `impl VerifiedStateMachine`
block, inferring `type State = ArchiveConnectionState` and
`type Invariant = ArchiveConnectionConsistent` from the naming convention
(strip `Machine` suffix → `{Prefix}State`, `{Prefix}Consistent`).

Both `state` and `invariant` can be overridden via `#[vsm(state = ...,
invariant = ...)]` when the naming convention doesn't apply.

### Declaring a transition

```rust
#[formal_method(contracts = [ArchiveConnectionConsistent])]
#[instrument(skip(proof))]
pub fn begin_connect_sql(
    _state: ArchiveConnectionState,
    proof: Established<ArchiveConnectionConsistent>,
    profile_name: String,
    backend: BackendKind,
) -> (ArchiveConnectionState, Established<ArchiveConnectionConsistent>) {
    (
        ArchiveConnectionState::Connecting { profile_name, backend },
        proof,
    )
}
```

`#[formal_method(contracts = [...])]` generates three things:

1. A `BeginConnectSqlTransition` companion struct with a
   `kani_harness() -> proc_macro2::TokenStream` method, enabling
   `build.rs` proof composition.
2. A `#[cfg(kani)] #[kani::proof] fn begin_connect_sql__kani()` harness
   directly in the module — ready to run with `cargo kani`.
3. Stub companions under `#[cfg(creusot)]` and `#[cfg(verus)]` for future
   Creusot / Verus verification.

Every transition must accept and return an `Established<Invariant>` token.
This is the compile-time proof that the invariant is preserved across the
state change.

### Formal method pattern summary

| Element | What it does |
|---|---|
| `#[derive(Elicit)]` on the state enum | Derives `JsonSchema`, `Serialize`, `Deserialize` + MCP elicitation support |
| `#[derive(Prop)]` on the invariant struct | Makes it a verifiable proposition |
| `#[formal_method(contracts = [I])]` | Emits Kani/Creusot/Verus harnesses; enforces invariant at every call site |
| `#[derive(VerifiedStateMachine)]` | Wires state + invariant + transitions into one verifiable unit |
| `Established<I>` token | The runtime proof witness — cannot be forged without `Established::assert()` |

### Kani proof composition (`elicit_proofs`)

`crates/elicit_proofs` is the unified formal verification crate that composes
the four archive VSM proofs. Its `build.rs` calls `vsm_kani_proof()` on each
machine, deduplicates shared harnesses, and writes the results to
`src/kani/generated/{archive_connection,archive_nav,archive_overlay,archive_panel}.rs`.

```text
ArchiveConnectionMachine::vsm_kani_proof()
  = ArchiveConnectionConsistent::kani_proof()      ← invariant is a valid Prop
  + BeginConnectSqlTransition::kani_harness()       ← each transition preserves it
  + BeginConnectKvTransition::kani_harness()
  + … (7 transitions total)
```

The composed proof says: **"the invariant is a valid proposition AND every
registered transition preserves it without panicking for any reachable input."**

Run the proofs:

```bash
cargo kani -p elicit_proofs --features kani
```

Each archive VSM harness is also registered in `ProofHarness::all()` and can
be run individually via the `kani-verify-tracked` workflow in the
`elicitation_kani` test harness:

```bash
cargo test -p elicitation_kani -- kani_harness_run
```
