use crate::assets::{AssetId, Assets};
use std::path::PathBuf;
use std::fs;

mod project_settings;
use project_settings::ProjectSettings;

mod project_state;
use project_state::ProjectState;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project
{
    pub name: String,
    pub state: ProjectState,
    pub location: PathBuf,

    pub assets: Assets,
    pub settings: ProjectSettings,

    pub entry_graph: AssetId,
}

impl Project
{
    pub fn new() -> Self
    {
        let temp_directory_root = std::env::temp_dir()
                                    .join("empower_studio_temporary_project");

        // In case there already is a temp directory
        let remove_dir_result = fs::remove_dir_all( temp_directory_root.clone() );
                                        
        match remove_dir_result
        {
            Ok(_) => {},
            Err( error ) => 
            {
                match error.kind()
                {
                    std::io::ErrorKind::NotFound =>
                    {
                        println!("Error when removing temp directory : {}", error.to_string());
                        println!("Setting up new temporary project instead");
                    },
                    _ =>
                    {
                        panic!("Error when removing temp directory (error case not handled) : {}", error.to_string());
                    }
                }
            }
        }

        let _ = fs::create_dir(temp_directory_root.clone() );
        let _ = fs::create_dir(temp_directory_root.join("assets").clone());

        let mut assets = Assets::new();

        let entry_graph_location = assets.create_node_graph(&temp_directory_root.join("assets"), Some( "entry_graph" )).unwrap(); // @TODO, find a better way of creating the entry graph

        let entry_graph_asset_id = assets.import_asset(&entry_graph_location);
        assets.load_asset(entry_graph_asset_id);

        Self
        {
            name: String::from("untitled"),
            state: ProjectState::Temporary, 
            location: temp_directory_root,
            assets: assets,
            settings: ProjectSettings::new(),
            entry_graph: entry_graph_asset_id,
        }
    }

    pub fn save(&mut self) -> Result<(), String>
    {
        match self.state
        {
            ProjectState::Temporary => self.save_as(),
            ProjectState::Saved =>
            {
                let project_file_path = self.location.join("project.json"); // @TODO, rename this project?
        
                let project_json_string = serde_json::to_string_pretty(&self).unwrap();

                let created_project_json_file_results = std::fs::File::create(&project_file_path);

                match created_project_json_file_results
                {
                    Ok(_) => {},
                    Err( error ) => println!("Error, failed to create file: {}", error.kind().to_string()),
                }
        
                let saving_project_file_results = std::fs::write(project_file_path, project_json_string);

                match saving_project_file_results
                {
                    Ok(_) => println!("Saved succesfully"),
                    Err( error ) => println!("Error, failed to save: {}", error.kind().to_string()),
                }

                self.assets.save();

                Ok(())
            },
        }
    }

    pub fn load(&mut self)
    {
        let folder_path = rfd::FileDialog::new()
                                        .set_title("Choose project location")
                                        .set_can_create_directories(true)
                                        .pick_folder();


        if folder_path.is_none()
        {
            panic!("Error when loading project directory");
        }

        // @TODO, do more checks here

        let corrected_path = folder_path.unwrap().join("project.json");
        let node_graph_json = std::fs::read_to_string(corrected_path).unwrap();
        *self = serde_json::from_str(&node_graph_json).unwrap();
    }

    pub fn load_specific_path(&mut self, project_path: PathBuf)
    {
        // @TODO, do more checks here

        let corrected_path = project_path.join("project.json");
        let node_graph_json = std::fs::read_to_string(corrected_path).unwrap();
        *self = serde_json::from_str(&node_graph_json).unwrap();
    }

    pub fn save_as(&mut self) -> Result<(), String> // @TODO, consider adding a name as input?
    {
        
        let folder_path = rfd::FileDialog::new()
                                        .set_title("Choose project location")
                                        .set_can_create_directories(true)
                                        .pick_folder();


        if folder_path.is_none()
        {
            return Err( "picked folder failed in save project".to_string() );
        }

        let parent_directory = folder_path.unwrap();
        let project_directory = parent_directory.join(self.name.clone());

        let copy_options = fs_extra::dir::CopyOptions::new().copy_inside(true);

        match self.state
        {
            ProjectState::Temporary =>
            {
                let moved_project_result = fs_extra::dir::copy(&self.location, &project_directory, &copy_options);

                match moved_project_result 
                {
                    Ok(_) => {},
                    Err( error ) => 
                    {
                        panic!("Error when running save as on project : {}", error.to_string() );
                    }
                }
            },
            ProjectState::Saved =>
            {
                let copy_project_result = fs_extra::dir::copy(&self.location, &project_directory, &copy_options);

                match copy_project_result
                {
                    Ok(_) => {},
                    Err( error ) => 
                    {
                        panic!("Error in save_as, when copying a project : {}", error.to_string());
                    },
                }
                
            },
        }

        self.state = ProjectState::Saved;
        self.location = project_directory;
        self.save()
    }
}

