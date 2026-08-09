use std::{collections::HashMap};

use crate::node_graph::NodeGraph;

pub type AssetId = i32;

mod asset_meta;
pub use asset_meta::AssetMeta;

mod asset_kind;
pub use asset_kind::AssetKind;

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

    pub meta: HashMap<AssetId, AssetMeta>,
}

impl Assets
{
    pub fn new(project_name: &String) -> Self
    {
        let mut meta = HashMap::new();

        meta.insert(ROOT_FOLDER_ASSET_ID, AssetMeta { id: ROOT_FOLDER_ASSET_ID, name: project_name.clone(), parent: None, kind: AssetKind::Folder });
        meta.insert(ASSET_FOLDER_ASSET_ID, AssetMeta { id: ASSET_FOLDER_ASSET_ID, name: "assets".to_string(), parent: Some( ROOT_FOLDER_ASSET_ID ), kind: AssetKind::Folder });
        
        Self
        {
            loaded_assets: LoadedAssets::new(),

            meta,
        }
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

        asset_creation_result
    }

    fn create_node_graph(&mut self, asset_id: AssetId, name: &'static str) -> Result<AssetId, String>
    {
        if self.meta.values().any(|m| m.kind == AssetKind::NodeGraph && m.name == name )
        {
            return Err(format!("Cannot create node graph with name ({}), because node graph with said name already exists", name));
        }

        let node_graph = NodeGraph::new(name);
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
                images.insert(*id, meta.name.clone());
            }
        }

        images
    }

    pub fn get_node_graph(&self, id: &AssetId) -> Option<&NodeGraph> // @TODO, this idea needs a second look
    {
        self.loaded_assets.loaded_node_graphs.get(id)
    }

    pub fn get_node_graph_mut(&mut self, id: &AssetId) -> Option<&mut NodeGraph>
    {
        self.loaded_assets.loaded_node_graphs.get_mut(id)
    }

    fn get_asset_id(&self) -> AssetId
    {
        self.meta.keys().max().unwrap_or(&0) + 1 
    }
}
