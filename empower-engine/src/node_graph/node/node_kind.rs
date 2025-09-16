use crate::analyser::TextBuffer;
use crate::node_graph::port::PortCompatability;
use crate::node_graph::PortValue;

// @TODO, make these private again?
pub mod start_node;
pub mod number_node;
pub mod bool_node;
pub mod text_node;
pub mod addition_node;
pub mod multiply_node;
pub mod print_node;

pub trait NodeKind 
{
    fn name(&self) -> &'static str;
    fn input_ports_compatabilities(&self) -> Vec<PortCompatability>;
    fn output_ports_compatabilities(&self) -> Vec<PortCompatability>;
    fn execute(&self, inputs: Vec<&PortValue>, log: &mut TextBuffer) -> Vec<PortValue>;
}

