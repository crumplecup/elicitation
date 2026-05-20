//! Trenchcoat wrapper for [`redb::TableStats`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Informational storage statistics for a single redb table.
///
/// Wraps `redb::TableStats` to add [`JsonSchema`] for MCP boundary crossing.
/// All fields are derived from getter methods since `redb::TableStats` fields
/// are private.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TableStats {
    /// Maximum traversal distance to reach the deepest key-value pair in the table.
    pub tree_height: u32,
    /// Number of leaf pages storing user data.
    pub leaf_pages: u64,
    /// Number of branch pages in the B-tree storing user data.
    pub branch_pages: u64,
    /// Bytes consumed by inserted keys and values (excluding indexing overhead).
    pub stored_bytes: u64,
    /// Bytes consumed by internal branch-page keys and other metadata.
    pub metadata_bytes: u64,
    /// Bytes consumed by fragmentation in data pages and internal tables.
    pub fragmented_bytes: u64,
}

#[cfg(feature = "redb-types")]
impl From<redb::TableStats> for TableStats {
    fn from(s: redb::TableStats) -> Self {
        Self {
            tree_height: s.tree_height(),
            leaf_pages: s.leaf_pages(),
            branch_pages: s.branch_pages(),
            stored_bytes: s.stored_bytes(),
            metadata_bytes: s.metadata_bytes(),
            fragmented_bytes: s.fragmented_bytes(),
        }
    }
}

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    FieldInfo, PatternDetails, Prompt, TypeMetadata,
};

impl Prompt for TableStats {
    fn prompt() -> Option<&'static str> {
        Some("Enter redb table statistics:")
    }
}

crate::default_style!(TableStats => TableStatsStyle);

impl Elicitation for TableStats {
    type Style = TableStatsStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RedbTableStats");
        let tree_height = u32::elicit(communicator).await?;
        let leaf_pages = u64::elicit(communicator).await?;
        let branch_pages = u64::elicit(communicator).await?;
        let stored_bytes = u64::elicit(communicator).await?;
        let metadata_bytes = u64::elicit(communicator).await?;
        let fragmented_bytes = u64::elicit(communicator).await?;
        Ok(Self { tree_height, leaf_pages, branch_pages, stored_bytes, metadata_bytes, fragmented_bytes })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u64 as crate::Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u64 as crate::Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u64 as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TableStats {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::RedbTableStats",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "tree_height", type_name: "u32", prompt: Some("B-tree height:") },
                    FieldInfo { name: "leaf_pages", type_name: "u64", prompt: Some("Leaf pages:") },
                    FieldInfo { name: "branch_pages", type_name: "u64", prompt: Some("Branch pages:") },
                    FieldInfo { name: "stored_bytes", type_name: "u64", prompt: Some("Stored bytes:") },
                    FieldInfo { name: "metadata_bytes", type_name: "u64", prompt: Some("Metadata bytes:") },
                    FieldInfo { name: "fragmented_bytes", type_name: "u64", prompt: Some("Fragmented bytes:") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TableStats {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "RedbTableStats".to_string(),
            fields: vec![
                ("tree_height".to_string(), Box::new(u32::prompt_tree())),
                ("leaf_pages".to_string(), Box::new(u64::prompt_tree())),
                ("branch_pages".to_string(), Box::new(u64::prompt_tree())),
                ("stored_bytes".to_string(), Box::new(u64::prompt_tree())),
                ("metadata_bytes".to_string(), Box::new(u64::prompt_tree())),
                ("fragmented_bytes".to_string(), Box::new(u64::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for TableStats {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let tree_height = self.tree_height;
        let leaf_pages = self.leaf_pages;
        let branch_pages = self.branch_pages;
        let stored_bytes = self.stored_bytes;
        let metadata_bytes = self.metadata_bytes;
        let fragmented_bytes = self.fragmented_bytes;
        quote::quote! {
            elicitation::RedbTableStats {
                tree_height: #tree_height,
                leaf_pages: #leaf_pages,
                branch_pages: #branch_pages,
                stored_bytes: #stored_bytes,
                metadata_bytes: #metadata_bytes,
                fragmented_bytes: #fragmented_bytes,
            }
        }
    }
}
