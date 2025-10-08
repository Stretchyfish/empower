use empower_engine::node_graph::{port::PortValue};

use crate::graph_editor::{display_port::display_port_value::DisplayPortValue};

use empower_engine::node_graph::node::node_kind::NodeKind;

pub mod display_start_node;
pub mod display_number_node;
pub mod display_print_node;
pub mod display_text_node;
pub mod display_bool_node;
pub mod display_addition_node;
pub mod display_multiplication_node;
pub mod display_vector_node;
pub use display_vector_node::DisplayVectorState;
pub mod display_file_path_node;
pub mod display_show_image_node;
pub mod display_math_graph_node;

#[derive(Clone, PartialEq, Eq)]
pub enum DisplayState
{
    None,
    Vector(DisplayVectorState),
}

pub fn get_display_state(node_kind: &NodeKind) -> DisplayState
{
    match  node_kind
    {
        NodeKind::Start => DisplayState::None,
        NodeKind::Number(_) => DisplayState::None,
        NodeKind::Bool => DisplayState::None,
        NodeKind::Text => DisplayState::None,
        NodeKind::Addition => DisplayState::None,
        NodeKind::Multiply => DisplayState::None,
        NodeKind::Print => DisplayState::None,
        NodeKind::Vector(_) => DisplayState::Vector( DisplayVectorState::new() ),
        NodeKind::FilePath(_) => DisplayState::None,
        NodeKind::ShowImage => DisplayState::None,
        NodeKind::MathGraph => DisplayState::None,
    }
}

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
        NodeKind::Print => display_print_node::get_display_node_size(),
        NodeKind::Vector( state ) => display_vector_node::get_display_node_size(state),
        NodeKind::FilePath(_) => display_file_path_node::get_display_node_size(),
        NodeKind::ShowImage => display_show_image_node::get_display_node_size(),
        NodeKind::MathGraph => display_math_graph_node::get_display_node_size(),
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
        NodeKind::Print => display_print_node::get_display_node_state_size(),
        NodeKind::Vector(_) => display_vector_node::get_display_node_state_size(),
        NodeKind::FilePath(_) => display_file_path_node::get_state_size(),
        NodeKind::ShowImage => display_show_image_node::get_display_node_state_size(),
        NodeKind::MathGraph => display_math_graph_node::get_display_node_state_size(),
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
        NodeKind::Print => display_print_node::get_display_node_input_ports(inputs),
        NodeKind::Vector(_) => display_vector_node::get_display_node_input_ports(inputs),
        NodeKind::FilePath(_) => display_file_path_node::get_display_input_ports(),
        NodeKind::ShowImage => display_show_image_node::get_display_node_input_ports(inputs),
        NodeKind::MathGraph => display_math_graph_node::get_display_node_input_ports(inputs),

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
        NodeKind::Print => display_print_node::get_display_node_output_ports(),
        NodeKind::Vector(_) => display_vector_node::get_display_node_output_ports(outputs),
        NodeKind::FilePath(_) => display_file_path_node::get_display_output_ports(outputs),
        NodeKind::ShowImage => display_show_image_node::get_display_node_output_ports(outputs),
        NodeKind::MathGraph => display_math_graph_node::get_display_node_output_ports(outputs),
    }
}

// @TODO, update the naming between view and show
pub fn show(ui: &mut egui::Ui, node_kind: &NodeKind, display_state: &DisplayState) -> Option<(NodeKind, DisplayState)>
{
    // @TODO, this could potentially be expensive, be aware of that!
    let mut node_kind_to_modify = node_kind.clone(); 
    let mut display_state_to_modify = display_state.clone();
    
    match &mut node_kind_to_modify
    {
        NodeKind::Start => {},
        NodeKind::Number( state ) => display_number_node::show(ui, state),
        NodeKind::Bool => {},
        NodeKind::Text => {},
        NodeKind::Addition => {},
        NodeKind::Multiply => {},
        NodeKind::Print => {},
        NodeKind::Vector( state ) => 
        {
            let display_vector_state = match &mut display_state_to_modify
            {
                DisplayState::Vector( state ) => state,
                _ => panic!("The display state of the vector is wrong!"),
            };

            display_vector_node::show(ui, state, display_vector_state);
        },
        NodeKind::FilePath( state ) => display_file_path_node::show(ui, state),
        NodeKind::ShowImage => {},
        NodeKind::MathGraph => {},
    };

    if *node_kind == node_kind_to_modify && *display_state == display_state_to_modify
    {
        return None;
    }

    return Some( (node_kind_to_modify, display_state_to_modify) )
}
