//! Leptos/Axum browser frontend for the archive CLI.
//!
//! Exposes a REST-ish HTTP API backed by the same plugin functions used by the
//! ratatui and egui frontends.  Every HTML response is gated on an
//! [`Established<IrSourced>`] proof token minted by
//! [`ArchiveNavModel::to_verified_tree`], preserving the IR-sourced contract
//! across all three frontends.
//!
//! ## Route summary
//!
//! | Method | Path | Description |
//! |--------|------|-------------|
//! | GET | `/` | Full page (IR-sourced HTML) |
//! | GET | `/api/nav` | Nav-tree HTML fragment |
//! | GET | `/api/preview` | Fetch table rows, return content fragment |
//! | POST | `/api/sql` | Execute SQL, return content fragment |
//! | GET | `/api/inspect` | Table inspection JSON |
//! | GET | `/api/stats` | Column statistics JSON |
//! | GET | `/api/explain` | EXPLAIN viewer content fragment (with params) |
//! | GET | `/api/history` | Query history JSON |
//! | POST | `/api/history` | Append history entry |
//! | GET | `/api/saved` | Saved queries JSON |
//! | POST | `/api/saved` | Save a query |
//! | DELETE | `/api/saved/:id` | Delete a saved query |
//! | GET | `/api/export` | Export data file download |
//! | GET | `/api/open-sql-editor` | Open SQL editor panel (content fragment) |
//! | GET | `/api/history-panel` | History browser panel (content fragment) |
//! | GET | `/api/saved-panel` | Saved queries panel (content fragment) |
//! | GET | `/api/export-panel` | Export picker panel (content fragment) |
//! | GET | `/api/ddl-panel` | DDL viewer panel (content fragment) |
//! | GET | `/api/explain-panel` | EXPLAIN viewer panel (content fragment) |
//! | GET | `/api/col-detail-panel` | Column detail panel (content fragment) |
//! | GET | `/api/load-history` | Load history entry into SQL editor |
//! | GET | `/api/load-saved` | Load saved query into SQL editor |
//! | GET | `/api/open-help` | Help / key bindings panel (content fragment) |
//! | POST | `/api/refresh` | Reload nav tree from DB |
//! | GET | `/api/monitor` | Live monitor snapshot → MonitorPanel (content fragment) |
//! | GET | `/api/monitor-stream` | SSE stream — emits `monitor` events every 5 s |
//! | GET | `/api/admin` | Admin snapshot → AdminPanel (content fragment) |
//! | GET | `/api/admin-tab-next` | Cycle admin tab forward |
//! | GET | `/api/admin-tab-prev` | Cycle admin tab backward |
//! | GET | `/api/erd` | ERD diagram for `?schema=` → ErdPanel (content fragment) |

use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{
        Html, IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{delete, get, post},
};
use elicit_leptos::LeptosRenderer;
use elicit_ui::{UiTreeRenderer as _, VerifiedTree};
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::instrument;

use crate::archive::{
    ArchiveDbBackend, ArchiveResult, BackendKind, ConnectionProfile, ConnectionSet, SslMode,
    errors::{ArchiveError, ArchiveErrorKind},
    nav_model::{ArchiveNavModel, FetchRequest, PanelMode},
    nav_tree::{NavTree, build_nav_tree},
    plugins::{
        export::export_query_result,
        history::HistoryStore,
        inspect::{
            explain_sql_direct, generate_ddl_direct, get_column_stats_direct, inspect_table_direct,
        },
        query::{execute_sql_direct, preview_table_direct},
        saved::SavedQueryStore,
    },
    types::{ExportFormat, QueryHistoryEntry},
};

// ── Shared state ──────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    model: Arc<Mutex<ConnectionSet>>,
    renderer: Arc<LeptosRenderer>,
    history: Arc<Mutex<Option<HistoryStore>>>,
    saved: Arc<Mutex<Option<SavedQueryStore>>>,
}

impl AppState {
    /// Resolve the active connection URL, or `None` in demo mode.
    async fn active_url(&self) -> Option<Arc<String>> {
        self.model.lock().await.conn_active_url().map(Arc::new)
    }
}

// ── IR gate helpers ───────────────────────────────────────────────────────────

fn render_leptos_from_ir(
    renderer: &LeptosRenderer,
    tree: &VerifiedTree,
    _proof: elicitation::Established<elicit_ui::IrSourced>,
) -> Result<String, String> {
    let (html, _stats, _proof) = renderer.render(tree).map_err(|e| e.to_string())?;
    Ok(html)
}

/// Render the full page IR tree.  Used by `serve_page` for the initial page load.
fn full_html(state: &AppState) -> Result<String, String> {
    let model = state
        .model
        .try_lock()
        .map_err(|_| "model locked".to_string())?;
    let (tree, proof) = model.to_verified_tree().map_err(|e| e.to_string())?;
    render_leptos_from_ir(&state.renderer, &tree, proof)
}

/// Render only the content panel fragment (`<div id="content">…</div>`).
///
/// All content-swapping endpoints return this instead of the full page so that
/// HTMX `hx-target="#content" hx-swap="outerHTML"` can find `#content` on the
/// next swap as well.
fn content_html(state: &AppState) -> Result<String, String> {
    let model = state
        .model
        .try_lock()
        .map_err(|_| "model locked".to_string())?;
    let (tree, proof) = model.to_content_tree().map_err(|e| e.to_string())?;
    render_leptos_from_ir(&state.renderer, &tree, proof)
}

/// Set `PanelMode::Error` on the model and render the content HTML for it.
///
/// All API endpoints that need to surface a user-facing error message through
/// the AccessKit IR should call this instead of returning hardcoded HTML.
async fn error_content_html(state: &AppState, message: impl Into<String>) -> Html<String> {
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::Error {
            message: message.into(),
        };
    }
    Html(content_html(state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

/// Render only the nav-tree fragment (`<ul role="tree" id="nav-tree">…</ul>`).
///
/// Used by `/api/nav` with `hx-target="#nav-tree" hx-swap="outerHTML"`.
fn nav_items_html(state: &AppState) -> Result<String, String> {
    let model = state
        .model
        .try_lock()
        .map_err(|_| "model locked".to_string())?;
    let (tree, proof) = model.to_nav_tree().map_err(|e| e.to_string())?;
    render_leptos_from_ir(&state.renderer, &tree, proof)
}

// ── Error type ────────────────────────────────────────────────────────────────

struct ApiError(StatusCode, String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

impl ApiError {
    fn internal(e: impl ToString) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }

    fn bad_request(e: impl ToString) -> Self {
        Self(StatusCode::BAD_REQUEST, e.to_string())
    }

    fn no_db() -> Self {
        Self(
            StatusCode::SERVICE_UNAVAILABLE,
            "No database URL configured — run 'archive serve' instead of 'archive demo'."
                .to_string(),
        )
    }
}

type ApiResult<T> = Result<T, ApiError>;

// ── GET / ─────────────────────────────────────────────────────────────────────

async fn serve_page(State(state): State<AppState>) -> Html<String> {
    match full_html(&state) {
        Ok(body) => Html(wrap_page(&body)),
        Err(e) => Html(format!("<pre>render error: {e}</pre>")),
    }
}

// ── GET /api/nav ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct NavParams {
    #[serde(default)]
    filter: String,
}

async fn api_nav(State(state): State<AppState>, Query(p): Query<NavParams>) -> Html<String> {
    {
        let mut model = state.model.lock().await;
        model.set_filter_str(&p.filter);
    }
    match nav_items_html(&state) {
        Ok(html) => Html(html),
        Err(e) => Html(format!(
            "<ul role=\"tree\" id=\"nav-tree\"><li>{e}</li></ul>"
        )),
    }
}

// ── GET /api/nav-up ───────────────────────────────────────────────────────────

async fn api_nav_up(State(state): State<AppState>) -> Html<String> {
    {
        let mut model = state.model.lock().await;
        model.move_up();
    }
    match nav_items_html(&state) {
        Ok(html) => Html(html),
        Err(e) => Html(format!(
            "<ul role=\"tree\" id=\"nav-tree\"><li>{e}</li></ul>"
        )),
    }
}

// ── GET /api/nav-down ─────────────────────────────────────────────────────────

async fn api_nav_down(State(state): State<AppState>) -> Html<String> {
    {
        let mut model = state.model.lock().await;
        model.move_down();
    }
    match nav_items_html(&state) {
        Ok(html) => Html(html),
        Err(e) => Html(format!(
            "<ul role=\"tree\" id=\"nav-tree\"><li>{e}</li></ul>"
        )),
    }
}

// ── GET /api/nav-enter ────────────────────────────────────────────────────────

async fn api_nav_enter(State(state): State<AppState>) -> Html<String> {
    let fetch_req = {
        let mut model = state.model.lock().await;
        model.toggle_expand()
    };
    match fetch_req {
        None => {
            // Schema toggle: only nav tree changed.
            match nav_items_html(&state) {
                Ok(html) => Html(html),
                Err(e) => Html(format!(
                    "<ul role=\"tree\" id=\"nav-tree\"><li>{e}</li></ul>"
                )),
            }
        }
        Some(FetchRequest::PreviewTable { schema, table }) => {
            // Table selected: return updated nav tree plus an OOB content update.
            let nav_html = nav_items_html(&state)
                .unwrap_or_else(|e| format!("<ul role=\"tree\" id=\"nav-tree\"><li>{e}</li></ul>"));
            // HTMX out-of-band swap for the content panel.
            let content_oob = format!(
                "<div id=\"content\" hx-swap-oob=\"outerHTML\" \
                 hx-get=\"/api/preview?schema={schema}&table={table}\" \
                 hx-trigger=\"load\">Loading {schema}.{table}…</div>"
            );
            Html(format!("{nav_html}{content_oob}"))
        }
        Some(_) => match nav_items_html(&state) {
            Ok(html) => Html(html),
            Err(e) => Html(format!(
                "<ul role=\"tree\" id=\"nav-tree\"><li>{e}</li></ul>"
            )),
        },
    }
}

#[derive(Debug, Deserialize)]
struct SchemaTable {
    schema: String,
    table: String,
}

#[derive(Debug, Deserialize)]
struct ExplainParams {
    schema: String,
    table: String,
    sql: String,
}

#[derive(Debug, Deserialize)]
struct HistoryParams {
    #[serde(default = "default_history_limit")]
    limit: i64,
}

fn default_history_limit() -> i64 {
    50
}

#[derive(Debug, Deserialize)]
struct LoadHistoryParams {
    idx: usize,
}

#[derive(Debug, Deserialize)]
struct LoadSavedParams {
    id: i64,
}

#[derive(Debug, Deserialize)]
struct ExportDownloadParams {
    schema: String,
    table: String,
    format: ExportFormat,
}

// ── Request body structs ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct SqlBody {
    sql: String,
}

#[derive(Debug, Deserialize)]
struct AppendHistoryBody {
    sql: String,
    duration_ms: u64,
    row_count: Option<u64>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SaveQueryBody {
    name: String,
    sql: String,
}

// ── GET /api/preview ──────────────────────────────────────────────────────────

async fn api_preview(
    State(state): State<AppState>,
    Query(p): Query<SchemaTable>,
) -> ApiResult<Html<String>> {
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let result = preview_table_direct(&url, &p.schema, &p.table, 200)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.select_table(&p.schema, &p.table);
        model.panel = PanelMode::DataGrid {
            schema: p.schema,
            table: p.table,
            result,
            page: 0,
            grid_row: 0,
            grid_col: 0,
            edit_state: None,
        };
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

// ── POST /api/sql ─────────────────────────────────────────────────────────────
//
// Always returns 200 Ok with Html<String> — errors are embedded in the IR as
// an Alert node so the content fragment remains self-contained.

async fn api_sql(
    State(state): State<AppState>,
    axum::Json(body): axum::Json<SqlBody>,
) -> Html<String> {
    let url = match state.active_url().await {
        Some(u) => u,
        None => {
            let mut model = state.model.lock().await;
            model.panel = PanelMode::SqlEditor {
                text: body.sql.clone(),
                result: None,
                running: false,
                error: Some(
                    "No database URL configured — run 'archive serve' instead of 'archive demo'."
                        .to_string(),
                ),
            };
            drop(model);
            return Html(
                content_html(&state)
                    .unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")),
            );
        }
    };
    let start = std::time::Instant::now();
    match execute_sql_direct(&url, &body.sql).await {
        Ok(result) => {
            let duration_ms = start.elapsed().as_millis() as u64;
            let row_count = result.row_count;
            {
                let mut model = state.model.lock().await;
                let entry = QueryHistoryEntry {
                    id: 0,
                    executed_at: chrono::Utc::now(),
                    sql: body.sql.clone(),
                    duration_ms,
                    row_count: Some(row_count),
                    error: None,
                };
                model.history_cache.insert(0, entry);
                model.panel = PanelMode::SqlEditor {
                    text: body.sql.clone(),
                    result: Some(result),
                    running: false,
                    error: None,
                };
            }
            {
                let history = state.history.lock().await;
                if let Some(ref store) = *history {
                    store.append_spawn(body.sql, duration_ms, Some(row_count), None);
                }
            }
        }
        Err(e) => {
            let duration_ms = start.elapsed().as_millis() as u64;
            let err_msg = e.to_string();
            {
                let mut model = state.model.lock().await;
                let entry = QueryHistoryEntry {
                    id: 0,
                    executed_at: chrono::Utc::now(),
                    sql: body.sql.clone(),
                    duration_ms,
                    row_count: None,
                    error: Some(err_msg.clone()),
                };
                model.history_cache.insert(0, entry);
                model.panel = PanelMode::SqlEditor {
                    text: body.sql.clone(),
                    result: None,
                    running: false,
                    error: Some(err_msg),
                };
            }
            {
                let history = state.history.lock().await;
                if let Some(ref store) = *history {
                    store.append_spawn(body.sql, duration_ms, None, Some(e.to_string()));
                }
            }
        }
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/inspect ──────────────────────────────────────────────────────────

async fn api_inspect(
    State(state): State<AppState>,
    Query(p): Query<SchemaTable>,
) -> ApiResult<axum::Json<serde_json::Value>> {
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let inspection = inspect_table_direct(&url, &p.schema, &p.table)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.store_inspection(p.schema, p.table, inspection.clone());
    }
    Ok(axum::Json(
        serde_json::to_value(&inspection).map_err(ApiError::internal)?,
    ))
}

// ── GET /api/stats ────────────────────────────────────────────────────────────

async fn api_stats(
    State(state): State<AppState>,
    Query(p): Query<SchemaTable>,
) -> ApiResult<axum::Json<serde_json::Value>> {
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let stats = get_column_stats_direct(&url, &p.schema, &p.table)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.store_column_stats(p.schema, p.table, stats.clone());
    }
    Ok(axum::Json(
        serde_json::to_value(&stats).map_err(ApiError::internal)?,
    ))
}

// ── GET /api/explain (parametric, for old clients) ────────────────────────────

async fn api_explain(
    State(state): State<AppState>,
    Query(p): Query<ExplainParams>,
) -> ApiResult<Html<String>> {
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let root = explain_sql_direct(&url, &p.sql)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        let new_schema = p.schema.clone();
        let new_table = p.table.clone();
        let new_label = format!("EXPLAIN: {}.{}", p.schema, p.table);
        model.panel = if let PanelMode::ExplainPlan {
            schema,
            table,
            root: old_root,
        } = std::mem::take(&mut model.panel)
        {
            let old_label = format!("EXPLAIN: {schema}.{table}");
            PanelMode::ExplainCompare {
                schema: new_schema,
                table: new_table,
                left: old_root,
                label_left: old_label,
                right: root,
                label_right: new_label,
            }
        } else {
            PanelMode::ExplainPlan {
                schema: p.schema,
                table: p.table,
                root,
            }
        };
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

// ── GET /api/history ──────────────────────────────────────────────────────────

async fn api_history_list(
    State(state): State<AppState>,
    Query(p): Query<HistoryParams>,
) -> ApiResult<axum::Json<serde_json::Value>> {
    let entries: Vec<QueryHistoryEntry> = {
        let history = state.history.lock().await;
        if let Some(ref store) = *history {
            store.recent(p.limit).await.map_err(ApiError::internal)?
        } else {
            state
                .model
                .lock()
                .await
                .history_cache
                .iter()
                .take(p.limit as usize)
                .cloned()
                .collect()
        }
    };
    Ok(axum::Json(
        serde_json::to_value(&entries).map_err(ApiError::internal)?,
    ))
}

// ── POST /api/history ─────────────────────────────────────────────────────────

async fn api_history_append(
    State(state): State<AppState>,
    axum::Json(body): axum::Json<AppendHistoryBody>,
) -> ApiResult<StatusCode> {
    let history = state.history.lock().await;
    if let Some(ref store) = *history {
        store
            .append(
                &body.sql,
                body.duration_ms,
                body.row_count,
                body.error.as_deref(),
            )
            .await
            .map_err(ApiError::internal)?;
    }
    Ok(StatusCode::CREATED)
}

// ── GET /api/saved ────────────────────────────────────────────────────────────

async fn api_saved_list(State(state): State<AppState>) -> ApiResult<axum::Json<serde_json::Value>> {
    let saved = state.saved.lock().await;
    let queries = if let Some(ref store) = *saved {
        store.all().await.map_err(ApiError::internal)?
    } else {
        vec![]
    };
    Ok(axum::Json(
        serde_json::to_value(&queries).map_err(ApiError::internal)?,
    ))
}

// ── POST /api/saved ───────────────────────────────────────────────────────────

async fn api_saved_create(
    State(state): State<AppState>,
    axum::Json(body): axum::Json<SaveQueryBody>,
) -> ApiResult<StatusCode> {
    let saved = state.saved.lock().await;
    if let Some(ref store) = *saved {
        store
            .save(&body.name, &body.sql)
            .await
            .map_err(ApiError::internal)?;
    }
    Ok(StatusCode::CREATED)
}

// ── DELETE /api/saved/:id ─────────────────────────────────────────────────────

async fn api_saved_delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    let saved = state.saved.lock().await;
    if let Some(ref store) = *saved {
        store.delete(id).await.map_err(ApiError::internal)?;
    }
    Ok(StatusCode::OK)
}

// ── GET /api/export (file download) ───────────────────────────────────────────

async fn api_export(
    State(state): State<AppState>,
    Query(p): Query<ExportDownloadParams>,
) -> ApiResult<Response> {
    let result = {
        let model = state.model.lock().await;
        match &model.panel {
            PanelMode::DataGrid { result, .. } => result.clone(),
            PanelMode::SqlEditor {
                result: Some(r), ..
            } => r.clone(),
            _ => {
                return Err(ApiError::bad_request(
                    "No active data grid — navigate to a table first",
                ));
            }
        }
    };
    let export = export_query_result(&result, p.format);
    let filename = format!("{}__{}.{}", p.schema, p.table, p.format.extension());
    let content_type = match p.format {
        ExportFormat::Csv | ExportFormat::Tsv => "text/plain; charset=utf-8",
        ExportFormat::Json | ExportFormat::Ndjson => "application/json",
    };
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{filename}\""))
            .map_err(ApiError::internal)?,
    );
    Ok((headers, export.content).into_response())
}

// ── GET /api/open-sql-editor ──────────────────────────────────────────────────

async fn api_open_sql_editor(State(state): State<AppState>) -> Html<String> {
    {
        let mut model = state.model.lock().await;
        // Preserve text if already in SQL editor mode, else start fresh.
        if !matches!(model.panel, PanelMode::SqlEditor { .. }) {
            model.panel = PanelMode::SqlEditor {
                text: String::new(),
                result: None,
                running: false,
                error: None,
            };
        }
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/history-panel ───────────────────────────────────────────────────

async fn api_history_panel(State(state): State<AppState>) -> Html<String> {
    let entries = {
        let history = state.history.lock().await;
        if let Some(ref store) = *history {
            store.recent(50).await.unwrap_or_default()
        } else {
            state
                .model
                .lock()
                .await
                .history_cache
                .iter()
                .take(50)
                .cloned()
                .collect()
        }
    };
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::HistoryPanel { entries };
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/saved-panel ──────────────────────────────────────────────────────

async fn api_saved_panel(State(state): State<AppState>) -> Html<String> {
    let entries = {
        let saved = state.saved.lock().await;
        if let Some(ref store) = *saved {
            store.all().await.unwrap_or_default()
        } else {
            vec![]
        }
    };
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::SavedPanel { entries };
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/export-panel ─────────────────────────────────────────────────────

async fn api_export_panel(State(state): State<AppState>) -> Html<String> {
    let pair = state.model.lock().await.selected_schema_table();
    let (schema, table) = match pair {
        Some(p) => p,
        None => {
            return error_content_html(&state, "Select a table first.").await;
        }
    };
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::ExportPanel { schema, table };
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/ddl-panel ────────────────────────────────────────────────────────

async fn api_ddl_panel(State(state): State<AppState>) -> Html<String> {
    let url = match state.active_url().await {
        Some(u) => u,
        None => {
            return error_content_html(&state, "No database connected.").await;
        }
    };
    let pair = state.model.lock().await.selected_schema_table();
    let (schema, table) = match pair {
        Some(p) => p,
        None => {
            return error_content_html(&state, "Select a table first.").await;
        }
    };
    match generate_ddl_direct(&url, &schema, &table).await {
        Ok(ddl_result) => {
            let mut model = state.model.lock().await;
            model.panel = PanelMode::Ddl {
                schema,
                table,
                ddl: ddl_result.ddl,
            };
        }
        Err(e) => {
            return error_content_html(&state, e.to_string()).await;
        }
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/explain-panel ────────────────────────────────────────────────────

async fn api_explain_panel(State(state): State<AppState>) -> Html<String> {
    let url = match state.active_url().await {
        Some(u) => u,
        None => {
            return error_content_html(&state, "No database connected.").await;
        }
    };
    let (schema, table, sql) = {
        let model = state.model.lock().await;
        let pair = model.selected_schema_table();
        let sql = if let PanelMode::SqlEditor { text, .. } = &model.panel {
            if !text.trim().is_empty() {
                text.clone()
            } else {
                pair.as_ref()
                    .map(|(s, t)| format!("SELECT * FROM {s}.{t} LIMIT 10"))
                    .unwrap_or_default()
            }
        } else {
            pair.as_ref()
                .map(|(s, t)| format!("SELECT * FROM {s}.{t} LIMIT 10"))
                .unwrap_or_default()
        };
        let (s, t) = pair.unwrap_or_else(|| ("public".to_string(), "query".to_string()));
        (s, t, sql)
    };
    if sql.is_empty() {
        return error_content_html(&state, "Select a table or open the SQL editor first.").await;
    }
    match explain_sql_direct(&url, &sql).await {
        Ok(root) => {
            let mut model = state.model.lock().await;
            let new_label = format!("EXPLAIN: {schema}.{table}");
            model.panel = if let PanelMode::ExplainPlan {
                schema: old_schema,
                table: old_table,
                root: old_root,
            } = std::mem::take(&mut model.panel)
            {
                let old_label = format!("EXPLAIN: {old_schema}.{old_table}");
                PanelMode::ExplainCompare {
                    schema: schema.clone(),
                    table: table.clone(),
                    left: old_root,
                    label_left: old_label,
                    right: root,
                    label_right: new_label,
                }
            } else {
                PanelMode::ExplainPlan {
                    schema,
                    table,
                    root,
                }
            };
        }
        Err(e) => {
            return error_content_html(&state, e.to_string()).await;
        }
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/col-detail-panel ─────────────────────────────────────────────────

async fn api_col_detail_panel(State(state): State<AppState>) -> Html<String> {
    let url = match state.active_url().await {
        Some(u) => u,
        None => {
            return error_content_html(&state, "No database connected.").await;
        }
    };
    let pair = state.model.lock().await.selected_schema_table();
    let (schema, table) = match pair {
        Some(p) => p,
        None => {
            return error_content_html(&state, "Select a table first.").await;
        }
    };
    let (inspect_res, stats_res) = tokio::join!(
        inspect_table_direct(&url, &schema, &table),
        get_column_stats_direct(&url, &schema, &table),
    );
    {
        let mut model = state.model.lock().await;
        if let Ok(inspection) = inspect_res {
            model.store_inspection(schema.clone(), table.clone(), inspection);
        }
        if let Ok(stats) = stats_res {
            model.store_column_stats(schema.clone(), table.clone(), stats);
        }
        model.select_table(&schema, &table);
        model.panel = PanelMode::ColumnDetail;
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/load-history ─────────────────────────────────────────────────────

async fn api_load_history(
    State(state): State<AppState>,
    Query(p): Query<LoadHistoryParams>,
) -> Html<String> {
    let sql = {
        let history = state.history.lock().await;
        if let Some(ref store) = *history {
            let entries = store.recent(p.idx as i64 + 1).await.unwrap_or_default();
            entries.into_iter().nth(p.idx).map(|e| e.sql)
        } else {
            let model = state.model.lock().await;
            model.history_cache.iter().nth(p.idx).map(|e| e.sql.clone())
        }
    };
    let sql = match sql {
        Some(s) => s,
        None => {
            return error_content_html(&state, "History entry not found.").await;
        }
    };
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::SqlEditor {
            text: sql,
            result: None,
            running: false,
            error: None,
        };
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/load-saved ───────────────────────────────────────────────────────

async fn api_load_saved(
    State(state): State<AppState>,
    Query(p): Query<LoadSavedParams>,
) -> Html<String> {
    let sql = {
        let saved = state.saved.lock().await;
        if let Some(ref store) = *saved {
            let entries = store.all().await.unwrap_or_default();
            entries.into_iter().find(|q| q.id == p.id).map(|q| q.sql)
        } else {
            None
        }
    };
    let sql = match sql {
        Some(s) => s,
        None => {
            return error_content_html(&state, "Saved query not found.").await;
        }
    };
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::SqlEditor {
            text: sql,
            result: None,
            running: false,
            error: None,
        };
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/monitor ─────────────────────────────────────────────────────────

/// Optional schema query param for monitor endpoint.
#[derive(Debug, serde::Deserialize)]
struct MonitorSchemaQuery {
    /// Schema for table-bloat and index-usage queries; defaults to `"public"`.
    #[serde(default = "default_public_schema")]
    schema: String,
}

fn default_public_schema() -> String {
    "public".to_string()
}

/// Fetch live monitor data and transition the panel to `MonitorPanel`.
///
/// Queries active sessions, roles, cache hit ratio, backup labels, slow
/// queries, lock waits, table bloat, and index usage from the active
/// connection, then applies the resulting [`MonitorSnapshot`] to the
/// shared model before returning the updated content fragment.
async fn api_monitor(
    State(state): State<AppState>,
    Query(params): Query<MonitorSchemaQuery>,
) -> Result<Html<String>, ApiError> {
    use crate::archive::types::{MonitorSnapshot, MonitorTab};
    use elicit_db::{DbBackupManager, DbMonitor, DbRoleManager};
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let backend = ArchiveDbBackend::connect(&url)
        .await
        .map_err(ApiError::internal)?;
    let stat = backend
        .active_sessions()
        .await
        .map_err(ApiError::internal)?;
    let roles = backend.list_roles().await.map_err(ApiError::internal)?;
    let cache_hit = backend.cache_hit_ratio().await.ok();
    let backups = backend.list_backups().await.unwrap_or_default();
    let slow_queries = backend.slow_queries(1_000).await.unwrap_or_default();
    let lock_waits = backend.lock_waits().await.unwrap_or_default();
    let table_bloat = backend
        .table_bloat(&params.schema)
        .await
        .unwrap_or_default();
    let index_usage = backend
        .index_usage(&params.schema)
        .await
        .unwrap_or_default();
    let snapshot = MonitorSnapshot {
        sessions: stat.sessions,
        roles,
        cache_hit,
        backups,
        slow_queries,
        lock_waits,
        table_bloat,
        index_usage,
        active_tab: MonitorTab::Sessions,
    };
    {
        let mut model = state.model.lock().await;
        model.apply_monitor_snapshot(snapshot);
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

// ── GET /api/monitor-stream ───────────────────────────────────────────────────

/// Collect a fresh [`MonitorSnapshot`] from `url` with the given schema.
///
/// Shared by [`api_monitor`] and the SSE stream to avoid duplicating DB logic.
async fn fetch_monitor_snapshot(
    url: &str,
    schema: &str,
) -> Option<crate::archive::types::MonitorSnapshot> {
    use crate::archive::types::{MonitorSnapshot, MonitorTab};
    use elicit_db::{DbBackupManager, DbMonitor, DbRoleManager};
    let backend = ArchiveDbBackend::connect(url).await.ok()?;
    let stat = backend.active_sessions().await.ok()?;
    let roles = backend.list_roles().await.ok()?;
    let cache_hit = backend.cache_hit_ratio().await.ok();
    let backups = backend.list_backups().await.unwrap_or_default();
    let slow_queries = backend.slow_queries(1_000).await.unwrap_or_default();
    let lock_waits = backend.lock_waits().await.unwrap_or_default();
    let table_bloat = backend.table_bloat(schema).await.unwrap_or_default();
    let index_usage = backend.index_usage(schema).await.unwrap_or_default();
    Some(MonitorSnapshot {
        sessions: stat.sessions,
        roles,
        cache_hit,
        backups,
        slow_queries,
        lock_waits,
        table_bloat,
        index_usage,
        active_tab: MonitorTab::Sessions,
    })
}

/// Server-Sent Events stream that emits a refreshed [`MonitorSnapshot`] every
/// 5 seconds as a named `monitor` event.
///
/// The browser subscribes via `EventSource('/api/monitor-stream')` and, on
/// each event, re-fetches `/api/monitor` to update the content panel if it is
/// currently showing the monitor view (detected by the `data-panel="monitor"`
/// sentinel injected by the IR pipeline).
///
/// When no DB connection is active the stream still runs but emits no data
/// events; the axum keep-alive mechanism sends a ping every 15 s to prevent
/// proxy/browser timeout.
#[instrument(skip(state))]
async fn api_monitor_stream(
    State(state): State<AppState>,
    Query(params): Query<MonitorSchemaQuery>,
) -> Sse<impl futures::Stream<Item = Result<Event, std::convert::Infallible>>> {
    use futures::stream;
    use std::convert::Infallible;
    use std::time::Duration;

    let schema = params.schema;
    let stream = stream::unfold((state, schema), |(state, schema)| async move {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let event = if let Some(url) = state.active_url().await {
            match fetch_monitor_snapshot(&url, &schema).await {
                Some(snapshot) => {
                    let data =
                        serde_json::to_string(&snapshot).unwrap_or_else(|_| "{}".to_string());
                    Event::default().event("monitor").data(data)
                }
                None => Event::default().comment("error"),
            }
        } else {
            Event::default().comment("no-connection")
        };
        Some((Ok::<Event, Infallible>(event), (state, schema)))
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("ping"),
    )
}

// ── GET /api/admin ────────────────────────────────────────────────────────────

/// Fetch administration data and transition the panel to `AdminPanel`.
///
/// Assembles an [`AdminSnapshot`] from roles, backups, WAL status, server
/// version, extensions, and top-20 GUC settings, then applies it to the shared
/// model before returning the updated content fragment.
async fn api_admin(State(state): State<AppState>) -> Result<Html<String>, ApiError> {
    use crate::archive::types::{AdminSnapshot, AdminTab};
    use elicit_db::{DbBackupManager, DbRoleManager, DbServerAdmin};
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let backend = ArchiveDbBackend::connect(&url)
        .await
        .map_err(ApiError::internal)?;
    let roles = backend.list_roles().await.map_err(ApiError::internal)?;
    let backups = backend.list_backups().await.unwrap_or_default();
    let wal_ready = backend.wal_status().await.map(|_| true).unwrap_or(false);
    let server_version = backend
        .server_version()
        .await
        .unwrap_or_else(|_| "unknown".to_string());
    let extensions = backend.list_extensions().await.unwrap_or_default();
    let settings = backend
        .list_settings()
        .await
        .unwrap_or_default()
        .into_iter()
        .take(20)
        .collect();
    let snapshot = AdminSnapshot {
        roles,
        backups,
        wal_ready,
        server_version,
        extensions,
        settings,
        active_tab: AdminTab::default(),
    };
    {
        let mut model = state.model.lock().await;
        model.apply_admin_snapshot(snapshot);
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

// ── GET /api/admin-tab-next / prev ────────────────────────────────────────────

/// Cycle the admin panel to the next tab and return the updated content.
async fn api_admin_tab_next(State(state): State<AppState>) -> Html<String> {
    {
        let mut model = state.model.lock().await;
        model.admin_tab_next();
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

/// Cycle the admin panel to the previous tab and return the updated content.
async fn api_admin_tab_prev(State(state): State<AppState>) -> Html<String> {
    {
        let mut model = state.model.lock().await;
        model.admin_tab_prev();
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── GET /api/erd ──────────────────────────────────────────────────────────────

/// Query params for `GET /api/erd`.
#[derive(serde::Deserialize)]
struct ErdQuery {
    /// Schema to diagram.
    schema: String,
}

/// Fetch the ERD diagram for the given `?schema=` and render `ErdPanel`.
async fn api_erd(
    State(state): State<AppState>,
    Query(params): Query<ErdQuery>,
) -> Result<Html<String>, ApiError> {
    use crate::archive::nav_tree::fetch_erd;
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let backend = ArchiveDbBackend::connect(&url)
        .await
        .map_err(ApiError::internal)?;
    let diagram = fetch_erd(&backend, &url, &params.schema)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.apply_erd_diagram(diagram);
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

// ── GET /api/constraints ──────────────────────────────────────────────────────

/// Query params for `GET /api/constraints`.
#[derive(serde::Deserialize)]
struct SchemaTableQuery {
    /// Schema of the table.
    schema: String,
    /// Table name.
    table: String,
}

/// Fetch constraint data for `?schema=&table=` and render `ConstraintPanel`.
async fn api_constraints(
    State(state): State<AppState>,
    Query(params): Query<SchemaTableQuery>,
) -> Result<Html<String>, ApiError> {
    use crate::archive::plugins::inspect::inspect_table_direct;
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let insp = inspect_table_direct(&url, &params.schema, &params.table)
        .await
        .map_err(ApiError::bad_request)?;
    {
        let mut model = state.model.lock().await;
        model.apply_constraints(params.schema, params.table, insp.constraints);
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

// ── GET /api/indexes ──────────────────────────────────────────────────────────

/// Fetch index data for `?schema=&table=` and render `IndexPanel`.
async fn api_indexes(
    State(state): State<AppState>,
    Query(params): Query<SchemaTableQuery>,
) -> Result<Html<String>, ApiError> {
    use crate::archive::plugins::inspect::inspect_table_direct;
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let insp = inspect_table_direct(&url, &params.schema, &params.table)
        .await
        .map_err(ApiError::bad_request)?;
    {
        let mut model = state.model.lock().await;
        model.apply_indexes(params.schema, params.table, insp.indexes);
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

async fn api_open_help(State(state): State<AppState>) -> Html<String> {
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::HelpPanel;
    }
    Html(content_html(&state).unwrap_or_else(|e| format!("<div id='content'><pre>{e}</pre></div>")))
}

// ── POST /api/refresh ─────────────────────────────────────────────────────────

async fn api_refresh(State(state): State<AppState>) -> ApiResult<Html<String>> {
    let url = state.active_url().await.ok_or_else(ApiError::no_db)?;
    let backend = ArchiveDbBackend::connect(&url)
        .await
        .map_err(ApiError::internal)?;
    let nav = build_nav_tree(&backend, &url)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.apply_refresh(nav);
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

// ── Connection switching ──────────────────────────────────────────────────────

/// Request body for `POST /api/switch-connection`.
#[derive(serde::Deserialize)]
struct SwitchConnectionParams {
    /// Zero-based index of the connection to make active.
    index: usize,
}

/// Switch the active connection and refresh the nav tree.
async fn api_switch_connection(
    State(state): State<AppState>,
    axum::Json(params): axum::Json<SwitchConnectionParams>,
) -> Result<Html<String>, ApiError> {
    {
        let mut model = state.model.lock().await;
        if !model.conn_set_active(params.index) {
            let len = model.conn_len();
            return Err(ApiError::bad_request(format!(
                "Connection index {} out of range (have {})",
                params.index, len
            )));
        }
    }
    // Refresh nav tree on the new connection.
    if let Some(url) = state.active_url().await {
        let backend = ArchiveDbBackend::connect(&url)
            .await
            .map_err(ApiError::internal)?;
        let nav = build_nav_tree(&backend, &url)
            .await
            .map_err(ApiError::internal)?;
        let mut model = state.model.lock().await;
        model.apply_refresh(nav);
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

// ── Keyboard action endpoint ──────────────────────────────────────────────────

/// `POST /api/action` — receive a JSON `{action: "VariantName"}` from the
/// browser's keydown listener (generated by [`ArchiveKeyMap::to_js_listener`]),
/// parse it as an [`ArchiveAction`], and dispatch it on the shared model state.
///
/// Returns the updated `#content` partial HTML on success or a 400 on parse
/// failure.
async fn api_action(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Html<String>, ApiError> {
    use crate::archive::{ArchiveAction, ArchiveKeyMap};
    let name = body
        .get("action")
        .and_then(|v: &serde_json::Value| v.as_str())
        .ok_or_else(|| ApiError::bad_request("missing action field"))?;
    let keymap = ArchiveKeyMap::default_map();
    let mode = {
        let model = state.model.lock().await;
        model.current_mode()
    };
    let action: ArchiveAction = keymap
        .entries()
        .iter()
        .find(|e| format!("{:?}", e.action) == name && (e.mode == Some(mode) || e.mode.is_none()))
        .map(|e| e.action.clone())
        .ok_or_else(|| ApiError::bad_request(format!("unknown action: {name}")))?;
    {
        let mut model = state.model.lock().await;
        dispatch_action_on_model(&mut model, action);
    }
    Ok(Html(content_html(&state).map_err(ApiError::internal)?))
}

/// Dispatch a parsed [`ArchiveAction`] against the shared model.
///
/// This is the server-side equivalent of [`ArchiveFrontend::dispatch_action`].
/// Navigation actions mutate the model; data-fetching actions are no-ops
/// because the browser sends dedicated HTMX requests for those.
fn dispatch_action_on_model(
    model: &mut crate::archive::nav_model::ConnectionSet,
    action: crate::archive::ArchiveAction,
) {
    use crate::archive::ArchiveAction as A;
    use crate::archive::nav_model::PanelMode;
    match action {
        A::MoveUp => {
            model.move_up();
        }
        A::MoveDown => {
            model.move_down();
        }
        A::Select => {
            model.toggle_expand();
        }
        A::Refresh => {} // HTMX handles refreshes via /api/refresh
        A::ToggleHelp => model.toggle_help(),
        A::OpenFilter => model.open_filter(),
        A::OpenSqlEditor => {
            model.panel = PanelMode::SqlEditor {
                text: String::new(),
                result: None,
                running: false,
                error: None,
            };
        }
        A::OpenSavedBrowser => model.toggle_saved_browser(),
        A::OpenMonitor => {
            // Transition to loading state; /api/monitor HTMX call supplies the snapshot.
            let _ = model.toggle_monitor_panel();
        }
        A::OpenAdmin => {
            // Transition to loading state; /api/admin HTMX call supplies the snapshot.
            let _ = model.toggle_admin_panel();
        }
        A::AdminTabNext => model.admin_tab_next(),
        A::AdminTabPrev => model.admin_tab_prev(),
        A::OpenErd => {
            // Transition to loading state; /api/erd HTMX call supplies the diagram.
            let _ = model.toggle_erd_panel();
        }
        A::OpenConstraints => {
            // Transition to loading state; /api/constraints HTMX call supplies data.
            let _ = model.toggle_constraint_panel();
        }
        A::OpenIndexes => {
            // Transition to loading state; /api/indexes HTMX call supplies data.
            let _ = model.toggle_index_panel();
        }
        A::EditConnection => {
            let profile = model.conn_active_profile().clone();
            model.toggle_connection_editor(profile);
        }
        A::ToggleExportPicker => {
            if model.panel.is_data_grid() {
                model.toggle_export_picker();
            }
        }
        A::RequestDdl => {} // HTMX handles via /api/ddl-panel
        A::RequestExplain => {}
        A::ClearExplainCompare => {
            if let PanelMode::ExplainCompare {
                schema,
                table,
                left,
                label_left: _,
                right: _,
                label_right: _,
            } = std::mem::take(&mut model.panel)
            {
                model.panel = PanelMode::ExplainPlan {
                    schema,
                    table,
                    root: left,
                };
            }
        }
        A::PageNext => model.page_next(),
        A::PagePrev => model.page_prev(),
        A::PageFirst => model.page_first(),
        A::PageLast => model.page_last(),
        A::ConnNext => model.conn_next(),
        A::ConnPrev => model.conn_prev(),
        // All interactive-modal actions are handled by their own HTMX endpoints
        A::Quit
        | A::FilterClose
        | A::FilterBackspace
        | A::SavePromptClose
        | A::SavePromptBackspace
        | A::SavePromptConfirm
        | A::SavedBrowserClose
        | A::SavedBrowserUp
        | A::SavedBrowserDown
        | A::SavedBrowserSelect
        | A::SavedBrowserDelete
        | A::ExportPickerClose
        | A::ExportPickerUp
        | A::ExportPickerDown
        | A::ExportPickerConfirm
        | A::SqlRun
        | A::SqlHistoryPrev
        | A::SqlHistoryNext
        | A::SqlClose
        | A::SqlSave
        | A::SqlBackspace
        | A::SqlNewline => {}
    }
}

// ── Router assembly ───────────────────────────────────────────────────────────

fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(serve_page))
        .route("/api/nav", get(api_nav))
        .route("/api/nav-up", get(api_nav_up))
        .route("/api/nav-down", get(api_nav_down))
        .route("/api/nav-enter", get(api_nav_enter))
        .route("/api/preview", get(api_preview))
        .route("/api/sql", post(api_sql))
        .route("/api/inspect", get(api_inspect))
        .route("/api/stats", get(api_stats))
        .route("/api/explain", get(api_explain))
        .route(
            "/api/history",
            get(api_history_list).post(api_history_append),
        )
        .route("/api/saved", get(api_saved_list).post(api_saved_create))
        .route("/api/saved/{id}", delete(api_saved_delete))
        .route("/api/export", get(api_export))
        .route("/api/refresh", post(api_refresh))
        .route("/api/open-sql-editor", get(api_open_sql_editor))
        .route("/api/monitor", get(api_monitor))
        .route("/api/monitor-stream", get(api_monitor_stream))
        .route("/api/admin", get(api_admin))
        .route("/api/admin-tab-next", get(api_admin_tab_next))
        .route("/api/admin-tab-prev", get(api_admin_tab_prev))
        .route("/api/erd", get(api_erd))
        .route("/api/constraints", get(api_constraints))
        .route("/api/indexes", get(api_indexes))
        .route("/api/open-help", get(api_open_help))
        .route("/api/history-panel", get(api_history_panel))
        .route("/api/saved-panel", get(api_saved_panel))
        .route("/api/export-panel", get(api_export_panel))
        .route("/api/ddl-panel", get(api_ddl_panel))
        .route("/api/explain-panel", get(api_explain_panel))
        .route("/api/col-detail-panel", get(api_col_detail_panel))
        .route("/api/load-history", get(api_load_history))
        .route("/api/load-saved", get(api_load_saved))
        .route("/api/switch-connection", post(api_switch_connection))
        .route("/api/action", post(api_action))
        .with_state(state)
}

// ── HTML shell ────────────────────────────────────────────────────────────────

/// SQL syntax highlighting JavaScript (inline, no CDN dependency).
///
/// Defines `_sqlHL(text) → HTML` using a simple token-splitting regex that
/// recognises strings, `--`/`/* */` comments, numeric literals, and SQL
/// keywords (case-insensitive).  The result uses inline `style=` colouring
/// with Catppuccin Mocha palette values.
///
/// Wired up via `input` events on `.sql-ta` elements and re-run after every
/// HTMX settle so freshly-injected fragments also get highlighted.
const SQL_HL_JS: &str = r#"
(function(){
var KW=new Set("SELECT FROM WHERE JOIN LEFT RIGHT INNER OUTER FULL CROSS ON GROUP BY ORDER HAVING LIMIT OFFSET INSERT INTO VALUES UPDATE SET DELETE CREATE TABLE VIEW INDEX DROP ALTER ADD COLUMN CONSTRAINT PRIMARY KEY FOREIGN REFERENCES UNIQUE NOT NULL DEFAULT AND OR IN IS LIKE ILIKE BETWEEN EXISTS CASE WHEN THEN ELSE END AS DISTINCT ALL UNION INTERSECT EXCEPT WITH RETURNING BEGIN COMMIT ROLLBACK TRANSACTION EXPLAIN ANALYZE TRUNCATE GRANT REVOKE SCHEMA DATABASE SEQUENCE FUNCTION PROCEDURE TRIGGER EXTENSION".split(' '));
var RE=/(\'(?:[^\'\\]|\\.)*\'|"(?:[^"\\]|\\.)*"|--[^\n]*|\/\*[\s\S]*?\*\/|\b\d+(?:\.\d+)?\b|\b[A-Za-z_][A-Za-z0-9_]*\b)/;
function esc(t){return t.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');}
function hl(text){
  var out='',s=text;
  while(s.length){
    var m=RE.exec(s);
    if(!m){out+=esc(s);break;}
    if(m.index>0)out+=esc(s.slice(0,m.index));
    var v=m[0],st='';
    if(v[0]==="'"||v[0]==='"')st='color:#a6e3a1';
    else if(v[0]==='-'||v[0]==='/')st='color:#6c7086;font-style:italic';
    else if(/^\d/.test(v))st='color:#fab387';
    else if(KW.has(v.toUpperCase()))st='color:#cba6f7;font-weight:bold';
    out+=st?'<span style="'+st+'">'+esc(v)+'</span>':esc(v);
    s=s.slice(m.index+v.length);
  }
  return out;
}
function sync(ta){
  var pre=ta.parentElement&&ta.parentElement.querySelector('.code-output');
  if(!pre)return;
  pre.innerHTML=hl(ta.value);
  var h=ta.style.height;
  if(h)pre.style.minHeight=h;
}
document.addEventListener('input',function(e){if(e.target.classList.contains('sql-ta'))sync(e.target);});
document.addEventListener('htmx:afterSettle',function(){
  document.querySelectorAll('.sql-ta').forEach(sync);
});
document.querySelectorAll('.sql-ta').forEach(sync);
})();
"#;

fn wrap_page(body: &str) -> String {
    let css = concat!(
        "*{box-sizing:border-box;margin:0;padding:0}",
        "body{font-family:'Cascadia Code','Fira Code',Consolas,monospace;",
        "background:#1e1e2e;color:#cdd6f4;height:100vh;display:flex;",
        "flex-direction:column;overflow:hidden}",
        // toolbar
        "header.toolbar{display:flex;gap:.4rem;padding:.3rem .6rem;",
        "background:#181825;border-bottom:1px solid #45475a;flex-shrink:0;",
        "align-items:center}",
        "header.toolbar [data-role='statictext']{color:#cba6f7;font-weight:bold;",
        "margin-right:.6rem;font-size:.9rem}",
        "header.toolbar button{background:#313244;color:#cdd6f4;border:1px solid #45475a;",
        "border-radius:3px;padding:.15rem .55rem;font-family:inherit;font-size:.8rem;",
        "cursor:pointer}",
        "header.toolbar button:hover{background:#45475a}",
        "header.toolbar button.active{border-color:#89b4fa;color:#89b4fa}",
        "main{flex:1;display:flex;overflow:hidden}",
        "nav{width:22rem;min-width:12rem;border-right:1px solid #45475a;",
        "display:flex;flex-direction:column;overflow:hidden}",
        "#nav-filter{width:100%;background:#313244;color:#cdd6f4;",
        "border:none;border-bottom:1px solid #313244;padding:.35rem .5rem;",
        "font-family:inherit;font-size:.85rem;outline:none;flex-shrink:0}",
        "#nav-filter:focus{border-bottom-color:#89b4fa}",
        "#nav-tree{flex:1;overflow-y:auto;padding:.25rem 0}",
        "#content{flex:1;overflow:auto;padding:.5rem 1rem}",
        // SQL results feedback
        ".sql-status{font-size:.8rem;color:#a6adc8;margin:.25rem 0}",
        ".sql-status.ok{color:#a6e3a1}",
        ".sql-status.err{color:#f38ba8}",
        // nav tree
        "ul[role='tree']{list-style:none;padding:.25rem 0}",
        "ul[role='group']{list-style:none;padding-left:1.5rem}",
        "details.schema-group>summary{padding:.2rem .75rem;cursor:pointer;",
        "font-size:.9rem;color:#89b4fa;list-style:none;outline:none}",
        "li[role='treeitem']{padding:.2rem .75rem;cursor:pointer;",
        "font-size:.9rem;color:#a6adc8;user-select:none}",
        "li[role='treeitem']:hover{background:#313244;border-radius:3px}",
        "li[role='treeitem'].selected{background:#313244;border-radius:4px;",
        "outline:2px solid #89b4fa;color:#cdd6f4}",
        // data table
        "table{border-collapse:collapse;width:100%;font-size:.85rem}",
        "thead th{background:#181825;border-bottom:2px solid #45475a;",
        "padding:.3rem .6rem;text-align:left;color:#89b4fa;position:sticky;top:0}",
        "tbody tr:nth-child(even){background:#181825}",
        "tbody td{padding:.25rem .6rem;border-bottom:1px solid #313244}",
        // code/pre (DDL display)
        "code,pre{white-space:pre;display:block;overflow-x:auto;",
        "background:#181825;border:1px solid #45475a;border-radius:3px;",
        "padding:.5rem;font-size:.8rem;color:#a6e3a1;line-height:1.5}",
        "textarea{width:100%;background:#181825;color:#cdd6f4;",
        "border:1px solid #45475a;padding:.5rem;font-family:inherit;",
        "font-size:.9rem;resize:vertical;min-height:8rem}",
        "textarea:focus{border-color:#89b4fa;outline:none}",
        "h2,h3{color:#cba6f7;margin:.5rem 0 .25rem}",
        ".htmx-request .spinner{display:inline-block}",
        ".spinner{display:none;margin-left:.5rem}",
        "footer[role='status']{padding:.2rem .5rem;background:#313244;",
        "border-top:1px solid #45475a;font-size:.75rem;flex-shrink:0}",
        // export buttons inside the ExportPanel content
        ".export-fmt-btn{background:#313244;color:#cdd6f4;border:1px solid #45475a;",
        "border-radius:3px;padding:.3rem .7rem;font-family:inherit;font-size:.85rem;cursor:pointer}",
        ".export-fmt-btn:hover{background:#45475a}",
        // ERD diagram (SVG-based)
        "svg.erd-diagram{display:block;max-width:100%;height:auto;overflow:visible}",
        ".erd-box{fill:#313244;stroke:#45475a;stroke-width:1}",
        ".erd-header{fill:#1e1e2e;stroke:#45475a;stroke-width:1}",
        ".erd-title{fill:#89b4fa;font-size:12px;text-anchor:middle;dominant-baseline:middle;",
        "font-family:'Cascadia Code','Fira Code',monospace;font-weight:bold}",
        ".erd-col{fill:#cdd6f4;font-size:10px;font-family:'Cascadia Code','Fira Code',monospace}",
        ".erd-edge{stroke:#6c7086;stroke-width:1.5;fill:none;marker-end:url(#erd-arrow)}",
        // SQL editor overlay (textarea on top of highlighted <pre>)
        ".code-wrap{display:grid;min-height:8rem}",
        ".code-wrap>*{grid-area:1/1;width:100%;min-height:8rem;",
        "padding:.5rem;font-family:'Cascadia Code','Fira Code',Consolas,monospace;",
        "font-size:.9rem;line-height:1.5;tab-size:4;white-space:pre-wrap;word-break:break-word;",
        "box-sizing:border-box;border:1px solid #45475a;border-radius:3px;margin:0}",
        ".code-output{background:#181825;color:#cdd6f4;pointer-events:none;overflow:hidden;",
        "z-index:0;resize:none}",
        ".sql-ta{background:transparent;color:transparent;caret-color:#cdd6f4;",
        "outline:none;resize:vertical;z-index:1;position:relative}",
        ".sql-ta:focus~.code-output,.sql-ta:focus+.code-output{border-color:#89b4fa}",
    );

    // JS: only IR-safe helpers — no HTML building, no side panels.
    let js_static = concat!(
        // ── run SQL via fetch (200 always; IR embeds errors as Alert nodes) ──
        "function runSql(){",
        "var ta=document.querySelector('#content textarea');",
        "if(!ta)return;",
        "var sql=ta.value;",
        "if(!sql.trim())return;",
        "fetch('/api/sql',{",
        "method:'POST',",
        "headers:{'Content-Type':'application/json'},",
        "body:JSON.stringify({sql:sql})",
        "}).then(function(r){return r.text();}).then(function(html){",
        "var el=document.getElementById('content');",
        "if(el){el.outerHTML=html;htmx.process(document.body);}",
        "}).catch(function(err){",
        "var el=document.getElementById('content');",
        "if(el)el.innerHTML='<p class=\"sql-status err\">Network error: '+err.message+'</p>';",
        "});",
        "}",
        // ── save current SQL via native prompt ──
        "function saveCurrentSql(){",
        "var ta=document.querySelector('#content textarea');",
        "if(!ta||!ta.value.trim()){alert('No SQL to save.');return;}",
        "var name=prompt('Save query as:');",
        "if(!name)return;",
        "fetch('/api/saved',{method:'POST',",
        "headers:{'Content-Type':'application/json'},",
        "body:JSON.stringify({name:name,sql:ta.value})})",
        ".then(function(r){if(r.ok)alert('Saved!');else r.text().then(function(t){alert('Error: '+t);});});",
        "}",
        // ── data-action button dispatcher ──
        "document.addEventListener('click',function(e){",
        "var btn=e.target.closest('[data-action]');",
        "if(!btn)return;",
        "var action=btn.getAttribute('data-action');",
        "if(action==='run-sql'){e.preventDefault();runSql();}",
        "if(action==='save-sql'){e.preventDefault();saveCurrentSql();}",
        "});",
        // ── SSE live monitor refresh ──
        // One persistent EventSource for the lifetime of the page.  On each
        // "monitor" event: if the content area is currently showing the monitor
        // panel (sentinel attr data-panel="monitor"), re-fetch /api/monitor and
        // swap #content so the IR-rendered HTML stays consistent.
        "(function(){",
        "var src=new EventSource('/api/monitor-stream');",
        "src.addEventListener('monitor',function(){",
        "if(!document.querySelector('[data-panel=\"monitor\"]'))return;",
        "fetch('/api/monitor').then(function(r){return r.text();}).then(function(html){",
        "if(!document.querySelector('[data-panel=\"monitor\"]'))return;",
        "var el=document.getElementById('content');",
        "if(el){el.outerHTML=html;htmx.process(document.body);}",
        "});",
        "});",
        "})();",
        // ── keyboard shortcuts (derived from ArchiveKeyMap) ──
    );
    // Append the dynamically generated key listener from the IR key map.
    let js = format!(
        "{js_static}{}{SQL_HL_JS}",
        crate::archive::ArchiveKeyMap::default_map()
            .to_js_listener(crate::archive::KeyMapMode::Default)
    );

    format!(
        "<!DOCTYPE html><html lang=\"en\"><head>\
<meta charset=\"utf-8\"/>\
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"/>\
<title>Archive</title>\
<script src=\"https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js\"></script>\
<style>{css}</style>\
</head><body>\
{body}\
<script>{js}</script>\
</body></html>"
    )
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Serve the archive browser as a dynamic axum HTTP server on `port`.
///
/// When `url` is `Some`, all API routes that interact with the database become
/// active.  Without a URL (demo mode) the page renders the pre-loaded nav tree
/// but data-fetching endpoints return 503.
#[instrument(skip(nav))]
pub async fn run_browser(nav: NavTree, url: Option<String>, port: u16) -> ArchiveResult<()> {
    let profile = ConnectionProfile {
        name: "primary".to_string(),
        url_env_key: url.clone().unwrap_or_default(),
        backend: BackendKind::Postgres,
        color: None,
        ssh_host: None,
        ssh_port: None,
        ssh_user: None,
        ssh_key_env: None,
        ssl_mode: SslMode::Prefer,
        ssl_cert_env: None,
        ssl_key_env: None,
        ssl_ca_env: None,
    };
    let connections = ConnectionSet::from_single(profile, ArchiveNavModel::new(nav), url);
    let model = Arc::new(Mutex::new(connections));
    let renderer = Arc::new(LeptosRenderer::html());
    let history = Arc::new(Mutex::new(HistoryStore::open().await.ok()));
    let saved = Arc::new(Mutex::new(SavedQueryStore::open().await.ok()));

    let state = AppState {
        model,
        renderer,
        history,
        saved,
    };
    let router = build_router(state);

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))?;

    tracing::info!(addr = %addr, "archive browser frontend listening");
    eprintln!("Archive browser: http://localhost:{port}/");

    axum::serve(listener, router)
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))?;

    Ok(())
}
