use empower_engine::{node_graph::port::PortValue, NodeKind};

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;

mod display_start_node;
mod display_number_node;
mod display_print_node;

pub fn get_display_node_size(node_kind: &NodeKind) -> egui::Vec2
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_display_start_node_size(),
        NodeKind::Number => display_number_node::get_display_number_node_size(),
        NodeKind::Print => display_print_node::get_display_print_node_size(),
        _ => egui::Vec2::ZERO,
    }
}

pub fn get_display_node_value_offset(node_kind: &NodeKind) -> f32
{
    match node_kind
    {
        _ => 0.0,
    }
} 

pub fn get_display_input_ports(node_kind: &NodeKind, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_display_start_node_input_ports(),
        NodeKind::Number => display_number_node::get_display_number_node_input_ports(inputs),
        NodeKind::Print => display_print_node::get_display_print_node_input_ports(inputs),
        _ => Vec::new(),
    }
}

// @TODO, consider if all elements in DisplayPortValue should be shown in the output nodes
pub fn get_display_output_ports(node_kind: &NodeKind, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_display_start_node_output_ports(outputs),
        NodeKind::Number => display_number_node::get_display_number_node_output_ports(outputs),
        NodeKind::Print => display_print_node::get_display_print_node_output_ports(),
        _ => Vec::new(),
    }
}
