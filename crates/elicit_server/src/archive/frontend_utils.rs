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

use std::collections::BTreeMap;

use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};
use elicit_accesskit::NodeJson;
use elicit_ui::{VerifiedTree, Viewport};
use tracing::instrument;

use crate::archive::{
    ArchiveResult, DatabaseDescriptor,
    display::{ArchiveDisplay, DatabaseDescriptorMode},
    errors::{ArchiveError, ArchiveErrorKind},
    nav_tree::NavTree,
};
use elicit_db::{DbSchemaManager, DbServerAdmin};

// ── IR helpers ────────────────────────────────────────────────────────────────

/// Convert a `(elicit_accesskit::NodeId, Vec<(NodeId, NodeJson)>)` pair list
/// from `to_ak_nodes` into the `BTreeMap<accesskit::NodeId, accesskit::Node>`
/// that [`VerifiedTree::from_parts`] expects.
fn convert_nodes(
    pairs: Vec<(elicit_accesskit::NodeId, NodeJson)>,
) -> BTreeMap<accesskit::NodeId, accesskit::Node> {
    pairs
        .into_iter()
        .map(|(eid, json)| (eid.0, accesskit::Node::from(json)))
        .collect()
}

/// Wrap a content [`VerifiedTree`] in a `Role::Window` root that also contains
/// an archive status bar (derived from [`ArchiveKeyMap::default_map`]) as its
/// last child.
///
/// The composed tree has three node layers:
/// - `NodeId(0)` — `Role::Window` root (Window containing content + status bar)
/// - original content nodes (starting at `NodeId(1)`)
/// - status bar nodes (starting at `NodeId(10_000)`)
///
/// This is the canonical archive layout: content fills available space, the
/// status bar occupies one line at the bottom.
fn with_status_bar(
    content_root: accesskit::NodeId,
    mut nodes: BTreeMap<accesskit::NodeId, accesskit::Node>,
    viewport: Viewport,
) -> VerifiedTree {
    use crate::archive::actions::{ArchiveKeyMap, KeyMapMode};
    // Status bar subtree (id_base=10_000 avoids clashing with content nodes)
    let status = ArchiveKeyMap::default_map().to_status_bar(KeyMapMode::Default);
    let (status_root_eid, status_pairs) = status.to_ak_nodes(10_000);
    for (eid, json) in status_pairs {
        nodes.insert(eid.0, accesskit::Node::from(json));
    }

    // Window root wrapping both subtrees
    let window_id = accesskit::NodeId::from(0u64);
    let mut window = accesskit::Node::new(accesskit::Role::Window);
    window.set_children(vec![content_root, status_root_eid.0]);
    nodes.insert(window_id, window);

    VerifiedTree::from_parts(nodes, window_id, viewport)
}

// ── Public entry points ───────────────────────────────────────────────────────

/// Build a [`VerifiedTree`] from a pre-loaded [`NavTree`].
///
/// Constructs a full schema+table AccessKit tree for use in the browser
/// (leptos/axum) frontend.  The tree layout:
///
/// ```text
/// Window (0)
///   Tree (1)  — nav panel
///     TreeItem (schema 0)
///       TreeItem (table 0.0)
///       TreeItem (table 0.1)
///       ...
///     TreeItem (schema 1)
///       ...
///   Status bar (10_000+)
/// ```
#[instrument(skip(nav))]
pub fn nav_tree_to_verified_tree(nav: &NavTree) -> ArchiveResult<VerifiedTree> {
    let mut nodes: BTreeMap<AkNodeId, AkNode> = BTreeMap::new();
    let mut counter: u64 = 1; // 0 is reserved for Window

    // DB header — Banner so it renders as <header>, sits outside the Tree
    let header_id = AkNodeId::from(counter);
    counter += 1;
    let header_label = format!(
        "{} — {} ({})",
        nav.db_name,
        nav.version.as_deref().unwrap_or("unknown"),
        nav.backend,
    );
    let mut header_node = AkNode::new(AkRole::Banner);
    header_node.set_label(header_label);
    nodes.insert(header_id, header_node);

    // Nav panel root (Tree) — only schema TreeItems
    let nav_root_id = AkNodeId::from(counter);
    counter += 1;

    let mut schema_ids: Vec<AkNodeId> = Vec::new();

    for schema_entry in &nav.schemas {
        let schema_id = AkNodeId::from(counter);
        counter += 1;
        schema_ids.push(schema_id);

        let mut table_ids: Vec<AkNodeId> = Vec::new();

        for table in &schema_entry.tables {
            let table_id = AkNodeId::from(counter);
            counter += 1;
            table_ids.push(table_id);

            let label = format!(
                "{} [{}]{}",
                table.table_name,
                table.table_type,
                match table.estimated_rows {
                    Some(n) => format!(" ~{n}"),
                    None => String::new(),
                }
            );
            let mut table_node = AkNode::new(AkRole::TreeItem);
            table_node.set_label(label);
            nodes.insert(table_id, table_node);
        }

        let schema_label = format!(
            "{} ({}) — {} table{}",
            schema_entry.name,
            schema_entry.owner,
            schema_entry.tables.len(),
            if schema_entry.tables.len() == 1 {
                ""
            } else {
                "s"
            },
        );
        let mut schema_node = AkNode::new(AkRole::TreeItem);
        schema_node.set_label(schema_label);
        schema_node.set_children(table_ids);
        nodes.insert(schema_id, schema_node);
    }

    let mut nav_node = AkNode::new(AkRole::Tree);
    nav_node.set_label(format!("{} navigator", nav.db_name));
    nav_node.set_children(schema_ids);
    nodes.insert(nav_root_id, nav_node);

    // Window: Banner + Tree + Status (status added by with_status_bar)
    use crate::archive::actions::{ArchiveKeyMap, KeyMapMode};
    let status = ArchiveKeyMap::default_map().to_status_bar(KeyMapMode::Default);
    let (status_root_eid, status_pairs) = status.to_ak_nodes(10_000);
    for (eid, json) in status_pairs {
        nodes.insert(eid.0, accesskit::Node::from(json));
    }

    let window_id = AkNodeId::from(0u64);
    let mut window = AkNode::new(AkRole::Window);
    window.set_children(vec![header_id, nav_root_id, status_root_eid.0]);
    nodes.insert(window_id, window);

    let viewport = Viewport::new(800, 600);
    Ok(VerifiedTree::from_parts(nodes, window_id, viewport))
}

/// Build a [`VerifiedTree`] from a live [`crate::archive::ArchiveDbBackend`].
///
/// Queries the schema list, constructs a [`DatabaseDescriptor`] display root,
/// and runs the `ArchiveDisplay` → AccessKit → [`VerifiedTree`] pipeline.
/// The tree includes a Zellij-style keybinding status bar at the bottom.
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

    verified_tree_from_descriptor_with_bar(&db_desc)
}

/// Build a [`VerifiedTree`] from a pre-constructed [`DatabaseDescriptor`],
/// including the archive keybinding status bar.
///
/// Use this when a live connection is not available (e.g. demo mode or tests).
#[instrument(skip(desc))]
pub fn verified_tree_from_descriptor_with_bar(
    desc: &DatabaseDescriptor,
) -> ArchiveResult<VerifiedTree> {
    // ArchiveDisplay → AccessKit IR (id_base=1 so NodeId(0) is free for Window)
    let (root_eid, pairs) = desc.to_ak_nodes(&DatabaseDescriptorMode::Overview, 1);
    let content_root = root_eid.0;
    let nodes = convert_nodes(pairs);
    let viewport = Viewport::new(800, 600);
    Ok(with_status_bar(content_root, nodes, viewport))
}

/// Build a [`VerifiedTree`] from a pre-constructed [`DatabaseDescriptor`].
///
/// Use this when a live connection is not available (e.g. demo mode or tests).
/// This variant does **not** include the status bar and returns the raw content
/// tree; it is kept for tests that inspect the raw node structure.
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
/// Constructs a minimal [`DatabaseDescriptor`] for display purposes and
/// includes the archive keybinding status bar.
pub fn demo_verified_tree() -> ArchiveResult<VerifiedTree> {
    nav_tree_to_verified_tree(&NavTree::demo())
}
