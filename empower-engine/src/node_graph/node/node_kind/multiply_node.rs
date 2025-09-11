use crate::node_graph::port::{PortCompatability, PortValue};

use super::NodeKind2;

pub struct MultiplyNode
{

}

impl NodeKind2 for MultiplyNode
{
    fn name(&self) -> &'static str {
        "multiply node"
    }

    fn input_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
                PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
            ]
        )
    }

    fn output_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
            ]
        )
    }

    fn execute(&self, inputs: Vec<&PortValue>, log: &mut crate::analyser::TextBuffer) -> Vec<PortValue> {
        let output_value = inputs[0].clone() * inputs[1].clone();
        Vec::from( [ output_value ] )
    }
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

pub fn execute_multiply_node(inputs: Vec<&PortValue>) -> Vec<PortValue>
{
    let output_value = inputs[0].clone() * inputs[1].clone();
    Vec::from( [ output_value ] )
}
