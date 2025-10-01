use crate::node_graph::port::{PortCompatability, PortValue};

pub fn get_name() -> &'static str
{
    "addition"
}

pub fn get_addition_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
            PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
        ]
    )
}

pub fn get_addition_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
        ]
    )
}

pub fn execute_addition_node(inputs: Vec<&PortValue>) -> Vec<PortValue> 
{
    let output_value = inputs[0].clone() + inputs[1].clone();
    Vec::from( [ output_value ] )
}
