//! Smart pointer implementations (Box, Rc, Arc).

use crate::{ElicitResult, Elicitation, Prompt};
use std::rc::Rc;
use std::sync::Arc;

// Box<T>
impl<T> Prompt for Box<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        // Delegate to inner type's prompt
        T::prompt()
    }
}

impl<T> Elicitation for Box<T>
where
    T: Elicitation + Send,
{
    #[tracing::instrument(skip(client), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Box");
        T::elicit(client).await.map(Box::new)
    }
}

// Rc<T>
impl<T> Prompt for Rc<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        // Delegate to inner type's prompt
        T::prompt()
    }
}

impl<T> Elicitation for Rc<T>
where
    T: Elicitation + Send,
{
    #[tracing::instrument(skip(client), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Rc");
        T::elicit(client).await.map(Rc::new)
    }
}

// Arc<T>
impl<T> Prompt for Arc<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        // Delegate to inner type's prompt
        T::prompt()
    }
}

impl<T> Elicitation for Arc<T>
where
    T: Elicitation + Send,
{
    #[tracing::instrument(skip(client), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<U: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<U>,
    ) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Arc");
        T::elicit(client).await.map(Arc::new)
    }
}
