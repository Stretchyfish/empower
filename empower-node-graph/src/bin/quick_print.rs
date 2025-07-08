use empower_node_graph::EmpowerNodeGraph;

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let integer_node_type = empower_node_graph::NodeType::IntegerVariable;
    let print_node_type = empower_node_graph::NodeType::Print;

    let node1_key = node_graph.add_node(integer_node_type);
    let node2_key = node_graph.add_node(print_node_type);

    let node_1_output_port_keys = node_graph.get_node_output_port_keys(node1_key).unwrap();
    let node_2_input_port_keys = node_graph.get_node_input_port_keys(node2_key).unwrap();

    // node_graph.add_connection(node_1_output_port_keys.unwrap()[0], node_2_input_port_keys.unwrap()[0]);
    node_graph.add_connection(node_2_input_port_keys[1], node_1_output_port_keys[0]);

    println!("Nodes: {}", node_graph.nodes.len());
    println!("Ports IN: {}", node_graph.input_ports.len());
    println!("Ports OUT: {}", node_graph.output_ports.len());
    println!("Connections out: {}", node_graph.connections_out.len());
    println!("Connections in: {}", node_graph.connections_in.len());
}
