use crate::node_graph::{Node, NodeGraph, NodeHandle, NodeType, Port, PortCompatability, PortValue};

pub fn create_print_node(node_graph: &mut NodeGraph) -> NodeHandle
{
    let new_node_key = node_graph.get_available_node_key();

    let new_input_port_trigger_key = node_graph.get_available_input_port_key();
    let new_input_port_trigger = Port::new_input_port(
                                            new_input_port_trigger_key, 
                                            new_node_key, 
                                            PortCompatability::Exatch( PortValue::Trigger ),
                                            );

    node_graph.input_ports.insert(new_input_port_trigger_key, new_input_port_trigger);

    let new_input_port_value_key = node_graph.get_available_input_port_key();
    let new_input_port_value = Port::new_output_port(
                                            new_input_port_value_key, 
                                            new_node_key, 
                                            PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ) ]),
                                            );


    NodeHandle::empty()
}
