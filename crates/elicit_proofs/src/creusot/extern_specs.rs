//! Extern spec contracts for standard library types used in Creusot companions.
//!
//! Provides Creusot models for String methods that are not in `creusot-std`.
//! These are trusted axioms; the postconditions match the standard semantics.

use creusot_std::prelude::*;

extern_spec! {
    impl String {
        #[ensures(result@ == Seq::empty())]
        fn new() -> String;
        #[ensures(result == ((*self)@.len() == 0))]
        fn is_empty(&self) -> bool;
        #[ensures((^self)@ == (*self)@.push_back(ch))]
        fn push(&mut self, ch: char);
        #[ensures(match result {
            Some(t) =>
                (^self)@ == (*self)@.subsequence(0, (*self)@.len() - 1) &&
                (*self)@ == (^self)@.push_back(t),
            None => *self == ^self && (*self)@.len() == 0
        })]
        fn pop(&mut self) -> Option<char>;
    }
}

extern_spec! {
    impl str {
        #[ensures(result == ((*self)@.len() == 0))]
        fn is_empty(&self) -> bool;
    }
}
