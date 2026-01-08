use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone)]
pub struct LoopNode
{
    
}

impl NodeKind for LoopNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {} )
    }

    fn name(&self) -> &'static str {
        "loop"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        vec![ PortCompatability::Exatch( PortValue::Trigger( false ) ) ]
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        vec![ PortCompatability::Exatch( PortValue::Trigger( false ) ) ]
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn setup(&mut self, _: Vec<&PortValue>) -> NodeSetupResponse {
        NodeSetupResponse::CreateLoop
    }

    fn update(&mut self) -> NodeUpdateResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}
