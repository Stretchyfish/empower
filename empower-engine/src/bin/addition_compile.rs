use empower_node_graph::{node, EmpowerData, EmpowerNodeGraph, NodeType};

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let int_node = node_graph.add_node(NodeType::IntegerVariable);
    let add_node = node_graph.add_node(NodeType::Addition);

    node_graph.set_input_port_value(int_node.input_port_keys[0], EmpowerData::Integer(10));
    node_graph.set_input_port_value(add_node.input_port_keys[0], EmpowerData::Integer(33));

    node_graph.add_connection(int_node.output_port_keys[0], add_node.input_port_keys[1]);

    empower_engine::debug_compile(&mut node_graph);

    println!("Nodes: {}", node_graph.nodes.len());
    println!("Ports IN: {}", node_graph.input_ports.len());
    println!("Ports OUT: {}", node_graph.output_ports.len());
    println!("Connections out: {}", node_graph.connections_out.len());
    println!("Connections in: {}", node_graph.connections_in.len());
}
