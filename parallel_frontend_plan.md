# Archive Parallel Frontend Plan

## Context

The `archive` module (`crates/elicit_server/src/archive/`) is a pgAdmin-style
database manager built from the `elicit_*` ecosystem.  Phases 1–4 are complete
(descriptor types, AccessKit display layer, browse/query/spatial/display
plugins, and `ArchiveDbBackend`).  The `elicit_leptos` AccessKit → HTML5/`view!`
bridge is also complete.

The goal of this plan is to expose archive to real users by adding a CLI binary
and three parallel rendering modes — one per output surface — all driven from
the same AccessKit IR.

---

## Completed foundations

| Item | Commit |
|------|--------|
| Archive Phase 1–4 (types, display, plugins, `ArchiveDbBackend`) | `904afcdd` |
| `elicit_leptos` `LeptosRenderer` (AccessKit → HTML5 / `view!`) | `8a0b5409` |
| `VerifiedTree::from_parts()` added to `elicit_ui` | `8a0b5409` |

---

## Goal

Add a `[[bin]]` target to `elicit_server` named `archive`.

```
archive connect   <DB_URL>                       # test connection, print version
archive list-schemas <DB_URL>                    # print schema names
archive list-tables  <DB_URL> [--schema <S>]     # print table names
archive query        <DB_URL> --sql <SQL>         # run SQL, print rows
archive serve        <DB_URL> --mode <MODE> [--port <P>]
    MODE = ratatui   → terminal TUI   (ratatui + crossterm)
    MODE = egui      → native window  (winit + wgpu + egui, no eframe)
    MODE = browser   → HTTP server    (axum serving LeptosRenderer HTML, default :3000)
```

---

## Shared IR pipeline

Every frontend reads from the same source and passes through the same IR:

```
ArchiveDisplay::to_ak_nodes(mode, id_base)
  → (elicit_accesskit::NodeId, Vec<(NodeId, NodeJson)>)
  → accesskit::Node: From<NodeJson>   (lossless conversion)
  → HashMap<accesskit::NodeId, accesskit::Node>
  → VerifiedTree::from_parts(nodes, root, viewport)
          │                    │                     │
          ▼                    ▼                     ▼
   RatatuiBackend         EguiBackend          LeptosRenderer
   .render(&tree)    render_tree(ui,…)        .render(&tree)
   → TuiNode          → egui widgets          → HTML string
          │                    │                     │
          ▼                    ▼                     ▼
  terminal_tools::        winit window +         axum GET /
  render_node(frame)      wgpu surface           text/html
```

---

## Existing backend entry points

| Mode | Crate | Live upstream dep | Key function |
|------|-------|-------------------|--------------|
| Ratatui | `elicit_ratatui` | `ratatui 0.30` | `terminal_tools::render_node(frame, area, node)` |
| Egui | `elicit_egui` | `egui 0.34` | `accesskit_bridge::render_tree(ui, nodes, root)` |
| Leptos/browser | `elicit_leptos` | none (shadow) | `LeptosRenderer::render(&tree)` → `last_html()` |

**Gap**: `egui-winit` and `egui-wgpu` (the winit/wgpu integration glue for egui)
are not yet in the workspace.  They must be added before the egui frontend
can be built.

---

## Implementation steps

### Step 1 — Workspace and `elicit_server` deps

**Add to root `Cargo.toml` `[workspace.dependencies]`:**

```toml
egui-winit = { version = "0.34" }
egui-wgpu  = { version = "0.34", features = ["winit"] }
```

**Add to `crates/elicit_server/Cargo.toml` `[dependencies]`:**

```toml
clap           = { workspace = true }
ratatui        = { workspace = true }
axum           = { workspace = true }
tower          = { workspace = true }
tower-http     = { workspace = true }
egui-winit     = { workspace = true }
egui-wgpu      = { workspace = true }
elicit_ratatui = { workspace = true }
elicit_leptos  = { workspace = true }
```

Note: `elicit_winit` and `elicit_wgpu` are MCP codegen tools, not runtime
libraries.  The live `winit`/`wgpu` symbols come in through `egui-winit` /
`egui-wgpu`.

---

### Step 2 — `archive/frontend_utils.rs`

Single shared helper that converts the output of `ArchiveDisplay::to_ak_nodes`
into a `VerifiedTree` suitable for any backend:

```rust
pub fn nodes_to_verified_tree(
    root_eid: elicit_accesskit::NodeId,
    nodes: Vec<(elicit_accesskit::NodeId, elicit_accesskit::NodeJson)>,
    width: u32,
    height: u32,
) -> elicit_ui::VerifiedTree
```

Internally: `NodeJson → accesskit::Node` via `From`, collect into
`HashMap<NodeId, Node>`, then `VerifiedTree::from_parts(map, root, Viewport::new(width, height))`.

---

### Step 3 — `archive/ratatui_frontend.rs`

```rust
pub fn run_tui(tree: VerifiedTree) -> ArchiveResult<()>
```

1. Enable crossterm raw mode + alternate screen
2. Create `Terminal<CrosstermBackend<Stdout>>`
3. `RatatuiBackend::new().render(&tree)` → `TuiNode`
4. `terminal.draw(|f| render_node(f, f.area(), &node))`
5. Poll crossterm key events; `q` / `Esc` exits
6. Restore terminal (raw mode off, main screen) on exit — even on error

---

### Step 4 — `archive/leptos_frontend.rs`

```rust
pub async fn run_browser(tree: VerifiedTree, port: u16) -> ArchiveResult<()>
```

1. `LeptosRenderer::html()` after `renderer.render(&tree)` → HTML string
2. `axum::Router` with `GET /` → `axum::response::Html(html)`
3. Bind `TcpListener` on `0.0.0.0:{port}`, serve with `axum::serve`
4. Print `Listening on http://localhost:{port}` to stderr; suggest opening browser

This is **static SSR** — the tree is rendered once at startup.  A natural
follow-on (not in scope here) is to rebuild the tree per-request against a
live DB, or add a `GET /refresh` endpoint.

---

### Step 5 — `archive/egui_frontend.rs`

```rust
pub fn run_egui(tree: VerifiedTree) -> ArchiveResult<()>
```

Raw winit + wgpu + egui integration (no eframe):

1. `winit::event_loop::EventLoop::new()` + `WindowBuilder` → `Window`
2. `wgpu::Instance` → `Surface` → `Adapter` → `Device` + `Queue`
3. Configure surface format; `SurfaceConfiguration`
4. `egui::Context::default()`; `egui_winit::State::new(...)` for input mapping
5. `egui_wgpu::Renderer::new(&device, surface_fmt, None, 1, false)` for draw output
6. Snapshot `nodes` / `root` from the `VerifiedTree` once before the loop
7. Event loop:
   - `Event::WindowEvent::RedrawRequested`:
     - `state.take_egui_input(&window)` → raw input
     - `ctx.begin_pass(raw_input)` + `CentralPanel::default().show(...)`:
       - `elicit_egui::render_tree(&mut ui, &nodes, root)`
     - `ctx.end_pass()` → `FullOutput` (`paint_jobs`, `textures_delta`)
     - Upload texture deltas; tessellate; submit render pass via `egui_wgpu::Renderer`
     - `surface_texture.present()`
   - `Event::WindowEvent::CloseRequested` → `control_flow.exit()`
   - `Event::AboutToWait` → `window.request_redraw()`

---

### Step 6 — `src/bin/archive.rs`

```rust
#[derive(Parser)]
#[command(name = "archive", about = "Archive DB browser")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Connect { url: String },
    ListSchemas { url: String },
    ListTables {
        url: String,
        #[arg(long)] schema: Option<String>,
    },
    Query {
        url: String,
        #[arg(long)] sql: String,
    },
    Serve {
        url: String,
        #[arg(long, default_value = "ratatui")] mode: ServeMode,
        #[arg(long, default_value_t = 3000)]    port: u16,
    },
}

#[derive(ValueEnum, Clone)]
enum ServeMode { Ratatui, Egui, Browser }
```

Dispatch:

- `Connect` → `ArchiveDbBackend::connect(&url)`, print server version
- `ListSchemas` → connect → `list_schemas()` → print names
- `ListTables` → connect → `list_tables(schema)` → print names
- `Query` → connect → `execute_query(sql)` → print `DbRows` as table
- `Serve(Ratatui)` → connect → build `DatabaseDescriptor` display tree
  → `frontend_utils::nodes_to_verified_tree` → `ratatui_frontend::run_tui`
- `Serve(Egui)` → same → `egui_frontend::run_egui`
- `Serve(Browser)` → same → tokio runtime → `leptos_frontend::run_browser(tree, port)`

---

### Step 7 — Wire and validate

- `archive/mod.rs`: expose `pub mod frontend_utils`, `mod ratatui_frontend`,
  `mod egui_frontend`, `mod leptos_frontend`
- `just check elicit_server`
- `just check-all elicit_server`
- commit

---

## Open questions

1. **Egui native scope**: the winit + wgpu + egui loop is ~150 lines of
   integration boilerplate.  Should it be fully implemented now, or stubbed
   with a clear `todo!("egui native — see egui_frontend.rs")` and filled in
   as a follow-on?

2. **`Serve` demo tree**: when `--url` points to a real DB the tree is live.
   For offline demo/testing, should `Serve` accept an optional `--url` and
   fall back to a hardcoded `DatabaseDescriptor` stub, or always require a
   live connection?

3. **Browser live reload**: static HTML served once is the right MVP.
   Should a `GET /refresh` endpoint be included from the start, or deferred?

4. **Feature flags**: should the three frontend modes be behind Cargo features
   (`ratatui-frontend`, `egui-frontend`, `browser-frontend`) to avoid pulling
   in all three heavy dependency chains when only one is needed?
