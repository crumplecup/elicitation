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
//! | GET | `/api/ddl` | DDL viewer content fragment |
//! | GET | `/api/stats` | Column statistics JSON |
//! | GET | `/api/explain` | EXPLAIN viewer content fragment |
//! | GET | `/api/history` | Query history JSON |
//! | POST | `/api/history` | Append history entry |
//! | GET | `/api/saved` | Saved queries JSON |
//! | POST | `/api/saved` | Save a query |
//! | DELETE | `/api/saved/:id` | Delete a saved query |
//! | POST | `/api/export` | Export data (file download) |
//! | POST | `/api/refresh` | Rebuild nav tree from live DB |

use std::sync::Arc;

use axum::{
    Router,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post},
};
use elicit_leptos::LeptosRenderer;
use elicit_ui::{UiRenderer, VerifiedTree};
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::instrument;

use crate::archive::{
    ArchiveDbBackend, ArchiveResult,
    errors::{ArchiveError, ArchiveErrorKind},
    nav_model::{ArchiveNavModel, PanelMode},
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
    model: Arc<Mutex<ArchiveNavModel>>,
    renderer: Arc<LeptosRenderer>,
    db_url: Option<Arc<String>>,
    history: Arc<Mutex<Option<HistoryStore>>>,
    saved: Arc<Mutex<Option<SavedQueryStore>>>,
}

// ── IR gate ───────────────────────────────────────────────────────────────────

fn render_leptos_from_ir(
    renderer: &LeptosRenderer,
    tree: &VerifiedTree,
    _proof: elicitation::Established<elicit_ui::IrSourced>,
) -> Result<String, String> {
    renderer.render(tree).map_err(|e| e.to_string())?;
    Ok(renderer.last_html())
}

fn body_html(state: &AppState) -> Result<String, String> {
    let model = state
        .model
        .try_lock()
        .map_err(|_| "model locked".to_string())?;
    let (tree, proof) = model.to_verified_tree().map_err(|e| e.to_string())?;
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
    match body_html(&state) {
        Ok(body) => Html(wrap_page(&body)),
        Err(e) => Html(format!("<pre>render error: {e}</pre>")),
    }
}

// ── GET /api/nav ──────────────────────────────────────────────────────────────

async fn api_nav(State(state): State<AppState>) -> Html<String> {
    match body_html(&state) {
        Ok(html) => Html(html),
        Err(e) => Html(format!("<pre>nav error: {e}</pre>")),
    }
}

// ── Query param structs ───────────────────────────────────────────────────────

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

#[derive(Debug, Deserialize)]
struct ExportBody {
    schema: String,
    table: String,
    format: ExportFormat,
}

// ── GET /api/preview ──────────────────────────────────────────────────────────

async fn api_preview(
    State(state): State<AppState>,
    Query(p): Query<SchemaTable>,
) -> ApiResult<Html<String>> {
    let url = state.db_url.clone().ok_or_else(ApiError::no_db)?;
    let result = preview_table_direct(&url, &p.schema, &p.table, 200)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::DataGrid {
            schema: p.schema,
            table: p.table,
            result,
            page: 0,
        };
    }
    Ok(Html(body_html(&state).map_err(ApiError::internal)?))
}

// ── POST /api/sql ─────────────────────────────────────────────────────────────

async fn api_sql(
    State(state): State<AppState>,
    axum::Json(body): axum::Json<SqlBody>,
) -> ApiResult<Html<String>> {
    let url = state.db_url.clone().ok_or_else(ApiError::no_db)?;
    let start = std::time::Instant::now();
    let result = execute_sql_direct(&url, &body.sql)
        .await
        .map_err(ApiError::internal)?;
    let duration_ms = start.elapsed().as_millis() as u64;
    let row_count = result.row_count;
    {
        let mut model = state.model.lock().await;
        let current_text = match &model.panel {
            PanelMode::SqlEditor { text, .. } => text.clone(),
            _ => body.sql.clone(),
        };
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
            text: current_text,
            result: Some(result),
            running: false,
        };
    }
    {
        let history = state.history.lock().await;
        if let Some(ref store) = *history {
            store.append_spawn(body.sql, duration_ms, Some(row_count), None);
        }
    }
    Ok(Html(body_html(&state).map_err(ApiError::internal)?))
}

// ── GET /api/inspect ──────────────────────────────────────────────────────────

async fn api_inspect(
    State(state): State<AppState>,
    Query(p): Query<SchemaTable>,
) -> ApiResult<axum::Json<serde_json::Value>> {
    let url = state.db_url.clone().ok_or_else(ApiError::no_db)?;
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

// ── GET /api/ddl ──────────────────────────────────────────────────────────────

async fn api_ddl(
    State(state): State<AppState>,
    Query(p): Query<SchemaTable>,
) -> ApiResult<Html<String>> {
    let url = state.db_url.clone().ok_or_else(ApiError::no_db)?;
    let ddl_result = generate_ddl_direct(&url, &p.schema, &p.table)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::Ddl {
            schema: p.schema,
            table: p.table,
            ddl: ddl_result.ddl,
        };
    }
    Ok(Html(body_html(&state).map_err(ApiError::internal)?))
}

// ── GET /api/stats ────────────────────────────────────────────────────────────

async fn api_stats(
    State(state): State<AppState>,
    Query(p): Query<SchemaTable>,
) -> ApiResult<axum::Json<serde_json::Value>> {
    let url = state.db_url.clone().ok_or_else(ApiError::no_db)?;
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

// ── GET /api/explain ──────────────────────────────────────────────────────────

async fn api_explain(
    State(state): State<AppState>,
    Query(p): Query<ExplainParams>,
) -> ApiResult<Html<String>> {
    let url = state.db_url.clone().ok_or_else(ApiError::no_db)?;
    let root = explain_sql_direct(&url, &p.sql)
        .await
        .map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.panel = PanelMode::ExplainPlan {
            schema: p.schema,
            table: p.table,
            root,
        };
    }
    Ok(Html(body_html(&state).map_err(ApiError::internal)?))
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

// ── POST /api/export ──────────────────────────────────────────────────────────

async fn api_export(
    State(state): State<AppState>,
    axum::Json(body): axum::Json<ExportBody>,
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
    let export = export_query_result(&result, body.format);
    let filename = format!(
        "{}__{}.{}",
        body.schema,
        body.table,
        body.format.extension()
    );
    let content_type = match body.format {
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

// ── POST /api/refresh ─────────────────────────────────────────────────────────

async fn api_refresh(State(state): State<AppState>) -> ApiResult<Html<String>> {
    let url = state.db_url.clone().ok_or_else(ApiError::no_db)?;
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
    Ok(Html(body_html(&state).map_err(ApiError::internal)?))
}

// ── Router assembly ───────────────────────────────────────────────────────────

fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(serve_page))
        .route("/api/nav", get(api_nav))
        .route("/api/preview", get(api_preview))
        .route("/api/sql", post(api_sql))
        .route("/api/inspect", get(api_inspect))
        .route("/api/ddl", get(api_ddl))
        .route("/api/stats", get(api_stats))
        .route("/api/explain", get(api_explain))
        .route(
            "/api/history",
            get(api_history_list).post(api_history_append),
        )
        .route("/api/saved", get(api_saved_list).post(api_saved_create))
        .route("/api/saved/{id}", delete(api_saved_delete))
        .route("/api/export", post(api_export))
        .route("/api/refresh", post(api_refresh))
        .with_state(state)
}

// ── HTML shell ────────────────────────────────────────────────────────────────

fn wrap_page(body: &str) -> String {
    let css = concat!(
        "*{box-sizing:border-box;margin:0;padding:0}",
        "body{font-family:'Cascadia Code','Fira Code',Consolas,monospace;",
        "background:#1e1e2e;color:#cdd6f4;height:100vh;display:flex;",
        "flex-direction:column;overflow:hidden}",
        "main{flex:1;display:flex;overflow:hidden}",
        "nav.nav-panel{width:22rem;min-width:12rem;border-right:1px solid #45475a;",
        "display:flex;flex-direction:column;overflow:hidden}",
        ".nav-scroll{flex:1;overflow-y:auto;padding:.25rem 0}",
        "section.content-panel{flex:1;overflow:auto;padding:.5rem 1rem}",
        "ul[role='tree']{list-style:none;padding:.25rem 0}",
        "ul[role='group']{list-style:none;padding-left:1.5rem}",
        "details.schema-group>summary{padding:.2rem .75rem;cursor:pointer;",
        "font-size:.9rem;color:#89b4fa;list-style:none;outline:none}",
        "li[role='treeitem']{padding:.2rem .75rem;cursor:pointer;",
        "font-size:.9rem;color:#a6adc8}",
        "li[role='treeitem'].selected{background:#313244;border-radius:4px;",
        "outline:2px solid #89b4fa;color:#cdd6f4}",
        "table{border-collapse:collapse;width:100%;font-size:.85rem}",
        "thead th{background:#181825;border-bottom:2px solid #45475a;",
        "padding:.3rem .6rem;text-align:left;color:#89b4fa;position:sticky;top:0}",
        "tbody tr:nth-child(even){background:#181825}",
        "tbody td{padding:.25rem .6rem;border-bottom:1px solid #313244}",
        "textarea{width:100%;background:#181825;color:#cdd6f4;",
        "border:1px solid #45475a;padding:.5rem;font-family:inherit;",
        "font-size:.9rem;resize:vertical;min-height:6rem}",
        "h2,h3{color:#cba6f7;margin:.5rem 0 .25rem}",
        "footer[role='status']{padding:.2rem .5rem;background:#313244;",
        "border-top:1px solid #45475a;font-size:.75rem;flex-shrink:0}"
    );
    format!(
        "<!DOCTYPE html><html lang=\"en\"><head>\
<meta charset=\"utf-8\"/>\
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"/>\
<title>Archive</title>\
<style>{css}</style>\
</head><body>{body}</body></html>"
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
    let db_url = url.map(|s| Arc::new(s));
    let model = Arc::new(Mutex::new(ArchiveNavModel::new(nav)));
    let renderer = Arc::new(LeptosRenderer::html());
    let history = Arc::new(Mutex::new(HistoryStore::open().await.ok()));
    let saved = Arc::new(Mutex::new(SavedQueryStore::open().await.ok()));

    let state = AppState {
        model,
        renderer,
        db_url,
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
