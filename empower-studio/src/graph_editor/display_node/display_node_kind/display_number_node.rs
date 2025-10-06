use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::{node::node_kind::{number_node::{NumberNodeValueKind}, NumberNodeState}, port::PortValue};

pub fn get_display_node_size() -> egui::Vec2 
{
    egui::Vec2 { x: 320.0, y: 210.0 }
}

pub fn get_state_size() -> egui::Vec2 
{
    egui::Vec2 { x: 175.0, y: 50.0 }
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

pub fn show(ui: &mut egui::Ui, state: &mut NumberNodeState)
{
    ui.menu_button(state.desired_value.to_string(), |ui|
    {
        if ui.button("Automatic").clicked()
        {
            state.desired_value = NumberNodeValueKind::Automatic;
        }
        if ui.button("Integer").clicked()
        {
            state.desired_value = NumberNodeValueKind::Integer;
        }
        if ui.button("Float").clicked()
        {
            state.desired_value = NumberNodeValueKind::Float;
        }
    });
}

