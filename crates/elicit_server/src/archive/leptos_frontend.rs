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
//! | GET | `/api/open-sql-editor` | Open SQL editor panel |

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
    // body_html renders the full IR tree; for the initial page load we embed
    // it into the nav panel — the content panel is populated on first click.
    match body_html(&state) {
        Ok(nav_html) => Html(wrap_page(&nav_html)),
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
        model.select_table(&p.schema, &p.table);
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
        let entry = QueryHistoryEntry {
            id: 0,
            executed_at: chrono::Utc::now(),
            sql: body.sql.clone(),
            duration_ms,
            row_count: Some(row_count),
            error: None,
        };
        model.history_cache.insert(0, entry);
        // Use the submitted SQL as the displayed text so the textarea reflects
        // what was actually run, not the stale model state.
        model.panel = PanelMode::SqlEditor {
            text: body.sql.clone(),
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
            };
        }
    }
    match body_html(&state) {
        Ok(html) => Html(html),
        Err(e) => Html(format!("<pre>sql editor error: {e}</pre>")),
    }
}

// ── GET /api/col-detail ───────────────────────────────────────────────────────

async fn api_col_detail(
    State(state): State<AppState>,
    Query(p): Query<SchemaTable>,
) -> ApiResult<Html<String>> {
    let url = state.db_url.clone().ok_or_else(ApiError::no_db)?;
    // Fetch inspection and column stats concurrently.
    let (inspect_res, stats_res) = tokio::join!(
        inspect_table_direct(&url, &p.schema, &p.table),
        get_column_stats_direct(&url, &p.schema, &p.table),
    );
    let inspection = inspect_res.map_err(ApiError::internal)?;
    let stats = stats_res.map_err(ApiError::internal)?;
    {
        let mut model = state.model.lock().await;
        model.store_inspection(p.schema.clone(), p.table.clone(), inspection);
        model.store_column_stats(p.schema.clone(), p.table.clone(), stats);
        model.select_table(&p.schema, &p.table);
        model.panel = PanelMode::ColumnDetail;
    }
    Ok(Html(body_html(&state).map_err(ApiError::internal)?))
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
        .route("/api/open-sql-editor", get(api_open_sql_editor))
        .route("/api/col-detail", get(api_col_detail))
        .with_state(state)
}

// ── HTML shell ────────────────────────────────────────────────────────────────

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
        "header.toolbar span.title{color:#cba6f7;font-weight:bold;",
        "margin-right:.6rem;font-size:.9rem}",
        "header.toolbar button{background:#313244;color:#cdd6f4;border:1px solid #45475a;",
        "border-radius:3px;padding:.15rem .55rem;font-family:inherit;font-size:.8rem;",
        "cursor:pointer}",
        "header.toolbar button:hover{background:#45475a}",
        "header.toolbar button.active{border-color:#89b4fa;color:#89b4fa}",
        "main{flex:1;display:flex;overflow:hidden}",
        "nav.nav-panel{width:22rem;min-width:12rem;border-right:1px solid #45475a;",
        "display:flex;flex-direction:column;overflow:hidden}",
        ".filter-wrap{padding:.35rem .5rem;border-bottom:1px solid #313244;flex-shrink:0}",
        ".filter-wrap input{width:100%;background:#313244;color:#cdd6f4;",
        "border:1px solid #45475a;border-radius:3px;padding:.2rem .4rem;",
        "font-family:inherit;font-size:.85rem;outline:none}",
        ".filter-wrap input:focus{border-color:#89b4fa}",
        "#nav-tree{flex:1;overflow-y:auto;padding:.25rem 0}",
        "section.content-panel{flex:1;overflow:auto;padding:.5rem 1rem}",
        "#content{min-height:4rem}",
        // history/saved overlay panel
        ".side-panel{position:fixed;top:0;right:0;bottom:0;width:28rem;",
        "background:#1e1e2e;border-left:1px solid #45475a;",
        "display:flex;flex-direction:column;z-index:100;padding:0}",
        ".side-panel-header{display:flex;justify-content:space-between;align-items:center;",
        "padding:.4rem .7rem;background:#181825;border-bottom:1px solid #45475a}",
        ".side-panel-header h3{color:#cba6f7;font-size:.9rem}",
        ".side-panel-header button{background:none;border:none;color:#a6adc8;",
        "cursor:pointer;font-size:1rem;padding:0 .2rem}",
        ".side-panel-body{flex:1;overflow-y:auto;padding:.5rem}",
        ".side-panel-body table{width:100%;font-size:.8rem}",
        ".side-panel-body td,.side-panel-body th{padding:.25rem .5rem;",
        "border-bottom:1px solid #313244;vertical-align:top}",
        ".side-panel-body th{color:#89b4fa;background:#181825}",
        ".side-panel-body .load-btn{background:none;border:none;color:#89b4fa;",
        "cursor:pointer;text-decoration:underline;font-family:inherit;font-size:.8rem;",
        "text-align:left;padding:0}",
        ".side-panel-body .del-btn{background:none;border:none;color:#f38ba8;",
        "cursor:pointer;font-family:inherit;font-size:.8rem;padding:0}",
        // export format buttons inside side panel
        ".export-btn{background:#313244;color:#cdd6f4;border:1px solid #45475a;",
        "border-radius:3px;padding:.3rem .7rem;font-family:inherit;font-size:.85rem;cursor:pointer}",
        ".export-btn:hover{background:#45475a}",
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
        "border:1px solid #45475a;padding:.5rem;font-family:inherit;",
        "font-size:.9rem;resize:vertical;min-height:8rem}",
        "textarea:focus{border-color:#89b4fa;outline:none}",
        "h2,h3{color:#cba6f7;margin:.5rem 0 .25rem}",
        ".htmx-request .spinner{display:inline-block}",
        ".spinner{display:none;margin-left:.5rem}",
        "footer[role='status']{padding:.2rem .5rem;background:#313244;",
        "border-top:1px solid #45475a;font-size:.75rem;flex-shrink:0}"
    );

    // JS: keyboard shortcuts + Ctrl+Enter SQL execution + DDL/explain + history/saved sidepanels
    let js = concat!(
        // ── context state ──
        "var _curSchema='';",
        "var _curTable='';",
        // Track schema/table from preview requests so DDL/explain can use it.
        "document.addEventListener('htmx:afterRequest',function(evt){",
        "var path=evt.detail&&evt.detail.requestConfig&&evt.detail.requestConfig.path;",
        "if(path&&path.startsWith('/api/preview')){",
        "var params=new URLSearchParams(path.split('?')[1]||'');",
        "_curSchema=params.get('schema')||'';",
        "_curTable=params.get('table')||'';",
        "}}); ",
        // ── keyboard shortcuts ──
        "document.addEventListener('keydown',function(e){",
        "var inp=document.getElementById('nav-filter');",
        "var ta=document.querySelector('#content textarea');",
        "var inInput=document.activeElement.tagName==='INPUT'||",
        "document.activeElement.tagName==='TEXTAREA';",
        // '/' → focus nav filter
        "if(e.key==='/'&&!inInput){e.preventDefault();inp&&inp.focus();}",
        // Esc on nav filter → clear + blur
        "if(e.key==='Escape'&&document.activeElement===inp){",
        "inp.value='';htmx.trigger(inp,'change');inp.blur();}",
        // 's' → open SQL editor
        "if(e.key==='s'&&!inInput){",
        "e.preventDefault();",
        "htmx.ajax('GET','/api/open-sql-editor',{target:'#content',swap:'innerHTML'});",
        "}",
        // 'd' → show DDL
        "if(e.key==='d'&&!inInput){e.preventDefault();showDdl();}",
        // 'i' → column detail
        "if(e.key==='i'&&!inInput){e.preventDefault();showColDetail();}",
        // 'x' → export picker
        "if(e.key==='x'&&!inInput){e.preventDefault();openExportPanel();}",
        // Ctrl+Enter → run SQL
        "if(e.ctrlKey&&e.key==='Enter'&&ta){e.preventDefault();runSql();}",
        "});",
        // ── column detail ──
        "function showColDetail(){",
        "if(!_curSchema||!_curTable){alert('Select a table first (click one in the nav tree).');return;}",
        "htmx.ajax('GET','/api/col-detail?schema='+encodeURIComponent(_curSchema)+'&table='+encodeURIComponent(_curTable),",
        "{target:'#content',swap:'innerHTML'});",
        "}",
        // ── DDL viewer ──
        "function showDdl(){",
        "if(!_curSchema||!_curTable){alert('Select a table first (click one in the nav tree).');return;}",
        "htmx.ajax('GET','/api/ddl?schema='+encodeURIComponent(_curSchema)+'&table='+encodeURIComponent(_curTable),",
        "{target:'#content',swap:'innerHTML'});",
        "}",
        // ── EXPLAIN viewer ──
        "function showExplain(){",
        "var ta=document.querySelector('#content textarea');",
        "var sql=ta&&ta.value.trim()?ta.value:'SELECT * FROM '+_curSchema+'.'+_curTable+' LIMIT 10';",
        "if((!_curSchema||!_curTable)&&!ta){alert('Select a table or open the SQL editor first.');return;}",
        "var schema=_curSchema||'public';",
        "var table=_curTable||'query';",
        "var url='/api/explain?schema='+encodeURIComponent(schema)+'&table='+encodeURIComponent(table)+'&sql='+encodeURIComponent(sql);",
        "htmx.ajax('GET',url,{target:'#content',swap:'innerHTML'});",
        "}",
        // ── run SQL via fetch ──
        "function runSql(){",
        "var ta=document.querySelector('#content textarea');",
        "if(!ta)return;",
        "var sql=ta.value;",
        "if(!sql.trim())return;",
        "var status=document.getElementById('sql-status');",
        "if(status){status.textContent='Running…';status.className='sql-status';}",
        "fetch('/api/sql',{",
        "method:'POST',",
        "headers:{'Content-Type':'application/json'},",
        "body:JSON.stringify({sql:sql})",
        "}).then(function(r){",
        "if(!r.ok)return r.text().then(function(t){throw new Error(t);});",
        "return r.text();",
        "}).then(function(html){",
        "document.getElementById('content').innerHTML=html;",
        "htmx.process(document.getElementById('content'));",
        "}).catch(function(err){",
        "var c=document.getElementById('content');",
        "c.innerHTML='<p class=\"sql-status err\">Error: '+err.message+'</p>'+",
        "(ta?'<textarea style=\"width:100%;min-height:8rem;background:#181825;color:#cdd6f4;border:1px solid #45475a;padding:.5rem;font-family:inherit;font-size:.9rem;resize:vertical\">'+ta.value+'</textarea>':'');",
        "});",
        "}",
        // ── side panel helpers ──
        "function closeSidePanel(){",
        "var p=document.getElementById('side-panel');",
        "if(p)p.remove();",
        "}",
        "function openHistoryPanel(){",
        "closeSidePanel();",
        "fetch('/api/history').then(function(r){return r.json();}).then(function(entries){",
        "var rows=entries.map(function(e,i){",
        "var ts=new Date(e.executed_at).toLocaleTimeString();",
        "var sql=e.sql.length>60?e.sql.substring(0,60)+'…':e.sql;",
        "var rowCount=e.row_count!=null?e.row_count:'—';",
        "var dur=e.duration_ms!=null?(e.duration_ms+'ms'):'—';",
        "return '<tr><td><button class=\"load-btn\" onclick=\"loadSql('+i+')\">Load</button></td>'",
        "+'<td title=\"'+escHtml(e.sql)+'\">'+escHtml(sql)+'</td>'",
        "+'<td>'+rowCount+'</td><td>'+dur+'</td><td>'+ts+'</td></tr>';",
        "}).join('');",
        "_historyEntries=entries;",
        "var html='<div class=\"side-panel\" id=\"side-panel\">'",
        "+'<div class=\"side-panel-header\"><h3>Query History</h3>'",
        "+'<button onclick=\"closeSidePanel()\" title=\"Close\">✕</button></div>'",
        "+'<div class=\"side-panel-body\">'",
        "+(rows?'<table><thead><tr><th></th><th>SQL</th><th>Rows</th><th>Time</th><th>At</th></tr></thead><tbody>'+rows+'</tbody></table>'",
        ":'<p style=\"color:#a6adc8;padding:.5rem\">No history yet.</p>')",
        "+'</div></div>';",
        "document.body.insertAdjacentHTML('beforeend',html);",
        "}).catch(function(e){console.error('history load failed',e);});",
        "}",
        "var _historyEntries=[];",
        "function loadSql(idx){",
        "var e=_historyEntries[idx];",
        "if(!e)return;",
        "closeSidePanel();",
        "htmx.ajax('GET','/api/open-sql-editor',{target:'#content',swap:'innerHTML'}).then(function(){",
        "var ta=document.querySelector('#content textarea');",
        "if(ta){ta.value=e.sql;ta.focus();}",
        "});",
        "}",
        "function openSavedPanel(){",
        "closeSidePanel();",
        "fetch('/api/saved').then(function(r){return r.json();}).then(function(entries){",
        "_savedEntries=entries;",
        "var rows=entries.map(function(e,i){",
        "var sql=e.sql.length>50?e.sql.substring(0,50)+'…':e.sql;",
        "return '<tr>'",
        "+'<td><strong>'+escHtml(e.name)+'</strong></td>'",
        "+'<td title=\"'+escHtml(e.sql)+'\">'+escHtml(sql)+'</td>'",
        "+'<td><button class=\"load-btn\" onclick=\"loadSavedSql('+i+')\">Load</button></td>'",
        "+'<td><button class=\"del-btn\" onclick=\"deleteSaved('+i+')\">Del</button></td>'",
        "+'</tr>';",
        "}).join('');",
        "var html='<div class=\"side-panel\" id=\"side-panel\">'",
        "+'<div class=\"side-panel-header\"><h3>Saved Queries</h3>'",
        "+'<button onclick=\"closeSidePanel()\" title=\"Close\">✕</button></div>'",
        "+'<div class=\"side-panel-body\">'",
        "+(rows?'<table><thead><tr><th>Name</th><th>SQL</th><th></th><th></th></tr></thead><tbody>'+rows+'</tbody></table>'",
        ":'<p style=\"color:#a6adc8;padding:.5rem\">No saved queries.</p>')",
        "+'</div></div>';",
        "document.body.insertAdjacentHTML('beforeend',html);",
        "}).catch(function(e){console.error('saved load failed',e);});",
        "}",
        "var _savedEntries=[];",
        "function loadSavedSql(idx){",
        "var e=_savedEntries[idx];",
        "if(!e)return;",
        "closeSidePanel();",
        "htmx.ajax('GET','/api/open-sql-editor',{target:'#content',swap:'innerHTML'}).then(function(){",
        "var ta=document.querySelector('#content textarea');",
        "if(ta){ta.value=e.sql;ta.focus();}",
        "});",
        "}",
        "function deleteSaved(idx){",
        "var e=_savedEntries[idx];",
        "if(!e)return;",
        "if(!confirm('Delete \"'+e.name+'\"?'))return;",
        "fetch('/api/saved/'+e.id,{method:'DELETE'})",
        ".then(function(){closeSidePanel();openSavedPanel();});",
        "}",
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
        "function escHtml(s){",
        "return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;')",
        ".replace(/\"/g,'&quot;');",
        "}",

        // ── export panel ──
        "function openExportPanel(){",
        "closeSidePanel();",
        "if(!_curSchema||!_curTable){alert('Select a table first.');return;}",
        "var html='<div class=\"side-panel\" id=\"side-panel\">'",
        "+'<div class=\"side-panel-header\"><h3>Export Data</h3>'",
        "+'<button onclick=\"closeSidePanel()\" title=\"Close\">✕</button></div>'",
        "+'<div class=\"side-panel-body\" style=\"padding:1rem\">'",
        "+'<p style=\"color:#a6adc8;font-size:.85rem;margin-bottom:.75rem\">'",
        "+escHtml(_curSchema+'.'+_curTable)+'</p>'",
        "+'<div style=\"display:flex;gap:.6rem;flex-wrap:wrap\">'",
        "+'<button class=\"export-btn\" onclick=\"exportData(\\\"Csv\\\")\">CSV</button>'",
        "+'<button class=\"export-btn\" onclick=\"exportData(\\\"Json\\\")\">JSON</button>'",
        "+'<button class=\"export-btn\" onclick=\"exportData(\\\"Tsv\\\")\">TSV</button>'",
        "+'<button class=\"export-btn\" onclick=\"exportData(\\\"Ndjson\\\")\">NDJSON</button>'",
        "+'</div></div></div>';",
        "document.body.insertAdjacentHTML('beforeend',html);",
        "}",

        "function exportData(format){",
        "fetch('/api/export',{",
        "method:'POST',",
        "headers:{'Content-Type':'application/json'},",
        "body:JSON.stringify({schema:_curSchema,table:_curTable,format:format})",
        "}).then(function(r){",
        "if(!r.ok)return r.text().then(function(t){throw new Error(t);});",
        "var cd=r.headers.get('Content-Disposition')||'';",
        "var m=cd.match(/filename=\"([^\"]+)\"/);",
        "var fname=m?m[1]:(_curSchema+'_'+_curTable+'.'+format.toLowerCase());",
        "return r.blob().then(function(b){return{blob:b,fname:fname};});",
        "}).then(function(obj){",
        "var url=URL.createObjectURL(obj.blob);",
        "var a=document.createElement('a');",
        "a.href=url;a.download=obj.fname;a.click();",
        "URL.revokeObjectURL(url);",
        "closeSidePanel();",
        "}).catch(function(err){alert('Export failed: '+err.message);});",
        "}"
    );

    format!(
        "<!DOCTYPE html><html lang=\"en\"><head>\
<meta charset=\"utf-8\"/>\
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"/>\
<title>Archive</title>\
<script src=\"https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js\"></script>\
<style>{css}</style>\
</head><body>\
<header class=\"toolbar\">\
<span class=\"title\">▦ Archive</span>\
<button onclick=\"htmx.ajax('GET','/api/open-sql-editor',{{target:'#content',swap:'innerHTML'}})\" \
 title=\"SQL Editor (s)\">SQL Editor</button>\
<button onclick=\"showDdl()\" title=\"DDL viewer (d)\">DDL</button>\
<button onclick=\"showExplain()\" title=\"EXPLAIN plan\">EXPLAIN</button>\
<button onclick=\"showColDetail()\" title=\"Column detail + stats (i)\">Col Detail</button>\
<button onclick=\"openHistoryPanel()\" title=\"Query history\">History</button>\
<button onclick=\"openSavedPanel()\" title=\"Saved queries\">Saved</button>\
<button onclick=\"saveCurrentSql()\" title=\"Save current SQL\">Save SQL</button>\
<button onclick=\"openExportPanel()\" title=\"Export data\">Export</button>\
<button onclick=\"htmx.ajax('POST','/api/refresh',{{target:'#content',swap:'innerHTML'}})\" \
 title=\"Refresh nav tree\">⟳ Refresh</button>\
</header>\
<main>\
<nav class=\"nav-panel\">\
<div class=\"filter-wrap\">\
<input id=\"nav-filter\" type=\"search\" placeholder=\"/ filter…\" autocomplete=\"off\" \
 hx-get=\"/api/nav\" hx-trigger=\"keyup changed delay:250ms\" hx-target=\"#nav-tree\" hx-swap=\"innerHTML\" \
 name=\"filter\"/>\
</div>\
<div id=\"nav-tree\">{body}</div>\
</nav>\
<section class=\"content-panel\"><div id=\"content\"><p style=\"color:#6c7086;padding:1rem;font-size:.9rem\">Select a table in the nav tree, or press <kbd>s</kbd> to open the SQL editor.</p></div></section>\
</main>\
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
