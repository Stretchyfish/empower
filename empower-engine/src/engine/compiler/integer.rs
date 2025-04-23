use crate::EmpowerKey;
use crate::Node;
use crate::NodeType;
use crate::InputPort;
use crate::OutputPort;
use std::collections::HashMap;

pub fn execute_integer_node(node_to_compile_key: EmpowerKey, nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>)
{
    let node_to_compile = nodes.get_mut(&node_to_compile_key).unwrap(); // @TODO, handle exception

    if node_to_compile.node_type != NodeType::Integer
    {
        println!("Asked to execute integer node, but its type does't match, request will be ignored");
        return;
    }

    if node_to_compile.input_port_keys.len() != 1
    {
        println!("Asked to execute integer node, but its input ports size is invalid, request will be ignored");
        return;
    }

    if node_to_compile.output_port_keys.len() != 1
    {
        println!("Asked to execute integer node, but its output ports size is invalid, request will be ignored");
        return;
    }

    let input_port = input_ports.get_mut(&node_to_compile.input_port_keys[0]).unwrap(); // @TODO, handle exception

    let output_port = output_ports.get_mut(&node_to_compile.output_port_keys[0]).unwrap();
    output_port.value = input_port.value.clone();
}

pub fn execute_debug_integer_node(node_to_compile_key: EmpowerKey, nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>)
{
    let node_to_compile = nodes.get_mut(&node_to_compile_key).unwrap(); // @TODO, handle exception

    if node_to_compile.node_type != NodeType::Integer
    {
        println!("Asked to execute integer node, but its type does't match, request will be ignored");
        return;
    }

    if node_to_compile.input_port_keys.len() != 1
    {
        println!("Asked to execute integer node, but its input ports size is invalid, request will be ignored");
        return;
    }

    if node_to_compile.output_port_keys.len() != 1
    {
        println!("Asked to execute integer node, but its output ports size is invalid, request will be ignored");
        return;
    }

    let input_port = input_ports.get_mut(&node_to_compile.input_port_keys[0]).unwrap(); // @TODO, handle exception

    let output_port = output_ports.get_mut(&node_to_compile.output_port_keys[0]).unwrap();
    output_port.value = input_port.value.clone();


    print!("int node ({}) : [{}] , [{}]", node_to_compile_key, input_port.value, output_port.value);
}
