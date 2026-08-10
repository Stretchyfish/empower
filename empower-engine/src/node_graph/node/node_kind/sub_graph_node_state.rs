use serde::{Deserialize, Serialize};

use crate::assets::AssetId;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SubGraphState
{
    pub graph_asset_id: Option<AssetId>,
}

impl SubGraphState
{
    pub fn new() -> Self
    {
        Self
        {
            graph_asset_id: None,
        }
    }

    pub fn from(graph_asset_id: AssetId) -> Self
    {
        Self
        {
            graph_asset_id: Some(graph_asset_id),
        }
    }
}

