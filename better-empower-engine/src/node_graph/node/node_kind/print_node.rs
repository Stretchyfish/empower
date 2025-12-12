use crate::{node_graph::node::{NodeFunction, port::{PortCompatability, PortValue}}, utility::text_buffer::TextBuffer};

use super::NodeKind;

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

    fn function(&self) -> NodeFunction {
        NodeFunction::Instant
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Trigger ),
                PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ), PortValue::Vector( Vec::new() ) ])
        ]
        )
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::new()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn setup(&mut self, inputs: Vec<&PortValue>, log: &mut TextBuffer) -> Option<Vec<PortValue>> {

        println!("PRINTING: {}", inputs[1]);
        let text = format!("{}", inputs[1]);
        log.add_line(&text);
        
        Some( Vec::new() )
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        todo!()
    }

    fn execute(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}