use std::any::Any;
use crate::node_graph::port::{PortCompatability, PortValue};
use super::NodeKind;

pub fn get_name() -> &'static str
{
    "text"
}

pub fn get_text_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
        ]
    )
}

pub fn get_text_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
        ]
    )
}

pub fn execute_text_node(inputs: Vec<&PortValue>) -> Vec<PortValue> 
{
    let output_value = inputs[0].clone();
    Vec::from( [ output_value ] )
}

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
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}
