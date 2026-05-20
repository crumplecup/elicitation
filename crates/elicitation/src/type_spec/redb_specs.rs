//! [`ElicitSpec`](crate::ElicitSpec) and [`ElicitComplete`](crate::ElicitComplete)
//! implementations for redb type elicitation.

#[cfg(feature = "redb-types")]
mod redb_impls {
    use crate::{
        ElicitComplete, ElicitSpec, RedbCacheStats, RedbDatabaseStats, RedbDurability,
        RedbTableStats, RedbTypeName, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    // -------------------------------------------------------------------------
    // Macro: impl_redb_select_spec!
    //
    // Generates ElicitSpec for a two-variant redb Select enum.
    // -------------------------------------------------------------------------

    macro_rules! impl_redb_select_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            variants = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let variants = SpecCategoryBuilder::default()
                        .name("variants".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("redb v4 — embedded key-value store".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Select — choose one variant from the list".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitComplete for $ty {}
        };
    }

    // -------------------------------------------------------------------------
    // Macro: impl_redb_survey_spec!
    //
    // Generates ElicitSpec for a redb Survey (struct) type.
    // -------------------------------------------------------------------------

    macro_rules! impl_redb_survey_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("redb v4 — embedded key-value store".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — fill in each field".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![fields, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitComplete for $ty {}
        };
    }

    // -------------------------------------------------------------------------
    // RedbDurability — write durability level
    // -------------------------------------------------------------------------

    impl_redb_select_spec!(
        type    = RedbDurability,
        name    = "elicitation::RedbDurability",
        summary = "Write durability level for a redb write transaction. \
                   Immediate guarantees persistence; None is faster but requires a \
                   subsequent Immediate commit to persist.",
        variants = [
            ("none",      "Commits are not persisted until followed by an Immediate commit. Fastest writes."),
            ("immediate", "Commits are guaranteed durable when commit() returns. Safe but slower."),
        ]
    );

    // -------------------------------------------------------------------------
    // RedbCacheStats — in-memory cache statistics
    // -------------------------------------------------------------------------

    impl_redb_survey_spec!(
        type    = RedbCacheStats,
        name    = "elicitation::RedbCacheStats",
        summary = "In-memory cache usage statistics for a redb database. \
                   All counters are zero unless the redb `cache_metrics` feature is enabled.",
        fields  = [
            ("evictions",    "u64 — number of times data was evicted due to cache being full"),
            ("read_hits",    "u64 — times unmodified data was served from the cache"),
            ("read_misses",  "u64 — times unmodified data was not in cache and read from storage"),
            ("write_hits",   "u64 — times transaction-modified data was served from cache"),
            ("write_misses", "u64 — times transaction-modified data was not in cache"),
            ("cached_bytes", "u64 — current number of bytes held in the cache"),
        ]
    );

    // -------------------------------------------------------------------------
    // RedbDatabaseStats — database storage statistics
    // -------------------------------------------------------------------------

    impl_redb_survey_spec!(
        type    = RedbDatabaseStats,
        name    = "elicitation::RedbDatabaseStats",
        summary = "Informational storage statistics for a redb database file.",
        fields  = [
            ("tree_height",      "u32 — maximum traversal distance to the deepest key-value pair"),
            ("allocated_pages",  "u64 — total pages allocated in the database file"),
            ("leaf_pages",       "u64 — leaf pages storing user data"),
            ("branch_pages",     "u64 — branch pages in B-trees storing user data"),
            ("stored_bytes",     "u64 — bytes used by inserted keys and values"),
            ("metadata_bytes",   "u64 — bytes used by internal keys and metadata"),
            ("fragmented_bytes", "u64 — bytes used by fragmentation and internal tables"),
            ("page_size",        "usize — bytes per page in this database"),
        ]
    );

    // -------------------------------------------------------------------------
    // RedbTableStats — per-table storage statistics
    // -------------------------------------------------------------------------

    impl_redb_survey_spec!(
        type    = RedbTableStats,
        name    = "elicitation::RedbTableStats",
        summary = "Informational storage statistics for a single redb table.",
        fields  = [
            ("tree_height",      "u32 — maximum traversal distance to the deepest key-value pair"),
            ("leaf_pages",       "u64 — leaf pages storing user data"),
            ("branch_pages",     "u64 — branch pages in the B-tree"),
            ("stored_bytes",     "u64 — bytes used by inserted keys and values"),
            ("metadata_bytes",   "u64 — bytes used by internal keys and metadata"),
            ("fragmented_bytes", "u64 — bytes used by fragmentation"),
        ]
    );

    // -------------------------------------------------------------------------
    // RedbTypeName — globally unique redb type identifier
    // -------------------------------------------------------------------------

    impl_redb_survey_spec!(
        type    = RedbTypeName,
        name    = "elicitation::RedbTypeName",
        summary = "A globally unique type identifier used by redb to name key and value types. \
                   Prefix with the crate name to avoid collisions, e.g. \"my_crate::MyKey\".",
        fields  = [
            ("name", "String — fully-qualified type name (e.g. \"my_crate::MyKey\")"),
        ]
    );
}
