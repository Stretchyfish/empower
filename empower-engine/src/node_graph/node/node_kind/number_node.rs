use crate::node_graph::{node::node_value::NodeValue, port::{PortCompatability, PortValue}};

pub fn get_number_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::OneOf( vec!( PortValue::Integer(0), PortValue::Float(0.0)  ) ),
        ]
    )
}

pub fn get_number_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::OneOf( vec![ PortValue::Integer(0), PortValue::Float(0.0) ] ),
        ]
    )
}

pub fn execute_number_node(node: &mut NodeValue, inputs: Vec<&PortValue>) -> Vec<PortValue>
{
    match node // @TODO, implement the correct behavior here
    {
        NodeValue::NumberState => println!("Will read the state here"),
        NodeValue::None => {},
    }

    let output_port_value = inputs[0].clone();
    Vec::from( [ output_port_value ] )
}
