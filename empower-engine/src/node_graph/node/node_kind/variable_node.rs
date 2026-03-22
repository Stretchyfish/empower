use std::any::Any;

use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableNode
{
    pub variable_key: Option<String>,
}

#[typetag::serde]
impl NodeKind for VariableNode
{
    fn new() -> Box<dyn NodeKind>
        where Self:Sized
        {
            Box::new( Self
            {
                variable_key: None,
            }
        )
    }

    fn name(&self) ->  &'static str {
        "variable"
    }

    fn clone_box(&self) -> Box<dyn NodeKind>  {
        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability>  {
        Vec::new()
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability>  {
        Vec::new()
    }

    fn as_any_mut(&mut self) ->  &mut dyn Any {
        self
    }

    fn as_any(&self) ->  &dyn Any {
        self
    }

    fn setup(&mut self, _: Vec< &PortValue>) -> NodeSetupResponse {
        NodeSetupResponse::Finished( Vec::new() )
    }

    fn update(&mut self) -> NodeUpdateResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}
