use std::collections::HashMap;

use crate::EmpowerKey;
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
    let new_output_port = OutputPort::new(new_output_port_key);

    output_ports.insert(new_output_port_key, new_output_port);

    let new_output_port_keys = Vec::from([new_output_port_key]);
    let new_node = Node::new(new_node_key, NodeType::Start, Vec::new(), new_output_port_keys);

    nodes.insert(new_node_key, new_node);
}

pub fn create_integer_node(nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>) -> EmpowerKey
{
    let new_node_key = nodes.len() as EmpowerKey;

    let new_input_port_key = input_ports.len() as EmpowerKey;
    let new_input_port = InputPort::new(new_input_port_key);

    input_ports.insert(new_input_port_key, new_input_port);

    let new_output_port_key = output_ports.len() as EmpowerKey;
    let new_output_port = OutputPort::new(new_output_port_key);

    output_ports.insert(new_output_port_key, new_output_port);

    let new_input_port_keys = Vec::from([new_input_port_key]);
    let new_output_port_keys = Vec::from([new_output_port_key]);

    let new_node = Node::new(new_node_key, NodeType::Integer, new_input_port_keys, new_output_port_keys);
    nodes.insert(new_node_key, new_node);

    new_node_key
}
