use crate::compiler::CompiledGraphContext;
use crate::compiler::Instruction;
use crate::compiler::RegisterAddress;
use crate::node_graph::port::PortDefinition;
use crate::value::Value;
use super::ControlFlowKind;
use super::NodeSyncResponse; 
use super::NodeKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WaitNode
{

}

#[typetag::serde]
impl NodeKind for WaitNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self
        {

        })
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "wait"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(300.0, 220.0)
    }

    fn input_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {
        vec![
            PortDefinition::new_input_execution_port(),
            PortDefinition::new_input_data_port("seconds".to_string(), vec![Value::Float(1.0)]),
        ]
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition> {
        vec![
            PortDefinition::new_output_execution_port(),
        ]
    }

    fn node_edits(&mut self) -> Option<&mut Vec<super::NodeEdit>> {
        None
    }

    fn sync_node_edit(&mut self, _: usize) -> NodeSyncResponse {
        NodeSyncResponse::Nothing
    }

    fn compile(&self, ctx: &mut CompiledGraphContext, input_port_register_adresses: Vec<RegisterAddress>, _: Vec<RegisterAddress>) {

        ctx.add_instruction(
            Instruction::Wait(input_port_register_adresses[0])
        );
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Normal
    }
}
