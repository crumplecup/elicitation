//! `AnyRow` — elicitation-enabled wrapper around [`sqlx::any::AnyRow`].

use std::sync::Arc;

use elicitation::{ColumnDescriptor, ColumnEntry, ColumnValue, RowData, SqlTypeKind};
use elicitation_derive::reflect_methods;
use serde::{Deserialize, Serialize};
use sqlx::Column as _;
use sqlx_core::any::{AnyValue, AnyValueKind};
use tracing::instrument;

/// Elicitation-enabled wrapper around `sqlx::any::AnyRow`.
#[derive(Clone)]
pub struct AnyRow(pub Arc<sqlx::any::AnyRow>);

impl std::fmt::Debug for AnyRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnyRow")
            .field("columns", &self.column_names())
            .finish()
    }
}

impl schemars::JsonSchema for AnyRow {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "AnyRow".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "description": "Elicitation-enabled wrapper around sqlx::any::AnyRow"
        })
    }
}

impl std::ops::Deref for AnyRow {
    type Target = sqlx::any::AnyRow;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<sqlx::any::AnyRow> for AnyRow {
    fn as_ref(&self) -> &sqlx::any::AnyRow {
        &self.0
    }
}

impl From<sqlx::any::AnyRow> for AnyRow {
    fn from(inner: sqlx::any::AnyRow) -> Self {
        Self(Arc::new(inner))
    }
}

impl From<Arc<sqlx::any::AnyRow>> for AnyRow {
    fn from(arc: Arc<sqlx::any::AnyRow>) -> Self {
        Self(arc)
    }
}

#[reflect_methods]
impl AnyRow {
    /// Returns the columns of this row as elicitation-enabled [`crate::AnyColumn`] wrappers.
    #[instrument(skip(self))]
    pub fn columns(&self) -> Vec<crate::AnyColumn> {
        self.0
            .columns
            .iter()
            .cloned()
            .map(crate::AnyColumn::from)
            .collect()
    }

    /// Returns the number of columns in this row.
    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.0.columns.len()
    }

    /// Returns `true` if this row has no columns.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.0.columns.is_empty()
    }

    /// Returns column names in order.
    #[instrument(skip(self))]
    pub fn column_names(&self) -> Vec<String> {
        self.0
            .columns
            .iter()
            .map(|c: &sqlx_core::any::AnyColumn| c.name().to_string())
            .collect()
    }

    /// Returns [`ColumnDescriptor`]s describing each column's name, ordinal, and type.
    #[instrument(skip(self))]
    pub fn columns_as_descriptors(&self) -> Vec<ColumnDescriptor> {
        self.0
            .columns
            .iter()
            .map(|c| {
                ColumnDescriptor::new(
                    c.ordinal(),
                    c.name().to_string(),
                    SqlTypeKind::from(c.type_info.kind),
                )
            })
            .collect()
    }

    /// Materializes all columns and values into a serializable [`RowData`].
    ///
    /// This is the primary way to transport row data across the MCP boundary.
    #[instrument(skip(self))]
    pub fn to_row_data(&self) -> RowData {
        let columns = self
            .0
            .columns
            .iter()
            .zip(self.0.values.iter())
            .map(|(col, val)| ColumnEntry::new(col.name().to_string(), decode_val(val)))
            .collect();
        RowData::new(columns)
    }
}

impl Serialize for AnyRow {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_row_data().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AnyRow {
    fn deserialize<D: serde::Deserializer<'de>>(_d: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "sqlx::any::AnyRow cannot be reconstructed from JSON",
        ))
    }
}

// ── Elicitation framework traits ─────────────────────────────────────────────

impl elicitation::Prompt for AnyRow {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl elicitation::Elicitation for AnyRow {
    type Style = ();

    async fn elicit<C: elicitation::ElicitCommunicator>(
        _communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        Err(elicitation::ElicitError::new(
            elicitation::ElicitErrorKind::ParseError(
                "AnyRow cannot be interactively elicited".to_string(),
            ),
        ))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::kani_trusted_opaque("AnyRow")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::verus_trusted_opaque("AnyRow")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::creusot_trusted_opaque("AnyRow")
    }
}

impl elicitation::ElicitIntrospect for AnyRow {
    fn pattern() -> elicitation::ElicitationPattern {
        elicitation::ElicitationPattern::Primitive
    }

    fn metadata() -> elicitation::TypeMetadata {
        elicitation::TypeMetadata {
            type_name: "AnyRow",
            description: None,
            details: elicitation::PatternDetails::Primitive,
        }
    }
}

impl elicitation::ElicitPromptTree for AnyRow {
    fn prompt_tree() -> elicitation::PromptTree {
        elicitation::PromptTree::Leaf {
            prompt: "AnyRow".to_string(),
            type_name: "AnyRow".to_string(),
        }
    }
}

impl elicitation::ElicitSpec for AnyRow {
    fn type_spec() -> elicitation::TypeSpec {
        elicitation::TypeSpecBuilder::default()
            .type_name("AnyRow".to_string())
            .summary("Elicitation-enabled newtype wrapper around `sqlx::any::AnyRow`.".to_string())
            .build()
            .expect("valid TypeSpec")
    }
}

mod emit_impls {
    use super::AnyRow;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AnyRow {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { { unimplemented!("AnyRow cannot be reconstructed as a code literal") } }
        }
    }
}

impl elicitation::ElicitComplete for AnyRow {}

/// Converts a raw [`AnyValue`] directly to our serializable [`ColumnValue`].
///
/// Matches on `value.kind` rather than going through the `Decode` trait,
/// which avoids type-dispatch overhead and correctly handles null values
/// (encoded as `AnyValueKind::Null(kind)` rather than a failed decode).
///
/// `AnyValueKind` is `#[non_exhaustive]`; the wildcard arm is a future-proof
/// fallback that maps any unknown variant to `ColumnValue::Null`.
fn decode_val(value: &AnyValue) -> ColumnValue {
    match &value.kind {
        AnyValueKind::Null(_) => ColumnValue::Null,
        AnyValueKind::Bool(b) => ColumnValue::Bool(*b),
        AnyValueKind::SmallInt(i) => ColumnValue::SmallInt(*i),
        AnyValueKind::Integer(i) => ColumnValue::Integer(*i),
        AnyValueKind::BigInt(i) => ColumnValue::BigInt(*i),
        AnyValueKind::Real(f) => ColumnValue::Real(*f),
        AnyValueKind::Double(d) => ColumnValue::Double(*d),
        AnyValueKind::Text(s) => ColumnValue::Text(s.as_ref().to_string()),
        AnyValueKind::Blob(b) => ColumnValue::Blob(b.as_ref().to_vec()),
        _ => ColumnValue::Null,
    }
}
