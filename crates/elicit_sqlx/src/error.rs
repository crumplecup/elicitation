//! `SqlxError` — elicitation-enabled wrapper around [`sqlx::Error`].

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(sqlx::Error, as SqlxError);

#[reflect_methods]
impl SqlxError {
    /// Returns a human-readable label for the error kind.
    #[instrument(skip(self))]
    pub fn kind_label(&self) -> String {
        use sqlx::Error;
        match &*self.0 {
            Error::Configuration(_) => "Configuration".to_string(),
            Error::Database(_) => "Database".to_string(),
            Error::Io(_) => "Io".to_string(),
            Error::Tls(_) => "Tls".to_string(),
            Error::Protocol(_) => "Protocol".to_string(),
            Error::RowNotFound => "RowNotFound".to_string(),
            Error::TypeNotFound { .. } => "TypeNotFound".to_string(),
            Error::ColumnIndexOutOfBounds { .. } => "ColumnIndexOutOfBounds".to_string(),
            Error::ColumnNotFound(_) => "ColumnNotFound".to_string(),
            Error::ColumnDecode { .. } => "ColumnDecode".to_string(),
            Error::Decode(_) => "Decode".to_string(),
            Error::AnyDriverError(_) => "AnyDriverError".to_string(),
            Error::PoolTimedOut => "PoolTimedOut".to_string(),
            Error::PoolClosed => "PoolClosed".to_string(),
            Error::WorkerCrashed => "WorkerCrashed".to_string(),
            Error::Migrate(_) => "Migrate".to_string(),
            _ => "Other".to_string(),
        }
    }

    /// Returns the full error message string.
    #[instrument(skip(self))]
    pub fn message(&self) -> String {
        self.0.to_string()
    }
}
