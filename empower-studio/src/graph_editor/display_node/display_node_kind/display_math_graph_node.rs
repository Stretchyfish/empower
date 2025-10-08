use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::port::PortValue;

pub fn get_display_node_size() -> egui::Vec2
{
    egui::Vec2 { x: 350.0, y: 290.0 }
}

pub fn get_display_node_state_size() -> egui::Vec2
{
    egui::Vec2::ZERO
}

pub fn get_display_node_input_ports(inputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    vec![ 
        DisplayPortValue::from("".to_string(), inputs[0]),
        DisplayPortValue::from("x".to_string(), inputs[1]),
        DisplayPortValue::from("y".to_string(), inputs[2]),
    ]
}

pub fn get_display_node_output_ports(outputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    Vec::new()
}
    