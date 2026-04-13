//! Leptos/Axum browser frontend for the archive CLI.
//!
//! Each GET `/` request rebuilds the AccessKit IR from the live model and
//! renders via [`LeptosRenderer`].  The [`Established<IrSourced>`] proof token
//! (minted only by [`ArchiveNavModel::to_verified_tree`]) is threaded through
//! a gate function, making it a compile-time error to serve HTML that was not
//! produced from the verified IR pipeline.

use std::sync::Arc;

use axum::{Router, extract::State, response::Html, routing::get};
use elicit_leptos::LeptosRenderer;
use elicit_ui::{UiRenderer, VerifiedTree};
use tokio::sync::Mutex;
use tracing::instrument;

use crate::archive::{
    ArchiveResult,
    errors::{ArchiveError, ArchiveErrorKind},
    nav_model::ArchiveNavModel,
    nav_tree::NavTree,
};

// ── Gate function ─────────────────────────────────────────────────────────────

/// Render a verified tree to an HTML fragment.
///
/// Requires an [`Established<IrSourced>`] proof token — the only source of
/// which is [`ArchiveNavModel::to_verified_tree`].  This enforces the
/// invariant that the HTML content always originates from the AccessKit IR.
fn render_leptos_from_ir(
    renderer: &LeptosRenderer,
    tree: &VerifiedTree,
    _proof: elicitation::Established<elicit_ui::IrSourced>,
) -> Result<String, String> {
    renderer.render(tree).map_err(|e| e.to_string())?;
    Ok(renderer.last_html())
}

// ── Axum handler ─────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    model: Arc<Mutex<ArchiveNavModel>>,
    renderer: Arc<LeptosRenderer>,
}

async fn serve_page(State(state): State<AppState>) -> Html<String> {
    let model = state.model.lock().await;
    match model.to_verified_tree() {
        Ok((tree, ir_proof)) => match render_leptos_from_ir(&state.renderer, &tree, ir_proof) {
            Ok(body) => Html(wrap_page(&body)),
            Err(e) => Html(format!("<pre>render error: {e}</pre>")),
        },
        Err(e) => Html(format!("<pre>IR error: {e}</pre>")),
    }
}

fn wrap_page(body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8"/>
<title>Archive</title>
<style>
*{{box-sizing:border-box;margin:0;padding:0}}
body{{font-family:'Cascadia Code','Fira Code',Consolas,monospace;background:#1e1e2e;color:#cdd6f4;height:100vh;display:flex;flex-direction:column;overflow:hidden}}
header{{padding:.4rem 1rem;background:#181825;border-bottom:1px solid #45475a;font-size:.85rem;flex-shrink:0}}
.nav-scroll{{flex:1;overflow-y:auto;padding:.25rem 0}}
ul[role="tree"]{{list-style:none;padding:.25rem 0}}
ul[role="group"]{{list-style:none;padding-left:1.5rem}}
li[role="treeitem"]{{padding:.2rem .75rem;cursor:pointer;font-size:.9rem;outline:none;color:#a6adc8}}
li[role="treeitem"][data-focused],li[role="treeitem"]:focus{{background:#313244;border-radius:4px;outline:2px solid #89b4fa;color:#cdd6f4}}
footer[role="status"]{{padding:.2rem .5rem;background:#313244;border-top:1px solid #45475a;font-size:.75rem;flex-shrink:0}}
</style>
</head>
<body>
{}
</body></html>"#,
        body
    )
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Serve the archive tree as a dynamic HTML page via axum on `port`.
///
/// Each request rebuilds the AccessKit IR from the model, ensuring the browser
/// frontend always reflects the current state and preserves the IR proof contract.
#[instrument(skip(nav))]
pub async fn run_browser(nav: NavTree, port: u16) -> ArchiveResult<()> {
    let model = Arc::new(Mutex::new(ArchiveNavModel::new(nav)));
    let renderer = Arc::new(LeptosRenderer::html());

    let state = AppState { model, renderer };
    let router = Router::new().route("/", get(serve_page)).with_state(state);

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))?;

    tracing::info!(addr = %addr, "archive browser frontend listening");
    eprintln!("Archive browser frontend: http://localhost:{port}/");

    axum::serve(listener, router)
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))?;

    Ok(())
}
