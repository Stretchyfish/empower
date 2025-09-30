use empower_engine::node_graph::node::node_kind::start_node;
use empower_engine::node_graph::port::PortValue;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;

use empower_engine::node_graph::node::node_kind::NodeKind;
use empower_engine::node_graph::node::node_kind::NodeKind2;

pub mod display_start_node;
pub mod display_number_node;
pub mod display_print_node;
pub mod display_text_node;
pub mod display_bool_node;
pub mod display_addition_node;
pub mod display_multiplication_node;

pub fn get_display_node_size(node_kind: &NodeKind2) -> egui::Vec2
{
    match node_kind
    {
        NodeKind2::Start => display_start_node::get_start_display_node_size(),
        NodeKind2::Number(_) => display_number_node::get_display_node_size(),
        NodeKind2::Bool => display_bool_node::get_display_node_size(),
        NodeKind2::Text => display_text_node::get_display_text_node_size(),
        NodeKind2::Addition => display_addition_node::get_display_node_size(),
        NodeKind2::Multiply => display_multiplication_node::get_display_node_size(),
        NodeKind2::Print => display_print_node::get_display_print_node_size(),
    }
}

pub fn get_state_size(node_kind: &NodeKind2) -> egui::Vec2
{
    match node_kind
    {
        NodeKind2::Start => display_start_node::get_start_node_state_size(),
        NodeKind2::Number(_) => display_number_node::get_state_size(),
        NodeKind2::Bool => display_bool_node::get_state_size(),
        NodeKind2::Text => display_text_node::get_display_text_node_state_size(),
        NodeKind2::Addition => display_addition_node::get_state_size(),
        NodeKind2::Multiply => display_multiplication_node::get_state_size(), // @TODO, fix file naming
        NodeKind2::Print => display_print_node::get_display_print_node_state_size(),
    }
}

pub fn get_display_input_ports(node_kind: &NodeKind2, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    match node_kind
    {
        NodeKind2::Start => display_start_node::get_start_node_display_input_ports(),
        NodeKind2::Number(_) => display_number_node::get_display_input_ports(inputs),
        NodeKind2::Bool => display_bool_node::get_display_input_ports(inputs),
        NodeKind2::Text => display_text_node::get_display_text_node_input_ports(inputs),
        NodeKind2::Addition => display_addition_node::get_display_input_ports(inputs),
        NodeKind2::Multiply => display_multiplication_node::get_display_input_ports(inputs),
        NodeKind2::Print => display_print_node::get_display_print_node_input_ports(inputs),
    }
}

pub fn get_display_output_ports(node_kind: &NodeKind2, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    match node_kind
    {
        NodeKind2::Start => display_start_node::get_start_node_display_output_ports(outputs),
        NodeKind2::Number(_) => display_number_node::get_display_output_ports(outputs),
        NodeKind2::Bool => display_bool_node::get_display_output_ports(outputs),
        NodeKind2::Text => display_text_node::get_display_text_node_output_ports(outputs),
        NodeKind2::Addition => display_addition_node::get_display_output_ports(outputs),
        NodeKind2::Multiply => display_multiplication_node::get_display_output_ports(outputs),
        NodeKind2::Print => display_print_node::get_display_print_node_output_ports(),
    }
}

pub fn show(ui: &mut egui::Ui, node_kind: &NodeKind2) -> Option<NodeKind2>
{
    match node_kind
    {
        NodeKind2::Start => None,
        NodeKind2::Number( state ) => display_number_node::show(ui, state),
        NodeKind2::Bool => None,
        NodeKind2::Text => None,
        NodeKind2::Addition => None,
        NodeKind2::Multiply => None,
        NodeKind2::Print => None,
    }
}

pub trait DisplayNodeKind
{
    fn get_display_node_size(&self) -> egui::Vec2;
    fn get_state_size(&self) -> egui::Vec2;
    fn get_display_input_ports(&self, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue>;
    fn get_display_output_ports(&self, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue>;
    fn show(&self, ui: &mut egui::Ui, node_kind: &Box<dyn NodeKind>);
}
