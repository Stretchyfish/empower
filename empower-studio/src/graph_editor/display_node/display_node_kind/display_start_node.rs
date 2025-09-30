use egui;
use empower_engine::node_graph::port::PortValue;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::node::node_kind::NodeKind;
use empower_engine::node_graph::node::node_kind::NodeKind2;
use super::DisplayNodeKind;

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

pub struct DisplayStartNode
{

}

impl DisplayNodeKind for DisplayStartNode
{
    fn get_display_node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 250.0, y: 165.0 }
    }

    fn get_state_size(&self) -> egui::Vec2 {
        egui::Vec2::ZERO
    }

    fn get_display_input_ports(&self, _: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        Vec::new()
    }

    fn get_display_output_ports(&self, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        vec![ 
            DisplayPortValue::nothing_with_text( "Out".to_string(), outputs[0] ),
        ]
    }

    fn show(&self, _: &mut egui::Ui, _: &Box<dyn NodeKind>) {
        
    }
}
