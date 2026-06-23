use crate::{compiler::Instruction, node_graph::port::PortDefinition, value::Value};

use serde::{Deserialize, Serialize};
use super::NodeKind;

#[derive(Clone, Serialize, Deserialize)]
pub struct BranchNode
{
    
}

#[typetag::serde]
impl NodeKind for BranchNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {

        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "branch"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(230.0, 215.0)
    }

    fn input_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {
        vec![
            PortDefinition::new_input_execution_port(),
            PortDefinition::new_input_data_port("a".to_string(), vec![ Value::Bool(false) ]),
        ]
    }

    fn output_port_definitions(&self) -> Vec<crate::node_graph::port::PortDefinition> {
        vec![
            PortDefinition::new_output_execution_port(),
            PortDefinition::new_output_execution_port(),
        ]
    }

    fn node_edits(&mut self) -> Option<&mut Vec<super::NodeEdit>> {
        None
    }

    fn sync_node_edit(&mut self, _: usize) -> super::NodeSyncResponse {
        todo!()
    }

    fn compile(&self, ctx: &mut crate::compiler::CompiledGraphContext, input_register_addresses: Vec<crate::compiler::RegisterAddress>, _: Vec<crate::compiler::RegisterAddress>) {
        let _ = ctx.add_instruction_placeholder( Instruction::JumpIfFalse(0, input_register_addresses[0]));
    }

    fn control_flow(&self) -> super::ControlFlowKind {
        super::ControlFlowKind::Branch
    }
}
