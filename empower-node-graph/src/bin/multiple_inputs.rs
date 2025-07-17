use empower_node_graph::{EmpowerNodeGraph, NodeType, EmpowerData};

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let start_node = node_graph.add_node(NodeType::Start);

    let number_node = node_graph.add_node(NodeType::Number);
    let print_node = node_graph.add_node(NodeType::Print);

    node_graph.add_connection(start_node.output_port_keys[0], print_node.input_port_keys[0]);
    node_graph.add_connection(number_node.output_port_keys[0], print_node.input_port_keys[1]);
}
