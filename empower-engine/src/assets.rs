use std::{collections::HashMap, path::PathBuf};

use crate::node_graph::NodeGraph;

pub type AssetId = i32;

mod asset_meta;
pub use asset_meta::AssetMeta;

mod asset_kind;
pub use asset_kind::AssetKind;

use egui::{ahash::{HashSet, HashSetExt}};
use image::{ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};

mod loaded_assets;
pub use loaded_assets::LoadedAssets;

pub static ROOT_FOLDER_ASSET_ID: AssetId = 1;
pub static ASSET_FOLDER_ASSET_ID: AssetId = 2;

#[derive(Clone, Serialize, Deserialize)]
pub struct Assets
{
    #[serde(skip)]
    pub loaded_assets: LoadedAssets,

    #[serde(skip)]
    pub modified_assets: HashSet<AssetId>,

    pub meta: HashMap<AssetId, AssetMeta>,
}

impl Assets
{
    pub fn new(project_name: &String) -> Self
    {
        let mut meta = HashMap::new();

        meta.insert(ROOT_FOLDER_ASSET_ID, AssetMeta { id: ROOT_FOLDER_ASSET_ID, name: project_name.clone(), parent: None, kind: AssetKind::Folder });
        meta.insert(ASSET_FOLDER_ASSET_ID, AssetMeta { id: ASSET_FOLDER_ASSET_ID, name: "assets".to_string(), parent: Some( ROOT_FOLDER_ASSET_ID ), kind: AssetKind::Folder });

        let mut modified_assets = HashSet::new();
        modified_assets.insert(ROOT_FOLDER_ASSET_ID);
        modified_assets.insert(ASSET_FOLDER_ASSET_ID);
        
        Self
        {
            loaded_assets: LoadedAssets::new(),
            // modified_assets: HashSet::from([ROOT_FOLDER_ASSET_ID, ASSET_FOLDER_ASSET_ID]),
            modified_assets,

            meta,
        }
    }

    pub fn rename_asset(&mut self, asset_id: &AssetId, new_name: &str) -> bool
    {
        if !self.meta.contains_key(&asset_id)
        {
            return false;
        }

        let asset_meta = self.meta.get_mut(&asset_id).unwrap();
        asset_meta.name = new_name.to_string();

        match asset_meta.kind
        {
            AssetKind::NodeGraph =>
            {
                self.loaded_assets.loaded_node_graphs.get_mut(&asset_id).unwrap().name = new_name.to_string();
                
            },
            _ => {},
        }

        // @TODO, this approach doesn't rename the files, which can cause problem laters

        true
    }

    pub fn create_asset(&mut self, parent: Option<AssetId>, asset_kind: AssetKind, name: &'static str) -> Result<AssetId, String> // @TODO, not sure if the optional name is ever used in this case, but might be usefull in the future, so leaving it for now.
    {
        let asset_id = self.get_asset_id();

        let asset_creation_result = match asset_kind
        {
            AssetKind::NodeGraph => self.create_node_graph(asset_id, name),
            AssetKind::Image => todo!(),
            AssetKind::Json => todo!(),
            AssetKind::Folder => self.create_folder(&parent, asset_id, name),
        };

        if asset_creation_result.is_err()
        {
            return asset_creation_result;
        }

        self.meta.insert(asset_id, AssetMeta { id: asset_id, name: String::from(name), parent, kind: asset_kind });

        self.modified_assets.insert(asset_id);

        asset_creation_result
    }

    fn create_node_graph(&mut self, asset_id: AssetId, name: &'static str) -> Result<AssetId, String>
    {
        let name = String::from(name) + ".json";
        
        if self.meta.values().any(|m| m.kind == AssetKind::NodeGraph && m.name == name )
        {
            return Err(format!("Cannot create node graph with name ({}), because node graph with said name already exists", name));
        }

        let node_graph = NodeGraph::new(name.as_str());
        self.loaded_assets.loaded_node_graphs.insert(asset_id, node_graph);
        Ok(asset_id)
    }

    fn create_folder(&mut self, parent: &Option<AssetId>, asset_id: AssetId, name: &'static str) -> Result<AssetId, String> 
    {
        if self.meta.values().any(|m| m.kind == AssetKind::Folder && m.parent == *parent && m.name.as_str() == name )
        {
            return Err(format!("Cannot create folder with name ({}), because folder with said name already exists in current folder", name));
        }

        Ok( asset_id )
    }

    pub fn add_node_graph_asset(&mut self, name: String, parent: Option<AssetId>, node_graph: NodeGraph) -> AssetId
    {
        let asset_id = self.get_asset_id();

        self.loaded_assets.loaded_node_graphs.insert(asset_id, node_graph);
        self.meta.insert(asset_id, AssetMeta { id: asset_id, name, parent: if parent.is_some() { parent } else { Some(ASSET_FOLDER_ASSET_ID) }, kind: AssetKind::NodeGraph });

        asset_id
    }

    pub fn add_image_asset(&mut self, name: String, parent: Option<AssetId>, color_image: egui::ColorImage) -> AssetId
    {
        let asset_id = self.get_asset_id();

        self.loaded_assets.loaded_images.insert(asset_id, color_image);
        self.meta.insert(asset_id, AssetMeta { id: asset_id, name, parent: if parent.is_some() { parent } else { Some(ASSET_FOLDER_ASSET_ID) }, kind: AssetKind::Image });

        asset_id
    }

    pub fn get_node_graph(&self, id: &AssetId) -> Option<&NodeGraph> // @TODO, this idea needs a second look
    {
        self.loaded_assets.loaded_node_graphs.get(id)
    }

    pub fn get_node_graph_mut(&mut self, id: &AssetId) -> Option<&mut NodeGraph>
    {
        self.modified_assets.insert(*id); // Its assumed that if it is being accessed mut, then its modified
        self.loaded_assets.loaded_node_graphs.get_mut(id)
    }

    fn get_asset_id(&self) -> AssetId
    {
        self.meta.keys().max().unwrap_or(&0) + 1 
    }

    pub fn save_assets(&mut self, location: &PathBuf) -> Result<(), String>
    {
        let location = location.join("assets");

        let create_asset_directory_result = std::fs::create_dir( &location );

        match create_asset_directory_result
        {
            Ok(_) => {},
            Err( error ) =>
            {
                match error.kind()
                {
                    std::io::ErrorKind::AlreadyExists => {},
                    _ => { return Err( error.kind().to_string() ) },
                }
            },
        }

        for asset_id in &self.modified_assets
        {
            let save_asset_result = self.save_asset(&asset_id, &location);

            if save_asset_result.is_err() // @TODO, in the future this should probably get changed, so even if it fails to save one asset, it can still save the rest
            {
                return Err( format!("Failed to save asset: {}", save_asset_result.err().unwrap()) );
            }
        }

        self.modified_assets.clear();
        
        Ok(())
    }

    fn save_asset(&self, asset_id: &AssetId, asset_directory_location: &PathBuf) -> Result<(), String>
    {
        let asset_meta = self.meta.get(asset_id);

        if asset_meta.is_none()
        {
            return Err( format!("Cannot save modified asset with id {}, because it is not in assets", asset_id) );
        }

        let asset_meta = asset_meta.unwrap();
        let asset_path = asset_directory_location.join(&asset_meta.name);

        match asset_meta.kind
        {
            AssetKind::Folder => { return Ok(()); }, // Completely ignore folder right now
            AssetKind::NodeGraph =>
            {
                let node_graph = self.loaded_assets.loaded_node_graphs.get(asset_id);

                if node_graph.is_none()
                {
                    return Err(format!("NodeGraph with id {} is not in loaded_node_graphs, and therefore cannot be saved", asset_meta.id));
                }

                let node_graph_json = node_graph.unwrap().to_json();
                
                let write_node_graph_result = std::fs::write(asset_path, node_graph_json);

                if write_node_graph_result.is_err()
                {
                    return Err( format!("Unable to write to node graph with id: {}, cannot save due to {}", asset_id, write_node_graph_result.err().unwrap().kind().to_string()) );
                }
            },
            AssetKind::Image =>
            {
                let image = self.loaded_assets.loaded_images.get(asset_id);

                if image.is_none()
                {
                    return Err(format!("Image with id {} is not in loaded_images, and therefore cannot be saved", asset_meta.id));
                }

                let image = image.unwrap();

                // @TODO, find a more generalized / safe approach to save images

                let pixels: Vec<u8> = image.pixels.iter().flat_map(|color| color.to_array()).collect();
                let savable_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(image.width() as u32, image.height() as u32, pixels).unwrap();

                let saving_image_result = savable_image.save(asset_path);

                if saving_image_result.is_err()
                {
                    return Err(format!("Unable to save image with id: {}, because : {}", asset_id, saving_image_result.err().unwrap().to_string()));
                }
            },
            AssetKind::Json => todo!(),
        }

        Ok(())
    }

    pub fn import_asset(&mut self, path: &PathBuf) -> Result<AssetId, String>
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

            let asset_id = self.add_node_graph_asset(file_name, None, node_graph.unwrap());

            self.modified_assets.insert(asset_id);

            return Ok(asset_id);
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

            let asset_id = self.add_image_asset(file_name, None, color_image);

            self.modified_assets.insert(asset_id); // @TODO, this line is repeated in each case, look into another way

            return Ok(asset_id);
        }

        Err("Cannot import, imcompatible file type.".to_string())
    }
    
}


