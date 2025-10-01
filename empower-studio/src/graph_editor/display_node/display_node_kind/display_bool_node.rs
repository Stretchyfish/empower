use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::port::PortValue;

pub fn get_display_node_size() -> egui::Vec2 
{
    egui::Vec2 { x: 450.0, y: 165.0 }
}

pub fn get_state_size() -> egui::Vec2 
{
    egui::Vec2::ZERO
}

pub fn get_display_input_ports(inputs: Vec<&PortValue>) -> Vec<DisplayPortValue> 
{
    vec![ 
        DisplayPortValue::from("in".to_string(), inputs[0]),
    ]
}

pub fn get_display_output_ports(outputs: Vec<&PortValue>) -> Vec<DisplayPortValue> 
{
    vec![ 
        DisplayPortValue::nothing_with_text( "Out".to_string(), outputs[0] ), 
    ]
}
