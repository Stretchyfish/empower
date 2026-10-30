use serde::{Deserialize, Serialize};

use crate::{assets::{AssetId, AssetKind}, node_graph::{node::SettingsKind, port::PortDefinition}};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ReadFileState
{
    pub read_mode: String,
    pub asset_id: Option<AssetId>,
    pub path: String,
}

impl ReadFileState
{
    pub fn asset(asset_id: Option<AssetId>) -> Self
    {
        Self { read_mode: "asset".to_string(), asset_id, path: String::new() }
    }

    pub fn global_path(path: String) -> Self
    {
        Self { read_mode: "global".to_string(), asset_id: None, path }
    }

    pub fn relative_path(path: String) -> Self
    {
        Self { read_mode: "relative".to_string(), asset_id: None, path }
    }

    pub fn settings(&mut self) -> Vec<SettingsKind<'_>>
    {
        let uses_path = self.read_mode == "global" || self.read_mode == "relative";
        let uses_asset = self.read_mode == "asset";
        let uses_file_picker = self.read_mode == "global";
        let mut settings = vec![SettingsKind::Choice {
            label: "read mode",
            value: &mut self.read_mode,
            options: vec!["asset", "global", "relative"],
        }];

        if uses_asset
        {
            settings.push(SettingsKind::Asset { label: "asset", value: &mut self.asset_id, kind: AssetKind::Json });
        }
        else if uses_path
        {
            if uses_file_picker
            {
                settings.push(SettingsKind::FilePath { label: "path", value: &mut self.path });
            }
            else
            {
                settings.push(SettingsKind::Text { label: "path", value: &mut self.path });
            }
        }

        settings
    }

    pub fn get_input_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![PortDefinition::new_input_execution_port("")]
    }
}
