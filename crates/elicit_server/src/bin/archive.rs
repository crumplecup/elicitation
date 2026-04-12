//! `archive` — a pgAdmin-style database browser for the elicit_* ecosystem.
//!
//! # Design
//!
//! Every data-access step is a verified tool-call composition.  The same
//! `Established<P>` proofs that travel through code-generation also travel
//! through the live runtime paths, because both consume the same descriptor
//! types.
//!
//! # Usage
//!
//! ```text
//! archive connect <DB_URL>
//! archive list-schemas <DB_URL>
//! archive list-tables <DB_URL> [--schema <S>]
//! archive query <DB_URL> --sql <SQL>
//! archive serve <DB_URL> --mode <ratatui|browser> [--port <P>]
//! archive demo --mode <ratatui|browser> [--port <P>]   # no live DB required
//! ```

use clap::{Parser, Subcommand, ValueEnum};
use elicit_db::{DbQueryExecutor, DbSchemaManager, DbServerAdmin, DbTableManager};
use elicit_server::archive::{
    ArchiveDbBackend,
    frontend_utils::{build_verified_tree, demo_verified_tree},
    leptos_frontend::run_browser,
    ratatui_frontend::run_tui,
};
use tracing_subscriber::EnvFilter;

// ── CLI types ─────────────────────────────────────────────────────────────────

/// Archive — verified database browser powered by the elicit_* ecosystem.
#[derive(Parser)]
#[command(name = "archive", about = "Verified database browser")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Print the server version for the given database URL.
    Connect {
        /// Database connection URL.
        url: String,
    },
    /// List schema names in the database.
    ListSchemas {
        /// Database connection URL.
        url: String,
    },
    /// List tables in a schema.
    ListTables {
        /// Database connection URL.
        url: String,
        /// Schema to list (default: public).
        #[arg(long, default_value = "public")]
        schema: String,
    },
    /// Execute a SQL query and print the rows.
    Query {
        /// Database connection URL.
        url: String,
        /// SQL statement to execute.
        #[arg(long)]
        sql: String,
    },
    /// Serve the archive UI for a live database.
    Serve {
        /// Database connection URL.
        url: String,
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
}

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Connect { url } => {
            let backend = ArchiveDbBackend::connect(&url).await?;
            let version = backend.server_version().await?;
            println!("{version}");
        }

        Cmd::ListSchemas { url } => {
            let backend = ArchiveDbBackend::connect(&url).await?;
            let schemas = backend.list_schemas().await?;
            for s in schemas {
                println!("{s}");
            }
        }

        Cmd::ListTables { url, schema } => {
            let backend = ArchiveDbBackend::connect(&url).await?;
            let tables = backend.list_tables(&schema).await?;
            for t in tables {
                println!("{}.{}", t.schema, t.name);
            }
        }

        Cmd::Query { url, sql } => {
            let backend = ArchiveDbBackend::connect(&url).await?;
            let (rows, _proof) = backend.query_rows(&sql, &[]).await?;
            for row in &rows.rows {
                let cells: Vec<String> = row.0.iter().map(|(k, v)| format!("{k}={v:?}")).collect();
                println!("{}", cells.join(" | "));
            }
        }

        Cmd::Serve { url, mode, port } => {
            let backend = ArchiveDbBackend::connect(&url).await?;
            let tree = build_verified_tree(&backend).await?;
            match mode {
                ServeMode::Ratatui => run_tui(tree)?,
                ServeMode::Browser => run_browser(tree, port).await?,
            }
        }

        Cmd::Demo { mode, port } => {
            let tree = demo_verified_tree()?;
            match mode {
                ServeMode::Ratatui => run_tui(tree)?,
                ServeMode::Browser => run_browser(tree, port).await?,
            }
        }
    }

    Ok(())
}
