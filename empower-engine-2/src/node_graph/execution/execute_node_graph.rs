use crate::{NodeGraph, NodeGraphKey};
use std::collections::VecDeque;

use crate::node_graph::node::node_kind;

pub fn execute_node_graph(node_graph: &mut NodeGraph) -> bool
{
    if node_graph.node_count() == 0 { return false; }

    let mut node_keys_to_execute_queue: VecDeque<NodeGraphKey> = VecDeque::new();

    let start_node_key = 1; // @TODO, find a better approach
    node_keys_to_execute_queue.push_back( start_node_key );
 
    while !node_keys_to_execute_queue.is_empty()
    {
        let node_to_execute_key: NodeGraphKey = node_keys_to_execute_queue[0];

        node_kind::execute_node(&node_to_execute_key, node_graph);

        node_graph.distribute_outputs(&node_to_execute_key);

        node_keys_to_execute_queue.pop_front();
    }

    true
}