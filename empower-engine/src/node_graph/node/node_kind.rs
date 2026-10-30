use std::path::Path;

use serde::{Deserialize, Serialize};

use super::SettingsKind;
use crate::{assets::Assets, compiler::{CompilePlan, CompilePlanStep, Instruction, RegisterAddress}, node_graph::port::PortDefinition, project::ProjectState, value::Value};

mod image_node_state;
pub use image_node_state::ImageState;

mod sub_graph_node_state;
pub use sub_graph_node_state::SubGraphState;

mod list_node_state;
pub use list_node_state::ListState;

mod loop_node_state;
pub use loop_node_state::LoopMode;

mod read_file_state;
pub use read_file_state::ReadFileState;

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
    ReadFile ( ReadFileState ),
}

impl NodeKind
{
    pub fn settings(&mut self) -> Vec<SettingsKind<'_>>
    {
        match self
        {
            NodeKind::ReadFile(state) => state.settings(),
            _ => Vec::new(),
        }
    }

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
            NodeKind::ReadFile(_) => "read file",
        }
    }

    pub fn size(&self) -> egui::Vec2
    {
        match self
        {
            NodeKind::Start => egui::vec2(180.0, 165.0),
            NodeKind::Print => egui::vec2(300.0, 220.0),
            NodeKind::Branch => egui::vec2(330.0, 220.0),
            NodeKind::Loop( mode ) => mode.get_size(),
            NodeKind::Wait => egui::vec2(300.0, 220.0),
            NodeKind::List( state ) => state.get_size(),
            NodeKind::Image(_) => egui::vec2(300.0, 220.0),
            NodeKind::ShowImage => egui::vec2(300.0, 220.0),
            NodeKind::MathGraph => egui::vec2(300.0, 220.0),
            NodeKind::ReadFile(_) => egui::vec2(300.0, 360.0),
            NodeKind::SubGraph( state ) => state.get_size(),
        }
    }

    pub fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            NodeKind::Start => Vec::new(),
            NodeKind::Print => vec![
                                                PortDefinition::new_input_execution_port(""),
                                                PortDefinition::new_input_data_port("value".to_string(), vec![ Value::Integer(0), Value::Float(0.0), Value::Text(String::new())]) ],
            NodeKind::Branch => vec![
                                                PortDefinition::new_input_execution_port(""),
                                                PortDefinition::new_input_data_port("condition".to_string(), vec![ Value::Bool(false) ]) ],
            NodeKind::Loop( state ) => state.get_input_port_definitions(),
            NodeKind::Wait => vec![
                                                PortDefinition::new_input_execution_port(""),
                                                PortDefinition::new_input_data_port("seconds".to_string(), vec![Value::Float(1.0)]) ],
            NodeKind::List( state ) => state.get_input_port_definitions(),
            NodeKind::Image( _ ) => Vec::new(),
            NodeKind::ShowImage => vec![
                                                    PortDefinition::new_input_execution_port(""),
                                                    PortDefinition::new_input_data_port("image".to_string(), vec![ Value::Image( None ) ])
                                                ],
            NodeKind::MathGraph => vec![
                                                    PortDefinition::new_input_execution_port(""),
                                                    PortDefinition::new_input_data_port("math graph".to_string(), vec![ Value::List( Vec::new() ) ])
                                                ],
            NodeKind::SubGraph( state ) => state.get_input_port_definitions(),
            NodeKind::ReadFile( state ) => state.get_input_port_definitions(),
        }
    }

    pub fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self
        {
            NodeKind::Start => vec![ PortDefinition::new_output_execution_port("") ],
            NodeKind::Print => Vec::new(),
            NodeKind::Branch => vec![
                                                PortDefinition::new_output_execution_port("true"),
                                                PortDefinition::new_output_execution_port("false") ],
            NodeKind::Loop( state ) => state.get_output_port_definitions(),
            NodeKind::Wait => vec![
                                                PortDefinition::new_output_execution_port("") ],
            NodeKind::List( state ) => state.get_output_port_definitions(),
            NodeKind::Image( state ) => state.get_output_port_definitions(),
            NodeKind::ShowImage => Vec::new(),
            NodeKind::MathGraph => Vec::new(),
            NodeKind::SubGraph( state ) => state.get_output_port_definitions(),
            NodeKind::ReadFile(_) => vec![
                                                PortDefinition::new_output_execution_port(""),
                                                PortDefinition::new_output_data_port("text".to_string(), vec![Value::Text(String::new())]),
            ],
        }
    }

    pub fn compile(&self, input_register_addresses: &Vec<RegisterAddress>, output_register_addresses: &Vec<RegisterAddress>, assets: &Assets, project_state: &ProjectState) -> Result<CompilePlan, String>
    {
        let plan = match self
        {
            NodeKind::Start => CompilePlan { steps: vec![ CompilePlanStep::CompileFromOutput(0) ] },
            NodeKind::Print => CompilePlan { steps: vec![ CompilePlanStep::Emit( Instruction::Print( input_register_addresses[0] ) ) ] },
            NodeKind::Branch => CompilePlan { steps: vec![
                                                                CompilePlanStep::JumpIfFalse(input_register_addresses[0], "else"),
                                                                CompilePlanStep::CompileFromOutput(0),
                                                                CompilePlanStep::Jump("end"),
                                                                CompilePlanStep::Mark("else"),
                                                                CompilePlanStep::CompileFromOutput(1),
                                                                CompilePlanStep::Mark("end"),
                                                            ]},
            NodeKind::Loop(LoopMode::Forever) => CompilePlan { steps: vec![
                CompilePlanStep::Mark("entry"),
                CompilePlanStep::CompileFromOutput(0),
                CompilePlanStep::Jump("entry"),
            ]},
            NodeKind::Loop(LoopMode::Range) => CompilePlan { steps: vec![
                CompilePlanStep::Emit(Instruction::Copy(input_register_addresses[0], output_register_addresses[1])),
                CompilePlanStep::Mark("check"),
                CompilePlanStep::Emit(Instruction::Compare(input_register_addresses[2], output_register_addresses[1], output_register_addresses[0])),
                CompilePlanStep::JumpIfTrue(output_register_addresses[0], "end"),
                CompilePlanStep::CompileFromOutput(0),
                CompilePlanStep::Emit(Instruction::Add(output_register_addresses[1], input_register_addresses[1], output_register_addresses[1])),
                CompilePlanStep::Jump("check"),
                CompilePlanStep::Mark("end"),
            ]},
            NodeKind::Wait => CompilePlan { steps: vec![
                CompilePlanStep::Emit(Instruction::Wait(input_register_addresses[0])),
                CompilePlanStep::CompileFromOutput(0),
            ]},
            NodeKind::List(_) => CompilePlan { steps: vec![
                CompilePlanStep::Emit(Instruction::CreateList(input_register_addresses.clone(), output_register_addresses[0])),
            ]},
            NodeKind::Image(state) => CompilePlan { steps: vec![
                CompilePlanStep::Emit(Instruction::SetConst(output_register_addresses[0], Value::Image(state.image_asset_id))),
            ]},
            NodeKind::ShowImage => CompilePlan { steps: vec![
                CompilePlanStep::Emit(Instruction::ShowImage(input_register_addresses[0])),
            ]},
            NodeKind::MathGraph => CompilePlan { steps: vec![
                CompilePlanStep::Emit(Instruction::ShowMathGraph(input_register_addresses[0])),
            ]},
            NodeKind::SubGraph(state) =>
            {
                match state.graph_asset_id
                {
                    Some(graph_id) => CompilePlan { steps: vec![
                        CompilePlanStep::QueueGraph(graph_id),
                        CompilePlanStep::Emit(Instruction::CallGraph(graph_id)),
                    ]},
                    None => CompilePlan { steps: Vec::new() },
                }
            },
            NodeKind::ReadFile(state) =>
            {
                let instruction = match state.read_mode.as_str()
                {
                    "asset" =>
                    {
                        let Some(asset_id) = state.asset_id else
                        {
                            return Err("ReadFile has no valid asset id during compilation".to_string());
                        };
                        if !assets.loaded_assets.loaded_json.contains_key(&asset_id)
                        {
                            return Err(format!("ReadFile JSON asset with id {} is not loaded", asset_id));
                        }
                        Instruction::ReadAsset(output_register_addresses[0], asset_id)
                    },
                    "global" =>
                    {
                        let path = &state.path;
                        if !Path::new(path).is_absolute()
                        {
                            return Err(format!("ReadFile global path must be absolute: {}", path));
                        }
                        Instruction::ReadFile(output_register_addresses[0], Value::Text(path.clone()))
                    },
                    "relative" =>
                    {
                        let path = &state.path;
                        let ProjectState::Saved(project_directory) = project_state else
                        {
                            return Err("Save the project before reading a relative path".to_string());
                        };
                        if path.is_empty() || Path::new(path).is_absolute()
                        {
                            return Err(format!("ReadFile relative path must be a non-empty relative path: {}", path));
                        }
                        let full_path = project_directory.join(path);
                        Instruction::ReadFile(output_register_addresses[0], Value::Text(full_path.to_string_lossy().to_string()))
                    },
                    _ => return Err(format!("ReadFile has unknown read mode: {}", state.read_mode)),
                };
                CompilePlan { steps: vec![
                    CompilePlanStep::Emit(instruction),
                    CompilePlanStep::CompileFromOutput(0),
                ]}
            },
        };

        Ok(plan)
    }
}

