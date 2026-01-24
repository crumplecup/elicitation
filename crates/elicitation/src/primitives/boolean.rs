//! Boolean type implementation using the Affirm pattern.

use crate::{Affirm, ElicitClient, ElicitResult, Elicitation, Prompt};

// Generate default-only style enum
crate::default_style!(bool => BoolStyle);

impl Prompt for bool {
    fn prompt() -> Option<&'static str> {
        Some("Please answer yes or no:")
    }
}

impl Affirm for bool {}

impl Elicitation for bool {
    type Style = BoolStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        use crate::verification::types::BoolDefault;
        
        tracing::debug!("Eliciting bool via BoolDefault wrapper");
        
        // Use verification wrapper internally
        let wrapper = BoolDefault::elicit(client).await?;
        
        // Unwrap to primitive
        Ok(wrapper.into_inner())
    }
}
