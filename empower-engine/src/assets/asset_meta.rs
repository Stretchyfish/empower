use super::AssetId;

use super::asset_kind::AssetKind;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AssetMeta
{
    pub id: AssetId,
    pub name: String,
    pub parent: Option<AssetId>,
    pub kind: AssetKind,
}

impl AssetMeta
{
    pub fn to_string(&self) -> String
    {
        format!("{}, {}, {:?}, {}", self.id, self.name, self.parent, self.kind.to_string())
    }
}
