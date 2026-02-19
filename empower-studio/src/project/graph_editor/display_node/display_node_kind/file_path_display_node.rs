use empower_engine::PortValue;
use empower_engine::node_graph::node::NodeKind;
use empower_engine::node_graph::node::node_kind::FilePathNode;
use crate::project::graph_editor::display_node::DisplayNodeStateResponse;

use super::DisplayNodeKind;
use super::super::DisplayPort;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct FilePathDisplayNode
{

}

#[typetag::serde]
impl DisplayNodeKind for FilePathDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {

        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {

        Box::new( self.clone() )
    }

    fn node_size(&self, _: &Box<dyn NodeKind>) -> egui::Vec2 {
        egui::Vec2 { x: 650.0, y: 210.0 }
    }

    fn display_input_ports(&self, _: Vec<&PortValue>) -> Vec<DisplayPort> {

        Vec::new()
    }

    fn display_output_ports(&self, output_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {
        
        let mut display_outputs = Vec::new();

        display_outputs.reserve(output_port_values.len());
        for output_port_value in output_port_values
        {
            let display_port = DisplayPort::nothing(egui::vec2(0.0, 0.0), &output_port_value); // The position is just defaulted here, because it will be correct in refresh display node
            display_outputs.push(display_port);
        }

        display_outputs
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 600.0, y: 50.0 }
    }

    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>) -> DisplayNodeStateResponse {

        let file_path_state = node_kind.as_any_mut().downcast_mut::<FilePathNode>().expect("File path display node tried to unwrap a node_kind that is not the file path node kind");
        let original_file_path = file_path_state.path.clone();

        ui.horizontal(|ui|
        {
            let text_edit = egui::TextEdit::singleline(&mut file_path_state.path)
            .min_size( egui::Vec2 { x: 480.0, y: 50.0 });

            ui.add( text_edit);

            if ui.button("Find File").clicked()
            {
                let file_path = rfd::FileDialog::new().pick_file();

                if file_path.is_some()
                {
                    file_path_state.path = file_path.unwrap().to_str().unwrap().to_string();
                }
            }
        }); 

        if ui.button("Copy to clipboard").clicked()
        {
            ui.ctx().copy_text(file_path_state.path.clone());
        }

        if original_file_path != file_path_state.path
        {
            return DisplayNodeStateResponse::RefreshNodeStructure;
        }

        DisplayNodeStateResponse::NoChange
    }
}
