//! Trenchcoat wrapper for [`csv::ByteRecord`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A single CSV record as a sequence of raw byte fields.
///
/// Wraps `csv::ByteRecord` as `Vec<Vec<u8>>` to add [`JsonSchema`] for MCP
/// boundary crossing. Each element corresponds to one field in the record.
/// Use base64 encoding when passing binary field data over MCP.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CsvByteRecord(pub Vec<Vec<u8>>);

impl CsvByteRecord {
    /// Returns the number of fields in this record.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if this record contains no fields.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<Vec<u8>>> for CsvByteRecord {
    fn from(v: Vec<Vec<u8>>) -> Self {
        CsvByteRecord(v)
    }
}

impl From<CsvByteRecord> for Vec<Vec<u8>> {
    fn from(r: CsvByteRecord) -> Self {
        r.0
    }
}

#[cfg(feature = "csv-types")]
impl From<csv::ByteRecord> for CsvByteRecord {
    fn from(r: csv::ByteRecord) -> Self {
        CsvByteRecord(r.iter().map(|f| f.to_vec()).collect())
    }
}

#[cfg(feature = "csv-types")]
impl From<CsvByteRecord> for csv::ByteRecord {
    fn from(r: CsvByteRecord) -> Self {
        r.0.into_iter().collect()
    }
}

impl Prompt for CsvByteRecord {
    fn prompt() -> Option<&'static str> {
        Some("Provide the byte fields for this CSV record:")
    }
}

crate::default_style!(CsvByteRecord => CsvByteRecordStyle);

impl Elicitation for CsvByteRecord {
    type Style = CsvByteRecordStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CsvByteRecord");
        Ok(Self(Vec::<Vec<u8>>::elicit(communicator).await?))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <Vec<Vec<u8>> as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <Vec<Vec<u8>> as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <Vec<Vec<u8>> as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for CsvByteRecord {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "csv::ByteRecord",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "fields",
                    type_name: "Vec<Vec<u8>>",
                    prompt: Some("Byte fields of this CSV record:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for CsvByteRecord {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "CsvByteRecord".to_string(),
            fields: vec![(
                "fields".to_string(),
                Box::new(<Vec<Vec<u8>> as crate::ElicitPromptTree>::prompt_tree()),
            )],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for CsvByteRecord {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let inner =
            <Vec<Vec<u8>> as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.0);
        quote::quote! { ::elicitation::CsvByteRecord(#inner) }
    }
}
