use serde::{Deserialize, Serialize};
use super::NodeKind;
use crate::{assets::Assets, compiler::{CompilePlan, CompilePlanStep, Instruction, RegisterAddress}, node_graph::port::PortDefinition, project::ProjectState, value::Value};

#[derive(Clone, Serialize, Deserialize)]
pub struct MathGraphNode;

#[typetag::serde]
impl NodeKind for MathGraphNode
{
    fn name(&self) -> &str { "math graph" }
    fn size(&self) -> egui::Vec2 { egui::vec2(300.0, 220.0) }
    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![
            PortDefinition::new_input_execution_port(""),
            PortDefinition::new_input_data_port("math graph".to_string(), vec![Value::List(Vec::new())])]
    }
    fn output_port_definitions(&self) -> Vec<PortDefinition> { Vec::new() }
    fn compile(&self, input: &Vec<RegisterAddress>, _: &Vec<RegisterAddress>, _: &Assets, _: &ProjectState) -> Result<CompilePlan, String>
    {
        Ok(CompilePlan { steps: vec![
            CompilePlanStep::Emit(Instruction::ShowMathGraph(input[0]))]
        })
    }
}
