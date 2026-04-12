# archive

> A pgAdmin-style database browser powered by the `elicit_*` ecosystem.

`archive` is a verified command-line tool for exploring and querying databases.
It ships two display frontends — a crossterm terminal UI and a Leptos/Axum
browser UI — both driven by the same `VerifiedTree` intermediate representation
and the same formal `Established<P>` proof chain.

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

### `serve [URL] --mode <ratatui|browser> [--port <P>]`

Serve the archive UI for a live database.

```bash
# Terminal UI (default mode)
archive serve --mode ratatui

# Browser UI on port 3000 (default)
archive serve --mode browser --port 3000
# Archive browser frontend: http://localhost:3000/

# Explicit URL overrides .env
archive serve postgres://localhost/mydb --mode browser
```

**ratatui mode** — opens a crossterm alternate-screen TUI. Press `q` or `Esc`
to exit.

**browser mode** — starts an Axum HTTP server. Open the URL in any browser.
Stop with `Ctrl-C`.

### `demo [--mode <ratatui|browser>] [--port <P>]`

Try the UI without a live database. Uses synthetic metadata.

```bash
archive demo --mode browser --port 4000
# Archive browser frontend: http://localhost:4000/
```

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
DatabaseDescriptor::to_ak_nodes()       → AccessKit IR  (ValidRole + HasLabel)
  │
  ▼
VerifiedTree::from_parts()              → Established<RenderComplete>
  │
  ├─ RatatuiBackend::render(&tree)      → TuiNode  (terminal path)
  │
  └─ LeptosRenderer::render(&tree)      → HTML string  (browser path)
       │
       ├─ LeptosAxumPlugin              → Established<LeptosServerConfigured>
       ├─ LeptosAxumBridgePlugin        → Established<AxumRouterCreated>
       └─ axum::Router::new()           → live server (interprets same descriptor
            .route("/", get(...))         that axum_router__emit() would print)
            .with_state(html)
```

The `AxumRouterDescriptor` produced by the plugin composition is identical to
what `axum_router__emit()` would emit as Rust source code. Runtime interpretation
and code generation are two views of the same verified specification.
