use crate::compiler::CompiledGraphContext;
use crate::compiler::Instruction;
use crate::compiler::RegisterAddress;
use crate::node_graph::node::node_kind::NodeState;
use crate::node_graph::node::node_kind::NodeSyncResponse;
use crate::value::Value;
use crate::node_graph::port::PortDefinition;
use serde::{Deserialize, Serialize};

use super::NodeKind;
use super::ControlFlowKind;

#[derive(Clone, Serialize, Deserialize)]
pub struct PrintNode
{
    
}

#[typetag::serde]
impl NodeKind for PrintNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {

        Box::new( Self {} )
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "print"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(300.0, 220.0)
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![
            PortDefinition::new_input_execution_port(),
            PortDefinition::new_input_data_port("value".to_string(), vec![ Value::Integer(0), Value::Float(0.0)])
        ]
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        Vec::new()
    }

    fn compile(&self, ctx: &mut CompiledGraphContext, input_port_register_adresses: Vec<RegisterAddress>, _: Vec<RegisterAddress>)  {

        ctx.add_instruction(
            Instruction::Print( input_port_register_adresses[ 0 ])
        );
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Linear
    }

    fn node_edits(&mut self) -> Option<&mut Vec<super::NodeEdit>> {
        None
    }

    fn sync_node_edit(&mut self, _: usize) -> NodeSyncResponse {
        todo!()
    }

    fn sync_node_state(&mut self, _: NodeState) {
        
    }

}
