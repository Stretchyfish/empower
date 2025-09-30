use egui;
use egui::output;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::port::PortValue;
use super::DisplayNodeKind;
use empower_engine::node_graph::node::node_kind::NodeKind;
use empower_engine::node_graph::node::node_kind::NodeKind2;

pub fn get_display_print_node_size() -> egui::Vec2
{
        egui::Vec2 { x: 450.0, y: 250.0 }
}

pub fn get_display_print_node_state_size() -> egui::Vec2
{
    egui::Vec2::ZERO
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

pub struct DisplayPrintNode
{

}

impl DisplayNodeKind for DisplayPrintNode
{
    fn get_display_node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 450.0, y: 250.0 }
    }

    fn get_state_size(&self) -> egui::Vec2 {
        egui::Vec2::ZERO
    }

    fn get_display_input_ports(&self, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        vec![ 
            DisplayPortValue::from("a".to_string(), inputs[0]),
            DisplayPortValue::from("b".to_string(), inputs[1]),
        ]
    }

    fn get_display_output_ports(&self, _: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        Vec::new()
    }

    fn show(&self, _: &mut egui::Ui, _: &Box<dyn NodeKind>) {
        
    }
}
