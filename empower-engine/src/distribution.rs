use std::{fs, path::PathBuf};

use once_cell::sync::Lazy;

use crate::{assets::Assets, compiler::Program, node_graph::NodeGraph, project::Project};

pub static TEMP_PROJECT_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    std::env::temp_dir().join("empower_studio_temporary_project")
});

pub fn create_temporary_project(project: &Project) -> Result<(), String>
{
    let temp_directory_root = TEMP_PROJECT_LOCATION.to_path_buf();

    // In case there already is a temp directory
    let remove_dir_result = fs::remove_dir_all( temp_directory_root.clone() );
                                    
    match remove_dir_result
    {
        Ok(_) => {},
        Err( error ) => 
        {
            match error.kind()
            {
                std::io::ErrorKind::NotFound => {},
                _ =>
                {
                    return Err(format!("Error when removing temp directory ({})", error.to_string()));
                }
            }
        }
    }

    let create_temp_project_directory_result = fs::create_dir(temp_directory_root.clone() );

    match create_temp_project_directory_result
    {
        Ok(_) => {},
        Err( error ) =>
        {
            return Err(format!("Error when creating temp directory ({})", error.to_string()));
        },
    }

    let create_temp_assets_directory_result = fs::create_dir(temp_directory_root.join("assets").clone());

    match create_temp_assets_directory_result
    {
        Ok(_) => {},
        Err( error ) =>
        {
            return Err(format!("Error when creating temp assets directory ({})", error.to_string()));
        },
    }

    Ok(())
}

pub fn save_project(project: &Project)
{
    
}

pub fn save_assets(assets: &Assets)
{
    
}

pub fn load_project() -> Project
{
    Project::new()
}

pub fn import_asset(project: &mut Project, path: &PathBuf) -> Result<(), String>
{
    if !path.exists()
    {
        return Err( format!("Cannot import asset with path path {}, because it doesn't exist", path.to_string_lossy().to_string() ));
    }

    let file_name = path.file_name().unwrap().to_string_lossy().to_string(); // I believe this is safe, due to the exists check above

    if file_name.contains(".graph")
    {
        let node_graph_file = std::fs::read_to_string(&path);

        if node_graph_file.is_err()
        {
            return Err(node_graph_file.err().unwrap().to_string());
        }

        let node_graph_json = std::fs::read_to_string(&node_graph_file.unwrap());

        if node_graph_json.is_err()
        {
            return Err("cannot convert node_graph_file to sting".to_string());
        }

        let node_graph = NodeGraph::from_json( &node_graph_json.unwrap() );

        if node_graph.is_err()
        {
            return Err("cannot convert node_graph_json to node graph".to_string());
        }

        project.assets.add_node_graph_asset(file_name, None, node_graph.unwrap());

        return Ok(());
    }

    if file_name.contains(".png")
    {
        let image = image::open(&path);

        if image.is_err()
        {
            return Err( image.err().unwrap().to_string() );
        }

        let image = image.unwrap().to_rgba8();
        let (image_width, image_height) = image.dimensions();

        let color_image = egui::ColorImage::from_rgba_unmultiplied([image_width as usize, image_height as usize], image.as_raw());

        project.assets.add_image_asset(file_name, None, color_image);

        return Ok(());
    }

    Err("Cannot import, imcompatible file type.".to_string())
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
