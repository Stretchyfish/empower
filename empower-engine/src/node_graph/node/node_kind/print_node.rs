use crate::{node_graph::node::{NodeFunction, port::{PortCompatability, PortValue}}, utility::text_buffer::TextBuffer};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone)]
pub struct PrintNode
{

}

impl NodeKind for PrintNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn name(&self) -> &'static str {
        "print"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Trigger(false) ),
                PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ), PortValue::Vector( Vec::new() ) ])
        ]
        )
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::new()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> NodeSetupResponse {

        println!("PRINTING: {}", inputs[1]);
        NodeSetupResponse::FinishedWithLog( Vec::new(), format!("{}", inputs[1]))
    }

    fn update(&mut self) -> NodeUpdateResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}
