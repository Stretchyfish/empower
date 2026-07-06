use crate::{assets::{AssetId, AssetKind}, compiler::{CompiledGraphContext, RegisterAddress}, node_graph::{NodeEdit, node::node_kind::{ControlFlowKind, NodeState, NodeSyncResponse}, port::PortDefinition}, value::Value};

use super::NodeKind;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageNode
{
    edit_state: Vec<NodeEdit>,
    image_asset_id: Option<AssetId>,
}

#[typetag::serde]
impl NodeKind for ImageNode
{
    fn new() -> Box<dyn NodeKind>where Self:Sized {

        Box::new( Self
        {
            edit_state: vec![ NodeEdit::AssetSelector { asset_id: None, kind: AssetKind::Image } ],
            image_asset_id: None,
        })
    }

    fn clone_box(&self) -> Box<dyn NodeKind>  {
        Box::new( self.clone() )
    }

    fn name(&self) ->  &'static str {
        "image"
    }

    fn size(&self) -> egui::Vec2 {
        egui::vec2(300.0, 280.0)
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>  {
        Vec::new()
    }

    fn output_port_definitions(&self) -> Vec<PortDefinition>  {

        if self.image_asset_id.is_none()
        {
            return Vec::new();
        }

        vec![
            PortDefinition::new_output_data_port("image".to_string(), vec![ Value::Image ])
        ]
    }

    fn node_edits(&mut self) -> Option< &mut Vec<NodeEdit> >  {
        Some(
            &mut self.edit_state
        )
    }

    fn sync_node_edit(&mut self, _: usize) -> NodeSyncResponse {

        self.image_asset_id = match self.edit_state[0]
        {
            NodeEdit::AssetSelector { asset_id, kind: _ } => asset_id,
            _ => None,
        };

        if self.image_asset_id.is_some()
        {
            return NodeSyncResponse::NodesStructureChanged;
        }

        NodeSyncResponse::Nothing
    }

    fn sync_node_state(&mut self, _: NodeState) {
        todo!()
    }

    fn compile(&self, ctx: &mut CompiledGraphContext, input_port_register_adresses: Vec<RegisterAddress>, output_port_register_adresses: Vec<RegisterAddress>) {
    }

    fn control_flow(&self) -> ControlFlowKind {
        ControlFlowKind::Normal
    }
}
