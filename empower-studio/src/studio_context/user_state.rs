use empower_engine::{assets::AssetId, node_graph::NodeGraphKey};

#[derive(Clone, PartialEq, Eq)]
pub enum UserState
{
    None,
    HighlightingNode { graph_id: AssetId, node_key: NodeGraphKey },
}
