use std::collections::HashMap;

use crate::EmpowerKey;
use crate::EmpowerData;
use crate::NodeType;
use crate::InputPort;
use crate::OutputPort;
use crate::port::PortType;
use crate::Node;

pub fn create_text_node(nodes: &mut HashMap<EmpowerKey, Node>, input_ports: &mut HashMap<EmpowerKey, InputPort>, output_ports: &mut HashMap<EmpowerKey, OutputPort>) -> (EmpowerKey, Vec<EmpowerKey>, Vec<EmpowerKey>)
{
    // @TODO, find a way of reusing values between max
    let new_node_key = nodes.keys().copied().max().unwrap_or(0) + 1;

    let new_input_port_key = input_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_input_port = InputPort::new(
                                            new_input_port_key, 
                                            new_node_key,  
                                            PortType::Exatch( EmpowerData::Text( String::new() ) ), 
                                            EmpowerData::Text( String::new() ));

    input_ports.insert(new_input_port_key, new_input_port);

    let new_output_port_key = output_ports.keys().copied().max().unwrap_or(0) + 1;
    let new_output_port = OutputPort::new(new_output_port_key, EmpowerData::Text( String::new() ));

    output_ports.insert(new_output_port_key, new_output_port);

    let new_input_port_keys = Vec::from([new_input_port_key]);
    let new_output_port_keys = Vec::from([new_output_port_key]);

    let new_node = Node::new(
                                    new_node_key, 
                                    NodeType::Text, 
                                    new_input_port_keys.clone(), 
                                    new_output_port_keys.clone()
                                );

    nodes.insert(new_node_key, new_node);

    (new_node_key, new_input_port_keys, new_output_port_keys)
}
