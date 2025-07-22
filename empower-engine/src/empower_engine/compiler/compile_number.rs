use empower_node_graph::node::NodeState;
use empower_node_graph::node::NumberNodeState;
use empower_node_graph::node::node_state::number_node_state::ConvertionApproach;
use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerData;
use empower_node_graph::Node;
use empower_node_graph::NodeType;
use empower_node_graph::InputPort;
use empower_node_graph::OutputPort;
use std::collections::HashMap;

pub fn execute_debug_number_node(node_to_compile_key: EmpowerKey, nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>)
{
    let node_to_compile = nodes.get_mut(&node_to_compile_key).unwrap(); // @TODO, handle exception

    if node_to_compile.node_type != NodeType::Number
    {
        println!("Asked to execute number node, but its type does't match, request will be ignored");
        return;
    }

    if node_to_compile.input_port_keys.len() != 1
    {
        println!("Asked to execute number node, but its input ports size is invalid, request will be ignored");
        return;
    }

    if node_to_compile.output_port_keys.len() != 1
    {
        println!("Asked to execute number node, but its output ports size is invalid, request will be ignored");
        return;
    }

    let mut number_node_state = NumberNodeState::new();
    match &node_to_compile.state 
    {
        NodeState::NumberState( state ) =>
        {
            number_node_state = state.clone();
        }
        _ =>
        {
            println!("ERROR, asked to compile number node, which does not have the correct state");
            return;
        }
    }

    let input_port = input_ports.get_mut(&node_to_compile.input_port_keys[0]).unwrap(); // @TODO, handle exception

    let mut port_value = EmpowerData::Integer(0);
    let mut successfully_read_value = false;
    match input_port.value
    {
        EmpowerData::Undefined( ref text ) =>
        {
            let parsed_float = text.parse::<f32>();

            if parsed_float.is_ok()
            {
                port_value = EmpowerData::Float( parsed_float.unwrap() );
                successfully_read_value = true;
            }

            let parsed_integer = text.parse::<i32>();

            if parsed_integer.is_ok()
            {
                port_value = EmpowerData::Integer( parsed_integer.unwrap() );
                successfully_read_value = true; // @TODO, find a way to not do a double check here
            }
        },
        EmpowerData::Float( float ) =>
        {
            port_value = EmpowerData::Float( float ); // @TODO, this can be simplified
            successfully_read_value = true;
        },
        EmpowerData::Integer( int ) =>
        {
            port_value = EmpowerData::Integer( int );
            successfully_read_value = true;
        }
        _ =>
        {
            
        }
    }

    if !successfully_read_value
    {
        println!("Unable to read value in numbers");
        return;
    }

    let output_port = output_ports.get_mut(&node_to_compile.output_port_keys[0]).unwrap();
    output_port.value = port_value;

    print!("number node ({}) : [{}] , [{}]", node_to_compile_key, input_port.value, output_port.value);
}

fn automatically_convert_input_port(input_port: &InputPort, successfully_read_value: &mut bool) -> EmpowerData
{
    let mut port_value = EmpowerData::Integer(0);
    match input_port.value
    {
        EmpowerData::Undefined( ref text ) =>
        {
            let parsed_float = text.parse::<f32>();

            if parsed_float.is_ok()
            {
                port_value = EmpowerData::Float( parsed_float.unwrap() );
                *successfully_read_value = true;
            }

            let parsed_integer = text.parse::<i32>();

            if parsed_integer.is_ok()
            {
                port_value = EmpowerData::Integer( parsed_integer.unwrap() );
                *successfully_read_value = true; // @TODO, find a way to not do a double check here
            }
        },
        EmpowerData::Float( float ) =>
        {
            port_value = EmpowerData::Float( float ); // @TODO, this can be simplified
            *successfully_read_value = true;
        },
        EmpowerData::Integer( int ) =>
        {
            port_value = EmpowerData::Integer( int );
            *successfully_read_value = true;
        }
        _ =>
        {
            
        }
    }

    port_value
}