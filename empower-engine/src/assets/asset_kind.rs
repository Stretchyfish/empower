
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetKind
{
    Graph,
}

impl AssetKind
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            AssetKind::Graph => "graph".to_string(),
        }
    }
}
