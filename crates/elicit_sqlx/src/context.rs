//! `SqlxContext` — [`PluginContext`] carrying a shared database pool.

use elicitation::PluginContext;
use sqlx::AnyPool;
use tracing::instrument;

/// Holds a shared [`AnyPool`] for database tool calls.
pub struct SqlxContext {
    db: AnyPool,
}

impl SqlxContext {
    /// Creates a new context from an existing pool.
    pub fn new(db: AnyPool) -> Self {
        Self { db }
    }

    /// Returns a reference to the underlying pool.
    pub fn pool(&self) -> &AnyPool {
        &self.db
    }
}

impl PluginContext for SqlxContext {}

/// Connects to a database and produces a [`SqlxContext`].
///
/// Installs the Any driver if not already done.
#[instrument]
pub async fn connect(database_url: &str) -> Result<SqlxContext, sqlx::Error> {
    sqlx::any::install_default_drivers();
    let pool = AnyPool::connect(database_url).await?;
    Ok(SqlxContext::new(pool))
}
