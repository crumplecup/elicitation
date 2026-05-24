//! Trenchcoat wrapper for [`redb::DatabaseStats`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Informational storage statistics for a redb database.
///
/// Wraps `redb::DatabaseStats` to add [`JsonSchema`] for MCP boundary crossing.
/// All fields are derived from getter methods since `redb::DatabaseStats` fields
/// are private.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DatabaseStats {
    /// Maximum traversal distance to reach the deepest key-value pair across all tables.
    pub tree_height: u32,
    /// Total number of pages allocated in the database file.
    pub allocated_pages: u64,
    /// Number of leaf pages storing user data.
    pub leaf_pages: u64,
    /// Number of branch pages in B-trees storing user data.
    pub branch_pages: u64,
    /// Bytes consumed by inserted keys and values (excluding indexing overhead).
    pub stored_bytes: u64,
    /// Bytes consumed by internal branch-page keys and other metadata.
    pub metadata_bytes: u64,
    /// Bytes consumed by fragmentation in data pages and internal tables.
    pub fragmented_bytes: u64,
    /// Bytes per page in this database.
    pub page_size: usize,
}

#[cfg(feature = "redb-types")]
impl From<redb::DatabaseStats> for DatabaseStats {
    fn from(s: redb::DatabaseStats) -> Self {
        Self {
            tree_height: s.tree_height(),
            allocated_pages: s.allocated_pages(),
            leaf_pages: s.leaf_pages(),
            branch_pages: s.branch_pages(),
            stored_bytes: s.stored_bytes(),
            metadata_bytes: s.metadata_bytes(),
            fragmented_bytes: s.fragmented_bytes(),
            page_size: s.page_size(),
        }
    }
}

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

impl Prompt for DatabaseStats {
    fn prompt() -> Option<&'static str> {
        Some("Enter redb database statistics:")
    }
}

crate::default_style!(DatabaseStats => DatabaseStatsStyle);

impl Elicitation for DatabaseStats {
    type Style = DatabaseStatsStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RedbDatabaseStats");
        let tree_height = u32::elicit(communicator).await?;
        let allocated_pages = u64::elicit(communicator).await?;
        let leaf_pages = u64::elicit(communicator).await?;
        let branch_pages = u64::elicit(communicator).await?;
        let stored_bytes = u64::elicit(communicator).await?;
        let metadata_bytes = u64::elicit(communicator).await?;
        let fragmented_bytes = u64::elicit(communicator).await?;
        let page_size = usize::elicit(communicator).await?;
        Ok(Self {
            tree_height,
            allocated_pages,
            leaf_pages,
            branch_pages,
            stored_bytes,
            metadata_bytes,
            fragmented_bytes,
            page_size,
        })
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

impl ElicitIntrospect for DatabaseStats {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::RedbDatabaseStats",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "tree_height",
                        type_name: "u32",
                        prompt: Some("B-tree height:"),
                    },
                    FieldInfo {
                        name: "allocated_pages",
                        type_name: "u64",
                        prompt: Some("Allocated pages:"),
                    },
                    FieldInfo {
                        name: "leaf_pages",
                        type_name: "u64",
                        prompt: Some("Leaf pages:"),
                    },
                    FieldInfo {
                        name: "branch_pages",
                        type_name: "u64",
                        prompt: Some("Branch pages:"),
                    },
                    FieldInfo {
                        name: "stored_bytes",
                        type_name: "u64",
                        prompt: Some("Stored bytes:"),
                    },
                    FieldInfo {
                        name: "metadata_bytes",
                        type_name: "u64",
                        prompt: Some("Metadata bytes:"),
                    },
                    FieldInfo {
                        name: "fragmented_bytes",
                        type_name: "u64",
                        prompt: Some("Fragmented bytes:"),
                    },
                    FieldInfo {
                        name: "page_size",
                        type_name: "usize",
                        prompt: Some("Page size in bytes:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for DatabaseStats {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "RedbDatabaseStats".to_string(),
            fields: vec![
                ("tree_height".to_string(), Box::new(u32::prompt_tree())),
                ("allocated_pages".to_string(), Box::new(u64::prompt_tree())),
                ("leaf_pages".to_string(), Box::new(u64::prompt_tree())),
                ("branch_pages".to_string(), Box::new(u64::prompt_tree())),
                ("stored_bytes".to_string(), Box::new(u64::prompt_tree())),
                ("metadata_bytes".to_string(), Box::new(u64::prompt_tree())),
                ("fragmented_bytes".to_string(), Box::new(u64::prompt_tree())),
                ("page_size".to_string(), Box::new(usize::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for DatabaseStats {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let tree_height = self.tree_height;
        let allocated_pages = self.allocated_pages;
        let leaf_pages = self.leaf_pages;
        let branch_pages = self.branch_pages;
        let stored_bytes = self.stored_bytes;
        let metadata_bytes = self.metadata_bytes;
        let fragmented_bytes = self.fragmented_bytes;
        let page_size = self.page_size;
        quote::quote! {
            elicitation::RedbDatabaseStats {
                tree_height: #tree_height,
                allocated_pages: #allocated_pages,
                leaf_pages: #leaf_pages,
                branch_pages: #branch_pages,
                stored_bytes: #stored_bytes,
                metadata_bytes: #metadata_bytes,
                fragmented_bytes: #fragmented_bytes,
                page_size: #page_size,
            }
        }
    }
}
