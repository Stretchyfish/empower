use crate::{compiler::{CompiledGraphContext, Instruction, RegisterAddress}, node_graph::{NodeEdit, node::node_kind::{ControlFlowKind, NodeState, NodeSyncResponse}, port::PortDefinition}, value::Value};

use super::NodeKind;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ShowMathGraph
{
    
}

#[typetag::serde]
impl NodeKind for ShowMathGraph
{
    fn new() -> Box<dyn NodeKind>where Self:Sized {
        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn NodeKind>  {
        Box::new( self.clone() )
    }

    fn name(&self) ->  &'static str {
        "show math graph"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(300.0, 280.0)
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>  {
        vec![
            PortDefinition::new_input_execution_port(),
            PortDefinition::new_input_data_port("math graph".to_string(), vec![ Value::List( Vec::new() ) ])
        ]
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>  {
        Vec::new()
    }

    fn node_edits(&mut self) -> Option< &mut Vec<NodeEdit> >  {
        None
    }

    fn sync_node_edit(&mut self, _: usize) -> NodeSyncResponse {
        NodeSyncResponse::Nothing
    }

    fn sync_node_state(&mut self, _: NodeState) {
    }

    fn compile(&self, ctx: &mut CompiledGraphContext, input_port_register_adresses: Vec<RegisterAddress>, _: Vec<RegisterAddress>) {
        ctx.add_instruction( Instruction::ShowMathGraph( input_port_register_adresses[0] ) );
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Linear
    }
}
