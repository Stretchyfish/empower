use crate::node_graph::port::{PortCompatability, PortValue};

pub fn get_name() -> &'static str
{
    "multiply"
}

pub fn get_multiply_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
            PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
        ]
    )
}

pub fn get_multiply_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
        ]
    )
}

pub fn execute_multiply_node(inputs: Vec<&PortValue>) -> Option<Vec<PortValue>>
{
    let output_value = inputs[0].clone() * inputs[1].clone();
    Some( Vec::from( [ output_value ] ))
}
