use empower_node_graph::EmpowerNodeGraph;
use empower_node_graph::NodeType;
use empower_node_graph::EmpowerData;

fn main()
{
    let mut node_graph = EmpowerNodeGraph::default();

    let new_node_type_1 = NodeType::IntegerVariable;
    let new_node_type_2 = NodeType::Print;

    let node_key_1 = node_graph.add_node(new_node_type_1.clone());
    let node_key_2 = node_graph.add_node(new_node_type_2.clone());

    let input_port_key_1= node_graph.get_node_input_port_keys(node_key_1).unwrap()[0];
    let output_port_key_1= node_graph.get_node_output_port_keys(node_key_1).unwrap()[0];

    let input_port_key_2= node_graph.get_node_input_port_keys(node_key_2).unwrap()[1];

    node_graph.add_connection(input_port_key_2, output_port_key_1);

    node_graph.set_input_port_value(input_port_key_1, EmpowerData::Integer(50));
    empower_engine::debug_compile(&mut node_graph);
}
