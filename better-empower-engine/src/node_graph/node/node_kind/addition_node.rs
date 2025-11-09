use std::ops::Add;

use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

#[derive(Clone)]
pub struct AdditionNode
{
    added_value_to_send: PortValue,
}

impl NodeKind for AdditionNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new(
            Self {
                added_value_to_send: PortValue::Integer(0),
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

    fn state(&mut self, _: &mut egui::Ui) {
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> Option<Vec<PortValue>> {

        let output_value = inputs[0].clone() + inputs[1].clone();
        Some( Vec::from([ output_value ]) )
    }

    fn execute(&mut self, _: Option<&mut egui::Ui>) -> Option<Vec<PortValue>> {
        None
    }
}