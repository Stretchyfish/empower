use serde::{Deserialize, Serialize};

use crate::{node_graph::{port::PortDefinition}, value::Value};

mod image_node_state;
pub use image_node_state::ImageState;

mod sub_graph_node_state;
pub use sub_graph_node_state::SubGraphState;

mod list_node_state;
pub use list_node_state::ListState;

mod loop_node_state;
pub use loop_node_state::LoopMode;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum NodeKind
{
    Start,
    Print,
    Branch,
    Loop( LoopMode ),
    Wait,
    List( ListState ),
    Image( ImageState ),
    ShowImage,
    MathGraph,
    SubGraph ( SubGraphState ),
}

impl NodeKind
{
    pub fn name(&self) -> &'static str
    {
        match self
        {
            NodeKind::Start => "start",
            NodeKind::Print => "print",
            NodeKind::Branch => "branch",
            NodeKind::Loop(_) => "loop",
            NodeKind::Wait => "wait",
            NodeKind::List(_) => "list",
            NodeKind::Image(_) => "image",
            NodeKind::ShowImage => "show image",
            NodeKind::MathGraph => "math graph",
            NodeKind::SubGraph(_) => "sub graph",
        }
    }

    pub fn size(&self) -> egui::Vec2
    {
        match self
        {
            NodeKind::Start => egui::vec2(300.0, 220.0),
            NodeKind::Print => egui::vec2(300.0, 220.0),
            NodeKind::Branch => egui::vec2(300.0, 220.0),
            NodeKind::Loop(_) => egui::vec2(300.0, 220.0),
            NodeKind::Wait => egui::vec2(300.0, 220.0),
            NodeKind::List(_) => egui::vec2(300.0, 220.0),
            NodeKind::Image(_) => egui::vec2(300.0, 220.0),
            NodeKind::ShowImage => egui::vec2(300.0, 220.0),
            NodeKind::MathGraph => egui::vec2(300.0, 220.0),
            NodeKind::SubGraph(_) => egui::vec2(300.0, 220.0),
        }
    }

    pub fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            NodeKind::Start => Vec::new(),
            NodeKind::Print => vec![
                                        PortDefinition::new_input_execution_port(),
                                        PortDefinition::new_input_data_port("value".to_string(), vec![ Value::Integer(0), Value::Float(0.0)]) ],
            NodeKind::Branch => vec![
                                        PortDefinition::new_input_execution_port(),
                                        PortDefinition::new_input_data_port("a".to_string(), vec![ Value::Bool(false) ]) ],
            NodeKind::Loop( state ) => state.get_input_port_definitions(),
            NodeKind::Wait => vec![
                                        PortDefinition::new_input_execution_port(),
                                        PortDefinition::new_input_data_port("seconds".to_string(), vec![Value::Float(1.0)]) ],
            NodeKind::List( state ) => state.get_input_port_definitions(),
            NodeKind::Image( _ ) => Vec::new(),
            NodeKind::ShowImage => vec![
                                            PortDefinition::new_input_execution_port(),
                                            PortDefinition::new_input_data_port("image".to_string(), vec![ Value::Image( None ) ])
                                        ],
            NodeKind::MathGraph => vec![
                                            PortDefinition::new_input_execution_port(),
                                            PortDefinition::new_input_data_port("math graph".to_string(), vec![ Value::List( Vec::new() ) ])
                                        ],
            NodeKind::SubGraph(_) => Vec::new(),
        }
    }

    pub fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            NodeKind::Start => vec![ PortDefinition::new_output_execution_port() ],
            NodeKind::Print => Vec::new(),
            NodeKind::Branch => vec![
                                        PortDefinition::new_output_execution_port(),
                                        PortDefinition::new_output_execution_port() ],
            NodeKind::Loop( state ) => state.get_output_port_definitions(),
            NodeKind::Wait => vec![
                                        PortDefinition::new_output_execution_port() ],
            NodeKind::List( state ) => state.get_output_port_definitions(),
            NodeKind::Image( state ) => state.get_output_port_definitions(),
            NodeKind::ShowImage => Vec::new(),
            NodeKind::MathGraph => Vec::new(),
            NodeKind::SubGraph(_) => Vec::new(),
        }
    }
}

