use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

#[derive(Clone)]
pub struct FilePathNode
{
    path: String,

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

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn state(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui|
        {
            let text_edit = egui::TextEdit::singleline(&mut self.path)
            .min_size( egui::Vec2 { x: 480.0, y: 50.0 });

            ui.add( text_edit);

            if ui.button("Find File").clicked()
            {
                let file_path = rfd::FileDialog::new().pick_file();

                if file_path.is_some()
                {
                    self.path = file_path.unwrap().to_str().unwrap().to_string()
                }
            }
        }); 
    }

    fn setup(&mut self, _: Vec<&PortValue>) -> Option<Vec<PortValue>> {
        Some( Vec::from([PortValue::Text( self.path.clone() )]))
    }

    fn execute(&mut self, _: Option<&mut egui::Ui>) -> Option<Vec<PortValue>> {
        todo!()
    }
}
