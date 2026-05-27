//! Communicator middleware — transparent wrappers that enhance any
//! [`ElicitCommunicator`] with context accumulation, observability, etc.
//!
//! # Overview
//!
//! The middleware in this module follow a common pattern: wrap an inner
//! [`ElicitCommunicator`], forward all calls unchanged (or with minimal
//! enrichment), and provide side-effects like logging, state injection, or
//! channel notifications.
//!
//! All middleware implement [`ElicitCommunicator`] themselves, so they compose
//! naturally:
//!
//! ```rust,ignore
//! use elicitation::middleware::{ContextualCommunicator, ObservableCommunicator, knowledge_cache};
//! use elicitation::ElicitClient;
//! use tokio::sync::watch;
//!
//! let knowledge = knowledge_cache();
//! let (prompt_tx, _prompt_rx) = watch::channel(None);
//!
//! let comm = ObservableCommunicator::new(
//!     ContextualCommunicator::new(
//!         ElicitClient::new(config),
//!         knowledge
//!     ),
//!     prompt_tx
//! );
//! ```
//!
//! # Available middleware
//!
//! - [`ContextualCommunicator`] — prepends accumulated knowledge to every prompt
//!   (solves stateless elicitation protocol for stateful experiences)
//! - [`ObservableCommunicator`] — publishes exchanges to watch/mpsc channels
//!   (enables chat UIs, debug logging, replay systems)
//!
//! [`ElicitCommunicator`]: crate::ElicitCommunicator

mod contextual;
mod observable;

pub use contextual::{knowledge_cache, ContextualCommunicator, KnowledgeCache, SharedKnowledge};
pub use observable::{ChatMessage, ObservableCommunicator, Participant};
