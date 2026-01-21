//! Collection contract types demonstrating container verification.
//!
//! Collections wrap contract types - if all elements are valid contracts,
//! the collection is guaranteed valid by composition.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use super::ValidationError;
use std::sync::Arc;
use std::rc::Rc;

// VecNonEmpty - Non-empty Vec
/// A Vec that is guaranteed to be non-empty (has at least one element).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecNonEmpty<T>(Vec<T>);

impl<T> VecNonEmpty<T> {
    /// Create a new VecNonEmpty, validating the Vec is non-empty.
    pub fn new(vec: Vec<T>) -> Result<Self, ValidationError> {
        if vec.is_empty() {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(vec))
        }
    }

    /// Get the inner Vec.
    pub fn get(&self) -> &Vec<T> {
        &self.0
    }

    /// Unwrap into the inner Vec.
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    /// Get the length (always >= 1).
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Elicitation + Send> Prompt for VecNonEmpty<T> {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-empty list:")
    }
}

impl<T: Elicitation + Send> Elicitation for VecNonEmpty<T> {
    type Style = <Vec<T> as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting VecNonEmpty");
        loop {
            let vec = Vec::<T>::elicit(client).await?;
            match Self::new(vec) {
                Ok(valid) => {
                    tracing::debug!(count = valid.len(), "Valid non-empty vec");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Vec is empty, re-prompting");
                    continue;
                }
            }
        }
    }
}

// VecAllSatisfy - Vec where all elements satisfy a contract
/// A Vec where every element is a contract type C.
///
/// **Compositional verification:** If C is a valid contract, and all elements
/// are C, then VecAllSatisfy<C> is automatically valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecAllSatisfy<C>(Vec<C>);

impl<C> VecAllSatisfy<C> {
    /// Create a new VecAllSatisfy. All elements are already validated contract types.
    pub fn new(elements: Vec<C>) -> Self {
        Self(elements)
    }

    /// Get the inner Vec.
    pub fn get(&self) -> &Vec<C> {
        &self.0
    }

    /// Unwrap into the inner Vec.
    pub fn into_inner(self) -> Vec<C> {
        self.0
    }
}

impl<C: Elicitation + Send> Prompt for VecAllSatisfy<C> {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a list:")
    }
}

impl<C: Elicitation + Send> Elicitation for VecAllSatisfy<C> {
    type Style = <Vec<C> as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting VecAllSatisfy");
        // Each element is C (contract type), so all guaranteed valid!
        let elements = Vec::<C>::elicit(client).await?;
        tracing::debug!(count = elements.len(), "All elements satisfy contract");
        Ok(Self::new(elements)) // Composition = automatic verification
    }
}

// OptionSome - Option that is guaranteed to be Some
/// An Option that is guaranteed to be Some (not None).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OptionSome<T>(T);

impl<T> OptionSome<T> {
    /// Create a new OptionSome from an Option, validating it's Some.
    pub fn new(opt: Option<T>) -> Result<Self, ValidationError> {
        match opt {
            Some(value) => Ok(Self(value)),
            None => Err(ValidationError::OptionIsNone),
        }
    }

    /// Create from a value (always succeeds).
    pub fn from_value(value: T) -> Self {
        Self(value)
    }

    /// Get the inner value.
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Unwrap into the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Elicitation + Send> Prompt for OptionSome<T> {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a value (required, not optional):")
    }
}

impl<T: Elicitation + Send> Elicitation for OptionSome<T> {
    type Style = <Option<T> as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OptionSome");
        loop {
            let opt = Option::<T>::elicit(client).await?;
            match Self::new(opt) {
                Ok(valid) => {
                    tracing::debug!("Valid Some value");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Option is None, re-prompting");
                    continue;
                }
            }
        }
    }
}

// ResultOk - Result that is guaranteed to be Ok
/// A Result that is guaranteed to be Ok (not Err).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultOk<T>(T);

impl<T> ResultOk<T> {
    /// Create a new ResultOk from a Result, validating it's Ok.
    pub fn new<E>(result: Result<T, E>) -> Result<Self, ValidationError> {
        match result {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(ValidationError::ResultIsErr),
        }
    }

    /// Create from a value (always succeeds).
    pub fn from_value(value: T) -> Self {
        Self(value)
    }

    /// Get the inner value.
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Unwrap into the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Elicitation + Send> Prompt for ResultOk<T> {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a successful value:")
    }
}

impl<T: Elicitation + Send> Elicitation for ResultOk<T> {
    type Style = <T as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ResultOk");
        // Just elicit T directly since we want guaranteed success
        let value = T::elicit(client).await?;
        Ok(Self::from_value(value))
    }
}

// BoxSatisfies - Box wrapping a contract type
/// A Box wrapping a contract type C.
///
/// **Compositional verification:** If C is valid, Box<C> is valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoxSatisfies<C>(Box<C>);

impl<C> BoxSatisfies<C> {
    /// Create a new BoxSatisfies. Inner value is already a validated contract type.
    pub fn new(value: C) -> Self {
        Self(Box::new(value))
    }

    /// Get the inner value.
    pub fn get(&self) -> &C {
        &self.0
    }

    /// Unwrap into the inner value.
    pub fn into_inner(self) -> Box<C> {
        self.0
    }
}

impl<C: Elicitation + Send> Prompt for BoxSatisfies<C> {
    fn prompt() -> Option<&'static str> {
        C::prompt()
    }
}

impl<C: Elicitation + Send> Elicitation for BoxSatisfies<C> {
    type Style = C::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting BoxSatisfies");
        let value = C::elicit(client).await?; // Guaranteed valid by contract!
        Ok(Self::new(value))
    }
}

// ArcSatisfies - Arc wrapping a contract type
/// An Arc wrapping a contract type C.
///
/// **Compositional verification:** If C is valid, Arc<C> is valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArcSatisfies<C>(Arc<C>);

impl<C> ArcSatisfies<C> {
    /// Create a new ArcSatisfies. Inner value is already a validated contract type.
    pub fn new(value: C) -> Self {
        Self(Arc::new(value))
    }

    /// Get the inner value.
    pub fn get(&self) -> &C {
        &self.0
    }

    /// Unwrap into the inner Arc.
    pub fn into_inner(self) -> Arc<C> {
        self.0
    }
}

impl<C: Elicitation + Send> Prompt for ArcSatisfies<C> {
    fn prompt() -> Option<&'static str> {
        C::prompt()
    }
}

impl<C: Elicitation + Send> Elicitation for ArcSatisfies<C> {
    type Style = C::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ArcSatisfies");
        let value = C::elicit(client).await?; // Guaranteed valid by contract!
        Ok(Self::new(value))
    }
}

// RcSatisfies - Rc wrapping a contract type
/// An Rc wrapping a contract type C.
///
/// **Compositional verification:** If C is valid, Rc<C> is valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RcSatisfies<C>(Rc<C>);

impl<C> RcSatisfies<C> {
    /// Create a new RcSatisfies. Inner value is already a validated contract type.
    pub fn new(value: C) -> Self {
        Self(Rc::new(value))
    }

    /// Get the inner value.
    pub fn get(&self) -> &C {
        &self.0
    }

    /// Unwrap into the inner Rc.
    pub fn into_inner(self) -> Rc<C> {
        self.0
    }
}

impl<C: Elicitation + Send> Prompt for RcSatisfies<C> {
    fn prompt() -> Option<&'static str> {
        C::prompt()
    }
}

impl<C: Elicitation + Send> Elicitation for RcSatisfies<C> {
    type Style = C::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RcSatisfies");
        let value = C::elicit(client).await?; // Guaranteed valid by contract!
        Ok(Self::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verification::types::{I8Positive, StringNonEmpty};

    #[test]
    fn test_vec_non_empty_valid() {
        let vec = vec![1, 2, 3];
        let result = VecNonEmpty::new(vec);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_vec_non_empty_empty() {
        let vec: Vec<i32> = vec![];
        let result = VecNonEmpty::new(vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_vec_all_satisfy() {
        let elements = vec![
            I8Positive::new(1).unwrap(),
            I8Positive::new(2).unwrap(),
            I8Positive::new(3).unwrap(),
        ];
        let vec = VecAllSatisfy::new(elements);
        assert_eq!(vec.get().len(), 3);
    }

    #[test]
    fn test_option_some_valid() {
        let opt = Some(42);
        let result = OptionSome::new(opt);
        assert!(result.is_ok());
        assert_eq!(*result.unwrap().get(), 42);
    }

    #[test]
    fn test_option_some_none() {
        let opt: Option<i32> = None;
        let result = OptionSome::new(opt);
        assert!(result.is_err());
    }

    #[test]
    fn test_result_ok_valid() {
        let res: Result<i32, String> = Ok(42);
        let result = ResultOk::new(res);
        assert!(result.is_ok());
        assert_eq!(*result.unwrap().get(), 42);
    }

    #[test]
    fn test_result_ok_err() {
        let res: Result<i32, String> = Err("failed".to_string());
        let result = ResultOk::<i32>::new(res);
        assert!(result.is_err());
    }

    #[test]
    fn test_box_satisfies() {
        let value = I8Positive::new(5).unwrap();
        let boxed = BoxSatisfies::new(value);
        assert_eq!(boxed.get().get(), 5);
    }

    #[test]
    fn test_arc_satisfies() {
        let value = StringNonEmpty::new("test".to_string()).unwrap();
        let arc = ArcSatisfies::new(value);
        assert_eq!(arc.get().get(), "test");
    }

    #[test]
    fn test_rc_satisfies() {
        let value = I8Positive::new(10).unwrap();
        let rc = RcSatisfies::new(value);
        assert_eq!(rc.get().get(), 10);
    }
}
