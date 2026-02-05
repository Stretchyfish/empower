use core::panic::{self, PanicMessage};
use std::{collections::HashMap, fs, path::PathBuf};

use chrono::ParseError;

mod asset;
pub use asset::{AssetId, Asset, AssetKind};
use egui::epaint::PathStroke;
use empower_engine::node_graph::node::node_kind::FilePathNode;

use crate::graph_editor::GraphEditor;

pub struct Project
{
    pub name: String,
    pub state: ProjectState,
    pub graph_editor: GraphEditor,
    pub assets: HashMap<AssetId, Asset>, // @TODO, should maybe be AssetMeta
    pub dirty: bool, // To detect if anything changed since last save
}

impl Project
{
    pub fn new() -> Self
    {
        
        let mut new_project = Self
        {
            name: String::from("Untitled"),
            state: ProjectState::Undefined, // @TODO, This is technically the parent directory, should maybe be changed or the name should be added
            graph_editor: GraphEditor::new(),
            assets: HashMap::new(),
            dirty: true, // since it is not saved yet
        };

        // new_project.setup_project_directory_2(temp_directory_root);

        new_project.setup_temporary_project_directory();

        new_project
    }

    pub fn save(&mut self)
    {
        match self.state
        {
            ProjectState::Undefined => {},
            ProjectState::Temporary(_) => self.save_as(),
            ProjectState::Saved(_) => {},
        }
        
        // @TODO, setup saving behavior
        println!("Saved project");

    }

    pub fn save_as(&mut self)
    {
        let folder_path = rfd::FileDialog::new()
                                            .set_title("Choose project location")
                                            .set_can_create_directories(true)
                                            .pick_folder();

        if folder_path.is_none()
        {
            panic!("Failed to get folder path!"); // @TODO, in the future, handle this error properly!
        }

        let parent_directory = folder_path.unwrap();
        let project_directory = parent_directory.join(self.name.clone());


        let original_location = match &self.state
        {
            ProjectState::Undefined => todo!(),
            ProjectState::Temporary(path_buf) => path_buf,
            ProjectState::Saved(path_buf) => path_buf,
        };

        let moved_project_result = fs::rename(original_location, project_directory.clone());

        match moved_project_result
        {
            Ok(_) => {},
            Err( error ) => 
            {
                match error.kind()
                {
                    std::io::ErrorKind::NotFound => println!("Original location: {}, new location: {}", original_location.to_string_lossy(), project_directory.to_string_lossy()),
                    _ => todo!(),
                }

                println!("Error when running save as on project : {}", error.to_string() );
                panic!("Failed to save project correctly");
            }
        }

        self.state = ProjectState::Saved( project_directory.clone() ); // @TODO, this kind of code is reused a lot, find a better way
    }

    pub fn load()
    {
        
    }

    fn setup_temporary_project_directory(&mut self)
    {
        // @TODO, this whole function is quite dangerous, take another look at it
            
        let temp_directory_root = std::env::temp_dir()
                                            .join("empower_studio_temporary_project");

        self.state = ProjectState::Temporary( temp_directory_root.clone() );
        
        // In case there already is a temp directory
        let remove_dir_result = fs::remove_dir_all( temp_directory_root.clone() );

        match remove_dir_result
        {
            Ok(_) => {},
            Err( error ) => 
            {
                println!("Error when removing temp directory : {}", error.to_string());
                println!("Setting up new temporary project instead");
            }
        }

        fs::create_dir( temp_directory_root.clone() );
        fs::create_dir(temp_directory_root.join("assets").clone());
        fs::create_dir(temp_directory_root.join("graphs"));
    }

    fn setup_project_directory_2(&mut self, setup_location: PathBuf)
    {
        self.state = ProjectState::Saved( setup_location ); // @TODO, this kind of code is reused a lot, find a better way

        let parent_directory = match &self.state
        {
            ProjectState::Undefined => todo!(),
            ProjectState::Temporary(_) => panic!("Tried to access location of a temporary project, should never happen."),
            ProjectState::Saved(path_buf) => path_buf,
        };

        let project_directory = parent_directory.join(self.name.clone());


        let create_project_result = fs::create_dir( project_directory.clone() );

        match create_project_result
        {
            Ok(_) => {},
            Err( error ) => 
            {
                println!("Error when creating project: {}", error.to_string());

                // @TODO, error handling if project already exists

                return;
            },
        }

        fs::create_dir(project_directory.join("assets").clone());
        fs::create_dir(project_directory.join("graphs"));
    }

    fn setup_project_directory(&mut self)
    {
        
        let file_path = rfd::FileDialog::new()
                                            .set_title("Choose project location")
                                            .set_can_create_directories(true)
                                            .pick_folder();

        if file_path.is_none()
        {
            panic!("Failed to get file path!"); // @TODO, in the future, handle this error properly!
        }

        self.state = ProjectState::Saved( file_path.unwrap() ); // @TODO, this kind of code is reused a lot, find a better way

        let parent_directory = match &self.state
        {
            ProjectState::Undefined => todo!(),
            ProjectState::Temporary(_) => panic!("Tried to access location of a temporary project, should never happen."),
            ProjectState::Saved(path_buf) => path_buf,
        };

        let project_directory = parent_directory.join(self.name.clone());


        let create_project_result = fs::create_dir( project_directory.clone() );

        match create_project_result
        {
            Ok(_) => {},
            Err( error ) => 
            {
                println!("Error when creating project: {}", error.to_string());

                // @TODO, error handling if project already exists

                return;
            },
        }

        fs::create_dir(project_directory.join("assets").clone());
        fs::create_dir(project_directory.join("graphs"));
    }

    pub fn import_asset(&mut self, path: &PathBuf)
    {
        let new_asset_id = self.assets.len() as AssetId;

        let new_asset = Asset
        {
            id: new_asset_id,
            path: path.clone(),
            kind: AssetKind::None,
        };

        self.assets.insert(new_asset_id, new_asset);
    }
}

#[derive(PartialEq, Eq)]
pub enum ProjectState
{
    Undefined, // @TODO, consider a way to remove this state
    Temporary( PathBuf ),
    Saved( PathBuf ),
}
