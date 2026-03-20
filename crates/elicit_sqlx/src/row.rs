//! `AnyRow` — elicitation-enabled wrapper around [`sqlx::any::AnyRow`].

use std::sync::Arc;

use elicitation::{ColumnDescriptor, ColumnEntry, ColumnValue, RowData, SqlTypeKind};
use elicitation_derive::reflect_methods;
use sqlx::Column as _;
use sqlx::Row as _;
use sqlx::any::AnyTypeInfoKind;
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
    /// Returns the number of columns in this row.
    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.0.columns().len()
    }

    /// Returns `true` if this row has no columns.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.0.columns().is_empty()
    }

    /// Returns column names in order.
    #[instrument(skip(self))]
    pub fn column_names(&self) -> Vec<String> {
        self.0
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect()
    }

    /// Returns [`ColumnDescriptor`]s describing each column's name, ordinal, and type.
    #[instrument(skip(self))]
    pub fn columns_as_descriptors(&self) -> Vec<ColumnDescriptor> {
        self.0
            .columns()
            .iter()
            .map(|c| {
                ColumnDescriptor::new(
                    c.ordinal(),
                    c.name().to_string(),
                    SqlTypeKind::from(c.type_info().kind),
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
            .columns()
            .iter()
            .map(|col| {
                let name = col.name().to_string();
                let value = decode_col(&self.0, col.ordinal(), col.type_info().kind);
                ColumnEntry::new(name, value)
            })
            .collect();
        RowData::new(columns)
    }
}

fn decode_col(row: &sqlx::any::AnyRow, i: usize, kind: AnyTypeInfoKind) -> ColumnValue {
    match kind {
        AnyTypeInfoKind::Null => ColumnValue::Null,
        AnyTypeInfoKind::Bool => row
            .try_get::<bool, _>(i)
            .map(ColumnValue::Bool)
            .unwrap_or(ColumnValue::Null),
        AnyTypeInfoKind::SmallInt => row
            .try_get::<i16, _>(i)
            .map(ColumnValue::SmallInt)
            .unwrap_or(ColumnValue::Null),
        AnyTypeInfoKind::Integer => row
            .try_get::<i32, _>(i)
            .map(ColumnValue::Integer)
            .unwrap_or(ColumnValue::Null),
        AnyTypeInfoKind::BigInt => row
            .try_get::<i64, _>(i)
            .map(ColumnValue::BigInt)
            .unwrap_or(ColumnValue::Null),
        AnyTypeInfoKind::Real => row
            .try_get::<f32, _>(i)
            .map(ColumnValue::Real)
            .unwrap_or(ColumnValue::Null),
        AnyTypeInfoKind::Double => row
            .try_get::<f64, _>(i)
            .map(ColumnValue::Double)
            .unwrap_or(ColumnValue::Null),
        AnyTypeInfoKind::Text => row
            .try_get::<String, _>(i)
            .map(ColumnValue::Text)
            .unwrap_or(ColumnValue::Null),
        AnyTypeInfoKind::Blob => row
            .try_get::<Vec<u8>, _>(i)
            .map(ColumnValue::Blob)
            .unwrap_or(ColumnValue::Null),
    }
}
