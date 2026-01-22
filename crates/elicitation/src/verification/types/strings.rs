//! String contract types.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use super::{ValidationError, Utf8Bytes};
use elicitation_macros::instrumented_impl;
use elicitation_derive::contract_type;

// ============================================================================

/// Contract type for non-empty String values with bounded length.
///
/// Built on top of `Utf8Bytes<N>` foundation, adding non-empty constraint.
///
/// # Type Parameters
///
/// * `MAX_LEN` - Maximum byte length (default: 4096)
///
/// # Invariants
///
/// 1. Content is valid UTF-8 (inherited from `Utf8Bytes`)
/// 2. Length is > 0 (non-empty)
/// 3. Length <= MAX_LEN (bounded)
///
/// Validates on construction, then can unwrap to stdlib String via `into_inner()`.
#[contract_type(
    requires = "!value.is_empty()",
    ensures = "!result.get().is_empty()"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringNonEmpty<const MAX_LEN: usize = 4096> {
    utf8: Utf8Bytes<MAX_LEN>,
}

#[instrumented_impl]
impl<const MAX_LEN: usize> StringNonEmpty<MAX_LEN> {
    /// Constructs a non-empty, bounded String from stdlib String.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if:
    /// - String is empty
    /// - String length exceeds MAX_LEN bytes
    /// - String is not valid UTF-8 (should never happen for stdlib String)
    pub fn new(value: String) -> Result<Self, ValidationError> {
        if value.is_empty() {
            return Err(ValidationError::EmptyString);
        }

        let bytes = value.as_bytes();
        if bytes.len() > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: bytes.len(),
            });
        }

        // Copy into fixed array
        let mut array = [0u8; MAX_LEN];
        array[..bytes.len()].copy_from_slice(bytes);

        let utf8 = Utf8Bytes::new(array, bytes.len())?;
        
        Ok(Self { utf8 })
    }

    /// Gets the wrapped value as a string slice.
    pub fn get(&self) -> &str {
        self.utf8.as_str()
    }

    /// Gets the byte length of the string.
    pub fn len(&self) -> usize {
        self.utf8.len()
    }

    /// Checks if the string is empty (always false due to invariant).
    pub fn is_empty(&self) -> bool {
        false // Guaranteed by non-empty invariant
    }

    /// Unwraps to stdlib String (trenchcoat off).
    pub fn into_inner(self) -> String {
        self.utf8.to_string()
    }
}

crate::default_style!(StringNonEmpty<4096> => StringNonEmptyStyle);

#[instrumented_impl]
impl<const MAX_LEN: usize> Prompt for StringNonEmpty<MAX_LEN> {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-empty string:")
    }
}

#[instrumented_impl]
impl<const MAX_LEN: usize> Elicitation for StringNonEmpty<MAX_LEN> {
    type Style = StringNonEmptyStyle;

    #[tracing::instrument(skip(client), fields(type_name = "StringNonEmpty", max_len = MAX_LEN))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!(max_len = MAX_LEN, "Eliciting StringNonEmpty (non-empty, bounded string)");

        loop {
            let value = String::elicit(client).await?;
            
            match Self::new(value) {
                Ok(non_empty) => {
                    tracing::debug!(
                        value = %non_empty.get(),
                        len = non_empty.len(),
                        max_len = MAX_LEN,
                        "Valid StringNonEmpty constructed"
                    );
                    return Ok(non_empty);
                }
                Err(e) => {
                    tracing::warn!(error = %e, max_len = MAX_LEN, "Invalid StringNonEmpty, re-prompting");
                }
            }
        }
    }
}

#[cfg(test)]
mod string_nonempty_tests {
    use super::*;

    #[test]
    fn string_nonempty_new_valid() {
        let result: Result<StringNonEmpty, _> = StringNonEmpty::new("hello".to_string());
        assert!(result.is_ok());
        let non_empty = result.unwrap();
        assert_eq!(non_empty.get(), "hello");
        assert_eq!(non_empty.len(), 5);
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn string_nonempty_new_empty_invalid() {
        let result = StringNonEmpty::<1024>::new(String::new());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::EmptyString));
    }

    #[test]
    fn string_nonempty_into_inner() {
        let non_empty: StringNonEmpty = StringNonEmpty::new("world".to_string()).unwrap();
        let value: String = non_empty.into_inner();
        assert_eq!(value, "world");
    }

    #[test]
    fn string_nonempty_respects_max_len() {
        // Should accept string at limit
        let at_limit = "a".repeat(100);
        let result = StringNonEmpty::<100>::new(at_limit);
        assert!(result.is_ok());

        // Should reject string over limit
        let over_limit = "a".repeat(101);
        let result = StringNonEmpty::<100>::new(over_limit);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::TooLong { max: 100, actual: 101 }
        ));
    }

    #[test]
    fn string_nonempty_default_max_len() {
        // Default should be 4096
        let large = "a".repeat(4096);
        let result: Result<StringNonEmpty, _> = StringNonEmpty::new(large);
        assert!(result.is_ok());

        let too_large = "a".repeat(4097);
        let result: Result<StringNonEmpty, _> = StringNonEmpty::new(too_large);
        assert!(result.is_err());
    }

    #[test]
    fn string_nonempty_utf8_preserved() {
        let emoji = "Hello 👋 世界 🌍".to_string();
        let non_empty: StringNonEmpty = StringNonEmpty::new(emoji.clone()).unwrap();
        assert_eq!(non_empty.get(), emoji);
        assert_eq!(non_empty.into_inner(), emoji);
    }
}

// ============================================================================
