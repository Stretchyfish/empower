use crate::node_graph::port::{PortCompatability, PortValue};
use super::NodeKind;

pub struct TextNode
{

}

impl NodeKind for TextNode
{
    fn name(&self) -> &'static str {
        "text"
    }

    fn input_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
            PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
            ]
        )
    }

    fn output_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
            PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
            ]
        )
    }

    fn execute(&self, inputs: Vec<&PortValue>, _: &mut crate::analyser::TextBuffer) -> Vec<PortValue> {
        let output_value = inputs[0].clone();
        Vec::from( [ output_value ] )
    }
}
