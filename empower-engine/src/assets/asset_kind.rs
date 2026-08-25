
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetKind
{
    Folder,
    NodeGraph,
    Image, // @TODO, this should probably be named Png, to seperate saving behavior between different image file types
    Json,
}

impl AssetKind
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            AssetKind::Folder => "folder".to_string(),
            AssetKind::NodeGraph => "node_graph".to_string(),
            AssetKind::Image => "image".to_string(),
            AssetKind::Json => "json".to_string(),
        }
    }
}
