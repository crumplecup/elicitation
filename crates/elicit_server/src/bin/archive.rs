//! `archive` — a pgAdmin-style database browser for the elicit_* ecosystem.
//!
//! # Design
//!
//! Every data-access step is a verified tool-call composition.  The same
//! `Established<P>` proofs that travel through code-generation also travel
//! through the live runtime paths, because both consume the same descriptor
//! types.
//!
//! # URL resolution
//!
//! Every command that needs a database URL accepts an optional positional
//! argument.  If omitted, the `DATABASE_URL` environment variable is used
//! (loaded from `.env` automatically via dotenvy).
//!
//! # Usage
//!
//! ```text
//! archive connect [DB_URL]
//! archive list-schemas [DB_URL]
//! archive list-tables [DB_URL] [--schema <S>]
//! archive query [DB_URL] --sql <SQL>
//! archive serve [DB_URL] --mode <ratatui|browser|egui> [--port <P>]
//! archive demo --mode <ratatui|browser|egui> [--port <P>]   # no live DB required
//! ```

use clap::{Parser, Subcommand, ValueEnum};
use elicit_db::{DbQueryExecutor, DbSchemaManager, DbServerAdmin, DbTableManager};
use elicit_server::archive::{
    ArchiveDbBackend, NavTree, egui_frontend::run_egui, leptos_frontend::run_browser,
    nav_tree::build_nav_tree, nav_tree_to_verified_tree, ratatui_frontend::run_tui,
};
use tracing_subscriber::EnvFilter;

// ── URL resolution ────────────────────────────────────────────────────────────

/// Resolve a database URL from an explicit argument or `DATABASE_URL` env var.
///
/// Priority:
/// 1. Explicit `url` argument (if `Some`)
/// 2. `DATABASE_URL` environment variable (may come from `.env`)
fn resolve_url(url: Option<String>) -> anyhow::Result<String> {
    if let Some(u) = url {
        return Ok(u);
    }
    std::env::var("DATABASE_URL").map_err(|_| {
        anyhow::anyhow!(
            "No database URL provided and DATABASE_URL is not set.\n\
             Pass a URL as the first argument or add DATABASE_URL to your .env file."
        )
    })
}

// ── CLI types ─────────────────────────────────────────────────────────────────

/// Archive — verified database browser powered by the elicit_* ecosystem.
#[derive(Parser)]
#[command(
    name = "archive",
    about = "Verified database browser",
    after_help = "DATABASE_URL is read from .env if not provided as an argument."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Print the server version for the given database URL.
    Connect {
        /// Database connection URL (falls back to DATABASE_URL env var).
        url: Option<String>,
    },
    /// List schema names in the database.
    ListSchemas {
        /// Database connection URL (falls back to DATABASE_URL env var).
        url: Option<String>,
    },
    /// List tables in a schema.
    ListTables {
        /// Database connection URL (falls back to DATABASE_URL env var).
        url: Option<String>,
        /// Schema to list (default: public).
        #[arg(long, default_value = "public")]
        schema: String,
    },
    /// Execute a SQL query and print the rows.
    Query {
        /// Database connection URL (falls back to DATABASE_URL env var).
        url: Option<String>,
        /// SQL statement to execute.
        #[arg(long)]
        sql: String,
    },
    /// Serve the archive UI for a live database.
    Serve {
        /// Database connection URL (falls back to DATABASE_URL env var).
        url: Option<String>,
        /// Display mode.
        #[arg(long, default_value = "ratatui")]
        mode: ServeMode,
        /// HTTP port (browser mode only).
        #[arg(long, default_value_t = 3000)]
        port: u16,
    },
    /// Serve the archive UI in demo mode (no live database required).
    Demo {
        /// Display mode.
        #[arg(long, default_value = "browser")]
        mode: ServeMode,
        /// HTTP port (browser mode only).
        #[arg(long, default_value_t = 3000)]
        port: u16,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum ServeMode {
    /// Ratatui terminal UI.
    Ratatui,
    /// Leptos/Axum browser UI served on HTTP.
    Browser,
    /// Native egui window (winit + wgpu).
    Egui,
}

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env (non-fatal: missing file is fine)
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Connect { url } => {
            let url = resolve_url(url)?;
            let backend = ArchiveDbBackend::connect(&url).await?;
            let version = backend.server_version().await?;
            println!("{version}");
        }

        Cmd::ListSchemas { url } => {
            let url = resolve_url(url)?;
            let backend = ArchiveDbBackend::connect(&url).await?;
            let schemas = backend.list_schemas().await?;
            for s in schemas {
                println!("{s}");
            }
        }

        Cmd::ListTables { url, schema } => {
            let url = resolve_url(url)?;
            let backend = ArchiveDbBackend::connect(&url).await?;
            let tables = backend.list_tables(&schema).await?;
            for t in tables {
                println!("{}.{}", t.schema, t.name);
            }
        }

        Cmd::Query { url, sql } => {
            let url = resolve_url(url)?;
            let backend = ArchiveDbBackend::connect(&url).await?;
            let (rows, _proof) = backend.query_rows(&sql, &[]).await?;
            for row in &rows.rows {
                let cells: Vec<String> = row.0.iter().map(|(k, v)| format!("{k}={v:?}")).collect();
                println!("{}", cells.join(" | "));
            }
        }

        Cmd::Serve { url, mode, port } => {
            let url = resolve_url(url)?;
            let backend = ArchiveDbBackend::connect(&url).await?;
            let nav = build_nav_tree(&backend, &url).await?;
            match mode {
                ServeMode::Ratatui => run_tui(nav)?,
                ServeMode::Egui => run_egui(nav)?,
                ServeMode::Browser => {
                    let tree = nav_tree_to_verified_tree(&nav)?;
                    run_browser(tree, port).await?;
                }
            }
        }

        Cmd::Demo { mode, port } => {
            let nav = NavTree::demo();
            match mode {
                ServeMode::Ratatui => run_tui(nav)?,
                ServeMode::Egui => run_egui(nav)?,
                ServeMode::Browser => {
                    let tree = nav_tree_to_verified_tree(&nav)?;
                    run_browser(tree, port).await?;
                }
            }
        }
    }

    Ok(())
}
