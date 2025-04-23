use empower_engine::EmpowerEngine;

fn main()
{
    let mut engine = EmpowerEngine::new();

    let new_node_type = empower_engine::node::NodeType::Integer;

    let node_key_1 = engine.add_node(new_node_type.clone());
    let node_key_2 = engine.add_node(new_node_type);

    let input_port_key_1= engine.get_node_input_port_keys(node_key_1).unwrap()[0];
    let output_port_key_1= engine.get_node_output_port_keys(node_key_1).unwrap()[0];

    let output_port_key_2 = engine.get_node_output_port_keys(node_key_1).unwrap()[0];
    let input_port_key_2= engine.get_node_input_port_keys(node_key_2).unwrap()[0];

    engine.add_connection(input_port_key_2, output_port_key_2);

    engine.set_input_port_value(input_port_key_1, 50);
    engine.get_input_port_value(input_port_key_1);

    engine.get_input_port_value(input_port_key_2);
    engine.compile();
    engine.get_output_port_value(output_port_key_1);
    engine.get_input_port_value(input_port_key_2);
}
