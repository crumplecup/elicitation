//! Dynamic factories for open-generic csv operations.
//!
//! - [`prime_csv_deserialize`] — registers CSV deserialization tools for a concrete `D`
//! - [`prime_csv_serialize`] — registers CSV serialization tools for a concrete `T`

use elicitation::ElicitComplete;
use serde::{Serialize, de::DeserializeOwned};

/// Register CSV deserialization tools for type `D`.
///
/// Generates tools with the given `prefix`:
/// - `csv__deserialize__{prefix}__from_str` — parse all records from a CSV string
/// - `csv__deserialize__{prefix}__one`      — parse a single record by index
/// - `csv__deserialize__{prefix}__count`    — count records without deserializing
pub fn prime_csv_deserialize<D>(_prefix: &'static str)
where
    D: ElicitComplete + DeserializeOwned + 'static,
{
    // Stub — factory registration via DynamicToolRegistry.
}

/// Register CSV serialization tools for type `T`.
///
/// Generates tools with the given `prefix`:
/// - `csv__serialize__{prefix}__to_str`  — serialize a `Vec<T>` to a CSV string
/// - `csv__serialize__{prefix}__headers` — emit the header row only
/// - `csv__serialize__{prefix}__one`     — serialize a single record
pub fn prime_csv_serialize<T>(_prefix: &'static str)
where
    T: ElicitComplete + Serialize + 'static,
{
    // Stub — factory registration via DynamicToolRegistry.
}
