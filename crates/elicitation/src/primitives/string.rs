//! String type implementation.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};

// Generate default-only style enum
crate::default_style!(String => StringStyle);

impl Prompt for String {
    fn prompt() -> Option<&'static str> {
        Some("Please enter text:")
    }
}

impl Elicitation for String {
    type Style = StringStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        use crate::verification::types::StringDefault;
        
        tracing::debug!("Eliciting String via StringDefault wrapper");
        
        // Use verification wrapper internally
        let wrapper = StringDefault::elicit(client).await?;
        
        // Unwrap to primitive
        Ok(wrapper.into_inner())
    }
}
