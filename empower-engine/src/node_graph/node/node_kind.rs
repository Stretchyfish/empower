use std::fmt;

use crate::analyser::TextBuffer;
use crate::node_graph::PortValue;
use crate::node_graph::port::PortCompatability;

// @TODO, make these private again?
pub mod number_node;
pub mod start_node;
pub use number_node::NumberNodeState;
pub mod addition_node;
pub mod bool_node;
pub mod multiply_node;
pub mod print_node;
pub mod text_node;
pub mod vector_node;
pub use vector_node::VectorState;
pub mod file_path_node;
pub use file_path_node::FilePathState;
pub mod show_image_node;
pub mod math_graph_node;
pub use math_graph_node::MathGraphState;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum NodeKind {
    #[default]
    Start,
    Number(NumberNodeState),
    Bool,
    Text,
    Addition,
    Multiply,
    Print,
    Vector(VectorState),
    FilePath(FilePathState),
    ShowImage,
    MathGraph(MathGraphState),
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl NodeKind {
    pub fn name(&self) -> &'static str {
        match self {
            NodeKind::Start => start_node::get_name(),
            NodeKind::Number(_) => number_node::get_name(),
            NodeKind::Bool => bool_node::get_name(),
            NodeKind::Text => text_node::get_name(),
            NodeKind::Addition => addition_node::get_name(),
            NodeKind::Multiply => multiply_node::get_name(),
            NodeKind::Print => print_node::get_name(),
            NodeKind::Vector(_) => vector_node::get_name(),
            NodeKind::FilePath(_) => file_path_node::get_name(),
            NodeKind::ShowImage => show_image_node::get_name(),
            NodeKind::MathGraph(_) => math_graph_node::get_name(),
        }
    }

    pub fn input_ports_compatabilities(&self) -> Vec<PortCompatability> {
        match self {
            NodeKind::Start => start_node::get_start_node_input_ports_compatabilities(),
            NodeKind::Number(state) => {
                number_node::get_number_node_input_ports_compatabilities(state)
            }
            NodeKind::Bool => bool_node::get_bool_node_input_ports_compatabilities(),
            NodeKind::Text => text_node::get_text_node_input_ports_compatabilities(),
            NodeKind::Addition => addition_node::get_addition_node_input_ports_compatabilities(),
            NodeKind::Multiply => multiply_node::get_multiply_node_input_ports_compatabilities(),
            NodeKind::Print => print_node::get_print_node_input_ports_compatabilities(),
            NodeKind::Vector( state ) => vector_node::get_input_ports_compatabilities(state),
            NodeKind::FilePath(_) => file_path_node::get_node_input_ports_compatabilities(),
            NodeKind::ShowImage => show_image_node::get_node_input_ports_compatabilities(),
            NodeKind::MathGraph(_) => math_graph_node::get_node_input_ports_compatabilities(),
        }
    }

    pub fn output_ports_compatabilities(&self) -> Vec<PortCompatability> {
        match self {
            NodeKind::Start => start_node::get_start_node_output_ports_compatabilities(),
            NodeKind::Number(state) => {
                number_node::get_number_node_output_ports_compatabilities(state)
            }
            NodeKind::Bool => bool_node::get_bool_node_output_ports_compatabilities(),
            NodeKind::Text => text_node::get_text_node_output_ports_compatabilities(),
            NodeKind::Addition => addition_node::get_addition_node_output_ports_compatabilities(),
            NodeKind::Multiply => multiply_node::get_multiply_node_output_ports_compatabilities(),
            NodeKind::Print => print_node::get_print_node_output_ports_compatabilities(),
            NodeKind::Vector(_) => vector_node::get_output_ports_compatabilities(),
            NodeKind::FilePath(_) => file_path_node::get_node_output_ports_compatabilities(),
            NodeKind::ShowImage => show_image_node::get_node_output_ports_compatabilities(),
            NodeKind::MathGraph(_) => math_graph_node::get_node_output_ports_compatabilities(),
        }
    }

    pub fn setup(&mut self, inputs: Vec<&PortValue>)
    {
        match self
        {
            NodeKind::Start => {},
            NodeKind::Number(number_node_state) => {},
            NodeKind::Bool => {},
            NodeKind::Text => {},
            NodeKind::Addition => {},
            NodeKind::Multiply => {},
            NodeKind::Print => {},
            NodeKind::Vector(vector_state) => {},
            NodeKind::FilePath(file_path_state) => {},
            NodeKind::ShowImage => {},
            NodeKind::MathGraph(state) => math_graph_node::setup(state, inputs),
        }
    }

    pub fn execute(&self, inputs: Vec<&PortValue>, ui: &mut egui::Ui, log: &mut TextBuffer) -> Option<Vec<PortValue>> {
        // @TODO, consider changing these names to just say execute and same for the compatabilities
        match self {
            NodeKind::Start => start_node::execute_start_node(),
            NodeKind::Number(_) => number_node::execute_number_node(inputs),
            NodeKind::Bool => bool_node::execute_bool_node(inputs),
            NodeKind::Text => text_node::execute_text_node(inputs),
            NodeKind::Addition => addition_node::execute_addition_node(inputs),
            NodeKind::Multiply => multiply_node::execute_multiply_node(inputs),
            NodeKind::Print => print_node::execute_print_node(inputs, log),
            NodeKind::Vector(_) => vector_node::execute_vector_node(inputs),
            NodeKind::FilePath( state ) => file_path_node::execute(state),
            NodeKind::ShowImage => show_image_node::execute(inputs),
            NodeKind::MathGraph( state ) => math_graph_node::execute(state, ui),
        }
    }
}
