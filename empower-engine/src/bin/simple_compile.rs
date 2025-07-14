use empower_node_graph::EmpowerNodeGraph;
use empower_node_graph::NodeType;
use empower_node_graph::EmpowerData;

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let new_node_type = NodeType::IntegerVariable;

    let node_1 = node_graph.add_node(NodeType::IntegerVariable);
    let node_2 = node_graph.add_node(NodeType::IntegerVariable);
    let node_3 = node_graph.add_node(NodeType::IntegerVariable);
    let node_4 = node_graph.add_node(NodeType::IntegerVariable);

    node_graph.add_connection(node_1.output_port_keys[0], node_2.input_port_keys[0]);
    node_graph.add_connection(node_2.output_port_keys[0], node_3.input_port_keys[0]);
    node_graph.add_connection(node_3.output_port_keys[0], node_4.input_port_keys[0]);

    node_graph.set_input_port_value(node_1.input_port_keys[0], EmpowerData::Integer(50));
    empower_engine::debug_compile(&mut node_graph);
}
