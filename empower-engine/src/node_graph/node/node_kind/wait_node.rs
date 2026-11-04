use serde::{Deserialize, Serialize};
use super::NodeKind;
use crate::{assets::Assets, compiler::{CompilePlan, CompilePlanStep, Instruction, RegisterAddress}, node_graph::port::PortDefinition, project::ProjectState, value::Value};

#[derive(Clone, Serialize, Deserialize)]
pub struct WaitNode;

#[typetag::serde]
impl NodeKind for WaitNode
{
    fn name(&self) -> &str { "wait" }
    fn size(&self) -> egui::Vec2 { egui::vec2(300.0, 220.0) }
    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![PortDefinition::new_input_execution_port(""), PortDefinition::new_input_data_port("seconds".to_string(), vec![Value::Float(1.0)])]
    }
    fn output_port_definitions(&self) -> Vec<PortDefinition> { vec![PortDefinition::new_output_execution_port("")] }
    fn compile(&self, input: &Vec<RegisterAddress>, _: &Vec<RegisterAddress>, _: &Assets, _: &ProjectState) -> Result<CompilePlan, String>
    {
        Ok(CompilePlan { steps: vec![CompilePlanStep::Emit(Instruction::Wait(input[0])), CompilePlanStep::CompileFromOutput(0)] })
    }
}
