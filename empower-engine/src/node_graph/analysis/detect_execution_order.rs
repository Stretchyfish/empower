use crate::{node_graph, NodeGraph, NodeGraphKey};

use super::detect_rouge_nodes;

// @TODO, consider finding a way of making the node graph not mut
pub fn detect_execution_order(node_graph: &mut NodeGraph) -> Vec<NodeGraphKey>
{
    let mut node_execution_order = Vec::new();

    let detected_rouge_nodes = detect_rouge_nodes(node_graph);
    node_execution_order.extend(detected_rouge_nodes);

    let start_node_key = 1; // @TODO, find a better approach for this!

    node_execution_order.push(start_node_key);

    let mut number_of_nodes_checked = 0;

    // @TODO, find a smarter check than 1000
    let max_number_of_checks = 1000;

    while number_of_nodes_checked != node_execution_order.len() && number_of_nodes_checked < max_number_of_checks
    {
        // @TODO, handle this result better
        let next_nodes_to_execute = node_graph.distribute_outputs(&node_execution_order[number_of_nodes_checked]);

        node_execution_order.extend(next_nodes_to_execute.unwrap());

        number_of_nodes_checked += 1;
    }

    node_execution_order
}