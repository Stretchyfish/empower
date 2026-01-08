use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone)]
pub struct TextNode
{

}

impl NodeKind for TextNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn name(&self) -> &'static str {
        "text"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
            ]
        )
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
            ]
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> NodeSetupResponse {
        let output_value = inputs[0].clone();
        NodeSetupResponse::Finished(  vec![ output_value ] )
    }

    fn update(&mut self) -> NodeUpdateResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}
