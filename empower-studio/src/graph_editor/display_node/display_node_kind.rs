use empower_engine::node_graph::port::PortValue;

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;

pub mod display_start_node;
pub mod display_number_node;
pub mod display_print_node;
pub mod display_text_node;
pub mod display_bool_node;
pub mod display_addition_node;
pub mod display_multiplication_node;

pub trait DisplayNodeKind
{
    fn get_display_node_size(&self) -> egui::Vec2;
    fn get_display_input_ports(&self, inputs: Vec<&PortValue>) -> Vec<DisplayPortValue>;
    fn get_display_output_ports(&self, outputs: Vec<&PortValue>) -> Vec<DisplayPortValue>;
}
