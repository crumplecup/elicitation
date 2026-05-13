//! [`surrealdb_types::Range`] and [`surrealdb_types::RecordIdKeyRange`] newtype wrappers.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Range ─────────────────────────────────────────────────────────────────────

elicit_newtype!(surrealdb_types::Range, as Range, forward_serde);
elicit_newtype_traits!(Range, surrealdb_types::Range, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::Range`.
impl From<Range> for surrealdb_types::Range {
    fn from(val: Range) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Range {
    /// Returns `true` if the lower bound is unbounded.
    #[tracing::instrument(skip(self))]
    pub fn is_start_unbounded(&self) -> bool {
        matches!(self.0.start(), std::ops::Bound::Unbounded)
    }

    /// Returns `true` if the upper bound is unbounded.
    #[tracing::instrument(skip(self))]
    pub fn is_end_unbounded(&self) -> bool {
        matches!(self.0.end(), std::ops::Bound::Unbounded)
    }

    /// Returns `true` if both bounds are unbounded.
    #[tracing::instrument(skip(self))]
    pub fn is_fully_unbounded(&self) -> bool {
        self.is_start_unbounded() && self.is_end_unbounded()
    }
}

impl elicitation::ElicitComplete for Range {}

mod range_emit_impls {
    use super::Range;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Range {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Range is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Range>(#json)
                    .expect("valid Range JSON")
                    .into()
            }
        }
    }
}

// ── RecordIdKeyRange ──────────────────────────────────────────────────────────

elicit_newtype!(surrealdb_types::RecordIdKeyRange, as RecordIdKeyRange, forward_serde);
elicit_newtype_traits!(RecordIdKeyRange, surrealdb_types::RecordIdKeyRange, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::RecordIdKeyRange`.
impl From<RecordIdKeyRange> for surrealdb_types::RecordIdKeyRange {
    fn from(val: RecordIdKeyRange) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl RecordIdKeyRange {
    /// Returns `true` if the start bound is unbounded.
    #[tracing::instrument(skip(self))]
    pub fn is_start_unbounded(&self) -> bool {
        matches!(self.0.start(), std::ops::Bound::Unbounded)
    }

    /// Returns `true` if the end bound is unbounded.
    #[tracing::instrument(skip(self))]
    pub fn is_end_unbounded(&self) -> bool {
        matches!(self.0.end(), std::ops::Bound::Unbounded)
    }
}

impl elicitation::ElicitComplete for RecordIdKeyRange {}

mod record_id_key_range_emit_impls {
    use super::RecordIdKeyRange;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for RecordIdKeyRange {
        fn to_code_literal(&self) -> TokenStream {
            let json =
                serde_json::to_string(self.0.as_ref()).expect("RecordIdKeyRange is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::RecordIdKeyRange>(#json)
                    .expect("valid RecordIdKeyRange JSON")
                    .into()
            }
        }
    }
}
