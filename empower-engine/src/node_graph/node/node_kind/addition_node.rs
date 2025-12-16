use crate::{node_graph::node::{NodeFunction, port::{PortCompatability, PortValue}}, utility::text_buffer::TextBuffer};

use super::NodeKind;

#[derive(Clone)]
pub struct AdditionNode
{
}

impl NodeKind for AdditionNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new(
            Self {
            }
        )
    }

    fn name(&self) -> &'static str {
        "addition"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new(self.clone())
    }

    fn function(&self) -> NodeFunction {
        NodeFunction::Instant
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
            Vec::from(
            [
                PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
                PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
            ]
        )
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::OneOf( vec![PortValue::Integer( 0 ), PortValue::Float( 0.0 )] ),
            ]
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn setup(&mut self, inputs: Vec<&PortValue>, _: &mut TextBuffer) -> Option<Vec<PortValue>> {

        let output_value = inputs[0].clone() + inputs[1].clone();
        Some( Vec::from([ output_value ]) )
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        todo!()
    }

    fn execute(&mut self, _: &mut egui::Ui) {
        panic!("Entered execute for addition node, which should not happend");
    }
    
}