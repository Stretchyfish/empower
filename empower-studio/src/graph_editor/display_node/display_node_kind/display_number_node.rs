use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::{node::node_kind::{number_node::{NumberNodeValueKind}, NodeKind, NumberNodeState}, port::PortValue};

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

pub fn show(ui: &mut egui::Ui, state: &NumberNodeState) -> Option<NodeKind>
{
    let mut state_to_modify = state.clone();

    ui.menu_button(state_to_modify.desired_value.to_string(), |ui|
    {
        if ui.button("Automatic").clicked()
        {
            state_to_modify.desired_value = NumberNodeValueKind::Automatic;
        }
        if ui.button("Integer").clicked()
        {
            state_to_modify.desired_value = NumberNodeValueKind::Integer;
        }
        if ui.button("Float").clicked()
        {
            state_to_modify.desired_value = NumberNodeValueKind::Float;
        }
    });

    if state.desired_value == state_to_modify.desired_value
    {
        return None;
    }

    Some ( NodeKind::Number( state_to_modify ) )
    
}

