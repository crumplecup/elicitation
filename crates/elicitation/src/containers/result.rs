//! Result<T, E> implementation for success/error elicitation.

use crate::{ElicitResult, Elicitation, Prompt};

impl<T, E> Prompt for Result<T, E>
where
    T: Elicitation + Send,
    E: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Is this a success (Ok) or failure (Err)?")
    }
}

impl<T, E> Elicitation for Result<T, E>
where
    T: Elicitation + Send,
    E: Elicitation + Send,
{
    #[tracing::instrument(skip(client), fields(
        ok_type = std::any::type_name::<T>(),
        err_type = std::any::type_name::<E>()
    ))]
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Result");

        // First, ask if it's Ok or Err
        // We use a bool here - true for Ok, false for Err
        tracing::debug!("Asking if result is Ok or Err");
        let is_ok = bool::elicit(client).await?;

        if is_ok {
            tracing::debug!("Eliciting Ok variant");
            let value = T::elicit(client).await?;
            tracing::debug!("Result::Ok created");
            Ok(Ok(value))
        } else {
            tracing::debug!("Eliciting Err variant");
            let error = E::elicit(client).await?;
            tracing::debug!("Result::Err created");
            Ok(Err(error))
        }
    }
}
