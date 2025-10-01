use egui;
use empower_engine::node_graph::port::PortValue;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;

pub fn get_start_display_node_size() -> egui::Vec2
{
    egui::Vec2 { x: 250.0, y: 165.0 }
}

pub fn get_start_node_state_size() -> egui::Vec2
{
    egui::Vec2::ZERO
}

pub fn get_start_node_display_input_ports() -> Vec<DisplayPortValue>
{
    Vec::new()
}

pub fn get_start_node_display_output_ports(outputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    vec![ 
        DisplayPortValue::nothing_with_text( "Out".to_string(), outputs[0] ),
    ]
}
