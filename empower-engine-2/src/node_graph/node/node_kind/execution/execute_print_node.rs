use crate::{NodeGraph, NodeGraphKey, NodeKind};

pub fn execute_print_node(node_key: &NodeGraphKey, node_graph: &mut NodeGraph) -> bool
{
    let node_to_execute;
    match node_graph.nodes.get_mut(node_key)
    {
        Some( value ) => node_to_execute = value,
        None => return false,
    }

    if node_to_execute.kind != NodeKind::Print { return false; }
    if node_to_execute.input_port_keys.len() != 2 { return false; }
    if node_to_execute.output_port_keys.len() != 0 { return false; }

    let input_port_2 = node_graph.input_ports.get_mut(&node_to_execute.input_port_keys[1]).unwrap();

    let printing_output = format!("PRINTING: {}", input_port_2.value.to_string());
    
    println!("{}", printing_output);

    true
}
