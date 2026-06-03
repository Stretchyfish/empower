use crate::compiler::Instruction;
use crate::value::Value;
use crate::node_graph::port::PortDefinition;

use super::NodeKind;
use super::DrawnStateResponse;
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

    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        Vec::new()
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![
            PortDefinition::new_input_execution_port(),
        ]
    }

    fn draw_state(&mut self, _: &mut egui::Ui) -> DrawnStateResponse {
        DrawnStateResponse::None
    }

    fn compile(&self) -> Vec<Instruction> {
        Vec::new()
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Normal
    }
}
