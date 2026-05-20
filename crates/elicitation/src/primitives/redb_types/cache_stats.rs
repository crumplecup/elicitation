//! Trenchcoat wrapper for [`redb::CacheStats`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// In-memory cache usage statistics for a redb database.
///
/// Wraps `redb::CacheStats` to add [`JsonSchema`] for MCP boundary crossing.
///
/// Note: upstream `redb::CacheStats` is only populated when the `cache_metrics`
/// feature of the `redb` crate is enabled; without it all counters are zero.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CacheStats {
    /// Number of times data was evicted due to the cache being full.
    pub evictions: u64,
    /// Number of times unmodified data was served from the cache (hit).
    pub read_hits: u64,
    /// Number of times unmodified data was not in the cache and read from storage (miss).
    pub read_misses: u64,
    /// Number of times transaction-modified data was served from the cache (hit).
    pub write_hits: u64,
    /// Number of times transaction-modified data was not in cache and read from storage (miss).
    pub write_misses: u64,
    /// Current number of bytes held in the cache.
    pub cached_bytes: u64,
}
#[cfg(feature = "redb-types")]
impl From<redb::CacheStats> for CacheStats {
    fn from(s: redb::CacheStats) -> Self {
        Self {
            evictions: s.evictions(),
            read_hits: s.read_hits(),
            read_misses: s.read_misses(),
            write_hits: s.write_hits(),
            write_misses: s.write_misses(),
            cached_bytes: s.used_bytes() as u64,
        }
    }
}

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    FieldInfo, PatternDetails, Prompt, TypeMetadata,
};

impl Prompt for CacheStats {
    fn prompt() -> Option<&'static str> {
        Some("Enter redb cache statistics:")
    }
}

crate::default_style!(CacheStats => CacheStatsStyle);

impl Elicitation for CacheStats {
    type Style = CacheStatsStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RedbCacheStats");
        let evictions = u64::elicit(communicator).await?;
        let read_hits = u64::elicit(communicator).await?;
        let read_misses = u64::elicit(communicator).await?;
        let write_hits = u64::elicit(communicator).await?;
        let write_misses = u64::elicit(communicator).await?;
        let cached_bytes = u64::elicit(communicator).await?;
        Ok(Self { evictions, read_hits, read_misses, write_hits, write_misses, cached_bytes })
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

impl ElicitIntrospect for CacheStats {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::RedbCacheStats",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "evictions", type_name: "u64", prompt: Some("Cache evictions:") },
                    FieldInfo { name: "read_hits", type_name: "u64", prompt: Some("Read cache hits:") },
                    FieldInfo { name: "read_misses", type_name: "u64", prompt: Some("Read cache misses:") },
                    FieldInfo { name: "write_hits", type_name: "u64", prompt: Some("Write cache hits:") },
                    FieldInfo { name: "write_misses", type_name: "u64", prompt: Some("Write cache misses:") },
                    FieldInfo { name: "cached_bytes", type_name: "u64", prompt: Some("Bytes currently cached:") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for CacheStats {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "RedbCacheStats".to_string(),
            fields: vec![
                ("evictions".to_string(), Box::new(u64::prompt_tree())),
                ("read_hits".to_string(), Box::new(u64::prompt_tree())),
                ("read_misses".to_string(), Box::new(u64::prompt_tree())),
                ("write_hits".to_string(), Box::new(u64::prompt_tree())),
                ("write_misses".to_string(), Box::new(u64::prompt_tree())),
                ("cached_bytes".to_string(), Box::new(u64::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for CacheStats {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let evictions = self.evictions;
        let read_hits = self.read_hits;
        let read_misses = self.read_misses;
        let write_hits = self.write_hits;
        let write_misses = self.write_misses;
        let cached_bytes = self.cached_bytes;
        quote::quote! {
            elicitation::RedbCacheStats {
                evictions: #evictions,
                read_hits: #read_hits,
                read_misses: #read_misses,
                write_hits: #write_hits,
                write_misses: #write_misses,
                cached_bytes: #cached_bytes,
            }
        }
    }
}
