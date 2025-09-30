use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::port::PortValue;
use empower_engine::node_graph::node::node_kind::NodeKind;
use empower_engine::node_graph::node::node_kind::NodeKind2;


pub fn get_display_node_size() -> egui::Vec2 
{
    egui::Vec2 { x: 350.0, y: 230.0 }
}

pub fn get_state_size() -> egui::Vec2 
{
    egui::Vec2::ZERO
}

pub fn get_display_input_ports(inputs: Vec<&PortValue>) -> Vec<DisplayPortValue> 
{
    vec![ 
        DisplayPortValue::from("a".to_string(), inputs[0]),
        DisplayPortValue::from("b".to_string(), inputs[1]),
    ]
}

pub fn get_display_output_ports(outputs: Vec<&PortValue>) -> Vec<DisplayPortValue> 
{
    vec![ 
        DisplayPortValue::nothing_with_text( "Out".to_string(), outputs[0] ), 
    ]
}

use super::DisplayNodeKind;

pub struct DisplayAdditionNode
{

}

impl DisplayNodeKind for DisplayAdditionNode
{
    fn get_display_node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 350.0, y: 230.0 }
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

    fn get_display_output_ports(&self, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue> {
        vec![ 
            DisplayPortValue::nothing_with_text( "Out".to_string(), outputs[0] ), 
        ]
    }

    fn show(&self, _: &mut egui::Ui, _: &Box<dyn NodeKind>) {
        
    }
}
