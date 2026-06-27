use crate::compiler::CompiledGraphContext;
use crate::compiler::Instruction;
use crate::compiler::RegisterAddress;
use crate::node_graph::NodeEdit;
use crate::node_graph::port::PortDefinition;
use crate::value::Value;
use super::ControlFlowKind;
use super::NodeSyncResponse; 
use super::NodeKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SubGraphNode
{
    
}

#[typetag::serde]
impl NodeKind for SubGraphNode
{
    fn new() -> Box<dyn NodeKind>where Self:Sized {

        Box::new( Self {
            
        })
    }

    fn clone_box(&self) -> Box<dyn NodeKind>  {

        Box::new( self.clone() )
    }

    fn name(&self) ->  &'static str {
        "sub graph"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(300.0, 220.0)
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>  {
        Vec::new()
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>  {
        Vec::new()
    }

    fn node_edits(&mut self) -> Option< &mut Vec<NodeEdit> >  {
        None
    }

    fn sync_node_edit(&mut self, index: usize) -> NodeSyncResponse {
        todo!()
    }

    fn compile(&self,ctx: &mut CompiledGraphContext,input_port_register_adresses:Vec<RegisterAddress> ,output_port_register_adresses:Vec<RegisterAddress>) {
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Normal
    }
}
