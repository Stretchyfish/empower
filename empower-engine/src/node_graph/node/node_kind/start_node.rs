use serde::{Deserialize, Serialize};
use super::NodeKind;
use crate::{assets::Assets, compiler::{CompilePlan, CompilePlanStep, RegisterAddress}, node_graph::port::PortDefinition, project::ProjectState};

#[derive(Clone, Serialize, Deserialize)]
pub struct StartNode;

#[typetag::serde]
impl NodeKind for StartNode
{
    fn name(&self) -> &str { "start" }
    fn size(&self) -> egui::Vec2 { egui::vec2(180.0, 165.0) }
    fn input_port_definitions(&self) -> Vec<PortDefinition> { Vec::new() }
    fn output_port_definitions(&self) -> Vec<PortDefinition> { vec![PortDefinition::new_output_execution_port("")] }
    fn compile(&self, _: &Vec<RegisterAddress>, _: &Vec<RegisterAddress>, _: &Assets, _: &ProjectState) -> Result<CompilePlan, String>
    {
        Ok(CompilePlan { steps: vec![CompilePlanStep::CompileFromOutput(0)] })
    }
}
