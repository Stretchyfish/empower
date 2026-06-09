use crate::compiler::Instruction;
use crate::node_graph::port::PortDefinition;
use crate::value::Value;

use super::NodeKind;
use super::DrawnStateResponse;
use super::ControlFlowKind;

#[derive(Clone)]
pub struct NumberNode
{
    
}

impl NodeKind for NumberNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {
            
        })
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "number"
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition> {
        Vec::new()
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition> {
        vec![
            PortDefinition::new_output_data_port("value", vec![ Value::Integer(0) ]),
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
