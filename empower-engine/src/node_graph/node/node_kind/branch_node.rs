
use crate::{node_graph::port::PortDefinition, value::Value};

use super::NodeKind;

#[derive(Clone)]
pub struct BranchNode
{
    
}

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
            PortDefinition::new_input_data_port("value".to_string(), vec![ Value::Integer(0) ]),
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

    fn compile(&self, _: &mut crate::compiler::CompiledGraphContext, _: Vec<crate::compiler::RegisterAddress>, _: Vec<crate::compiler::RegisterAddress>) {
    }

    fn control_flow(&self) -> super::ControlFlowKind {
        super::ControlFlowKind::Normal
    }
}
