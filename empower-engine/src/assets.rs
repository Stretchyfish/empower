use std::{collections::HashMap, fs, path::PathBuf};

use crate::node_graph::NodeGraph;

pub type AssetId = i32;

mod asset_meta;
use asset_meta::AssetMeta;

mod asset_kind;
pub use asset_kind::AssetKind;

use serde::{Deserialize, Serialize};

mod loaded_assets;
pub use loaded_assets::LoadedAssets;

#[derive(Clone, Serialize, Deserialize)]
pub struct Assets
{
    #[serde(skip)]
    pub loaded_assets: LoadedAssets,

    pub meta: HashMap<AssetId, AssetMeta>,
    pub path_to_asset_id: HashMap<PathBuf, AssetId>,
}

impl Assets
{
    pub fn new() -> Self
    {
        Self
        {
            loaded_assets: LoadedAssets::new(),

            meta: HashMap::new(),
            path_to_asset_id: HashMap::new(),
        }
    }

    pub fn create_asset(&mut self, project_path: &PathBuf, directory: &PathBuf, asset_kind: AssetKind, name: Option<&'static str>) -> Option<AssetId> // @TODO, not sure if the optional name is ever used in this case, but might be usefull in the future, so leaving it for now.
    {
        let asset_creation_result = match asset_kind
        {
            AssetKind::Graph => self.create_node_graph( project_path, directory, name ),
            AssetKind::Image => todo!(),
        };

        if asset_creation_result.is_err()
        {
            return None;
        }

        let asset_new_path = asset_creation_result.unwrap();
        let asset_id = self.import_asset( &asset_new_path );
        self.load_asset(project_path, asset_id); // This could potentially be problematic later, but we will here assume that if the user creates an asset they will most likely also use them soon, and preloading them makes sense.

        Some( asset_id )
    }

    pub fn load_asset(&mut self, project_path: &PathBuf, asset_id: AssetId) -> bool
    {
        let meta = self.meta.get(&asset_id);

        if meta.is_none()
        {
            return false;
        }

        let meta = meta.unwrap();

        println!("File path read: {}", meta.relative_path.to_string_lossy());

        let asset_path = project_path.join(&meta.relative_path);

        match meta.kind
        {
            AssetKind::Graph =>
            {
                if self.loaded_assets.loaded_node_graphs.contains_key(&asset_id)
                {
                    return true;
                }
                
                let file = std::fs::read_to_string(&asset_path);

                match file
                {
                    Ok(_) => {},
                    Err( error ) => { panic!( "Error when loading node graph asset: {}", error.kind().to_string() ) },
                }
        
                self.loaded_assets.loaded_node_graphs.insert(asset_id, NodeGraph::from_json( &std::fs::read_to_string(&asset_path).unwrap() ).unwrap() );
            },
            AssetKind::Image =>
            {
                if self.loaded_assets.loaded_images.contains_key(&asset_id)
                {
                    return true;
                }

                let image = image::open(&asset_path);

                if image.is_err()
                {
                    panic!("failed to load image asset");
                }

                let image = image.unwrap().to_rgba8();
                let (image_width, image_height) = image.dimensions();

                let color_image = egui::ColorImage::from_rgba_unmultiplied([image_width as usize, image_height as usize], image.as_raw());

                self.loaded_assets.loaded_images.insert(asset_id, color_image);
            },
        }

        true
    }

    pub fn load_assets(&mut self, project_path: &PathBuf, asset_ids: &Vec<AssetId>)
    {
        for asset_id in asset_ids
        {
            self.load_asset(project_path, *asset_id);
        }
    }

    pub fn create_node_graph(&mut self, project_path: &PathBuf, directory: &PathBuf, name: Option<&'static str>) -> Result<PathBuf, ()> // @TODO, this should probably be a bool or result aswell?
    {
        let new_node_graph = if name.is_some()
        {
            if name.unwrap() == "entry_graph" // @TODO, find a better way to create the entry graph, this is really bad
            {
                NodeGraph::new_entry_graph()
            }
            else
            {
                NodeGraph::new(name.unwrap())
            }
        }
        else
        {
            NodeGraph::new("unamed")
        };
        // let new_node_graph = NodeGraph::new("unamed"); // @TODO, this approach for naming the node graph needs a second look
        // self.save_node_graph(&new_node_graph, location)

        let node_graph_json = new_node_graph.to_json();
        let node_graph_save_relative_path = directory.join(format!("{}.graph", new_node_graph.name));
        let node_graph_save_path = project_path.join( &node_graph_save_relative_path );
        
        println!("asset imported path: {}", node_graph_save_path.to_string_lossy());
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

        Ok( node_graph_save_relative_path )
    }

    pub fn import_asset(&mut self, relative_path: &PathBuf) -> AssetId // @TODO, consider having an optional AssetKind in the input for determining the file type
    {
        let id = self.get_asset_id();
        let file_name = relative_path.file_name().unwrap().to_string_lossy();

        let mut asset_kind = AssetKind::Graph; // @TODO, find a better way of doing this
        if file_name.contains(".graph")
        {
            asset_kind = AssetKind::Graph;
        }

        if file_name.contains(".png")
        {
            asset_kind = AssetKind::Image;
        }

        let asset_meta = AssetMeta
        {
            id,
            relative_path: relative_path.clone(), // @TODO, change to relative later!
            kind: asset_kind,
        };

        self.meta.insert(id, asset_meta);
        self.path_to_asset_id.insert(relative_path.clone(), id);

        id
    }

    pub fn get_all_node_graph_names(&self) -> HashMap<AssetId, String> 
    {
        self.loaded_assets.loaded_node_graphs.iter().map(|(id, node_graph)| (id.clone(), node_graph.name.clone())).collect() // @TODO, maybe this should use asset meta instead
    }

    pub fn get_all_image_names(&self) -> HashMap<AssetId, String>
    {
        let mut images = HashMap::new();

        for (id, meta) in &self.meta
        {
            if meta.kind == AssetKind::Image
            {
                images.insert(*id, meta.relative_path.file_name().unwrap().to_string_lossy().to_string());
            }
        }

        images
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
    
    pub fn add_node_graph(&mut self, node_graph: NodeGraph, project_path: &PathBuf, relative_location: &PathBuf) -> AssetId // @TODO, don't know how good this name is
    {
        let id = self.get_asset_id();
        
        let _ = self.save_node_graph(&node_graph, project_path, &relative_location.join(format!("{}.graph", node_graph.name)));
        
        self.loaded_assets.loaded_node_graphs.insert(id, node_graph);
        id
    }

    pub fn save(&mut self, project_path: &PathBuf)
    {
        // We only need to save what is actively being worked on, so we only loop through the active assets
        
        for (node_graph_key, node_graph) in self.loaded_assets.loaded_node_graphs.clone() // @TODO, this is a potentially crazy expensive call, find a better way
        {
            let node_graph_path = self.meta.get(&node_graph_key).unwrap().relative_path.clone();
            let _ = self.save_node_graph(&node_graph, project_path, &node_graph_path);
        }
    }

    pub fn save_node_graph(&mut self, node_graph: &NodeGraph, project_path: &PathBuf, relative_location: &PathBuf) -> Result<PathBuf, ()> // @TODO, change this to load the loaded node graph, and use a key instead
    {
        let node_graph_json = node_graph.to_json();

        let node_graph_save_path = project_path.join( relative_location );
        
        // let created_node_graph_json_file_results = std::fs::File::create_new(&node_graph_save_path);

        // match created_node_graph_json_file_results
        // {
        //     Ok(_) => println!("Created file succesfully"),
        //     Err( error ) => println!("Error, failed to create file: {}, {}", node_graph_save_path.to_string_lossy(), error.kind().to_string()),
        // }

        println!("Saving node graph: {}", node_graph_save_path.to_string_lossy());
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

        Ok( node_graph_save_path.clone() )
    }

    pub fn load_node_graph(&mut self, project_path: &PathBuf, id: &AssetId) -> Option<&NodeGraph>
    {
        if !self.loaded_assets.loaded_node_graphs.contains_key(id)
        {
            self.load_asset(project_path, *id);
        }

        self.loaded_assets.loaded_node_graphs.get(id)
    }

    pub fn get_node_graph_naive(&self, id: &AssetId) -> Option<&NodeGraph> // @TODO, this idea needs a second look
    {
        self.loaded_assets.loaded_node_graphs.get(id)
    }

    pub fn get_node_graph_mut(&mut self, id: &AssetId) -> Option<&mut NodeGraph>
    {
        self.loaded_assets.loaded_node_graphs.get_mut(id)
    }

    pub fn get_image(&mut self, project_path: &PathBuf, id: &AssetId) -> Option<&egui::ColorImage>
    {
        if !self.loaded_assets.loaded_images.contains_key(id)
        {
            self.load_asset(project_path, *id);
        }
        
        self.loaded_assets.loaded_images.get(id)
    } 

    fn get_asset_id(&self) -> AssetId
    {
        self.loaded_assets.loaded_node_graphs.keys().max().unwrap_or(&0) + 1 // @TODO, this approach needs to get fixed later!
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
