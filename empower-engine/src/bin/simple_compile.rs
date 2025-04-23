use empower_engine::EmpowerEngine;

fn main()
{
    let mut engine = EmpowerEngine::new();

    let new_node_type = empower_engine::node::NodeType::Integer;

    let node_key_1 = engine.add_node(new_node_type.clone());
    let node_key_2 = engine.add_node(new_node_type.clone());
    let node_key_3 = engine.add_node(new_node_type.clone());
    let node_key_4 = engine.add_node(new_node_type.clone());

    let input_port_key_1= engine.get_node_input_port_keys(node_key_1).unwrap()[0];
    let output_port_key_1= engine.get_node_output_port_keys(node_key_1).unwrap()[0];

    let input_port_key_2= engine.get_node_input_port_keys(node_key_2).unwrap()[0];
    let output_port_key_2= engine.get_node_output_port_keys(node_key_2).unwrap()[0];

    let input_port_key_3= engine.get_node_input_port_keys(node_key_3).unwrap()[0];
    let output_port_key_3= engine.get_node_output_port_keys(node_key_3).unwrap()[0];

    let output_port_key_4 = engine.get_node_output_port_keys(node_key_4).unwrap()[0];
    let input_port_key_4= engine.get_node_input_port_keys(node_key_4).unwrap()[0];

    engine.add_connection(input_port_key_2, output_port_key_1);
    engine.add_connection(input_port_key_3, output_port_key_2);
    engine.add_connection(input_port_key_4, output_port_key_2);

    engine.set_input_port_value(input_port_key_1, 50);
    engine.debug_compile();
}
