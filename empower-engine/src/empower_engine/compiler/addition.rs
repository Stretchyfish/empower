use empower_node_graph::port::input_port;
use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerData;
use empower_node_graph::Node;
use empower_node_graph::NodeType;
use empower_node_graph::InputPort;
use empower_node_graph::OutputPort;
use std::collections::HashMap;

pub fn execute_debug_addition_node(node_to_compile_key: EmpowerKey, nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>)
{
    let node_to_compile = nodes.get_mut(&node_to_compile_key).unwrap(); // @TODO, handle exception

    if node_to_compile.node_type != NodeType::Addition
    {
        println!("Asked to execute addition node, but its type does't match, request will be ignored");
        return;
    }

    if node_to_compile.input_port_keys.len() != 2
    {
        println!("Asked to execute addition node, but its input ports size is invalid, request will be ignored");
        return;
    }

    if node_to_compile.output_port_keys.len() != 1
    {
        println!("Asked to execute addition node, but its output ports size is invalid, request will be ignored");
        return;
    }

    let input_port_addition = input_ports.get(&node_to_compile.input_port_keys[0]).unwrap(); // @TODO, handle exception
    let input_port_value = input_ports.get(&node_to_compile.input_port_keys[1]).unwrap(); // @TODO, handle exception

    // @TODO, make this code look better
    let mut addition_value = 0;
    if let EmpowerData::Integer(int_value) = input_port_addition.value
    {
        addition_value = int_value;
    }

    let mut value = 0;
    if let EmpowerData::Integer(int_value) = input_port_value.value
    {
        value = int_value;
    }

    let added_value = value + addition_value;

    let output_port = output_ports.get_mut(&node_to_compile.output_port_keys[0]).unwrap();
    output_port.value = EmpowerData::Integer(added_value);

    print!("addition node ({}) : [{}] [{}], [{}]", node_to_compile_key, addition_value, value, output_port.value);
}
