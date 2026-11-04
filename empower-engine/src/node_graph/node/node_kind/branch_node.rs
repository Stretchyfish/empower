use serde::{Deserialize, Serialize};
use super::NodeKind;
use crate::{assets::Assets, compiler::{CompilePlan, CompilePlanStep, RegisterAddress}, node_graph::port::PortDefinition, project::ProjectState, value::Value};

#[derive(Clone, Serialize, Deserialize)]
pub struct BranchNode;

#[typetag::serde]
impl NodeKind for BranchNode
{
    fn name(&self) -> &str { "branch" }
    fn size(&self) -> egui::Vec2 { egui::vec2(330.0, 220.0) }
    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![PortDefinition::new_input_execution_port(""), PortDefinition::new_input_data_port("condition".to_string(), vec![Value::Bool(false)])]
    }
    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![PortDefinition::new_output_execution_port("true"), PortDefinition::new_output_execution_port("false")]
    }
    fn compile(&self, input: &Vec<RegisterAddress>, _: &Vec<RegisterAddress>, _: &Assets, _: &ProjectState) -> Result<CompilePlan, String>
    {
        Ok(CompilePlan { steps: vec![
            CompilePlanStep::JumpIfFalse(input[0], "else"),
            CompilePlanStep::CompileFromOutput(0),
            CompilePlanStep::Jump("end"),
            CompilePlanStep::Mark("else"),
            CompilePlanStep::CompileFromOutput(1),
            CompilePlanStep::Mark("end"),
        ] })
    }
}
