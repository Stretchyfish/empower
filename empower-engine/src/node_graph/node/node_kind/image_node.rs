use serde::{Deserialize, Serialize};

use super::NodeKind;
use crate::{assets::{AssetId, AssetKind, Assets}, compiler::{CompilePlan, CompilePlanStep, Instruction, RegisterAddress}, node_graph::{node::SettingsKind, port::PortDefinition}, project::ProjectState, value::Value};

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageNode
{
    pub image_asset_id: Option<AssetId>,
}

impl ImageNode
{
    pub fn new() -> Self
    {
        Self
        {
            image_asset_id: None,
        }
    }

    pub fn from(image_asset_id: AssetId) -> Self
    {
        Self
        {
            image_asset_id: Some(image_asset_id),
        }
    }
}

#[typetag::serde]
impl NodeKind for ImageNode
{
    fn name(&self) -> &str { "image" }
    fn size(&self) -> egui::Vec2 { egui::vec2(300.0, 220.0) }
    fn settings(&mut self) -> Vec<SettingsKind<'_>>
    {
        vec![
            SettingsKind::Asset { label: "image", value: &mut self.image_asset_id, kind: AssetKind::Image }
        ]
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition> { Vec::new() }
    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        if self.image_asset_id.is_none()
        {
            return Vec::new();
        }

        vec![
            PortDefinition::new_output_data_port("image".to_string(), vec![ Value::Image( self.image_asset_id ) ])
        ]
    }
    fn compile(&self, _: &Vec<RegisterAddress>, output: &Vec<RegisterAddress>, _: &Assets, _: &ProjectState) -> Result<CompilePlan, String>
    {
        Ok(CompilePlan { steps: vec![
            CompilePlanStep::Emit(Instruction::SetConst(output[0], Value::Image(self.image_asset_id)))]
        })
    }
}

