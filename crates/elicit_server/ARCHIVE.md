# archive

> A pgAdmin-style database browser powered by the `elicit_*` ecosystem.

`archive` is a verified command-line tool for exploring and querying databases.
It ships three parallel display frontends — a crossterm terminal UI, a
Leptos/Axum browser UI, and a native egui window — all driven by the same
`ArchiveNavModel` flat-list navigation model and the same `StatusBarDescriptor`
keybinding definitions from the AccessKit IR layer.

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
# Archive browser frontend: http://localhost:3000/

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
# Archive browser frontend: http://localhost:4000/

archive demo --mode egui
# opens a native window with demo data
```

---

## Keyboard navigation

All interactive frontends (ratatui and egui) share the same keybindings,
sourced from `StatusBarDescriptor::archive_browse()` in the AccessKit IR:

| Key       | Action                          |
|-----------|---------------------------------|
| `↑` / `k` | Move selection up               |
| `↓` / `j` | Move selection down             |
| `Enter`   | Expand / collapse schema        |
| `r`       | Refresh                         |
| `?`       | Toggle keybinding help overlay  |
| `q` / `Esc` | Quit                          |

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
  │                                       (cursor, expand/collapse, keybindings)
  │
  ├─ ratatui path:
  │    TuiApp (model + ListState)       → crossterm alternate-screen TUI
  │
  ├─ browser path:
  │    DatabaseDescriptor → AccessKit IR (ValidRole + HasLabel)
  │    VerifiedTree::from_parts()       → Established<RenderComplete>
  │    LeptosRenderer::render(&tree)    → HTML string
  │    LeptosAxumPlugin                 → Established<LeptosServerConfigured>
  │    LeptosAxumBridgePlugin           → Established<AxumRouterCreated>
  │    axum::Router → live server
  │
  └─ egui path:
       ArchiveEguiApp (model + wgpu)    → native OS window (winit 0.30 + egui 0.34)
       egui-wgpu Renderer               → GPU-accelerated frame output
```

The `ArchiveNavModel` and `StatusBarDescriptor` keybinding definitions are the
shared IR that keeps all three frontends consistent: the same cursor logic,
the same key actions, and the same status-bar chip labels.
