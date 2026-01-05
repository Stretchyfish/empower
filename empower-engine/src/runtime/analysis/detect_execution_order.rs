use std::collections::HashSet;

use crate::{NodeGraph, NodeGraphKey, PortValue};

pub fn detect_execution_order(entry_node: &NodeGraphKey, node_graph: &NodeGraph) -> Vec<NodeGraphKey>
{
    let mut node_execution_order = Vec::new();

    node_execution_order.push(*entry_node);

    let mut cached_output_ports: HashSet<NodeGraphKey> = HashSet::new();

    let mut node_check_index = 0;
    while node_check_index < node_execution_order.len()
    {
        // @TODO, consider a placing this elsewhere!
        let mut potential_new_nodes_to_execute: Vec<NodeGraphKey> = Vec::new();

        // @TODO, this could be optimized, by reusing potential and cached across multiple nodes?
        determine_is_node_is_ready_for_exeuction(node_execution_order[node_check_index], node_graph, &mut potential_new_nodes_to_execute, &mut cached_output_ports);

        if !potential_new_nodes_to_execute.is_empty()
        {
            node_execution_order.splice(node_check_index..node_check_index, potential_new_nodes_to_execute);
            continue;
        }
        
        let next_nodes_to_execute = get_node_trigger_connections(node_execution_order[node_check_index], node_graph);
        node_execution_order.extend(next_nodes_to_execute);

        // Cache all input ports
        let executed_node_handle = node_graph.get_node_handle(&node_execution_order[node_check_index]);

        for output_port_key in executed_node_handle.output_port_keys
        {
            cached_output_ports.insert(output_port_key);
        }


        node_check_index += 1;
    }
    
    node_execution_order
}

// This function is written with a unique set of inputs to make it recursible
// @TODO, rewrite this with a helper function later.
pub fn determine_is_node_is_ready_for_exeuction(node_key: NodeGraphKey, node_graph: &NodeGraph, new_nodes_to_execute: &mut Vec<NodeGraphKey>, cached_output_ports: &mut HashSet<NodeGraphKey>)
{
    let node_handle = node_graph.get_node_handle(&node_key);

    if node_handle.input_port_keys.is_empty()
    {
        return;
    }

    for input_port_key in node_handle.input_port_keys
    {
        if !node_graph.input_port_has_connection(&input_port_key)
        {
            continue;
        }

        let connected_output_port = node_graph.get_input_port_connection_key(&input_port_key).unwrap();

        if cached_output_ports.contains(connected_output_port)
        {
            continue;
        }

        let connecte_node = node_graph.output_ports.get(connected_output_port).unwrap().node_key;

        determine_is_node_is_ready_for_exeuction(connecte_node, node_graph, new_nodes_to_execute, cached_output_ports);
        new_nodes_to_execute.push(connecte_node); // important that this is done after the recursive behavior for the correct order
    }
}

pub fn get_node_trigger_connections(node_key: NodeGraphKey, node_graph: &NodeGraph) -> Vec<NodeGraphKey>
{
    let mut trigger_connections: Vec<NodeGraphKey> = Vec::new();

    let node_handle = node_graph.get_node_handle(&node_key);

    for output_port_key in node_handle.output_port_keys
    {
        let output_port = node_graph.get_output_port(&output_port_key).unwrap();

        if !output_port.compatability.is_port_value_type( &PortValue::Trigger(true) )
        {
            continue;
        }

        if !node_graph.output_port_has_connection(&output_port_key)
        {
            continue;
        }

        let connected_input_ports = node_graph.get_output_port_connection_keys(&output_port_key).unwrap();

        // @TODO, consider finding a smarter way of doing this
        // @TODO, consider reserving
        for connected_input_port in connected_input_ports
        {
            let input_port = node_graph.get_input_port(connected_input_port).unwrap();
            trigger_connections.push( input_port.node_key );
        }
    }
    
    trigger_connections
}
