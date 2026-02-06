//! Server wrapper for style-aware elicitation.
//!
//! This is the server-side equivalent of `ElicitClient`. It wraps a `Peer<RoleServer>`
//! and provides the same style management API, but uses server-to-client communication
//! via `peer.create_message()`.

use rmcp::service::{Peer, RoleServer};

use crate::{ElicitCommunicator, ElicitResult, ElicitationStyle, ElicitErrorKind, ElicitError, StyleContext};

/// Server wrapper that carries style context.
///
/// Wraps an RMCP server peer and maintains style selections for different types.
/// This is the server-side equivalent of `ElicitClient` - it has the same API
/// but uses `Peer<RoleServer>` for server-to-client communication.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::{ElicitServer, ElicitationStyle, Elicitation};
///
/// // In a tool handler:
/// #[tool]
/// async fn my_tool(peer: Peer<RoleServer>) -> Result<Config, Error> {
///     let server = ElicitServer::new(peer);
///     let config = Config::elicit(&server).await?;
///     Ok(config)
/// }
/// ```
#[derive(Clone)]
pub struct ElicitServer {
    peer: Peer<RoleServer>,
    style_context: StyleContext,
}

impl ElicitServer {
    /// Create a new server wrapper from an RMCP peer.
    #[tracing::instrument(skip(peer))]
    pub fn new(peer: Peer<RoleServer>) -> Self {
        tracing::debug!("Creating new ElicitServer");
        Self {
            peer,
            style_context: StyleContext::default(),
        }
    }

    /// Get the underlying RMCP peer for making requests to the client.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn peer(&self) -> &Peer<RoleServer> {
        &self.peer
    }
}

// Implement ElicitCommunicator for server-side communication
impl ElicitCommunicator for ElicitServer {
    #[tracing::instrument(skip(self, prompt), fields(prompt_len = prompt.len()))]
    async fn send_prompt(&self, prompt: &str) -> ElicitResult<String> {
        tracing::debug!("Sending prompt to client via create_message");
        
        // Create message request
        let params = rmcp::model::CreateMessageRequestParams {
            meta: None,
            task: None,
            messages: vec![rmcp::model::SamplingMessage {
                role: rmcp::model::Role::User,
                content: rmcp::model::Content::text(prompt),
            }],
            model_preferences: None,
            system_prompt: Some(
                "You are helping elicit structured data. Provide clear, concise responses.".to_string()
            ),
            include_context: None,
            temperature: None,
            max_tokens: 1000,
            stop_sequences: None,
            metadata: None,
        };
        
        // Send request to client
        let result = self.peer.create_message(params)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "create_message failed");
                ElicitError::new(ElicitErrorKind::Service(e.into()))
            })?;
        
        tracing::debug!(model = %result.model, stop_reason = ?result.stop_reason, "Received response");
        
        // Extract text from response
        use rmcp::model::RawContent;
        match &*result.message.content {
            RawContent::Text(text_content) => {
                tracing::debug!(response_len = text_content.text.len(), "Extracted text response");
                Ok(text_content.text.clone())
            }
            RawContent::Image(_) => {
                tracing::warn!("Received image content when expecting text");
                Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "text".to_string(),
                    received: "image".to_string(),
                }))
            }
            _ => {
                tracing::warn!("Received unexpected content type");
                Err(ElicitError::new(ElicitErrorKind::InvalidFormat {
                    expected: "text".to_string(),
                    received: "other".to_string(),
                }))
            }
        }
    }

    async fn call_tool(
        &self,
        _params: rmcp::model::CallToolRequestParams,
    ) -> Result<rmcp::model::CallToolResult, rmcp::service::ServiceError> {
        // Servers don't call tools in the server-side elicitation model
        // (they use create_message instead)
        Err(rmcp::service::ServiceError::McpError(
            rmcp::ErrorData::internal_error(
                "call_tool not supported in server-side elicitation",
                None
            )
        ))
    }

    fn style_context(&self) -> &StyleContext {
        &self.style_context
    }

    fn with_style<T: 'static, S: ElicitationStyle>(&self, style: S) -> Self {
        let mut ctx = self.style_context.clone();
        ctx.set_style::<T, S>(style);
        Self {
            peer: self.peer.clone(),
            style_context: ctx,
        }
    }
}

