use std::{fs, path::PathBuf};

use once_cell::sync::Lazy;

use crate::{compiler::Program, project::Project};

pub static TEMP_PROJECT_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    std::env::temp_dir().join("empower_studio_temporary_project")
});

pub fn create_temporary_project_directory(_project: &Project)
{

}

#[derive(Clone)]
pub struct ExportConfig
{
    pub use_custom_export_name: bool,
    pub custom_export_name: String, 
    pub export_path: Option<PathBuf>,
    pub platform: Option<PathBuf>,
    pub application_type: ApplicationType,
}

impl ExportConfig
{
    pub fn new() -> Self
    {
        Self
        {
            use_custom_export_name: false,
            custom_export_name: String::from("application"),
            export_path: None,
            platform: None,

            application_type: ApplicationType::Undefined,
        }
    }

    pub fn valid_to_export(&self) -> bool
    {
        self.export_path.is_some() && self.application_type != ApplicationType::Undefined && self.platform.is_some()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ApplicationType
{
    Undefined,
    Graphical,
    CLI
}

impl ApplicationType
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            ApplicationType::Undefined => String::from("Undefined"),
            ApplicationType::Graphical => String::from("Graphical"),
            ApplicationType::CLI => String::from("CLI"),
        }
    }
}

pub fn export(program: &Program, config: &ExportConfig) -> Result<(), &'static str>
{
    if !config.valid_to_export()
    {
        return Err("asked to export with an invalid export config");
    }

    let platform_path = config.platform.as_ref().unwrap(); // these unwraps are safe to do due to valid_to_export check
    let export_location = config.export_path.as_ref().unwrap();

    if !platform_path.exists()
    {
        panic!("Runtime when exporting doesn't exist: {}", platform_path.to_string_lossy().to_string());
    }

    let export_name = if config.use_custom_export_name
    {
        config.custom_export_name.clone()
    }
    else
    {
        String::from("test_export")
    };

    let export_directory = export_location.join(export_name.clone());
    
    let copy_options = fs_extra::dir::CopyOptions::new().copy_inside(true);
    let copy_runtime_result = fs_extra::dir::copy(platform_path.clone(), &export_directory, &copy_options);

    match copy_runtime_result
    {
        Ok(_) => {},
        Err( error ) =>
        {
            println!("Error when copying runtime: {}", error.to_string());
        },
    }

    let _ = fs::rename(export_directory.join(platform_path.file_name().unwrap()), export_directory.join(export_name.clone()));

    let new_runtime_location_before_name_change = if cfg!(target_os = "windows")
    {
        export_directory.join("empower-application.exe")
    }
    else
    {
        export_directory.join("empower-application")
    };

    let new_runtime_location_after_name_change = if cfg!(target_os = "windows")
    {
         export_directory.join(format!("{}.exe", export_name.clone()))
    }
    else
    {
         export_directory.join(export_name.clone())
    };

    let rename_exutable_result = fs::rename(new_runtime_location_before_name_change, new_runtime_location_after_name_change.clone());

    match rename_exutable_result
    {
        Ok(_) => {},
        Err( error ) =>
        {
            println!("Failed to rename exported executable due to error : {}", error.kind().to_string());
        },
    }

    let program_json = program.to_json();

    let program_json_file_path = export_directory.join("program.json");
    let created_program_json_file_results = std::fs::File::create(&program_json_file_path);

    match created_program_json_file_results 
    {
        Ok(_) => println!("Created file succesfully"),
        Err( error ) => println!("Error, failed to create file: {}", error.kind().to_string()),
    }
    
    let saving_node_graph_file_results = std::fs::write(program_json_file_path , program_json);

    match saving_node_graph_file_results
    {
        Ok(_) => println!("Saved succesfully"),
        Err( error ) => println!("Error, failed to save : {}", error.kind().to_string()),
    }

    Ok(())
}
