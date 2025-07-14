use empower_node_graph::EmpowerNodeGraph;
use empower_node_graph::NodeType;
use empower_node_graph::EmpowerData;

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let start_node = node_graph.add_node(NodeType::Start);
    let int_node = node_graph.add_node(NodeType::IntegerVariable);
    let print_node = node_graph.add_node(NodeType::Print);

    node_graph.set_input_port_value(int_node.input_port_keys[0], EmpowerData::Integer(50));

    node_graph.add_connection(int_node.output_port_keys[0], print_node.input_port_keys[1]);
    node_graph.add_connection(start_node.output_port_keys[0], print_node.input_port_keys[0]);

    empower_engine::debug_compile(&mut node_graph);
}
