use std::{fs, path::PathBuf};

use chrono::ParseError;

pub struct Project
{
    pub name: String,
    pub location: ProjectLocation,
    pub dirty: bool, // To detect if anything changed since last save
}

impl Project
{
    pub fn new() -> Self
    {
        Self
        {
            name: String::from("Untitled"),
            location: ProjectLocation::Temporary, // @TODO, This is technically the parent directory, should maybe be changed or the name should be added

            dirty: true, // since it is not saved yet
        }
    }

    pub fn save(&mut self)
    {
        // @TODO, this function is still very unsafe, needs a lot more safety checks
       
        if self.location == ProjectLocation::Temporary
        {
            self.setup_project_directory();
            return;
        }

        // @TODO, setup saving behavior
        println!("Saved project");

    }

    pub fn load()
    {
        
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

        self.location = ProjectLocation::Path( file_path.unwrap() ); // @TODO, this kind of code is reused a lot, find a better way

        let parent_directory = match &self.location
        {
            ProjectLocation::Temporary => panic!("Tried to access location of a temporary project, should never happen."),
            ProjectLocation::Path(path_buf) => path_buf,
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
}

#[derive(PartialEq, Eq)]
pub enum ProjectLocation
{
    Temporary,
    Path( PathBuf ),
}
