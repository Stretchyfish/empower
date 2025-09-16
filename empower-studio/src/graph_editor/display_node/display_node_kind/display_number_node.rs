use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::port::PortValue;
use super::DisplayNodeKind;

pub struct DisplayNumberNode
{

}

impl DisplayNodeKind for DisplayNumberNode
{
    fn get_display_node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 450.0, y: 165.0 }
    }

    fn get_display_input_ports(&self, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        vec![ 
            DisplayPortValue::from("in".to_string(), inputs[0]),
        ]
    }

    fn get_display_output_ports(&self, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        vec![ 
            DisplayPortValue::nothing_with_text( "Out".to_string(), outputs[0] ), 
        ]
    }
}
