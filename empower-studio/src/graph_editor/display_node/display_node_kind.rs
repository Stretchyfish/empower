use empower_engine::{node_graph::port::PortValue, NodeKind};

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;

mod display_start_node;
mod display_number_node;
mod display_print_node;
mod display_text_node;
mod display_bool_node;
mod display_addition_node;
mod display_multiplication_node;

pub fn get_display_node_size(node_kind: &NodeKind) -> egui::Vec2
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_display_start_node_size(),
        NodeKind::Number => display_number_node::get_display_number_node_size(),
        NodeKind::Print => display_print_node::get_display_print_node_size(),
        NodeKind::Text => display_text_node::get_display_text_node_size(),
        NodeKind::Bool => display_bool_node::get_display_bool_node_size(),
        NodeKind::Addition => display_addition_node::get_display_addition_node_size(),
        NodeKind::Multiply => display_multiplication_node::get_display_multiplication_node_size(),
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
        NodeKind::Text => display_text_node::get_display_text_node_input_ports(inputs),
        NodeKind::Bool => display_bool_node::get_display_bool_node_input_ports(inputs),
        NodeKind::Addition => display_addition_node::get_display_addition_node_input_ports(inputs),
        NodeKind::Multiply => display_multiplication_node::get_display_multiplication_node_input_ports(inputs),
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
        NodeKind::Text => display_text_node::get_display_text_node_output_ports(outputs),
        NodeKind::Bool => display_bool_node::get_display_bool_node_output_ports(outputs),
        NodeKind::Addition => display_addition_node::get_display_addition_node_output_ports(outputs),
        NodeKind::Multiply => display_multiplication_node::get_display_multiplication_node_output_ports(outputs),
    }
}
