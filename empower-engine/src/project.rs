use crate::{assets::{ASSET_FOLDER_ASSET_ID, AssetId, AssetKind, Assets}, distribution::TEMP_PROJECT_LOCATION};
use std::path::PathBuf;

mod project_settings;
use project_settings::ProjectSettings;

mod project_state;
use project_state::ProjectState;

mod export_settings;
pub use export_settings::ExportSettings;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project
{
    pub name: String,
    pub state: ProjectState,

    pub assets: Assets,
    pub settings: ProjectSettings,

    pub entry_graph: AssetId,
}

impl Project
{
    pub fn new() -> Self
    {
        let default_project_name = "untitled".to_string();
        let mut assets = Assets::new(&default_project_name);
        let entry_graph_asset_id = assets.create_asset(Some( ASSET_FOLDER_ASSET_ID ), AssetKind::NodeGraph, "entry_graph");

        Self
        {
            name: default_project_name,
            state: ProjectState::Temporary, 
            assets: assets,
            settings: ProjectSettings::new(),
            entry_graph: entry_graph_asset_id.expect("failed to create entry_graph"), // This should be safe in new projects
        }
    }
}

