use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::{node::node_kind::FilePathState, port::PortValue};

pub fn get_display_node_size() -> egui::Vec2 
{
    egui::Vec2 { x: 650.0, y: 210.0 }
}

pub fn get_state_size() -> egui::Vec2 
{
    egui::Vec2 { x: 600.0, y: 50.0 }
}

pub fn get_display_input_ports() -> Vec<DisplayPortValue> 
{
    Vec::new()
}

pub fn get_display_output_ports(outputs: Vec<&PortValue>) -> Vec<DisplayPortValue> 
{
    vec![ 
        DisplayPortValue::from("a".to_string(), outputs[0]),
    ]
}

pub fn show(ui: &mut egui::Ui, state: &mut FilePathState)
{
    ui.horizontal(|ui|
    {
        let text_edit = egui::TextEdit::singleline(&mut state.path)
        .min_size( egui::Vec2 { x: 480.0, y: 50.0 });

        ui.add( text_edit);

        if ui.button("Find File").clicked()
        {
            let file_path = rfd::FileDialog::new().pick_file();

            if file_path.is_some()
            {
                state.path = file_path.unwrap().to_str().unwrap().to_string()
            }

        }
    }); 
}
