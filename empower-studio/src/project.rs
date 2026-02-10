use std::{collections::HashMap, fs, path::PathBuf};

mod asset;
pub use asset::{AssetId, Asset, AssetKind};
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
            name: String::from("untitled"),
            state: ProjectState::Undefined, // @TODO, This is technically the parent directory, should maybe be changed or the name should be added
            graph_editor: GraphEditor::new(),
            assets: HashMap::new(),
            dirty: true, // since it is not saved yet
        };

        new_project.new_project();

        new_project
    }

    pub fn new_project(&mut self)
    {
        // @TODO, this whole function is quite dangerous, take another look at it
            
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

        fs::create_dir(temp_directory_root.clone() );
        fs::create_dir(temp_directory_root.join("assets").clone());

        self.state = ProjectState::Temporary( temp_directory_root.clone() );
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
            println!("Failed to get folder path!"); // @TODO, in the future, handle this error properly!
            return;
        }

        let parent_directory = folder_path.unwrap();
        let project_directory = parent_directory.join(self.name.clone());


        match &self.state
        {
            ProjectState::Undefined => todo!(),
            ProjectState::Temporary(original_location) => 
            {
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
            },
            ProjectState::Saved(original_location) => 
            {
                let copy_project_result = fs::copy(original_location, project_directory.clone());

                match copy_project_result
                {
                    Ok(_) => {},
                    Err( error ) => 
                    {
                        panic!("Error in save_as, when copying a project : {}", error.to_string());
                    },
                }
            }
        };

        self.state = ProjectState::Saved( project_directory.clone() ); // @TODO, this kind of code is reused a lot, find a better way
    }

    pub fn load()
    {
        
    }

    pub fn import_asset(&mut self, path: &PathBuf)
    {
        let project_path = match &self.state
        {
            ProjectState::Undefined => return,
            ProjectState::Temporary(path_buf) => path_buf,
            ProjectState::Saved(path_buf) => path_buf,
        };

        let file_name = path.file_name().unwrap();

        // println!("File prefix: {}", path.file_prefix().unwrap().to_str().unwrap());

        // return;

        let new_file_location = project_path.join("assets").join(file_name.to_str().unwrap());

        let file_copy_result = fs::copy(path, new_file_location.clone());

        match file_copy_result
        {
            Ok(_) => {},
            Err( error ) => println!("Error when copying imported file: {}", error.kind().to_string()),
        }
        
        // @TODO, this is a dangerous way of assigning ids!
        let new_asset_id = self.assets.len() as AssetId;

        let new_asset = Asset
        {
            id: new_asset_id,
            path: path.clone(),
            kind: AssetKind::None,
        };

        self.assets.insert(new_asset_id, new_asset);
    }

    pub fn create_file(&mut self, path: &PathBuf)
    {
        let create_file_result = fs::File::create(path);
        match create_file_result
        {
            Ok(_) => {},
            Err( error ) => 
            {
                println!("Error when creating file : {}", error.kind().to_string());
            },
        }
    }

    pub fn create_folder(&mut self, path: &PathBuf)
    {
        let create_directory_result = fs::create_dir(path);

        match create_directory_result
        {
            Ok(_) => {},
            Err( error ) => 
            {
                println!("Error when creating directory in create folder : {}", error.kind().to_string());
            },
        }
    }

    pub fn rename_file(&mut self, original_path: PathBuf, new_path: PathBuf)
    {
        let rename_file_result = fs::rename(original_path, new_path);

        match rename_file_result
        {
            Ok(_) => {},
            Err( error ) => panic!("Tried to rename a file, and failed because : {}", error.kind().to_string()),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ProjectState
{
    Undefined, // @TODO, consider a way to remove this state
    Temporary( PathBuf ),
    Saved( PathBuf ),
}
