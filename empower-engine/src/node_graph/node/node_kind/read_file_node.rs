use std::path::Path;

use serde::{Deserialize, Serialize};

use super::NodeKind;
use crate::{assets::{AssetId, AssetKind, Assets}, compiler::{CompilePlan, CompilePlanStep, Instruction, RegisterAddress}, node_graph::{node::SettingsKind, port::PortDefinition}, project::ProjectState, value::Value};

#[derive(Serialize, Deserialize, Clone)]
pub struct ReadFileNode
{
    pub read_mode: String,
    pub asset_id: Option<AssetId>,
    pub path: String,
}

impl ReadFileNode
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
}

#[typetag::serde]
impl NodeKind for ReadFileNode
{
    fn name(&self) -> &str { "read file" }
    fn size(&self) -> egui::Vec2 { egui::vec2(300.0, 360.0) }
    fn settings(&mut self) -> Vec<SettingsKind<'_>>
    {
        let detail = match self.read_mode.as_str()
        {
            "asset" => Some( SettingsKind::Asset { label: "asset", value: &mut self.asset_id, kind: AssetKind::Json }),
            "global" => Some( SettingsKind::FilePath { label: "path", value: &mut self.path }),
            "relative" => Some( SettingsKind::Text { label: "path", value: &mut self.path }),
            _ => None,
        };

        let mut settings = vec![SettingsKind::Choice {
            label: "read mode",
            value: &mut self.read_mode,
            options: vec!["asset", "global", "relative"],
        }];
        settings.extend(detail);

        settings
    }

    fn input_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![PortDefinition::new_input_execution_port("")]
    }
    fn output_port_definitions(&self) -> Vec<PortDefinition>
    {
        vec![PortDefinition::new_output_execution_port(""), PortDefinition::new_output_data_port("text".to_string(), vec![Value::Text(String::new())])]
    }
    fn compile(&self, _: &Vec<RegisterAddress>, output: &Vec<RegisterAddress>, assets: &Assets, project_state: &ProjectState) -> Result<CompilePlan, String>
    {
        let instruction = match self.read_mode.as_str()
        {
            "asset" =>
            {
                let Some(asset_id) = self.asset_id else
                {
                    return Err("ReadFile has no valid asset id during compilation".to_string());
                };
                if !assets.loaded_assets.loaded_json.contains_key(&asset_id)
                {
                    return Err(format!("ReadFile JSON asset with id {} is not loaded", asset_id));
                }
                Instruction::ReadAsset(output[0], asset_id)
            },
            "global" =>
            {
                let path = &self.path;
                if !Path::new(path).is_absolute()
                {
                    return Err(format!("ReadFile global path must be absolute: {}", path));
                }
                Instruction::ReadFile(output[0], Value::Text(path.clone()))
            },
            "relative" =>
            {
                let path = &self.path;
                let ProjectState::Saved(project_directory) = project_state else
                {
                    return Err("Save the project before reading a relative path".to_string());
                };
                if path.is_empty() || Path::new(path).is_absolute()
                {
                    return Err(format!("ReadFile relative path must be a non-empty relative path: {}", path));
                }
                let full_path = project_directory.join(path);
                Instruction::ReadFile(output[0], Value::Text(full_path.to_string_lossy().to_string()))
            },
            _ => return Err(format!("ReadFile has unknown read mode: {}", self.read_mode)),
        };
        Ok(CompilePlan { steps: vec![CompilePlanStep::Emit(instruction), CompilePlanStep::CompileFromOutput(0)] })
    }
}
