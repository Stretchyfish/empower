use egui;

use crate::graph_editor::display_port::display_port_value::{DisplayPortValue, DisplayPortValueType};

pub fn get_display_start_node_size() -> egui::Vec2
{
    egui::Vec2 { x: 250.0, y: 165.0 }
}

pub fn get_display_start_node_input_ports() -> Vec<DisplayPortValue>
{
    Vec::new()
}

pub fn get_display_start_node_output_ports() -> Vec<DisplayPortValue>
{
    vec![ 
        DisplayPortValue::nothing_with_text( "Out".to_string() ),
    ]
}
