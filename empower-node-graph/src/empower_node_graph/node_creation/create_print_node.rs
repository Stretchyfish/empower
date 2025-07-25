use std::collections::HashMap;

use crate::EmpowerKey;
use crate::EmpowerData;
use crate::NodeType;
use crate::InputPort;
use crate::OutputPort;
use crate::port::PortType;
use crate::Node;

pub fn create_print_node(nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>) -> (EmpowerKey, Vec<EmpowerKey>, Vec<EmpowerKey>)
{
    let new_node_key = nodes.keys().copied().max().unwrap_or(0) + 1;

    let new_input_port_trigger_key = input_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_input_port_trigger = InputPort::new(
                                            new_input_port_trigger_key, 
                                            new_node_key, 
                                            PortType::Exatch( EmpowerData::Trigger ),
                                            EmpowerData::Trigger);

    input_ports.insert(new_input_port_trigger_key, new_input_port_trigger);

    let new_input_port_value_key = input_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_input_port_value = InputPort::new(
                                            new_input_port_value_key, 
                                            new_node_key, 
                                            PortType::OneOf( vec![EmpowerData::Integer(0), EmpowerData::Float(0.0), EmpowerData::Text( String::new() ), EmpowerData::Bool( false ) ]),
                                            EmpowerData::Integer(0));

    input_ports.insert(new_input_port_value_key, new_input_port_value);

    let new_input_port_keys = Vec::from([new_input_port_trigger_key, new_input_port_value_key]);

    let new_node = Node::new(new_node_key, NodeType::Print, new_input_port_keys.clone(), Vec::new());
    nodes.insert(new_node_key, new_node);

    (new_node_key, new_input_port_keys, Vec::new())
}
