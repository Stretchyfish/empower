use core::num;
use std::fmt;

use crate::NodeGraph;
use crate::NodeGraphKey;
use crate::node_graph::Node;
use crate::node_graph::Port;

use super::NodeHandle;

mod start_node;
mod number_node;
mod bool_node;
mod text_node;
mod addition_node;
mod multiply_node;
mod print_node;

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
    let new_node_key = node_graph.get_available_node_key();

    let input_ports_compatabilities = match node_kind
    { // @TODO, in the future this should probably get rewritten into a trait
        NodeKind::Start => start_node::get_start_node_input_ports_compatabilities(),
        NodeKind::Number => number_node::get_number_node_input_ports_compatabilities(),
        NodeKind::Bool => bool_node::get_bool_node_input_ports_compatabilities(),
        NodeKind::Text => text_node::get_text_node_input_ports_compatabilities(),
        NodeKind::Addition => addition_node::get_addition_node_input_ports_compatabilities(),
        NodeKind::Multiply => multiply_node::get_multiply_node_input_ports_compatabilities(),
        NodeKind::Print => print_node::get_print_node_input_ports_compatabilities(),
    };

    let output_ports_compatabilities = match node_kind
    {
        NodeKind::Start => start_node::get_start_node_output_ports_compatabilities(),
        NodeKind::Number => number_node::get_number_node_output_ports_compatabilities(),
        NodeKind::Bool => bool_node::get_bool_node_output_ports_compatabilities(),
        NodeKind::Text => text_node::get_text_node_output_ports_compatabilities(),
        NodeKind::Addition => addition_node::get_addition_node_output_ports_compatabilities(),
        NodeKind::Multiply => multiply_node::get_multiply_node_output_ports_compatabilities(),
        NodeKind::Print => print_node::get_print_node_output_ports_compatabilities(),
    };

    let mut new_input_port_keys = Vec::with_capacity(input_ports_compatabilities.len());
    for input_port_compatability in input_ports_compatabilities
    {
        let new_input_port_key = node_graph.get_available_input_port_key();
        let new_input_port = Port::new_input_port(
                                                new_input_port_key, 
                                                new_node_key, 
                                                input_port_compatability,
        );
        node_graph.input_ports.insert(new_input_port_key, new_input_port);

        new_input_port_keys.push(new_input_port_key);
    }

    let mut new_output_port_keys = Vec::with_capacity(output_ports_compatabilities.len());
    for output_port_compatability in output_ports_compatabilities
    {
        let new_output_port_key = node_graph.get_available_output_port_key();
        let new_output_port = Port::new_input_port(
                                                new_output_port_key, 
                                                new_node_key, 
                                                output_port_compatability,
        );
        node_graph.output_ports.insert(new_output_port_key, new_output_port);

        new_output_port_keys.push(new_output_port_key);
    }

    let new_node = Node::new(
                            new_node_key, 
                            *node_kind, 
                            new_input_port_keys.clone(), 
                            new_output_port_keys.clone()
    );
    node_graph.nodes.insert(new_node_key, new_node);

    NodeHandle::new(new_node_key, new_input_port_keys, new_output_port_keys)
}

pub fn execute_node(node_key: &NodeGraphKey, node_graph: &mut NodeGraph) -> bool
{
    let node_to_execute;
    match node_graph.nodes.get_mut(node_key)
    {
        Some( value ) => node_to_execute = value,
        None => return false,
    }

    let node_value = &mut node_to_execute.value;

    let mut input_port_values = Vec::with_capacity(node_to_execute.input_port_keys.len());
    for input_port_key in &node_to_execute.input_port_keys
    {
        let input_port;
        match node_graph.input_ports.get(input_port_key)
        {
            Some( value ) => input_port = value,
            None => return false,
        }

        input_port_values.push(&input_port.value);
    } 

    let executed_output_values = match node_to_execute.kind
    {
        NodeKind::Start =>
        {
            println!("Start Node");
            Vec::new()
        }
        NodeKind::Print => print_node::execute_print_node(input_port_values),
        NodeKind::Number => number_node::execute_number_node(node_value, input_port_values),
        NodeKind::Text => text_node::execute_text_node(input_port_values),
        NodeKind::Bool => bool_node::execute_bool_node(input_port_values),
        NodeKind::Addition => addition_node::execute_addition_node(input_port_values),
        NodeKind::Multiply => multiply_node::execute_multiply_node(input_port_values),
    };

    if executed_output_values.len() != node_to_execute.output_port_keys.len()
    {
        return false;
    }

    for (output_port_index, output_port_key) in node_to_execute.output_port_keys.iter().enumerate()
    {
        let output_port;
        match node_graph.output_ports.get_mut(output_port_key)
        {
            Some( value ) => output_port = value,
            None => return false,
        }

        let executed_output_value = &executed_output_values[output_port_index];

        if !output_port.compatability.contains_port_value_type(executed_output_value)
        {
            return false;
        }

        output_port.value = executed_output_value.clone(); // This needs to be a clone, or the value ends up on multiple input ports
    }

    true
}
