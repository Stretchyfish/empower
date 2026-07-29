use crate::assets::AssetId;
use super::NodeGraphKey;

#[derive(Clone)]
pub struct NodeAddress
{
    pub graph_id: AssetId,
    pub node_key: NodeGraphKey,
}

