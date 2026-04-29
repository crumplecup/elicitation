//! AccessKit display for [`ColumnStats`].
use elicitation::{Elicit, KaniCompose};

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::ColumnStats;

use super::ArchiveDisplay;

/// Display strategies for a [`ColumnStats`].
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniCompose,
)]
pub enum ColumnStatsMode {
    /// A compact one-line summary row.
    #[default]
    Summary,
    /// An expanded group showing all planner statistics.
    Detailed,
}

impl ArchiveDisplay for ColumnStats {
    type Mode = ColumnStatsMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            ColumnStatsMode::Summary => Role(AkRole::Article),
            ColumnStatsMode::Detailed => Role(AkRole::Group),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        match mode {
            ColumnStatsMode::Summary => {
                let corr = self
                    .correlation
                    .map_or("n/a".to_string(), |c| format!("{c:.3}"));
                let label = format!(
                    "{}: null={:.1}% width={}B distinct={} corr={}",
                    self.column_name,
                    self.null_fraction * 100.0,
                    self.avg_width_bytes,
                    self.n_distinct,
                    corr,
                );
                nodes.push((
                    root_id,
                    NodeJson::new(Role(AkRole::Article)).with_label(label),
                ));
            }
            ColumnStatsMode::Detailed => {
                let corr = self
                    .correlation
                    .map_or("n/a".to_string(), |c| format!("{c:.6}"));
                let props: &[(&str, String)] = &[
                    ("column", self.column_name.clone()),
                    ("null_fraction", format!("{:.6}", self.null_fraction)),
                    ("avg_width_bytes", self.avg_width_bytes.to_string()),
                    ("n_distinct", format!("{}", self.n_distinct)),
                    ("correlation", corr),
                ];
                let child_ids: Vec<NodeId> = (0..props.len())
                    .map(|i| NodeId::from(id_base + 1 + i as u64))
                    .collect();
                let root = NodeJson::new(Role(AkRole::Group))
                    .with_label(format!("stats: {}", self.column_name))
                    .with_children(child_ids);
                for (i, (k, v)) in props.iter().enumerate() {
                    let id = NodeId::from(id_base + 1 + i as u64);
                    nodes.push((
                        id,
                        NodeJson::new(Role(AkRole::Cell)).with_label(format!("{k}: {v}")),
                    ));
                }
                nodes.insert(0, (root_id, root));
            }
        }

        (root_id, nodes)
    }
}
