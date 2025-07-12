use std::collections::HashMap;

use crate::EmpowerKey;
use crate::EmpowerData;
use crate::NodeType;
use crate::InputPort;
use crate::OutputPort;
use crate::Node;

pub fn create_start_node(nodes: &mut HashMap<EmpowerKey, Node>, output_ports: &mut HashMap<EmpowerKey, OutputPort>)
{
    let new_node_key = nodes.len() as EmpowerKey;
    
    if new_node_key > 0
    {
        println!("More than one start node has been requested, which is not allowed, will not add");
    }

    let new_output_port_key = output_ports.len() as EmpowerKey;
    let new_output_port = OutputPort::new(new_output_port_key, EmpowerData::Trigger);

    output_ports.insert(new_output_port_key, new_output_port);

    let new_output_port_keys = Vec::from([new_output_port_key]);
    let new_node = Node::new(new_node_key, NodeType::Start, Vec::new(), new_output_port_keys);

    nodes.insert(new_node_key, new_node);
}

pub fn create_integer_node(nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>) -> EmpowerKey
{
    // @TODO, find a way of reusing values between max
    let new_node_key = nodes.keys().copied().max().unwrap_or(0) + 1;

    let new_input_port_key = input_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_input_port = InputPort::new(new_input_port_key, new_node_key, EmpowerData::Integer(0));

    input_ports.insert(new_input_port_key, new_input_port);

    let new_output_port_key = output_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_output_port = OutputPort::new(new_output_port_key, EmpowerData::Integer(0));

    output_ports.insert(new_output_port_key, new_output_port);

    let new_input_port_keys = Vec::from([new_input_port_key]);
    let new_output_port_keys = Vec::from([new_output_port_key]);

    let new_node = Node::new(new_node_key, NodeType::IntegerVariable, new_input_port_keys, new_output_port_keys);
    nodes.insert(new_node_key, new_node);

    new_node_key
}

pub fn create_addition_node(nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>) -> EmpowerKey
{
    let new_node_key = nodes.keys().copied().max().unwrap_or(0) + 1;

    let new_input_port_addition_key = input_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_input_port_addition = InputPort::new(new_input_port_addition_key, new_node_key, EmpowerData::Integer(0));

    input_ports.insert(new_input_port_addition_key, new_input_port_addition);

    let new_input_port_value_key = input_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_input_port_value = InputPort::new(new_input_port_value_key, new_node_key, EmpowerData::Integer(0));

    input_ports.insert(new_input_port_value_key, new_input_port_value);

    let new_output_port_key = output_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_output_port = OutputPort::new(new_output_port_key, EmpowerData::Integer(0));

    output_ports.insert(new_output_port_key, new_output_port);

    let new_input_port_keys = Vec::from([new_input_port_addition_key, new_input_port_value_key]);
    let new_output_port_keys = Vec::from([new_output_port_key]);

    let new_node = Node::new(new_node_key, NodeType::Addition, new_input_port_keys, new_output_port_keys);
    nodes.insert(new_node_key, new_node);

    new_node_key
}

pub fn create_print_node(nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>) -> EmpowerKey
{
    let new_node_key = nodes.keys().copied().max().unwrap_or(0) + 1;

    let new_input_port_trigger_key = input_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_input_port_trigger = InputPort::new(new_input_port_trigger_key, new_node_key, EmpowerData::Trigger);

    input_ports.insert(new_input_port_trigger_key, new_input_port_trigger);

    let new_input_port_value_key = input_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_input_port_value = InputPort::new(new_input_port_value_key, new_node_key, EmpowerData::Integer(0));

    input_ports.insert(new_input_port_value_key, new_input_port_value);

    let new_input_port_keys = Vec::from([new_input_port_trigger_key, new_input_port_value_key]);

    let new_node = Node::new(new_node_key, NodeType::Print, new_input_port_keys, Vec::new());
    nodes.insert(new_node_key, new_node);

    new_node_key
}
