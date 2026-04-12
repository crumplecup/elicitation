//! `ArchiveBrowsePlugin` — database structure introspection via `elicit_sqlx`.
//!
//! Composes `elicit_sqlx` query tools against `information_schema` to produce
//! [`SchemaDescriptor`], [`TableDescriptor`], and [`IndexDescriptor`] values.
//! Each result carries the appropriate `Established<P>` contract.

// TODO(archive-browse-plugin): implement ArchiveBrowsePlugin
