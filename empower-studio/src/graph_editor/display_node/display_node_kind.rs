use empower_engine::node_graph::port::PortValue;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;

use empower_engine::node_graph::node::node_kind::NodeKind;

pub mod display_start_node;
pub mod display_number_node;
pub mod display_print_node;
pub mod display_text_node;
pub mod display_bool_node;
pub mod display_addition_node;
pub mod display_multiplication_node;

pub fn get_display_node_size(node_kind: &NodeKind) -> egui::Vec2
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_start_display_node_size(),
        NodeKind::Number(_) => display_number_node::get_display_node_size(),
        NodeKind::Bool => display_bool_node::get_display_node_size(),
        NodeKind::Text => display_text_node::get_display_text_node_size(),
        NodeKind::Addition => display_addition_node::get_display_node_size(),
        NodeKind::Multiply => display_multiplication_node::get_display_node_size(),
        NodeKind::Print => display_print_node::get_display_print_node_size(),
    }
}

pub fn get_state_size(node_kind: &NodeKind) -> egui::Vec2
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_start_node_state_size(),
        NodeKind::Number(_) => display_number_node::get_state_size(),
        NodeKind::Bool => display_bool_node::get_state_size(),
        NodeKind::Text => display_text_node::get_display_text_node_state_size(),
        NodeKind::Addition => display_addition_node::get_state_size(),
        NodeKind::Multiply => display_multiplication_node::get_state_size(), // @TODO, fix file naming
        NodeKind::Print => display_print_node::get_display_print_node_state_size(),
    }
}

pub fn get_display_input_ports(node_kind: &NodeKind, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_start_node_display_input_ports(),
        NodeKind::Number(_) => display_number_node::get_display_input_ports(inputs),
        NodeKind::Bool => display_bool_node::get_display_input_ports(inputs),
        NodeKind::Text => display_text_node::get_display_text_node_input_ports(inputs),
        NodeKind::Addition => display_addition_node::get_display_input_ports(inputs),
        NodeKind::Multiply => display_multiplication_node::get_display_input_ports(inputs),
        NodeKind::Print => display_print_node::get_display_print_node_input_ports(inputs),
    }
}

pub fn get_display_output_ports(node_kind: &NodeKind, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    match node_kind
    {
        NodeKind::Start => display_start_node::get_start_node_display_output_ports(outputs),
        NodeKind::Number(_) => display_number_node::get_display_output_ports(outputs),
        NodeKind::Bool => display_bool_node::get_display_output_ports(outputs),
        NodeKind::Text => display_text_node::get_display_text_node_output_ports(outputs),
        NodeKind::Addition => display_addition_node::get_display_output_ports(outputs),
        NodeKind::Multiply => display_multiplication_node::get_display_output_ports(outputs),
        NodeKind::Print => display_print_node::get_display_print_node_output_ports(),
    }
}

pub fn show(ui: &mut egui::Ui, node_kind: &NodeKind) -> Option<NodeKind>
{
    match node_kind
    {
        NodeKind::Start => None,
        NodeKind::Number( state ) => display_number_node::show(ui, state),
        NodeKind::Bool => None,
        NodeKind::Text => None,
        NodeKind::Addition => None,
        NodeKind::Multiply => None,
        NodeKind::Print => None,
    }
}
