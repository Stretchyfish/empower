use crate::node_graph::port::{PortCompatability, PortValue};

use super::NodeKind;

pub struct NumberNode
{

}

impl NodeKind for NumberNode
{
    fn name(&self) -> &'static str {
        "number"
    }

    fn input_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::OneOf( vec!( PortValue::Integer(0), PortValue::Float(0.0)  ) ),
            ]
        )
    }

    fn output_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::OneOf( vec![ PortValue::Integer(0), PortValue::Float(0.0) ] ),
            ]
        )
    }

    fn execute(&self, inputs: Vec<&PortValue>, _: &mut crate::analyser::TextBuffer) -> Vec<PortValue> {
        let output_port_value = inputs[0].clone();
        Vec::from( [ output_port_value ] )
    }
}
