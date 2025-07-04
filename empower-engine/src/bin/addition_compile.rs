use empower_node_graph::{EmpowerData, EmpowerNodeGraph};

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let new_node_type = empower_node_graph::NodeType::IntegerVariable;
    let node1_key = node_graph.add_node(new_node_type);
    println!("Node 1 key: {}", node1_key);

    let new_addition_node_type= empower_node_graph::NodeType::Addition;
    let node2_key = node_graph.add_node(new_addition_node_type);
    println!("Node 2 key: {}", node2_key);

    let node_1_input_port_keys = node_graph.get_node_input_port_keys(node1_key).unwrap();
    let node_1_output_port_keys = node_graph.get_node_output_port_keys(node1_key).unwrap();

    let node_2_input_port_keys = node_graph.get_node_input_port_keys(node2_key).unwrap();

    node_graph.set_input_port_value(node_1_input_port_keys[0], EmpowerData::Integer(10));
    node_graph.set_input_port_value(node_2_input_port_keys[0], EmpowerData::Integer(33));

    // node_graph.add_connection(node_1_output_port_keys[1], node_2_input_port_keys[0]);
    node_graph.add_connection(node_2_input_port_keys[1], node_1_output_port_keys[0]);

    empower_engine::debug_compile(&mut node_graph);

    println!("Nodes: {}", node_graph.nodes.len());
    println!("Ports IN: {}", node_graph.input_ports.len());
    println!("Ports OUT: {}", node_graph.output_ports.len());
    println!("Connections out: {}", node_graph.connections_out.len());
    println!("Connections in: {}", node_graph.connections_in.len());
}
