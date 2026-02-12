use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct StartNode
{

}

#[typetag::serde]
impl NodeKind for StartNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

            Box::new(Self {} )
    }

    fn name(&self) -> &'static str {
        "start"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new(self.clone())
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::new()
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Trigger(false) )
            ]
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn setup(&mut self, _: Vec<&PortValue>) -> NodeSetupResponse {
        NodeSetupResponse::Finished(  vec![ PortValue::Trigger(true )] )
    }

    fn update(&mut self) -> NodeUpdateResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
    
    
}
