//! Single-shot JSON elicitation via schema.
//!
//! Instead of prompting field-by-field, [`ElicitJson`] shows an agent the
//! JSON schema for a type and asks for a matching JSON blob in one round-trip.
//! The response is deserialized directly into `T`.
//!
//! This is the fast path for agents that already understand the type's shape.
//! The interactive field-by-field [`Elicitation`] path remains the default.
//!
//! # Example
//!
//! ```rust,no_run
//! use elicitation::{ElicitJson, ElicitCommunicator};
//! use schemars::JsonSchema;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Serialize, Deserialize, JsonSchema)]
//! struct Config {
//!     host: String,
//!     port: u16,
//! }
//!
//! async fn elicit_config<C: ElicitCommunicator>(communicator: &C) -> Config {
//!     Config::elicit_json(communicator).await.unwrap()
//! }
//! ```

use crate::{ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

/// Maximum re-prompt attempts on JSON parse failure.
const MAX_ATTEMPTS: usize = 3;

/// Single-shot JSON elicitation for types with a known schema.
///
/// Any type that implements [`schemars::JsonSchema`] and [`serde::de::DeserializeOwned`]
/// automatically gets this blanket impl. The agent receives the JSON schema and
/// is asked to produce a matching JSON object — one prompt instead of N field prompts.
///
/// Re-prompts with the parse error message if deserialization fails, up to
/// [`MAX_ATTEMPTS`] times.
pub trait ElicitJson: Sized {
    /// Elicit a value by presenting the JSON schema and parsing the response.
    fn elicit_json<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send;
}

impl<T> ElicitJson for T
where
    T: JsonSchema + DeserializeOwned + Send + 'static,
{
    #[tracing::instrument(skip(communicator), fields(type_name = std::any::type_name::<T>()))]
    async fn elicit_json<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let schema = schemars::schema_for!(T);
        let schema_json = serde_json::to_string_pretty(&schema)
            .map_err(|e| ElicitError::from(ElicitErrorKind::Json(crate::JsonError::new(e))))?;

        let type_name = std::any::type_name::<T>();
        let base_prompt = format!(
            "Produce a JSON value for type `{type_name}` matching this schema:\n\n{schema_json}\n\nRespond with only the JSON value, no explanation."
        );

        let mut last_error = String::new();
        for attempt in 0..MAX_ATTEMPTS {
            let prompt = if attempt == 0 {
                base_prompt.clone()
            } else {
                format!(
                    "{base_prompt}\n\nPrevious attempt failed: {last_error}\nPlease correct the JSON and try again."
                )
            };

            tracing::debug!(attempt, "sending JSON elicitation prompt");
            let response = communicator.send_prompt(&prompt).await?;

            match serde_json::from_str::<T>(&response) {
                Ok(value) => {
                    tracing::debug!(attempt, "JSON elicitation succeeded");
                    return Ok(value);
                }
                Err(e) => {
                    last_error = e.to_string();
                    tracing::warn!(attempt, error = %last_error, "JSON parse failed, re-prompting");
                }
            }
        }

        Err(ElicitError::from(ElicitErrorKind::InvalidFormat {
            expected: format!("valid JSON for type `{type_name}`"),
            received: format!("unparseable after {MAX_ATTEMPTS} attempts: {last_error}"),
        }))
    }
}
