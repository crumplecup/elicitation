//! Dynamic choice sets for elicitation.
//!
//! This module provides helper types for selecting from runtime-generated choice sets.
//! Since the `Select` trait uses static methods, these helpers carry instance data
//! and provide their own `elicit` methods.
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::ChoiceSet;
//!
//! // Dynamic choices from game state
//! let available_moves = vec![1, 3, 5, 7, 9];
//! let selected_move = ChoiceSet::new(available_moves)
//!     .with_prompt("Pick your move:")
//!     .elicit(&client)
//!     .await?;
//! // selected_move is i32, not ChoiceSet<i32>
//! ```

use crate::{ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, mcp};
use std::fmt::Display;

/// A set of choices for elicitation.
///
/// Builder for selecting from dynamic choice sets. Items must implement `Display`
/// for presentation. The `elicit` method returns the selected item, not the
/// ChoiceSet itself.
///
/// # Example
///
/// ```rust,ignore
/// let moves = vec![1, 2, 3, 5, 7, 9];
/// let selected = ChoiceSet::new(moves)
///     .with_prompt("Choose a square:")
///     .elicit(&client)
///     .await?;
/// // selected is i32, not ChoiceSet<i32>
/// ```
#[derive(Debug, Clone)]
pub struct ChoiceSet<T> {
    items: Vec<T>,
    prompt: Option<String>,
}

impl<T> ChoiceSet<T>
where
    T: Display + Clone + PartialEq + Send + Sync + 'static,
{
    /// Create a new choice set from a vector of items.
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            prompt: None,
        }
    }

    /// Set a custom prompt for this choice set.
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Get the items in this choice set.
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Create a filtered choice set from a vector.
    pub fn filtered<F>(items: Vec<T>, filter: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        let filtered_items: Vec<T> = items.into_iter().filter(filter).collect();
        Self::new(filtered_items)
    }

    /// Apply a filter to this choice set.
    pub fn with_filter<F>(self, filter: F) -> Self
    where
        F: Fn(&T) -> bool,
    {
        let filtered_items = self.items.into_iter().filter(filter).collect();
        Self {
            items: filtered_items,
            prompt: self.prompt,
        }
    }

    /// Elicit a selection from this choice set.
    ///
    /// Presents the items as a select menu and returns the selected item.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The choice set is empty
    /// - The MCP tool call fails
    /// - The selected label doesn't match any item
    #[tracing::instrument(skip(communicator, self), fields(item_count = self.items.len()))]
    pub async fn elicit<C: ElicitCommunicator>(self, communicator: &C) -> ElicitResult<T> {
        if self.items.is_empty() {
            return Err(ElicitError::new(ElicitErrorKind::Validation(
                "Cannot elicit from empty choice set".to_string(),
            )));
        }

        // Build labels from Display
        let labels: Vec<String> = self.items.iter().map(|item| item.to_string()).collect();

        let prompt_text = self
            .prompt
            .as_deref()
            .unwrap_or("Choose an option:");

        let params = mcp::select_params(prompt_text, &labels);

        let result = communicator
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let selected_label = mcp::parse_string(value)?;

        // Find the item matching the selected label
        for (i, label) in labels.iter().enumerate() {
            if label == &selected_label {
                tracing::debug!(selected = %selected_label, "Item selected");
                return Ok(self.items[i].clone());
            }
        }

        Err(ElicitError::new(ElicitErrorKind::InvalidSelection(
            selected_label,
        )))
    }
}

