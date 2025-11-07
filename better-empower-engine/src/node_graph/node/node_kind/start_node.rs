use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

pub struct StartNode
{

}

impl NodeKind for StartNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

            Box::new(Self {} )
    }

    fn name(&self) -> &'static str {
        "start"
    }

    fn function(&self) -> NodeFunction {
        NodeFunction::Instant
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::new()
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Trigger )
            ]
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn state(&mut self, _: &mut egui::Ui) {
    }

    fn setup(&mut self, _: Vec<&PortValue>) {
    }

    fn update(&mut self, _: &mut egui::Ui) -> Option<Vec<PortValue>> {
        Some( Vec::from( [ PortValue::Trigger ] ) )
    }
    
}