//! Trenchcoat wrapper for [`csv::StringRecord`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A single CSV record as a sequence of UTF-8 string fields.
///
/// Wraps `csv::StringRecord` as `Vec<String>` to add [`JsonSchema`] for MCP
/// boundary crossing. Each element corresponds to one field in the record.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CsvStringRecord(pub Vec<String>);

impl CsvStringRecord {
    /// Returns the number of fields in this record.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if this record contains no fields.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<String>> for CsvStringRecord {
    fn from(v: Vec<String>) -> Self {
        CsvStringRecord(v)
    }
}

impl From<CsvStringRecord> for Vec<String> {
    fn from(r: CsvStringRecord) -> Self {
        r.0
    }
}

#[cfg(feature = "csv-types")]
impl From<csv::StringRecord> for CsvStringRecord {
    fn from(r: csv::StringRecord) -> Self {
        CsvStringRecord(r.iter().map(|s| s.to_string()).collect())
    }
}

#[cfg(feature = "csv-types")]
impl From<CsvStringRecord> for csv::StringRecord {
    fn from(r: CsvStringRecord) -> Self {
        r.0.iter().collect()
    }
}

impl Prompt for CsvStringRecord {
    fn prompt() -> Option<&'static str> {
        Some("Provide the string fields for this CSV record:")
    }
}

crate::default_style!(CsvStringRecord => CsvStringRecordStyle);

impl Elicitation for CsvStringRecord {
    type Style = CsvStringRecordStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CsvStringRecord");
        Ok(Self(Vec::<String>::elicit(communicator).await?))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for CsvStringRecord {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "csv::StringRecord",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "fields",
                    type_name: "Vec<String>",
                    prompt: Some("String fields of this CSV record:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for CsvStringRecord {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "CsvStringRecord".to_string(),
            fields: vec![(
                "fields".to_string(),
                Box::new(<Vec<String> as crate::ElicitPromptTree>::prompt_tree()),
            )],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for CsvStringRecord {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let inner =
            <Vec<String> as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.0);
        quote::quote! { ::elicitation::CsvStringRecord(#inner) }
    }
}
