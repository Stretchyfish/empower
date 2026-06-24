use std::{collections::HashMap, fs, path::PathBuf};

use crate::node_graph::NodeGraph;

pub type AssetId = i32;

mod asset_meta;
use asset_meta::AssetMeta;

mod asset_kind;
pub use asset_kind::AssetKind;

#[derive(Clone)]
pub struct Assets
{
    pub node_graphs: HashMap<AssetId, NodeGraph>,
    pub meta: HashMap<AssetId, AssetMeta>,
}

impl Assets
{
    pub fn new() -> Self
    {
        Self
        {
            node_graphs: HashMap::new(),
            meta: HashMap::new(),
        }
    }

    pub fn create_asset(&mut self, location: &PathBuf, asset_kind: AssetKind) -> Option<AssetId>
    {
        let asset_creation_result = match asset_kind
        {
            AssetKind::Graph => self.create_node_graph( location ),
        };

        if asset_creation_result.is_err()
        {
            return None;
        }
        
        Some( self.import_asset(&asset_creation_result.unwrap()) )
    }

    pub fn load_asset(&mut self, asset_id: AssetId)
    {
        let meta = self.meta.get(&asset_id).unwrap();

        println!("File path read: {}", meta.relative_path.to_string_lossy());

        match meta.kind
        {
            AssetKind::Graph =>
            {
                let file = std::fs::read_to_string(&meta.relative_path);

                match file
                {
                    Ok(_) => {},
                    Err( error ) => { panic!( "Error when loading node graph asset: {}", error.kind().to_string() ) },
                }
                
                self.node_graphs.insert(asset_id, NodeGraph::from_json( &std::fs::read_to_string(&meta.relative_path).unwrap() ).unwrap() );
            },
        }
    }

    pub fn create_node_graph(&mut self, location: &PathBuf) -> Result<PathBuf, ()> // @TODO, this should probably be a bool or result aswell?
    {
        let new_node_graph = NodeGraph::new("unamed"); // @TODO, this approach for naming the node graph needs a second look
        self.save_node_graph(&new_node_graph, location)
    }

    pub fn import_asset(&mut self, location: &PathBuf) -> AssetId // @TODO, consider having an optional AssetKind in the input for determining the file type
    {
        let id = self.get_asset_id();
        let file_name = location.file_name().unwrap().to_string_lossy();

        let mut asset_kind = AssetKind::Graph; // @TODO, find a better way of doing this
        if file_name.contains(".graph")
        {
            asset_kind = AssetKind::Graph;
        }

        let asset_meta = AssetMeta
        {
            id,
            relative_path: location.clone(), // @TODO, change to relative later!
            kind: asset_kind,
        };

        self.meta.insert(id, asset_meta);

        id
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

    pub fn get_all_node_graph_names(&mut self) -> Vec<String>
    {
        let mut names = Vec::new();

        let mut graph_assets_to_load = Vec::new();
        for (id, meta) in &self.meta
        {
            if meta.kind != AssetKind::Graph
            {
                continue;
            }

            if self.node_graphs.contains_key(id)
            {
                names.push( self.node_graphs.get(id).unwrap().name.clone() );
                continue;
            }

            graph_assets_to_load.push(id.clone());
        }

        for asset_id in &graph_assets_to_load
        {
            self.load_asset(*asset_id);
            names.push( self.node_graphs.get(asset_id).unwrap().name.clone() );
        }

        names
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
    
    pub fn add_node_graph(&mut self, node_graph: NodeGraph, location: &PathBuf) -> AssetId // @TODO, don't know how good this name is
    {
        let id = self.get_asset_id();
        self.save_node_graph(&node_graph, location);
        
        self.node_graphs.insert(id, node_graph);
        id
    }

    pub fn save_node_graph(&mut self, node_graph: &NodeGraph, location: &PathBuf) -> Result<PathBuf, ()>
    {
        let node_graph_json = node_graph.to_json();

        let node_graph_save_path = location.join(format!("{}.graph", node_graph.name));
        
        // let created_node_graph_json_file_results = std::fs::File::create_new(&node_graph_save_path);

        // match created_node_graph_json_file_results
        // {
        //     Ok(_) => println!("Created file succesfully"),
        //     Err( error ) => println!("Error, failed to create file: {}, {}", node_graph_save_path.to_string_lossy(), error.kind().to_string()),
        // }

        println!("File added path: {}", node_graph_save_path.to_string_lossy());
        let saving_node_graph_file_results = std::fs::write(&node_graph_save_path, node_graph_json);

        match saving_node_graph_file_results
        {
            Ok(_) => println!("Saved succesfully"),
            Err( error ) =>
            {
                println!("Error, failed to write: {}, {}", node_graph_save_path.to_string_lossy(), error.kind().to_string());
                return Result::Err(());
            }
        }

        Ok( node_graph_save_path )
    }

    pub fn get_node_graph(&self, id: &AssetId) -> Option<&NodeGraph>
    {
        self.node_graphs.get(id)
    }

    pub fn get_node_graph_mut(&mut self, id: &AssetId) -> Option<&mut NodeGraph>
    {
        self.node_graphs.get_mut(id)
    }

    fn get_asset_id(&self) -> AssetId
    {
        self.node_graphs.keys().max().unwrap_or(&0) + 1 // @TODO, this approach needs to get fixed later!
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
