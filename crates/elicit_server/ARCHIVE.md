# archive

> A pgAdmin-style database browser powered by the `elicit_*` ecosystem.

`archive` is a verified command-line tool for exploring and querying databases.
It ships three parallel display frontends — a crossterm terminal UI, a
Leptos/Axum browser UI, and a native egui window — all driven by the same
`ArchiveNavModel` and rendered through the AccessKit IR pipeline.

Every HTML response and every TUI/egui frame is gated by an
`Established<IrSourced>` proof token minted by `ArchiveNavModel`, guaranteeing
that all three frontends are contractually equivalent and IR-sourced.

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

### TUI frontends (ratatui + egui)

Keybindings are sourced from `StatusBarDescriptor::archive_browse()` in the
AccessKit IR — the same data that populates the status bar chips:

| Key          | Action                            |
|--------------|-----------------------------------|
| `↑` / `k`   | Move selection up                 |
| `↓` / `j`   | Move selection down               |
| `Enter`      | Expand / collapse schema          |
| `r`          | Refresh nav tree                  |
| `?`          | Toggle keybinding help overlay    |
| `q` / `Esc`  | Quit                              |

### Browser frontend

Keyboard shortcuts trigger HTMX panel swaps via JS event listener. All
resulting HTML is IR-sourced (same `ArchiveNavModel` pipeline):

| Key            | Action                            |
|----------------|-----------------------------------|
| `/`            | Focus nav filter                  |
| `Esc`          | Clear nav filter                  |
| `s`            | Open SQL editor panel             |
| `d`            | Open DDL panel for selected table |
| `i`            | Open column detail panel          |
| `x`            | Open export picker panel          |
| `?`            | Open help / keybindings panel     |
| `Ctrl+Enter`   | Run SQL (when editor is open)     |

---

## Browser API routes

All content-returning routes respond with IR-sourced HTML fragments.
HTMX swaps use `hx-target="#content" hx-swap="outerHTML"` so the `<div
id="content">` wrapper is always present for the next swap.

| Method | Path                    | Description                              |
|--------|-------------------------|------------------------------------------|
| GET    | `/`                     | Full page (IR-sourced)                   |
| GET    | `/api/nav`              | Nav-tree fragment (filter param)         |
| GET    | `/api/preview`          | Table data grid panel                    |
| POST   | `/api/sql`              | Execute SQL, return content fragment     |
| GET    | `/api/inspect`          | Table inspection JSON                    |
| GET    | `/api/stats`            | Column statistics JSON                   |
| GET    | `/api/explain`          | EXPLAIN plan panel (with params)         |
| GET    | `/api/history`          | Query history JSON                       |
| POST   | `/api/history`          | Append history entry                     |
| GET    | `/api/saved`            | Saved queries JSON                       |
| POST   | `/api/saved`            | Save a query                             |
| DELETE | `/api/saved/:id`        | Delete a saved query                     |
| GET    | `/api/export`           | Download exported data file              |
| POST   | `/api/refresh`          | Reload nav tree from DB                  |
| GET    | `/api/open-sql-editor`  | SQL editor panel fragment                |
| GET    | `/api/open-help`        | Help / keybindings panel fragment        |
| GET    | `/api/history-panel`    | History browser panel fragment           |
| GET    | `/api/saved-panel`      | Saved queries panel fragment             |
| GET    | `/api/export-panel`     | Export picker panel fragment             |
| GET    | `/api/ddl-panel`        | DDL viewer panel fragment                |
| GET    | `/api/explain-panel`    | EXPLAIN plan panel fragment              |
| GET    | `/api/col-detail-panel` | Column detail + stats panel fragment     |
| GET    | `/api/load-history`     | Load history entry into SQL editor       |
| GET    | `/api/load-saved`       | Load saved query into SQL editor         |

---

## Tracing / logging

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
  │
  ▼
NavTree / ArchiveNavModel               → shared flat-list nav state
  │                                       (cursor, PanelMode, keybindings)
  │
  ▼  ArchiveNavModel::to_verified_tree()
     ├─ Window → [Toolbar, Main[Nav+Content], StatusBar, (overlay?)]
     │    All structure defined in AccessKit IR (no hardcoded HTML)
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

### PanelMode

`PanelMode` is the central content-area state machine, shared by all frontends:

| Variant         | Renders                              | Frontends      |
|-----------------|--------------------------------------|----------------|
| `Welcome`       | Welcome / prompt message             | all            |
| `DataGrid`      | Paginated table data                 | all            |
| `SqlEditor`     | SQL editor + results (+ error)       | all            |
| `Ddl`           | DDL source for selected table        | all            |
| `ExplainPlan`   | Visual EXPLAIN plan tree             | all            |
| `ColumnDetail`  | Column type + stats table            | all            |
| `HistoryPanel`  | Query history list with load links   | browser        |
| `SavedPanel`    | Saved queries with load/delete       | browser        |
| `ExportPanel`   | Export format picker (download links)| browser        |
| `HelpPanel`     | Keybinding reference                 | browser        |

TUI overlays (help, export picker, save prompt, saved browser) are rendered
as AccessKit `Role::Dialog` nodes appended to the Window root in
`to_verified_tree()`, driven by the same boolean flags used for event routing.
