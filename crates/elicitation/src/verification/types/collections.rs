//! Collection contract types demonstrating container verification.
//!
//! Collections wrap contract types - if all elements are valid contracts,
//! the collection is guaranteed valid by composition.

use super::ValidationError;
use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
use std::rc::Rc;
use std::sync::Arc;

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

    /// Check if empty (always returns false for NonEmpty collections).
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl<T: Elicitation + Send> Prompt for VecNonEmpty<T> {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-empty list:")
    }
}

impl<T: Elicitation + Send> Elicitation for VecNonEmpty<T> {
    type Style = <Vec<T> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting VecNonEmpty");
        loop {
            let vec = Vec::<T>::elicit(communicator).await?;
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting VecAllSatisfy");
        // Each element is C (contract type), so all guaranteed valid!
        let elements = Vec::<C>::elicit(communicator).await?;
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OptionSome");
        loop {
            let opt = Option::<T>::elicit(communicator).await?;
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ResultOk");
        // Just elicit T directly since we want guaranteed success
        let value = T::elicit(communicator).await?;
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting BoxSatisfies");
        let value = C::elicit(communicator).await?; // Guaranteed valid by contract!
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ArcSatisfies");
        let value = C::elicit(communicator).await?; // Guaranteed valid by contract!
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RcSatisfies");
        let value = C::elicit(communicator).await?; // Guaranteed valid by contract!
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
        let value: StringNonEmpty = StringNonEmpty::new("test".to_string()).unwrap();
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

// ============================================================================
// Map Collection Contracts
// ============================================================================

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

// HashMapNonEmpty - Non-empty HashMap
/// A HashMap that is guaranteed to be non-empty (has at least one key-value pair).
#[derive(Debug, Clone)]
#[cfg(not(kani))]
pub struct HashMapNonEmpty<K, V>(HashMap<K, V>);

#[cfg(kani)]
pub struct HashMapNonEmpty<K, V>(std::marker::PhantomData<(K, V)>);

impl<K, V> HashMapNonEmpty<K, V> {
    /// Create a new HashMapNonEmpty, validating the map is non-empty.
    #[cfg(not(kani))]
    pub fn new(map: HashMap<K, V>) -> Result<Self, ValidationError> {
        if map.is_empty() {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(map))
        }
    }

    /// Kani version: trust stdlib HashMap, verify wrapper logic.
    ///
    /// We use PhantomData because HashMap internals cause state explosion
    /// in Kani (see https://github.com/model-checking/kani/issues/1727).
    /// This approach verifies our validation logic without re-verifying stdlib.
    #[cfg(kani)]
    pub fn new(_map: HashMap<K, V>) -> Result<Self, ValidationError> {
        // Symbolic boolean represents is_empty() result
        // Trust: HashMap::is_empty() correctly reports emptiness
        // Verify: Our wrapper's branching logic
        let is_empty: bool = kani::any();
        if is_empty {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(std::marker::PhantomData))
        }
    }

    /// Get the inner HashMap.
    #[cfg(not(kani))]
    pub fn get(&self) -> &HashMap<K, V> {
        &self.0
    }

    /// Kani version: accessor not verifiable (PhantomData).
    #[cfg(kani)]
    pub fn get(&self) -> &HashMap<K, V> {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner HashMap.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> HashMap<K, V> {
        self.0
    }

    /// Kani version: accessor not verifiable (PhantomData).
    #[cfg(kani)]
    pub fn into_inner(self) -> HashMap<K, V> {
        panic!("into_inner() not supported in Kani verification")
    }

    /// Get the length (always >= 1).
    #[cfg(not(kani))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty (always returns false for NonEmpty collections).
    #[cfg(not(kani))]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Kani version: accessor not verifiable (PhantomData).
    #[cfg(kani)]
    pub fn len(&self) -> usize {
        panic!("len() not supported in Kani verification")
    }

    /// Kani version: accessor not verifiable (PhantomData).
    #[cfg(kani)]
    pub fn is_empty(&self) -> bool {
        panic!("is_empty() not supported in Kani verification")
    }
}

impl<K, V> Prompt for HashMapNonEmpty<K, V>
where
    K: Elicitation + Send,
    V: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-empty map:")
    }
}

impl<K, V> Elicitation for HashMapNonEmpty<K, V>
where
    K: Elicitation + Eq + Hash + Send,
    V: Elicitation + Send,
{
    type Style = <HashMap<K, V> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting HashMapNonEmpty");
        loop {
            let map = HashMap::<K, V>::elicit(communicator).await?;
            match Self::new(map) {
                Ok(valid) => {
                    tracing::debug!(count = valid.len(), "Valid non-empty map");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "HashMap is empty, re-prompting");
                    continue;
                }
            }
        }
    }
}

// BTreeMapNonEmpty - Non-empty BTreeMap
/// A BTreeMap that is guaranteed to be non-empty.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct BTreeMapNonEmpty<K, V>(BTreeMap<K, V>);

#[cfg(kani)]
pub struct BTreeMapNonEmpty<K, V>(std::marker::PhantomData<(K, V)>);

impl<K, V> BTreeMapNonEmpty<K, V> {
    /// Create a new BTreeMapNonEmpty, validating the map is non-empty.
    #[cfg(not(kani))]
    pub fn new(map: BTreeMap<K, V>) -> Result<Self, ValidationError> {
        if map.is_empty() {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(map))
        }
    }

    /// Kani version: trust stdlib BTreeMap, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_map: BTreeMap<K, V>) -> Result<Self, ValidationError> {
        let is_empty: bool = kani::any();
        if is_empty {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(std::marker::PhantomData))
        }
    }

    /// Get the inner BTreeMap.
    #[cfg(not(kani))]
    pub fn get(&self) -> &BTreeMap<K, V> {
        &self.0
    }

    #[cfg(kani)]
    pub fn get(&self) -> &BTreeMap<K, V> {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner BTreeMap.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> BTreeMap<K, V> {
        self.0
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> BTreeMap<K, V> {
        panic!("into_inner() not supported in Kani verification")
    }

    /// Get the length (always >= 1).
    #[cfg(not(kani))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty (always returns false for NonEmpty collections).
    #[cfg(not(kani))]
    pub fn is_empty(&self) -> bool {
        false
    }

    #[cfg(kani)]
    pub fn len(&self) -> usize {
        panic!("len() not supported in Kani verification")
    }

    #[cfg(kani)]
    pub fn is_empty(&self) -> bool {
        panic!("is_empty() not supported in Kani verification")
    }
}

impl<K, V> Prompt for BTreeMapNonEmpty<K, V>
where
    K: Elicitation + Send,
    V: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-empty map:")
    }
}

impl<K, V> Elicitation for BTreeMapNonEmpty<K, V>
where
    K: Elicitation + Ord + Send,
    V: Elicitation + Send,
{
    type Style = <BTreeMap<K, V> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting BTreeMapNonEmpty");
        loop {
            let map = BTreeMap::<K, V>::elicit(communicator).await?;
            match Self::new(map) {
                Ok(valid) => {
                    tracing::debug!(count = valid.len(), "Valid non-empty btree map");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "BTreeMap is empty, re-prompting");
                    continue;
                }
            }
        }
    }
}

// ============================================================================
// Set Collection Contracts
// ============================================================================

// HashSetNonEmpty - Non-empty HashSet
/// A HashSet that is guaranteed to be non-empty (has at least one element).
#[derive(Debug, Clone)]
#[cfg(not(kani))]
pub struct HashSetNonEmpty<T>(HashSet<T>);

#[cfg(kani)]
pub struct HashSetNonEmpty<T>(std::marker::PhantomData<T>);

impl<T> HashSetNonEmpty<T> {
    /// Create a new HashSetNonEmpty, validating the set is non-empty.
    #[cfg(not(kani))]
    pub fn new(set: HashSet<T>) -> Result<Self, ValidationError> {
        if set.is_empty() {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(set))
        }
    }

    /// Kani version: trust stdlib HashSet, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_set: HashSet<T>) -> Result<Self, ValidationError> {
        let is_empty: bool = kani::any();
        if is_empty {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(std::marker::PhantomData))
        }
    }

    /// Get the inner HashSet.
    #[cfg(not(kani))]
    pub fn get(&self) -> &HashSet<T> {
        &self.0
    }

    #[cfg(kani)]
    pub fn get(&self) -> &HashSet<T> {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner HashSet.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> HashSet<T> {
        self.0
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> HashSet<T> {
        panic!("into_inner() not supported in Kani verification")
    }

    /// Get the length (always >= 1).
    #[cfg(not(kani))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty (always returns false for NonEmpty collections).
    #[cfg(not(kani))]
    pub fn is_empty(&self) -> bool {
        false
    }

    #[cfg(kani)]
    pub fn len(&self) -> usize {
        panic!("len() not supported in Kani verification")
    }

    #[cfg(kani)]
    pub fn is_empty(&self) -> bool {
        panic!("is_empty() not supported in Kani verification")
    }
}

impl<T> Prompt for HashSetNonEmpty<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-empty set:")
    }
}

impl<T> Elicitation for HashSetNonEmpty<T>
where
    T: Elicitation + Eq + Hash + Send,
{
    type Style = <HashSet<T> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting HashSetNonEmpty");
        loop {
            let set = HashSet::<T>::elicit(communicator).await?;
            match Self::new(set) {
                Ok(valid) => {
                    tracing::debug!(count = valid.len(), "Valid non-empty set");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "HashSet is empty, re-prompting");
                    continue;
                }
            }
        }
    }
}

// BTreeSetNonEmpty - Non-empty BTreeSet
/// A BTreeSet that is guaranteed to be non-empty.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct BTreeSetNonEmpty<T>(BTreeSet<T>);

#[cfg(kani)]
pub struct BTreeSetNonEmpty<T>(std::marker::PhantomData<T>);

impl<T> BTreeSetNonEmpty<T> {
    /// Create a new BTreeSetNonEmpty, validating the set is non-empty.
    #[cfg(not(kani))]
    pub fn new(set: BTreeSet<T>) -> Result<Self, ValidationError> {
        if set.is_empty() {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(set))
        }
    }

    /// Kani version: trust stdlib BTreeSet, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_set: BTreeSet<T>) -> Result<Self, ValidationError> {
        let is_empty: bool = kani::any();
        if is_empty {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(std::marker::PhantomData))
        }
    }

    /// Get the inner BTreeSet.
    #[cfg(not(kani))]
    pub fn get(&self) -> &BTreeSet<T> {
        &self.0
    }

    #[cfg(kani)]
    pub fn get(&self) -> &BTreeSet<T> {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner BTreeSet.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> BTreeSet<T> {
        self.0
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> BTreeSet<T> {
        panic!("into_inner() not supported in Kani verification")
    }

    /// Get the length (always >= 1).
    #[cfg(not(kani))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty (always returns false for NonEmpty collections).
    #[cfg(not(kani))]
    pub fn is_empty(&self) -> bool {
        false
    }

    #[cfg(kani)]
    pub fn len(&self) -> usize {
        panic!("len() not supported in Kani verification")
    }

    #[cfg(kani)]
    pub fn is_empty(&self) -> bool {
        panic!("is_empty() not supported in Kani verification")
    }
}

impl<T> Prompt for BTreeSetNonEmpty<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-empty set:")
    }
}

impl<T> Elicitation for BTreeSetNonEmpty<T>
where
    T: Elicitation + Ord + Send,
{
    type Style = <BTreeSet<T> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting BTreeSetNonEmpty");
        loop {
            let set = BTreeSet::<T>::elicit(communicator).await?;
            match Self::new(set) {
                Ok(valid) => {
                    tracing::debug!(count = valid.len(), "Valid non-empty btree set");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "BTreeSet is empty, re-prompting");
                    continue;
                }
            }
        }
    }
}

// ============================================================================
// Deque Collection Contracts
// ============================================================================

// VecDequeNonEmpty - Non-empty VecDeque
/// A VecDeque that is guaranteed to be non-empty.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct VecDequeNonEmpty<T>(VecDeque<T>);

#[cfg(kani)]
pub struct VecDequeNonEmpty<T>(std::marker::PhantomData<T>);

impl<T> VecDequeNonEmpty<T> {
    /// Create a new VecDequeNonEmpty, validating the deque is non-empty.
    #[cfg(not(kani))]
    pub fn new(deque: VecDeque<T>) -> Result<Self, ValidationError> {
        if deque.is_empty() {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(deque))
        }
    }

    /// Kani version: trust stdlib VecDeque, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_deque: VecDeque<T>) -> Result<Self, ValidationError> {
        let is_empty: bool = kani::any();
        if is_empty {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(std::marker::PhantomData))
        }
    }

    /// Get the inner VecDeque.
    #[cfg(not(kani))]
    pub fn get(&self) -> &VecDeque<T> {
        &self.0
    }

    #[cfg(kani)]
    pub fn get(&self) -> &VecDeque<T> {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner VecDeque.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> VecDeque<T> {
        self.0
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> VecDeque<T> {
        panic!("into_inner() not supported in Kani verification")
    }

    /// Get the length (always >= 1).
    #[cfg(not(kani))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty (always returns false for NonEmpty collections).
    #[cfg(not(kani))]
    pub fn is_empty(&self) -> bool {
        false
    }

    #[cfg(kani)]
    pub fn len(&self) -> usize {
        panic!("len() not supported in Kani verification")
    }

    #[cfg(kani)]
    pub fn is_empty(&self) -> bool {
        panic!("is_empty() not supported in Kani verification")
    }
}

impl<T> Prompt for VecDequeNonEmpty<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-empty deque:")
    }
}

impl<T> Elicitation for VecDequeNonEmpty<T>
where
    T: Elicitation + Send,
{
    type Style = <VecDeque<T> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting VecDequeNonEmpty");
        loop {
            let deque = VecDeque::<T>::elicit(communicator).await?;
            match Self::new(deque) {
                Ok(valid) => {
                    tracing::debug!(count = valid.len(), "Valid non-empty deque");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "VecDeque is empty, re-prompting");
                    continue;
                }
            }
        }
    }
}

// LinkedListNonEmpty - Non-empty LinkedList
/// A LinkedList that is guaranteed to be non-empty.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct LinkedListNonEmpty<T>(LinkedList<T>);

#[cfg(kani)]
pub struct LinkedListNonEmpty<T>(std::marker::PhantomData<T>);

impl<T> LinkedListNonEmpty<T> {
    /// Create a new LinkedListNonEmpty, validating the list is non-empty.
    #[cfg(not(kani))]
    pub fn new(list: LinkedList<T>) -> Result<Self, ValidationError> {
        if list.is_empty() {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(list))
        }
    }

    /// Kani version: trust stdlib LinkedList, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_list: LinkedList<T>) -> Result<Self, ValidationError> {
        let is_empty: bool = kani::any();
        if is_empty {
            Err(ValidationError::EmptyCollection)
        } else {
            Ok(Self(std::marker::PhantomData))
        }
    }

    /// Get the inner LinkedList.
    #[cfg(not(kani))]
    pub fn get(&self) -> &LinkedList<T> {
        &self.0
    }

    #[cfg(kani)]
    pub fn get(&self) -> &LinkedList<T> {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner LinkedList.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> LinkedList<T> {
        self.0
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> LinkedList<T> {
        panic!("into_inner() not supported in Kani verification")
    }

    /// Get the length (always >= 1).
    #[cfg(not(kani))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if empty (always returns false for NonEmpty collections).
    #[cfg(not(kani))]
    pub fn is_empty(&self) -> bool {
        false
    }

    #[cfg(kani)]
    pub fn len(&self) -> usize {
        panic!("len() not supported in Kani verification")
    }

    #[cfg(kani)]
    pub fn is_empty(&self) -> bool {
        panic!("is_empty() not supported in Kani verification")
    }
}

impl<T> Prompt for LinkedListNonEmpty<T>
where
    T: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-empty list:")
    }
}

impl<T> Elicitation for LinkedListNonEmpty<T>
where
    T: Elicitation + Send,
{
    type Style = <LinkedList<T> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LinkedListNonEmpty");
        loop {
            let list = LinkedList::<T>::elicit(communicator).await?;
            match Self::new(list) {
                Ok(valid) => {
                    tracing::debug!(count = valid.len(), "Valid non-empty linked list");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "LinkedList is empty, re-prompting");
                    continue;
                }
            }
        }
    }
}

// ============================================================================
// Array Contract
// ============================================================================

// ArrayAllSatisfy - Array where all elements satisfy a contract
/// An array where every element is a contract type C.
///
/// **Compositional verification:** If C is valid and all N elements are C,
/// then ArrayAllSatisfy<C, N> is automatically valid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayAllSatisfy<C, const N: usize>([C; N]);

impl<C, const N: usize> ArrayAllSatisfy<C, N> {
    /// Create a new ArrayAllSatisfy. All elements are already validated contract types.
    pub fn new(elements: [C; N]) -> Self {
        Self(elements)
    }

    /// Get the inner array.
    pub fn get(&self) -> &[C; N] {
        &self.0
    }

    /// Unwrap into the inner array.
    pub fn into_inner(self) -> [C; N] {
        self.0
    }
}

impl<C, const N: usize> Prompt for ArrayAllSatisfy<C, N>
where
    C: Elicitation + Send,
{
    fn prompt() -> Option<&'static str> {
        Some("Please provide array elements:")
    }
}

impl<C, const N: usize> Elicitation for ArrayAllSatisfy<C, N>
where
    C: Elicitation + Send,
{
    type Style = <[C; N] as Elicitation>::Style;

    #[tracing::instrument(skip(communicator), fields(array_size = N))]
    async fn elicit<Comm: ElicitCommunicator>(communicator: &Comm) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ArrayAllSatisfy");
        // Each element is C (contract type), so all guaranteed valid!
        let elements = <[C; N]>::elicit(communicator).await?;
        tracing::debug!(size = N, "All array elements satisfy contract");
        Ok(Self::new(elements))
    }
}

#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::verification::types::I8Positive;

    #[test]
    fn test_hashmap_non_empty_valid() {
        let mut map = HashMap::new();
        map.insert("key", 42);
        let result = HashMapNonEmpty::new(map);
        assert!(result.is_ok());
    }

    #[test]
    fn test_hashmap_non_empty_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        let result = HashMapNonEmpty::new(map);
        assert!(result.is_err());
    }

    #[test]
    fn test_hashset_non_empty_valid() {
        let mut set = HashSet::new();
        set.insert(1);
        let result = HashSetNonEmpty::new(set);
        assert!(result.is_ok());
    }

    #[test]
    fn test_btreemap_non_empty_valid() {
        let mut map = BTreeMap::new();
        map.insert(1, "value");
        let result = BTreeMapNonEmpty::new(map);
        assert!(result.is_ok());
    }

    #[test]
    fn test_btreeset_non_empty_valid() {
        let mut set = BTreeSet::new();
        set.insert(1);
        let result = BTreeSetNonEmpty::new(set);
        assert!(result.is_ok());
    }

    #[test]
    fn test_vecdeque_non_empty_valid() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        let result = VecDequeNonEmpty::new(deque);
        assert!(result.is_ok());
    }

    #[test]
    fn test_linkedlist_non_empty_valid() {
        let mut list = LinkedList::new();
        list.push_back(1);
        let result = LinkedListNonEmpty::new(list);
        assert!(result.is_ok());
    }

    #[test]
    fn test_array_all_satisfy() {
        let elements = [
            I8Positive::new(1).unwrap(),
            I8Positive::new(2).unwrap(),
            I8Positive::new(3).unwrap(),
        ];
        let array = ArrayAllSatisfy::new(elements);
        assert_eq!(array.get().len(), 3);
    }
}

// ============================================================================
// Smart Pointer NonNull Types (for Prusti proofs)
// ============================================================================

/// Contract type for non-null Box<T>.
#[derive(Debug, Clone)]
pub struct BoxNonNull<T>(Box<T>);

impl<T> BoxNonNull<T> {
    /// Creates a BoxNonNull (Box is always non-null by construction).
    pub fn new(value: Box<T>) -> Result<Self, super::ValidationError> {
        Ok(Self(value))
    }

    /// Gets a reference to the inner value.
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Unwraps to inner Box.
    pub fn into_inner(self) -> Box<T> {
        self.0
    }
}

/// Contract type for non-null Arc<T>.
#[derive(Debug, Clone)]
pub struct ArcNonNull<T>(std::sync::Arc<T>);

impl<T> ArcNonNull<T> {
    /// Creates an ArcNonNull (Arc is always non-null by construction).
    pub fn new(value: std::sync::Arc<T>) -> Result<Self, super::ValidationError> {
        Ok(Self(value))
    }

    /// Gets a reference to the inner value.
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Unwraps to inner Arc.
    pub fn into_inner(self) -> std::sync::Arc<T> {
        self.0
    }
}

/// Contract type for non-null Rc<T>.
#[derive(Debug, Clone)]
pub struct RcNonNull<T>(std::rc::Rc<T>);

impl<T> RcNonNull<T> {
    /// Creates an RcNonNull (Rc is always non-null by construction).
    pub fn new(value: std::rc::Rc<T>) -> Result<Self, super::ValidationError> {
        Ok(Self(value))
    }

    /// Gets a reference to the inner value.
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Unwraps to inner Rc.
    pub fn into_inner(self) -> std::rc::Rc<T> {
        self.0
    }
}
