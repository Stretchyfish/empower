use crate::node_graph::node::port::{PortCompatability, PortValue};

use super::NodeKind;
use super::NodeSetupResponse;
use super::NodeUpdateResponse;

#[derive(Clone)]
pub struct FilePathNode
{
    pub path: String,

}

impl NodeKind for FilePathNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {
        
        Box::new( Self { path: String::new() } )
    }

    fn name(&self) -> &'static str {
        "file path"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::new()
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [ 
                PortCompatability::Exatch( PortValue::Text( String::new() ))
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
        NodeSetupResponse::Finished( vec![  PortValue::Text( self.path.clone() ) ] )
    }

    fn update(&mut self) -> NodeUpdateResponse {
        todo!()
    }

    fn show(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}
