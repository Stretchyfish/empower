use std::collections::HashMap;
use better_empower_engine::{NodeGraph, NodeGraphKey};

pub mod display_node;
pub use display_node::DisplayNode;

pub struct GraphEditor
{
    pub node_graph: NodeGraph,
    pub display_nodes: HashMap<NodeGraphKey, DisplayNode>,
}

impl GraphEditor
{
    pub fn new() -> Self
    {
       Self
       {
        node_graph: NodeGraph::new(),
        display_nodes: HashMap::new(),
       } 
    }

    pub fn add_node(&mut self, node_kind: &'static str, position: egui::Pos2) -> bool
    {
        let node_handle= self.node_graph.add_node(&node_kind); 
        let node = self.node_graph.get_node(&node_handle.node_key).unwrap();

        let display_node_title = node.kind.name();

        let display_node = DisplayNode::new(display_node_title);

        self.display_nodes.insert(node_handle.node_key, display_node);

        true
    }
 
}