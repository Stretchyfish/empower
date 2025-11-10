use core::panic;
use std::thread;

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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn state(&mut self, _: &mut egui::Ui) {
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

    fn execute(&mut self, ui: Option<&mut egui::Ui>) -> Option<Vec<PortValue>> {

        let ui = ui.unwrap();

        // let full_file_path = format!("file://{}", self.image_path.clone().unwrap());
        let full_file_path = self.image_path.clone().unwrap();
        
        egui::Image::new(full_file_path)
        .corner_radius(5)
        .tint(egui::Color32::LIGHT_BLUE)
        .paint_at(ui, egui::Rect::from_min_size(egui::Pos2 { x: 0.0, y: 0.0 }, egui::Vec2 { x: 400.0, y: 400.0 }));

        None
    }
}
