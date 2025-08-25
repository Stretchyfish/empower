use crate::{NodeGraph, NodeGraphKey, NodeKind};

mod execute_start_node;

pub fn execute_node(node_graph: &mut NodeGraph, node_key: &NodeGraphKey) -> bool
{
    if !node_graph.contains_node(node_key)
    {
        return false;
    }

    let node_to_execute;
    match node_graph.get_node(node_key)
    {
        Some( value ) => node_to_execute = value,
        None => return false,
    }

    match node_to_execute.kind
    {
        NodeKind::Start => execute_start_node::execute_start_node(node_graph, node_key), 
        _ => todo!(),
    }

    true
}
