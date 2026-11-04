use serde::{Deserialize, Serialize};

use super::NodeKind;
use crate::{assets::Assets, compiler::{CompilePlan, CompilePlanStep, Instruction, RegisterAddress}, node_graph::{node::SettingsKind, port::PortDefinition}, project::ProjectState, value::Value};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoopNode
{
    pub mode: String,
}

impl LoopNode
{
    pub fn forever() -> Self
    {
        Self { mode: "forever".to_string() }
    }

    pub fn ranged() -> Self
    {
        Self { mode: "ranged".to_string() }
    }
}

#[typetag::serde]
impl NodeKind for LoopNode
{
    fn name(&self) -> &str { "loop" }
    fn settings(&mut self) -> Vec<SettingsKind<'_>>
    {
        vec![
            SettingsKind::Choice {
                label: "mode",
                value: &mut self.mode,
                options: vec!["forever", "ranged"],
        }]
    }

    fn size(&self) -> egui::Vec2
    {
        match self.mode.as_str()
        {
            "ranged" => egui::vec2(480.0, 400.0),
            _ => egui::vec2(300.0, 220.0),
        }
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self.mode.as_str()
        {
            "ranged" =>
            {
                vec![
                        PortDefinition::new_input_execution_port(""),
                        PortDefinition::new_input_data_port("start".to_string(), vec![ Value::Integer( 0 ) ] ),
                        PortDefinition::new_input_data_port("interval".to_string(), vec![ Value::Integer( 1 ) ] ),
                        PortDefinition::new_input_data_port("stop".to_string(), vec![ Value::Integer( 10 ) ] ),
                ]
            },
            _ =>
            {
                vec![
                        PortDefinition::new_input_execution_port(""),
                ]
            },
        }
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        match self.mode.as_str()
        {
            "ranged" =>
            {
                vec![
                    PortDefinition::new_output_execution_port(""),
                    PortDefinition::new_output_data_port("condition".to_string(), vec![ Value::Bool( false ) ] ),
                    PortDefinition::new_output_data_port("value".to_string(), vec![ Value::Integer( 0 ) ] ),
                ]
            },
            _ =>
            {
                vec![
                    PortDefinition::new_output_execution_port(""),
                ]
            },
        }
    }
    fn compile(&self, input: &Vec<RegisterAddress>, output: &Vec<RegisterAddress>, _: &Assets, _: &ProjectState) -> Result<CompilePlan, String>
    {
        let steps = match self.mode.as_str()
        {
            "forever" => vec![
                CompilePlanStep::Mark("entry"),
                CompilePlanStep::CompileFromOutput(0),
                CompilePlanStep::Jump("entry")],
            "ranged" => vec![
                CompilePlanStep::Emit(Instruction::Copy(input[0], output[1])),
                CompilePlanStep::Mark("check"),
                CompilePlanStep::Emit(Instruction::Compare(input[2], output[1], output[0])),
                CompilePlanStep::JumpIfTrue(output[0], "end"),
                CompilePlanStep::CompileFromOutput(0),
                CompilePlanStep::Emit(Instruction::Add(output[1], input[1], output[1])),
                CompilePlanStep::Jump("check"),
                CompilePlanStep::Mark("end"),
            ],
            _ => return Err(format!("Unknown loop mode: {}", self.mode)),
        };
        Ok(CompilePlan { steps })
    }
}
