use egui;

use crate::graph_editor::display_port::display_port_value::{DisplayPortValue, DisplayPortValueType};
use empower_engine::node_graph::port::PortValue;
use empower_engine::node_graph::node::node_kind::print_node::PrintNode;
use super::DisplayNodeKind;

pub struct DisplayPrintNode
{

}

impl DisplayNodeKind for DisplayPrintNode
{
    fn get_display_node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 450.0, y: 250.0 }
    }

    fn get_display_input_ports(&self, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        vec![ 
            DisplayPortValue::from("a".to_string(), inputs[0]),
            DisplayPortValue::from("b".to_string(), inputs[1]),
        ]
    }

    fn get_display_output_ports(&self, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        Vec::new()
    }
}

pub fn get_display_print_node_size() -> egui::Vec2
{
    egui::Vec2 { x: 450.0, y: 250.0 }
}

pub fn get_display_print_node_input_ports(inputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    vec![ 
        DisplayPortValue::from("a".to_string(), inputs[0]),
        DisplayPortValue::from("b".to_string(), inputs[1]),
    ]
}

pub fn get_display_print_node_output_ports() -> Vec<DisplayPortValue>
{
    Vec::new()
}
