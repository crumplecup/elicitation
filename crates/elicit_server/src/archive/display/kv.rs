//! AccessKit display for KV descriptor types.
//!
//! Covers [`KvTableDescriptor`], [`KvScanResult`], [`KvSnapshotDescriptor`],
//! and [`KvStatsDescriptor`].

use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::{
    KvEntryDescriptor, KvScanResult, KvSnapshotDescriptor, KvStatsDescriptor, KvTableDescriptor,
};

use super::ArchiveDisplay;

// ── KvTableDescriptor ─────────────────────────────────────────────────────────

/// Display strategies for a [`KvTableDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
)]
pub enum KvTableDescriptorMode {
    /// One-line summary card: name + entry count.
    #[default]
    Summary,
}

impl ArchiveDisplay for KvTableDescriptor {
    type Mode = KvTableDescriptorMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::Article)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let label = format!("{} ({} entries)", self.name, self.entry_count);
        let root = NodeJson::new(Role(AkRole::Article)).with_label(label);
        (root_id, vec![(root_id, root)])
    }
}

// ── KvEntryDescriptor ─────────────────────────────────────────────────────────

/// Display strategies for a [`KvEntryDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
)]
pub enum KvEntryDescriptorMode {
    /// Key = value on one row.
    #[default]
    Row,
}

impl ArchiveDisplay for KvEntryDescriptor {
    type Mode = KvEntryDescriptorMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::Row)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let label = format!("{} = {}", self.key, self.value);
        let root = NodeJson::new(Role(AkRole::Row)).with_label(label);
        (root_id, vec![(root_id, root)])
    }
}

// ── KvScanResult ──────────────────────────────────────────────────────────────

/// Display strategies for a [`KvScanResult`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
)]
pub enum KvScanResultMode {
    /// Data grid: one row per entry.
    #[default]
    DataGrid,
    /// Summary: table name + count.
    Summary,
}

#[cfg(not(kani))]
impl ArchiveDisplay for KvScanResult {
    type Mode = KvScanResultMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            KvScanResultMode::DataGrid => Role(AkRole::Grid),
            KvScanResultMode::Summary => Role(AkRole::Article),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        match mode {
            KvScanResultMode::DataGrid => {
                // Header row with "Key" / "Value" column headers.
                let header_id = NodeId::from(id_base + 1);
                let key_hdr_id = NodeId::from(id_base + 2);
                let val_hdr_id = NodeId::from(id_base + 3);

                let key_hdr = NodeJson::new(Role(AkRole::ColumnHeader)).with_label("Key".into());
                let val_hdr = NodeJson::new(Role(AkRole::ColumnHeader)).with_label("Value".into());
                let header_row = NodeJson::new(Role(AkRole::Row))
                    .with_label("column headers".into())
                    .with_children(vec![key_hdr_id, val_hdr_id]);

                nodes.push((key_hdr_id, key_hdr));
                nodes.push((val_hdr_id, val_hdr));
                nodes.push((header_id, header_row));

                let row_base = id_base + 4;
                let row_ids: Vec<NodeId> = self
                    .entries
                    .iter()
                    .enumerate()
                    .map(|(i, _)| NodeId::from(row_base + i as u64))
                    .collect();

                for (i, entry) in self.entries.iter().enumerate() {
                    let row_node = NodeJson::new(Role(AkRole::Row))
                        .with_label(format!("{} = {}", entry.key, entry.value));
                    nodes.push((NodeId::from(row_base + i as u64), row_node));
                }

                let mut all_children = vec![header_id];
                all_children.extend_from_slice(&row_ids);

                let page_label = if self.total_count > self.entries.len() as u64 {
                    format!(
                        "{} — {} entries (showing {}-{} of {})",
                        self.table_name,
                        self.total_count,
                        self.offset + 1,
                        self.offset + self.entries.len() as u64,
                        self.total_count,
                    )
                } else {
                    format!("{} — {} entries", self.table_name, self.entries.len())
                };

                let root = NodeJson::new(Role(AkRole::Grid))
                    .with_label(page_label)
                    .with_children(all_children);
                nodes.insert(0, (root_id, root));
            }
            KvScanResultMode::Summary => {
                let label = format!("{}: {} entries total", self.table_name, self.total_count);
                let root = NodeJson::new(Role(AkRole::Article)).with_label(label);
                nodes.push((root_id, root));
            }
        }

        (root_id, nodes)
    }
}

// ── KvSnapshotDescriptor ──────────────────────────────────────────────────────

/// Display strategies for a [`KvSnapshotDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
)]
pub enum KvSnapshotDescriptorMode {
    /// One-line list item: name + id.
    #[default]
    ListItem,
}

impl ArchiveDisplay for KvSnapshotDescriptor {
    type Mode = KvSnapshotDescriptorMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::ListItem)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let label = format!("{} (id: {})", self.name, self.id);
        let root = NodeJson::new(Role(AkRole::ListItem)).with_label(label);
        (root_id, vec![(root_id, root)])
    }
}

// ── KvStatsDescriptor ─────────────────────────────────────────────────────────

/// Display strategies for a [`KvStatsDescriptor`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
)]
pub enum KvStatsDescriptorMode {
    /// Stat list: one item per metric.
    #[default]
    StatList,
}

impl ArchiveDisplay for KvStatsDescriptor {
    type Mode = KvStatsDescriptorMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::Article)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);

        let stats = [
            ("path", self.path.clone()),
            ("stored_bytes", self.stored_bytes.to_string()),
            ("fragmented_bytes", self.fragmented_bytes.to_string()),
            ("metadata_bytes", self.metadata_bytes.to_string()),
            ("table_count", self.table_count.to_string()),
            (
                "cache_hit_ratio",
                format!("{:.1}%", self.cache_hit_ratio * 100.0),
            ),
        ];

        let child_ids: Vec<NodeId> = stats
            .iter()
            .enumerate()
            .map(|(i, _)| NodeId::from(id_base + 1 + i as u64))
            .collect();

        let mut nodes = Vec::new();
        for (i, (k, v)) in stats.iter().enumerate() {
            let item = NodeJson::new(Role(AkRole::ListItem)).with_label(format!("{k}: {v}"));
            nodes.push((NodeId::from(id_base + 1 + i as u64), item));
        }

        let root = NodeJson::new(Role(AkRole::Article))
            .with_label(format!("KV stats — {}", self.path))
            .with_children(child_ids);
        nodes.insert(0, (root_id, root));

        (root_id, nodes)
    }
}
