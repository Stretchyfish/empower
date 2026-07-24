use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{assets::AssetId, node_graph::NodeGraph};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LoadedAssets
{
    pub loaded_node_graphs: HashMap<AssetId, NodeGraph>,
    pub loaded_images: HashMap<AssetId, egui::ColorImage>,
}

impl LoadedAssets
{
    pub fn new() -> Self
    {
        Self
        {
            loaded_node_graphs: HashMap::new(),
            loaded_images: HashMap::new(),
        }
    }
}
