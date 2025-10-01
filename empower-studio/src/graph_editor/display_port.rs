// use empower_node_graph::EmpowerKey;
use empower_engine::NodeGraphKey;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;

pub mod display_port_value;

// #[derive(Default, Clone)]
// pub struct DisplayPort
// {
//     pub node_key: NodeGraphKey,
//     pub relative_position: egui::Vec2,
//     pub text: String,
//     // pub value_representation: DisplayPortValueRepresentation, // @TODO, consider changing this name?
//     // pub value_representation_valid: bool,
// }

#[derive(Default, Clone, PartialEq)]
pub struct DisplayPort
{
    pub node_key: NodeGraphKey,
    pub relative_position: egui::Vec2,
    pub display_value: DisplayPortValue,
    // pub text: String,
    // pub value_representation: DisplayPortValueRepresentation, // @TODO, consider changing this name?
    // pub value_representation_valid: bool,
}

// #[derive(Default, Clone)]
// pub enum DisplayPortValueRepresentation // @TODO, find a better name?
// {
//     Text(String),
//     Checkbox(bool),
//     #[default] None, //@TODO, consider if this should be named something else?
// }

