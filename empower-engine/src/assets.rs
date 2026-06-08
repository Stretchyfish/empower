use std::collections::HashMap;

use crate::node_graph::NodeGraph;

pub type AssetId = i32;

pub struct Assets
{
    node_graphs: HashMap<AssetId, NodeGraph>,
}

impl Assets
{
    pub fn new() -> Self
    {
        Self
        {
            node_graphs: HashMap::new(),
        }
    }

    pub fn add_node_graph(&mut self, node_graph: NodeGraph) -> AssetId
    {
        let id = self.get_asset_id();
        self.node_graphs.insert(id, node_graph);
        id
    }

    pub fn get_node_graph(&self, id: &AssetId) -> Option<&NodeGraph>
    {
        self.node_graphs.get(id)
    }

    pub fn get_node_graph_mut(&mut self, id: &AssetId) -> Option<&mut NodeGraph>
    {
        self.node_graphs.get_mut(id)
    }

    fn get_asset_id(&self) -> AssetId
    {
        self.node_graphs.keys().max().unwrap_or(&0) + 1 // @TODO, this approach needs to get fixed later!
    }
    
}
