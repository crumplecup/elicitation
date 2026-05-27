//! [`ContextualCommunicator`] — wraps any communicator and prepends accumulated
//! knowledge to every elicitation prompt.
//!
//! # Problem
//!
//! Each elicitation call is stateless — the communicator does not carry context
//! from one sampling request to the next.  When making sequential decisions, game
//! state or accumulated facts must be injected into the prompt so the player or
//! agent can make informed choices.
//!
//! # Solution
//!
//! [`ContextualCommunicator`] wraps any inner communicator and maintains a
//! growing [`KnowledgeCache`].  Each prompt is prefixed with all accumulated
//! knowledge so the player/agent sees the current state in every interaction.
//!
//! The application pushes context entries into the cache (via [`SharedKnowledge`])
//! before eliciting each decision.  The cache can be cleared between decision
//! points to avoid stale state accumulation.
//!
//! # Usage
//!
//! ```rust,ignore
//! use elicitation::middleware::{ContextualCommunicator, knowledge_cache};
//! use elicitation::ElicitClient;
//!
//! let knowledge = knowledge_cache();
//! let comm = ContextualCommunicator::new(ElicitClient::new(config), knowledge.clone());
//!
//! // Application loop pushes context before each decision:
//! knowledge.lock().unwrap().push("Current HP: 45/100");
//! knowledge.lock().unwrap().push("Enemy HP: 80/120");
//! let action = player.choose_action(&comm).await?;
//!
//! // Clear between turns:
//! knowledge.lock().unwrap().clear();
//! ```

use crate::{ElicitCommunicator, ElicitResult, ElicitationContext, StyleContext, StyleMarker};
use std::sync::{Arc, Mutex};
use tracing::instrument;

/// Accumulated knowledge injected into every subsequent prompt.
///
/// Append-only within a decision cycle; call [`KnowledgeCache::clear`] between
/// cycles to avoid stale state.
#[derive(Debug, Clone, Default)]
pub struct KnowledgeCache {
    entries: Vec<String>,
}

impl KnowledgeCache {
    /// Append a new entry (e.g. current game state, recent events).
    pub fn push(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
    }

    /// Remove all entries (call between decision cycles to avoid stale state).
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    fn format_preamble(&self) -> String {
        if self.entries.is_empty() {
            return String::new();
        }
        let mut preamble = String::from("[Context]\n");
        for (i, entry) in self.entries.iter().enumerate() {
            preamble.push_str(&format!("{}. {}\n", i + 1, entry));
        }
        preamble.push('\n');
        preamble
    }
}

/// Thread-safe handle to a shared knowledge cache.
pub type SharedKnowledge = Arc<Mutex<KnowledgeCache>>;

/// Creates a new, empty shared knowledge cache.
#[instrument]
pub fn knowledge_cache() -> SharedKnowledge {
    Arc::new(Mutex::new(KnowledgeCache::default()))
}

/// Wraps any [`ElicitCommunicator`], prepending accumulated knowledge to
/// every prompt before delegating.
///
/// The owning application pushes context entries via [`SharedKnowledge`]
/// before calling elicitation methods.  The communicator reads those entries
/// and prepends them, so the player/agent always sees up-to-date state without
/// the application having to format every prompt manually.
#[derive(Clone)]
pub struct ContextualCommunicator<C> {
    inner: C,
    knowledge: SharedKnowledge,
}

impl<C> ContextualCommunicator<C> {
    /// Wrap `inner` with a shared knowledge cache.
    pub fn new(inner: C, knowledge: SharedKnowledge) -> Self {
        Self { inner, knowledge }
    }
}

impl<C: ElicitCommunicator + Clone> ElicitCommunicator for ContextualCommunicator<C> {
    /// Prepend accumulated knowledge to `prompt`, then delegate to inner.
    #[instrument(skip(self), level = "debug", fields(prompt_len = prompt.len()))]
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl std::future::Future<Output = ElicitResult<String>> + Send {
        let preamble = {
            let cache = self.knowledge.lock().unwrap();
            cache.format_preamble()
        };
        let enriched = if preamble.is_empty() {
            prompt.to_string()
        } else {
            format!("{preamble}{prompt}")
        };
        let inner = self.inner.clone();
        tracing::debug!(prompt_len = enriched.len(), "Sending enriched prompt");
        async move { inner.send_prompt(&enriched).await }
    }

    /// Pass tool calls through to the inner communicator unchanged.
    #[instrument(skip(self, params), level = "debug", fields(tool = %params.name))]
    fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
    ) -> impl std::future::Future<
        Output = Result<rmcp::model::CallToolResult, rmcp::service::ServiceError>,
    > + Send {
        self.inner.call_tool(params)
    }

    fn style_context(&self) -> &StyleContext {
        self.inner.style_context()
    }

    fn elicitation_context(&self) -> &ElicitationContext {
        self.inner.elicitation_context()
    }

    fn with_style<
        T: 'static,
        S: StyleMarker + crate::style::ElicitationStyle + 'static,
    >(
        &self,
        style: S,
    ) -> Self {
        Self {
            inner: self.inner.with_style::<T, S>(style),
            knowledge: self.knowledge.clone(),
        }
    }
}
