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

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_single_variant_enum("BoxStyle")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_single_variant_enum("BoxStyle")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_single_variant_enum("BoxStyle")
    }
}

impl crate::style::ElicitationStyle for BoxStyle {}

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

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_single_variant_enum("RcStyle")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_single_variant_enum("RcStyle")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_single_variant_enum("RcStyle")
    }
}

impl crate::style::ElicitationStyle for RcStyle {}

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

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_single_variant_enum("ArcStyle")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_single_variant_enum("ArcStyle")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_single_variant_enum("ArcStyle")
    }
}

impl crate::style::ElicitationStyle for ArcStyle {}

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

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::kani_proof()
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::verus_proof()
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::creusot_proof()
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

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::kani_proof()
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::verus_proof()
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::creusot_proof()
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

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::kani_proof()
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::verus_proof()
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::creusot_proof()
    }
}
