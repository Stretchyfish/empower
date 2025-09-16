use crate::node_graph::port::{PortCompatability, PortValue};

use super::NodeKind;

pub struct BoolNode
{

}

impl NodeKind for BoolNode
{
    fn name(&self) -> &'static str {
        "bool"
    }

    fn input_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
            PortCompatability::Exatch( PortValue::Bool( false )), 
            ]
        )
    }

    fn output_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
            PortCompatability::Exatch( PortValue::Bool( false )), 
            ]
        )
    }

    fn execute(&self, inputs: Vec<&PortValue>, _: &mut crate::analyser::TextBuffer) -> Vec<PortValue> {
        let output_value = inputs[0].clone();
        Vec::from( [ output_value ] )
    }
}
