//! Smart pointer implementations (Box, Rc, Arc).

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
use std::rc::Rc;
use std::sync::Arc;

// Default-only styles for smart pointers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BoxStyle {
    #[default]
    Default,
}

impl Prompt for BoxStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for BoxStyle {
    type Style = BoxStyle;

    #[tracing::instrument(skip(_communicator), level = "trace")]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RcStyle {
    #[default]
    Default,
}

impl Prompt for RcStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for RcStyle {
    type Style = RcStyle;

    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ArcStyle {
    #[default]
    Default,
}

impl Prompt for ArcStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for ArcStyle {
    type Style = ArcStyle;

    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

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
    type Style = BoxStyle;

    #[tracing::instrument(skip(communicator), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Box");
        T::elicit(communicator).await.map(Box::new)
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
    type Style = RcStyle;

    #[tracing::instrument(skip(communicator), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Rc");
        T::elicit(communicator).await.map(Rc::new)
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
    type Style = ArcStyle;

    #[tracing::instrument(skip(communicator), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Arc");
        T::elicit(communicator).await.map(Arc::new)
    }
}
