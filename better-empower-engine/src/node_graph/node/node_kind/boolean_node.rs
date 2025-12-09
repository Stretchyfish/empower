use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

#[derive(Clone)]
pub struct BooleanNode
{

}

impl NodeKind for BooleanNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {
        
        Box::new( Self {} )
    }

    fn name(&self) -> &'static str {
        "boolean"
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
                PortCompatability::Exatch( PortValue::Bool( false )), 
            ]
        )
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Bool( false )), 
            ]
        )
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn state(&mut self, _: &mut egui::Ui) {
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> Option<Vec<PortValue>> {
        let output_value = inputs[0].clone();
        Some( Vec::from( [ output_value ] ) )
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        todo!()
    }

    fn execute(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}