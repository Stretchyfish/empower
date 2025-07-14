use empower_node_graph::{EmpowerNodeGraph, NodeType, EmpowerData};

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let start_node_key = node_graph.add_node(NodeType::Start);

    println!("Nodes: {}", node_graph.nodes.len());
    println!("Ports IN: {}", node_graph.input_ports.len());
    println!("Ports OUT: {}", node_graph.output_ports.len());
    println!("Connections out: {}", node_graph.connections_out.len());
    println!("Connections in: {}", node_graph.connections_in.len());
}
