use crate::node_graph::{Node, NodeGraph, NodeGraphKey, NodeType, OutputPort, NodeHandle};

pub fn create_start_node(node_graph: &mut NodeGraph) -> NodeHandle
{
    let new_node_key = node_graph.get_available_node_key();

    let new_output_port_key = node_graph.get_available_output_port_key();
    let new_output_port = OutputPort::new();

    node_graph.output_ports.insert(new_output_port_key, new_output_port);

    let new_output_port_keys = Vec::from([new_output_port_key]);

    let new_node = Node::new(new_node_key, NodeType::Start, Vec::new(), new_output_port_keys.clone());

    node_graph.nodes.insert(new_node_key, new_node);

    NodeHandle::new(new_node_key, Vec::new(), new_output_port_keys)
}

