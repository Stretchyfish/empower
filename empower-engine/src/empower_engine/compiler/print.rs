use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerData;
use empower_node_graph::Node;
use empower_node_graph::NodeType;
use empower_node_graph::InputPort;
use empower_node_graph::OutputPort;
use std::collections::HashMap;

use crate::empower_engine::empower_result::EmpowerResult;

pub fn execute_debug_print_node(node_to_compile_key: EmpowerKey, nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>, empower_result: &mut EmpowerResult)
{
    let node_to_compile = nodes.get_mut(&node_to_compile_key).unwrap(); // @TODO, handle exception

    if node_to_compile.node_type != NodeType::Print
    {
        println!("Asked to execute print node, but its type does't match, request will be ignored");
        return;
    }

    if node_to_compile.input_port_keys.len() != 2
    {
        println!("Asked to execute print node, but its input ports size is invalid, request will be ignored");
        return;
    }

    if node_to_compile.output_port_keys.len() != 0
    {
        println!("Asked to execute print node, but its output ports size is invalid, request will be ignored");
        return;
    }

    let input_port = input_ports.get_mut(&node_to_compile.input_port_keys[1]).unwrap(); // @TODO, handle exception

    // @TODO, make a better implementation of this
    //
    //
    //
    let printing_output = format!("PRINTING: {}", input_port.value.to_string());

    println!("{}", printing_output);
    empower_result.add_line(input_port.value.to_string());

    print!("print node ({}) : [{}] ", node_to_compile_key, input_port.value);
}
