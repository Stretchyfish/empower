use crate::{node_graph::PortValue, NodeGraph, NodeGraphKey};

pub fn detect_rouge_nodes(node_graph: &NodeGraph) -> Vec<NodeGraphKey>
{
    let mut rouge_nodes = Vec::new();

    for (node_key, node) in node_graph.nodes.iter()
    {
        let mut node_contains_trigger = false;

        let node_input_port_keys = &node.input_port_keys;

        let mut node_has_connections = false;

        for input_port_key in node_input_port_keys
        {
            if node_graph.connections_in.contains_key(input_port_key)
            {
                node_has_connections = true;
                break;
            }

            let input_port = match node_graph.input_ports.get(input_port_key)
            {
                Some( input_port ) => input_port,
                None => panic!("In detect rouge nodes, requested to get input port {}, that does not exist", input_port_key),
            };

            if input_port.value == PortValue::Trigger
            {
                node_contains_trigger = true;
                break;
            }
        }

        if node_has_connections
        {
            continue;
        }

        for output_port_key in &node.output_port_keys
        {
            let output_port = match node_graph.output_ports.get(output_port_key)
            {
                Some( output_port ) => output_port,
                None => panic!("In detect rouge nodes, requested to get input port {}, that does not exist", output_port_key),
            };

            if output_port.value == PortValue::Trigger
            {
                node_contains_trigger = true;
                break;
            }
        }

        if !node_contains_trigger
        {
            rouge_nodes.push(node_key.clone());
        }
    }

    rouge_nodes
}