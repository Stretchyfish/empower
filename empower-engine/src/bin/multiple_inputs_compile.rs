use empower_node_graph::{node, EmpowerData, EmpowerNodeGraph, NodeType};

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let start_node = node_graph.add_node(NodeType::Start);
    let number_node = node_graph.add_node(NodeType::Number);
    let print_node = node_graph.add_node(NodeType::Print);

    node_graph.set_input_port_value(number_node.input_port_keys[0], EmpowerData::Float(10.0));
    node_graph.add_connection(start_node.output_port_keys[0], print_node.input_port_keys[0]);
    node_graph.add_connection(number_node.output_port_keys[0], print_node.input_port_keys[1]);

    empower_engine::debug_compile(&mut node_graph);

    println!("Nodes: {}", node_graph.nodes.len());
    println!("Ports IN: {}", node_graph.input_ports.len());
    println!("Ports OUT: {}", node_graph.output_ports.len());
    println!("Connections out: {}", node_graph.connections_out.len());
    println!("Connections in: {}", node_graph.connections_in.len());
}
