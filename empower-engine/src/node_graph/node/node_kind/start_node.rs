use crate::compiler::CompilerContext;
use crate::compiler::Instruction;
use crate::compiler::RegisterAddress;
use crate::node_graph::port::PortDefinition;

use super::NodeKind;
use super::ControlFlowKind;

#[derive(Clone)]
pub struct StartNode
{
    
}

impl NodeKind for StartNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "start"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(200.0, 170.0)
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        Vec::new()
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![
            PortDefinition::new_output_execution_port(),
        ]
    }

    fn compile(&self, _: &mut CompilerContext, _: Vec<RegisterAddress>, _: Vec<RegisterAddress>) {
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Normal
    }

    fn node_edits(&mut self) -> Option<&mut Vec<super::NodeEdit>> {
        None
    }

    fn sync_node_edit(&mut self, index: usize) {
        todo!()
    }
}
