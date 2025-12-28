//! Fixed-size array [T; N] implementation using const generics.

use crate::{ElicitResult, Elicitation, Prompt};

impl<T, const N: usize> Prompt for [T; N]
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Eliciting fixed-size array elements:")
    }
}

impl<T, const N: usize> Elicitation for [T; N]
where
    T: Elicitation + Send,
{
    #[tracing::instrument(skip(client), fields(
        item_type = std::any::type_name::<T>(),
        size = N
    ))]
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
        tracing::debug!(size = N, "Eliciting fixed-size array");

        // Collect items into a Vec first
        let mut items = Vec::with_capacity(N);

        for i in 0..N {
            tracing::debug!(index = i, total = N, "Eliciting array element");
            let item = T::elicit(client).await?;
            items.push(item);
        }

        // Convert Vec to array using try_into
        items.try_into().map_err(|_| {
            // This should never happen since we pre-allocated N items
            crate::ElicitError::new(crate::ElicitErrorKind::InvalidFormat {
                expected: format!("array of size {}", N),
                received: "incorrect size".to_string(),
            })
        })
    }
}
