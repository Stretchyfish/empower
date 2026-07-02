use super::AssetId;

use std::path::PathBuf;

use super::asset_kind::AssetKind;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AssetMeta
{
    pub id: AssetId,
    pub relative_path: PathBuf,
    pub kind: AssetKind,
}

impl AssetMeta
{
    pub fn to_string(&self) -> String
    {
        format!("{}, {}, {}", self.id, self.relative_path.to_string_lossy(), self.kind.to_string())
    }
}
