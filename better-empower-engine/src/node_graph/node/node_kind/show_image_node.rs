use core::panic;

use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

#[derive(Clone)]
pub struct ShowImageNode
{
    image_path: Option<String>,
}

impl NodeKind for ShowImageNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {
        
        Box::new( Self { image_path: None } )
    }

    fn name(&self) -> &'static str {
        "show image"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn function(&self) -> NodeFunction {
        NodeFunction::Window
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
        [
            PortCompatability::Exatch( PortValue::Trigger ), 
            PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
        ]
    )
 
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::new()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> Option<Vec<PortValue>> {

        let input_text = match inputs[1]
        {
            PortValue::Text( text ) => text,
            _ => panic!("show image node was requested to show an image without a path"), 
        };

        self.image_path = Some( input_text.clone() );

        None
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        None
    }

    fn execute(&mut self, ui: &mut egui::Ui) {

        let full_file_path = self.image_path.clone().unwrap();

        let image = egui::Image::new( format!(
           "file://{}",
           full_file_path 
        ));

        ui.add(image).on_hover_text_at_pointer(full_file_path);
    }
}
