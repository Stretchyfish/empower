use std::fmt;

use crate::{NodeGraph, NodeGraphKey};

use super::NodeHandle;

mod creator;
mod execution;

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

pub fn create_node(node_kind: &NodeKind, node_graph: &mut NodeGraph) -> NodeHandle
{
    match node_kind
    {
        NodeKind::Start => creator::create_start_node(node_graph),
        NodeKind::Number => creator::create_number_node(node_graph),
        NodeKind::Bool => creator::create_bool_node(node_graph),
        NodeKind::Text => creator::create_text_node(node_graph),
        NodeKind::Print => creator::create_print_node(node_graph),
        NodeKind::Addition => creator::create_addition_node(node_graph),
        NodeKind::Multiply => creator::create_multiply_node(node_graph),
    }
}

pub fn execute_node(node_key: &NodeGraphKey, node_graph: &mut NodeGraph) -> bool
{
    if !node_graph.contains_node(node_key)
    {
        return false;
    }

    let node_to_execute;
    match node_graph.nodes.get(node_key)
    {
        Some( value ) => node_to_execute = value,
        None => return false,
    }

    match node_to_execute.kind
    {
        NodeKind::Start => todo!(), 
        NodeKind::Print => execution::execute_print_node(node_key, node_graph),
        _ => todo!(),
    }
}
