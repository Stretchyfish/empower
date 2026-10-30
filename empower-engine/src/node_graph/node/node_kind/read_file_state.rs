use serde::{Deserialize, Serialize};

use crate::{assets::AssetId, node_graph::{node::SettingsKind, port::PortDefinition}};

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
        let mut settings = vec![SettingsKind::Choice {
            label: "read mode",
            value: &mut self.read_mode,
            options: vec!["asset", "global", "relative"],
        }];

        if uses_path
        {
            settings.push(SettingsKind::Text { label: "path", value: &mut self.path });
        }

        settings
    }

    pub fn get_input_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![PortDefinition::new_input_execution_port("")]
    }
}
