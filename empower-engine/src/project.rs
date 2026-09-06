use crate::assets::{ASSET_FOLDER_ASSET_ID, AssetId, AssetKind, Assets};
use std::{fs, path::PathBuf};

mod project_settings;
use project_settings::ProjectSettings;

mod project_state;
pub use project_state::ProjectState;

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
        Project::with_name("untitled")
    }

    pub fn with_name(name: &str) -> Self
    {
        let default_project_name = name.to_string();
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

    

    pub fn save(&mut self, location: Option<PathBuf>) -> Result<PathBuf, String> // @TODO, find a way to make the modified assets optional
    {
        match &mut self.state
        {
            ProjectState::Temporary =>
            {
                if location.is_none()
                {
                    return Err("cannot save temporary project without a path.".to_string());
                }

                let location = location.as_ref().unwrap();

                let create_project_directory_result = self.create_project_directory(location);

                if create_project_directory_result.is_err()
                {
                    return Err(create_project_directory_result.err().unwrap());
                }

                self.state = ProjectState::Saved( create_project_directory_result.ok().unwrap() );
            },
            ProjectState::Saved(path_buf) =>
            {
                if location.is_some()
                {
                    let location = location.as_ref().unwrap();
                    if path_buf != location
                    {
                        let create_project_directory_result = self.create_project_directory(location);

                        if create_project_directory_result.is_err()
                        {
                            return Err(create_project_directory_result.err().unwrap());
                        }

                        self.state = ProjectState::Saved( create_project_directory_result.ok().unwrap() );
                    }
                }
            },
        }

        let location = match &self.state
        {
            ProjectState::Saved(path_buf) => path_buf,
            _ => panic!("Saving project tried to get location from temporary state"), // This is not possible to reach
        };

        let save_config_result = self.save_project_config_file(location);

        save_config_result?;

        // @TODO, The seperation is here a bit unclear, as the asset_meta now is saved by the project, so think of a better way
        
        let _ = self.assets.save_assets(location);

        Ok(location.clone())
    }

    fn create_project_directory(&self, location: &PathBuf) -> Result<PathBuf, String>
    {
        if !location.is_dir()
        {
            return Err(format!("cannot save temporary project at path ({}), because its not a directory.", location.to_string_lossy().to_string()));
        }

        let project_location = location.join(&self.name);

        let create_project_directory_result = fs::create_dir( &project_location );

        if create_project_directory_result.is_err()
        {
            return Err( format!("Cannot create project directory: {}", create_project_directory_result.err().unwrap().to_string()) );
        }

        Ok(project_location)
    }

    fn save_project_config_file(&self, location: &PathBuf) -> Result<(), String>
    {
        let project_config_location = location.join("project.json");

        let project_string_json = serde_json::to_string_pretty(&self).unwrap();

        {
            let project_file_json = std::fs::read_to_string(&project_config_location);

            match project_file_json
            {
                Ok( json ) =>
                {
                    // Check if we even need to save anything

                    if json == project_string_json
                    {
                        return Ok(()); // No need to save anything if they already are the same
                    }
                },
                Err( error ) =>
                {
                    let error_kind = error.kind();
                    match error_kind
                    {
                        std::io::ErrorKind::NotFound =>
                        {
                            let created_project_json_file_results = std::fs::File::create(&project_config_location);

                            if created_project_json_file_results.is_err()
                            {
                                return Err( format!("Cannot save project configuration due to error: {}", created_project_json_file_results.err().unwrap().to_string()) );
                            }
                        },
                        _ =>
                        {
                            return Err( format!("Cannot save project configuration due to error: {}", error_kind.to_string()) );
                        },
                    }
                },
            }
        }

        let write_project_config_result = std::fs::write(project_config_location, project_string_json);

        if write_project_config_result.is_err()
        {
            return Err( format!("Cannot save project configuration due to error: {}", write_project_config_result.err().unwrap().kind().to_string()) );
        }

        Ok(())
    }

    pub fn load(project_location: &PathBuf) -> Result<Self, String>
    {
        if !project_location.is_dir()
        {
            return Err(format!("Cannot load project at path: {}, is not a directory", project_location.to_string_lossy()));
        }

        let project_config_location = project_location.join("project.json");

        let project_config_read_result = std::fs::read_to_string(&project_config_location);

        if project_config_read_result.is_err()
        {
            return Err(format!("Cannot load project at path: {}, project config file cannot be read because: {}", project_config_location.to_string_lossy(), project_config_read_result.err().unwrap().to_string()));
        }

        let mut project: Project = serde_json::from_str(&project_config_read_result.unwrap()).unwrap();

        let loaded_all_asset_result = project.assets.load_all_assets(project_location);

        if loaded_all_asset_result.is_err()
        {
            return Err(format!("Could not load all assets because: {}", loaded_all_asset_result.err().unwrap()));
        }

        Ok(project)
    }

}

