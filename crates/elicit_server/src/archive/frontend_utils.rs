//! Shared IR-pipeline helper for the archive frontends.
//!
//! Converts the archive display layer output into a [`VerifiedTree`] ready for
//! any rendering backend (ratatui, leptos/axum).
//!
//! # Verification chain
//!
//! `ArchiveDisplay::to_ak_nodes` produces an AccessKit IR.  This module
//! converts that IR to the [`VerifiedTree`] type, which carries
//! `Established<RenderComplete>` through to whichever renderer consumes it.
//! Both runtime interpretation (live server) and code-generation (emit) read
//! the same verified specification.

use std::collections::HashMap;

use elicit_accesskit::NodeJson;
use elicit_ui::{VerifiedTree, Viewport};
use tracing::instrument;

use crate::archive::{
    ArchiveResult, DatabaseDescriptor,
    display::{ArchiveDisplay, DatabaseDescriptorMode},
    errors::{ArchiveError, ArchiveErrorKind},
};
use elicit_db::{DbSchemaManager, DbServerAdmin};

// ── IR helpers ────────────────────────────────────────────────────────────────

/// Convert a `(elicit_accesskit::NodeId, Vec<(NodeId, NodeJson)>)` pair list
/// from `to_ak_nodes` into the `HashMap<accesskit::NodeId, accesskit::Node>`
/// that [`VerifiedTree::from_parts`] expects.
fn convert_nodes(
    pairs: Vec<(elicit_accesskit::NodeId, NodeJson)>,
) -> HashMap<accesskit::NodeId, accesskit::Node> {
    pairs
        .into_iter()
        .map(|(eid, json)| (eid.0, accesskit::Node::from(json)))
        .collect()
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Build a [`VerifiedTree`] from a live [`crate::archive::ArchiveDbBackend`].
///
/// Queries the schema list, constructs a [`DatabaseDescriptor`] display root,
/// and runs the `ArchiveDisplay` → AccessKit → [`VerifiedTree`] pipeline.
#[instrument(skip(backend))]
pub async fn build_verified_tree(
    backend: &crate::archive::ArchiveDbBackend,
) -> ArchiveResult<VerifiedTree> {
    // Step 1 — query live metadata
    let version = backend
        .server_version()
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Query(e.to_string())))?;

    let schema_names = backend
        .list_schemas()
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Query(e.to_string())))?;

    // Step 2 — build DatabaseDescriptor (the AccessKit root)
    let db_desc = DatabaseDescriptor {
        connection_id: "live".to_string(),
        db_name: schema_names
            .first()
            .cloned()
            .unwrap_or_else(|| "archive".to_string()),
        version: Some(version),
        backend: crate::archive::BackendKind::Postgres,
    };

    verified_tree_from_descriptor(&db_desc)
}

/// Build a [`VerifiedTree`] from a pre-constructed [`DatabaseDescriptor`].
///
/// Use this when a live connection is not available (e.g. demo mode or tests).
#[instrument(skip(desc))]
pub fn verified_tree_from_descriptor(desc: &DatabaseDescriptor) -> ArchiveResult<VerifiedTree> {
    // Step 1 — ArchiveDisplay → AccessKit IR (Establishes: ValidRole + HasLabel)
    let (root_eid, pairs) = desc.to_ak_nodes(&DatabaseDescriptorMode::Overview, 1);

    // Step 2 — convert to raw accesskit types
    let nodes = convert_nodes(pairs);
    let root = root_eid.0;
    let viewport = Viewport::new(800, 600);

    // Step 3 — VerifiedTree::from_parts carries all established proofs forward
    Ok(VerifiedTree::from_parts(nodes, root, viewport))
}

/// Build a demo [`VerifiedTree`] without a live database connection.
///
/// Constructs a minimal [`DatabaseDescriptor`] for display purposes.
pub fn demo_verified_tree() -> ArchiveResult<VerifiedTree> {
    let desc = DatabaseDescriptor {
        connection_id: "demo".to_string(),
        db_name: "archive (demo)".to_string(),
        version: Some("PostgreSQL 15.0".to_string()),
        backend: crate::archive::BackendKind::Postgres,
    };
    verified_tree_from_descriptor(&desc)
}
