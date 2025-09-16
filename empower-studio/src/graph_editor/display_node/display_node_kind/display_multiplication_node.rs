use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::port::PortValue;
use super::DisplayNodeKind;

pub struct DisplayMultiplyNode
{

}

impl DisplayNodeKind for DisplayMultiplyNode
{
    fn get_display_node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 350.0, y: 230.0 }
    }

    fn get_display_input_ports(&self, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        vec![ 
            DisplayPortValue::from("a".to_string(), inputs[0]),
            DisplayPortValue::from("b".to_string(), inputs[1]),
        ]
    }

    fn get_display_output_ports(&self, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        vec![ 
            DisplayPortValue::nothing_with_text( "Out".to_string(), outputs[0] ), 
        ]
    }
}
