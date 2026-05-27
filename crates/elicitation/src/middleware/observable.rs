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
//! use elicitation::middleware::{ChatMessage, ObservableCommunicator, Participant};
//! use elicitation::ElicitClient;
//!
//! let (prompt_tx, prompt_rx) = watch::channel(None);
//! let (chat_tx, chat_rx) = mpsc::unbounded_channel();
//!
//! // Default: use elicitation's own ChatMessage, human responder
//! let comm = ObservableCommunicator::new(ElicitClient::new(config), prompt_tx)
//!     .with_chat(chat_tx, Participant::Human, |p, text| ChatMessage::new(p, text));
//!
//! // Custom message type, AI agent responder:
//! let comm = ObservableCommunicator::new(ElicitClient::new(config), prompt_tx)
//!     .with_chat(app_tx, Participant::Agent(Some("GPT-4o".into())), |participant, text| {
//!         let sender = match participant {
//!             Participant::Host => AppSender::Gm,
//!             Participant::Agent(_) => AppSender::Ai,
//!             _ => AppSender::Other,
//!         };
//!         AppMessage::new(sender, text)
//!     });
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

/// Type alias for the chat sink stored inside [`ObservableCommunicator`].
type ChatSink<M> = (
    mpsc::UnboundedSender<M>,
    Participant,
    std::sync::Arc<dyn Fn(Participant, String) -> M + Send + Sync>,
);

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
/// The `M` type parameter lets you use your own message type instead of the
/// default [`ChatMessage`].  Supply a factory closure via [`with_chat`] that
/// converts a [`Participant`] + text into `M`.  This is essential when the
/// downstream application has its own message type (e.g. one that derives
/// `Elicit`, `Serialize`, or carries AccessKit IR).
///
/// # Examples
///
/// Using the default [`ChatMessage`]:
///
/// ```rust,ignore
/// let comm = ObservableCommunicator::new(inner, prompt_tx)
///     .with_chat(chat_tx, |p, text| ChatMessage::new(p, text));
/// ```
///
/// Using a custom downstream message type:
///
/// ```rust,ignore
/// // AppMessage is defined in your crate with its own derives
/// let comm = ObservableCommunicator::new(inner, prompt_tx)
///     .with_chat(app_tx, |participant, text| {
///         let sender = match participant {
///             Participant::Host => AppSender::Host,
///             _ => AppSender::Responder,
///         };
///         AppMessage::new(sender, text)
///     });
/// ```
///
/// Construct with [`ObservableCommunicator::new`], optionally attach a chat
/// sink with [`with_chat`].
///
/// [`with_chat`]: ObservableCommunicator::with_chat
pub struct ObservableCommunicator<C, M = ChatMessage> {
    inner: C,
    prompt_tx: watch::Sender<Option<String>>,
    /// Optional chat history sink: channel, responder identity, message factory.
    chat: Option<ChatSink<M>>,
}

impl<C: Clone, M> Clone for ObservableCommunicator<C, M> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            prompt_tx: self.prompt_tx.clone(),
            chat: self.chat.clone(),
        }
    }
}

impl<C, M> ObservableCommunicator<C, M> {
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

    /// Attach a chat history sink.
    ///
    /// `responder` identifies who is answering (e.g. [`Participant::Human`],
    /// [`Participant::Agent`], or a custom label).  The prompt side always
    /// receives [`Participant::Host`].
    ///
    /// `make_message` converts a [`Participant`] and text into your message
    /// type `M`, letting you map to any downstream type including ones with
    /// extra derives or domain-specific sender enums.
    pub fn with_chat(
        mut self,
        chat_tx: mpsc::UnboundedSender<M>,
        responder: Participant,
        make_message: impl Fn(Participant, String) -> M + Send + Sync + 'static,
    ) -> Self {
        self.chat = Some((chat_tx, responder, std::sync::Arc::new(make_message)));
        self
    }
}

impl<C: ElicitCommunicator, M: Send + 'static> ElicitCommunicator for ObservableCommunicator<C, M> {
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
            if let Some((ref tx, _, ref make)) = chat {
                tx.send(make(Participant::Host, prompt_owned)).ok();
            }

            let result = inner_future.await;

            // Clear the in-flight prompt: exchange complete.
            watch_tx.send(None).ok();

            // Append the reply to chat history if configured.
            if let Some((ref tx, ref responder, ref make)) = chat
                && let Ok(ref response) = result
            {
                tx.send(make(responder.clone(), response.clone())).ok();
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

    fn with_style<T: 'static, S: StyleMarker + crate::style::ElicitationStyle + 'static>(
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
