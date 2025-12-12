use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

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

    fn function(&self) -> NodeFunction {
        NodeFunction::Instant
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

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn setup(&mut self, _: Vec<&PortValue>) -> Option<Vec<PortValue>> {

        println!("Ran this with path: {}", self.path.clone());
        Some( Vec::from([PortValue::Text( self.path.clone() )]))
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        todo!()
    }

    fn execute(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}
