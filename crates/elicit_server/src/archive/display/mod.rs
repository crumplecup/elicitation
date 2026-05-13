//! AccessKit display layer for archive descriptor types.
//!
//! Provides the [`ArchiveDisplay`] trait and per-type `DisplayMode` enums.
//! Each mode maps to a different accesskit node tree structure suitable for
//! different UI contexts (data grids, tree navigators, summary cards, etc.).

mod admin_snapshot;
mod column;
mod column_stats;
mod composite_type;
mod connection_profile;
mod constraint;
mod database;
mod ddl;
mod domain;
mod enum_type;
mod erd;
mod explain_node;
mod foreign_key;
mod function;
mod index;
mod monitor_snapshot;
mod query_history;
mod query_result;
mod saved_query;
mod schema;
mod sequence;
mod staged_edit;
mod table;
mod table_inspection;
mod trait_def;
mod trigger;

pub use admin_snapshot::AdminSnapshotMode;
pub use column::ColumnDescriptorMode;
pub use column_stats::ColumnStatsMode;
pub use composite_type::CompositeTypeDescriptorMode;
pub use connection_profile::ConnectionProfileMode;
pub use constraint::ConstraintDescriptorMode;
pub use database::DatabaseDescriptorMode;
pub use ddl::DdlDescriptorMode;
pub use domain::DomainDescriptorMode;
pub use enum_type::EnumDescriptorMode;
pub use erd::{ErdColumnMode, ErdDiagramMode, ErdEdgeMode, ErdNodeMode};
pub use explain_node::ExplainNodeMode;
pub use foreign_key::ForeignKeyDescriptorMode;
pub use function::FunctionDescriptorMode;
pub use index::IndexDescriptorMode;
pub use monitor_snapshot::MonitorSnapshotMode;
pub use query_history::QueryHistoryEntryMode;
pub use query_result::QueryResultMode;
pub use saved_query::SavedQueryMode;
pub use schema::SchemaDescriptorMode;
pub use sequence::SequenceDescriptorMode;
pub use staged_edit::StagedEditMode;
pub use table::TableDescriptorMode;
pub use table_inspection::TableInspectionMode;
pub use trait_def::ArchiveDisplay;
pub use trigger::TriggerDescriptorMode;
