use serde::{Deserialize, Serialize};

use crate::{assets::AssetId, node_graph::port::PortDefinition, value::Value};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ImageState
{
    pub image_asset_id: Option<AssetId>,
}

impl ImageState
{
    pub fn new() -> Self
    {
        Self
        {
            image_asset_id: None,
        }
    }

    pub fn get_output_port_definitions(&self) -> Vec<PortDefinition>
    {
        if self.image_asset_id.is_none()
        {
            return Vec::new();
        }

        vec![
            PortDefinition::new_output_data_port("image".to_string(), vec![ Value::Image( self.image_asset_id ) ])
        ]
    }
}

