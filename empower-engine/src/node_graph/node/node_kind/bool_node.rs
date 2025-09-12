use crate::node_graph::port::{PortCompatability, PortValue};

use super::NodeKind2;

pub struct BoolNode
{

}

impl NodeKind2 for BoolNode
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

    fn execute(&self, inputs: Vec<&PortValue>, log: &mut crate::analyser::TextBuffer) -> Vec<PortValue> {
        let output_value = inputs[0].clone();
        Vec::from( [ output_value ] )
    }
}

pub fn get_bool_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
           PortCompatability::Exatch( PortValue::Bool( false )), 
        ]
    )
}

pub fn get_bool_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
           PortCompatability::Exatch( PortValue::Bool( false )), 
        ]
    )
}

pub fn execute_bool_node(inputs: Vec<&PortValue>) -> Vec<PortValue>
{
    let output_value = inputs[0].clone();
    Vec::from( [ output_value ] )
}
