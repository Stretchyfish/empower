use crate::assets::{AssetId, Assets};
use crate::node_graph::NodeGraph;
use std::path::PathBuf;
use std::fs;

mod project_settings;
use project_settings::ProjectSettings;

mod project_state;
use project_state::ProjectState;

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

        let entry_graph = NodeGraph::new_entry_graph();
        let entry_graph_asset_id = assets.add_node_graph(entry_graph, &temp_directory_root.join("assets"));

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
}

