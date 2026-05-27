//! [`ObservableCommunicator`] — transparent middleware that captures elicitation
//! exchanges and publishes them to external channels.
//!
//! # Problem
//!
//! Applications often need to observe elicitation exchanges for:
//! - Debug logging / replay systems
//! - Chat UI displays showing player/agent conversations
//! - In-flight prompt indicators during long LLM completions
//!
//! Without middleware, every call site would need manual instrumentation,
//! coupling workflow logic to display concerns.
//!
//! # Solution
//!
//! [`ObservableCommunicator`] wraps any inner [`ElicitCommunicator`] and
//! forwards every call unchanged.  The only side-effect is in [`send_prompt`]:
//! before delegating to the inner communicator, it publishes the assembled
//! prompt to a `watch::Sender<Option<String>>` (for in-flight display) and
//! optionally appends the exchange to an `mpsc` chat history channel.
//!
//! After the response returns, it clears the in-flight prompt and optionally
//! appends the reply to the chat history.
//!
//! ## Why `watch` for in-flight prompts?
//!
//! `watch` has "latest value" semantics: non-blocking reads, no backpressure,
//! stale values are automatically dropped.  Perfect for "show the current
//! prompt" widgets.
//!
//! ## Why `mpsc` for chat history?
//!
//! The chat log is append-only; every message must survive.  `mpsc` preserves
//! ordering and never drops entries.
//!
//! # Usage
//!
//! ```rust,ignore
//! use tokio::sync::{mpsc, watch};
//! use elicitation::middleware::{ObservableCommunicator, Participant};
//! use elicitation::ElicitClient;
//!
//! let (prompt_tx, prompt_rx) = watch::channel(None);
//! let (chat_tx, chat_rx) = mpsc::unbounded_channel();
//!
//! let comm = ObservableCommunicator::new(ElicitClient::new(config), prompt_tx)
//!     .with_chat(chat_tx, Participant::Agent(Some("GPT-4o".into())));
//!
//! // prompt_rx can be read by a UI widget to show in-flight prompts
//! // chat_rx receives ChatMessage entries as exchanges complete
//! ```
//!
//! [`send_prompt`]: crate::ElicitCommunicator::send_prompt

use crate::{ElicitCommunicator, ElicitResult, ElicitationContext, StyleContext, StyleMarker};
use std::time::Instant;
use tokio::sync::{mpsc, watch};
use tracing::instrument;

/// Identity of the participant in an elicitation exchange.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Participant {
    /// The application host (poses questions).
    Host,
    /// A human player/user.
    Human,
    /// An AI agent with optional model name.
    Agent(Option<String>),
    /// Custom participant type.
    Custom(String),
}

impl std::fmt::Display for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Participant::Host => write!(f, "Host"),
            Participant::Human => write!(f, "Human"),
            Participant::Agent(None) => write!(f, "Agent"),
            Participant::Agent(Some(model)) => write!(f, "Agent({model})"),
            Participant::Custom(name) => write!(f, "{name}"),
        }
    }
}

/// A chat message capturing one side of an elicitation exchange.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    /// Who sent this message.
    pub sender: Participant,
    /// The message content.
    pub content: String,
    /// When the message was created.
    pub timestamp: Instant,
}

impl ChatMessage {
    /// Create a new chat message with the current timestamp.
    pub fn new(sender: Participant, content: impl Into<String>) -> Self {
        Self {
            sender,
            content: content.into(),
            timestamp: Instant::now(),
        }
    }
}

/// Transparent middleware that captures elicitation exchanges and publishes
/// them to external channels.
///
/// Wraps any inner [`ElicitCommunicator`].  `send_prompt` intercepts the
/// assembled prompt (already formatted with numbered options by the elicitation
/// runtime), publishes it to a `watch` channel for in-flight display, and
/// optionally routes it to a chat history channel.
///
/// Construct with [`ObservableCommunicator::new`], optionally attach a chat
/// sink with [`with_chat`].
///
/// [`with_chat`]: ObservableCommunicator::with_chat
#[derive(Clone)]
pub struct ObservableCommunicator<C> {
    inner: C,
    prompt_tx: watch::Sender<Option<String>>,
    /// Optional chat history sink and participant identity.
    chat: Option<(mpsc::UnboundedSender<ChatMessage>, Participant)>,
}

impl<C> ObservableCommunicator<C> {
    /// Wrap `inner`, publishing in-flight prompts to `prompt_tx`.
    ///
    /// Use [`with_chat`] to also route exchanges to a chat history sink.
    ///
    /// [`with_chat`]: ObservableCommunicator::with_chat
    pub fn new(inner: C, prompt_tx: watch::Sender<Option<String>>) -> Self {
        Self {
            inner,
            prompt_tx,
            chat: None,
        }
    }

    /// Attach a chat history sink so each exchange is appended as
    /// [`ChatMessage`] entries.
    ///
    /// `participant` identifies who is replying (e.g. [`Participant::Human`]
    /// or [`Participant::Agent(Some("GPT-4o".into()))`]).
    pub fn with_chat(
        mut self,
        chat_tx: mpsc::UnboundedSender<ChatMessage>,
        participant: Participant,
    ) -> Self {
        self.chat = Some((chat_tx, participant));
        self
    }
}

impl<C: ElicitCommunicator> ElicitCommunicator for ObservableCommunicator<C> {
    /// Publish the prompt to the watch channel (and chat log if configured),
    /// delegate to the inner communicator, then clear the watch channel on return.
    #[instrument(skip(self), level = "debug", fields(prompt_len = prompt.len()))]
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl std::future::Future<Output = ElicitResult<String>> + Send {
        let prompt_owned = prompt.to_string();
        let watch_tx = self.prompt_tx.clone();
        let chat = self.chat.clone();
        let inner_future = self.inner.send_prompt(prompt);

        async move {
            // Publish in-flight prompt for any live display widget.
            watch_tx.send(Some(prompt_owned.clone())).ok();

            // Append host prompt to chat history if configured.
            if let Some((ref tx, _)) = chat {
                tx.send(ChatMessage::new(Participant::Host, prompt_owned))
                    .ok();
            }

            let result = inner_future.await;

            // Clear the in-flight prompt: exchange complete.
            watch_tx.send(None).ok();

            // Append the reply to chat history if configured.
            if let Some((ref tx, ref participant)) = chat
                && let Ok(ref response) = result
            {
                tx.send(ChatMessage::new(participant.clone(), response.clone()))
                    .ok();
            }

            result
        }
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
            prompt_tx: self.prompt_tx.clone(),
            chat: self.chat.clone(),
        }
    }
}
