use std::{fs, path::PathBuf};

pub mod graph_editor;
pub use graph_editor::GraphEditor;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project
{
    pub name: String,
    pub state: ProjectState, // @TODO, seperate path and state now that path is always present
    pub graph_editor: GraphEditor,
    pub location: PathBuf,
    pub dirty: bool, // To detect if anything changed since last save
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

        Self
        {
            name: String::from("untitled"),
            state: ProjectState::Temporary, 
            graph_editor: GraphEditor::new(),
            location: temp_directory_root,
            dirty: true, // since it is not saved yet
        }
    }

    pub fn save(&mut self)
    {
        match &self.state
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
            },
        }
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

        let copy_options = fs_extra::dir::CopyOptions::new().copy_inside(true);

        match &self.state
        {
            ProjectState::Temporary => // If is temporary, copy the temporary project to the desired location 
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
            }
        };

        self.state = ProjectState::Saved;
        self.location = project_directory;
        self.save();
    }

    pub fn load(project_path: PathBuf) -> Self
    {
        // @TODO, do more checks here
        let corrected_path = project_path.join("project.json");
        let node_graph_json = std::fs::read_to_string(corrected_path).unwrap();
        serde_json::from_str(&node_graph_json).unwrap()
    }

    pub fn import_asset(&mut self, path: &PathBuf)
    {
        let project_path = self.location.clone();
        let file_name = path.file_name().unwrap();

        let new_file_location = project_path.join("assets").join(file_name.to_str().unwrap());

        let file_copy_result = fs::copy(path, new_file_location.clone());

        match file_copy_result
        {
            Ok(_) => {},
            Err( error ) => println!("Error when copying imported file: {}", error.kind().to_string()),
        }
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

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProjectState
{
    Temporary,
    Saved,
}
