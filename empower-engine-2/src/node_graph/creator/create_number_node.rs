use crate::node_graph::{Node, NodeGraph, NodeGraphKey, NodeType, OutputPort, NodeHandle};

pub fn create_number_node(node_graph: &mut NodeGraph) -> NodeHandle
{
    let new_node_key = node_graph.get_available_node_key();


    NodeHandle::empty()
}