use crate::node_graph::{Node, NodeGraph, NodeHandle, NodeKind, Port, PortCompatability, PortValue};

pub fn create_number_node(node_graph: &mut NodeGraph) -> NodeHandle
{
    let new_node_key = node_graph.get_available_node_key();

    let new_input_port_key = node_graph.get_available_input_port_key();
    let new_input_port_compatabilities = PortCompatability::OneOf( vec!( PortValue::Undefined( String::from("0") ), PortValue::Float(0.0), PortValue::Integer(0) ));
    let new_input_port = Port::new_input_port(new_input_port_key, new_node_key, new_input_port_compatabilities); 

    node_graph.input_ports.insert(new_input_port_key, new_input_port);

    let new_output_port_key = node_graph.get_available_output_port_key();
    let new_output_port_compatabilities = PortCompatability::OneOf( vec![ PortValue::Integer(0), PortValue::Float(0.0) ] );
    let new_output_port = Port::new_output_port( new_output_port_key, new_node_key, new_output_port_compatabilities );

    node_graph.output_ports.insert(new_output_port_key, new_output_port);

    let new_input_port_keys = Vec::from([new_input_port_key]);
    let new_output_port_keys = Vec::from([new_output_port_key]);

    let new_node = Node::new(
                                    new_node_key, 
                                    NodeKind::Number, 
                                    new_input_port_keys.clone(),
                                    new_output_port_keys.clone(),
                                );

    node_graph.nodes.insert(new_node_key, new_node);

    NodeHandle::new(new_node_key, new_input_port_keys, new_output_port_keys)
}