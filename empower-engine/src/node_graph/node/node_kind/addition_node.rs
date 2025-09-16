use std::ops::Add;

use crate::node_graph::port::{PortCompatability, PortValue};

use super::NodeKind;

pub struct AdditionNode
{

}

impl NodeKind for AdditionNode
{
    fn name(&self) -> &'static str {
        "addition"
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
        let output_value = inputs[0].clone() + inputs[1].clone();
        Vec::from( [ output_value ] )
    }
}
