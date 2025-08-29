use std::fmt;

use crate::node_graph::port::PortCompatability;
use crate::node_graph::PortValue;

use super::NodeValue;

mod start_node;
mod number_node;
mod bool_node;
mod text_node;
mod addition_node;
mod multiply_node;
mod print_node;

// @TODO, in the future this should probably get rewritten into a trait 
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKind
{
    #[default] Start,
    Number,
    Addition,
    Multiply,
    Text,
    Bool,
    Print,
}

impl fmt::Display for NodeKind
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}

pub fn get_node_input_port_compatabilities(node_kind: &NodeKind) -> Vec<PortCompatability>
{
    match node_kind
    { 
        NodeKind::Start => start_node::get_start_node_input_ports_compatabilities(),
        NodeKind::Number => number_node::get_number_node_input_ports_compatabilities(),
        NodeKind::Bool => bool_node::get_bool_node_input_ports_compatabilities(),
        NodeKind::Text => text_node::get_text_node_input_ports_compatabilities(),
        NodeKind::Addition => addition_node::get_addition_node_input_ports_compatabilities(),
        NodeKind::Multiply => multiply_node::get_multiply_node_input_ports_compatabilities(),
        NodeKind::Print => print_node::get_print_node_input_ports_compatabilities(),
    }
}

pub fn get_node_output_port_compatabilities(node_kind: &NodeKind) -> Vec<PortCompatability>
{
    match node_kind
    {
        NodeKind::Start => start_node::get_start_node_output_ports_compatabilities(),
        NodeKind::Number => number_node::get_number_node_output_ports_compatabilities(),
        NodeKind::Bool => bool_node::get_bool_node_output_ports_compatabilities(),
        NodeKind::Text => text_node::get_text_node_output_ports_compatabilities(),
        NodeKind::Addition => addition_node::get_addition_node_output_ports_compatabilities(),
        NodeKind::Multiply => multiply_node::get_multiply_node_output_ports_compatabilities(),
        NodeKind::Print => print_node::get_print_node_output_ports_compatabilities(),
    }
}

pub fn execute_node(node_kind: &NodeKind, node: &mut NodeValue, inputs: Vec<&PortValue>) -> Vec<PortValue>
{
    match node_kind
    {
        NodeKind::Start => start_node::execute_start_node(),
        NodeKind::Print => print_node::execute_print_node(inputs),
        NodeKind::Number => number_node::execute_number_node(node, inputs),
        NodeKind::Text => text_node::execute_text_node(inputs),
        NodeKind::Bool => bool_node::execute_bool_node(inputs),
        NodeKind::Addition => addition_node::execute_addition_node(inputs),
        NodeKind::Multiply => multiply_node::execute_multiply_node(inputs),
    }
}
