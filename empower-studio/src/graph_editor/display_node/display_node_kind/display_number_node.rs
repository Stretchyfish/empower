use egui;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::{node::node_kind::{number_node::{NumberNode, NumberNodeValueKind}, NodeKind, NodeKind2, NumberNodeState}, port::PortValue};
use super::DisplayNodeKind;

pub fn get_display_node_size() -> egui::Vec2 
{
    egui::Vec2 { x: 320.0, y: 200.0 }
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

pub fn show(ui: &mut egui::Ui, state: &NumberNodeState) -> Option<NodeKind2>
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

    Some ( NodeKind2::Number( state_to_modify ) )
    
}

pub struct DisplayNumberNode
{

}

impl DisplayNodeKind for DisplayNumberNode
{
    fn get_display_node_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 450.0, y: 175.0 }
    }

    fn get_state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 100.0, y: 25.0 }
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

    fn show(&self, ui: &mut egui::Ui, node_kind: &Box<dyn NodeKind>){

        let number_node = node_kind.as_any().downcast_ref::<NumberNode>().expect("Tried to visualize node that is not compatible with display node");

        let test = &number_node.desired_type;

        ui.menu_button("test", |ui|
        {
            ui.button("See me");
            ui.button("See me");
            ui.button("See me");
            ui.button("See me");
        });
    }
}
