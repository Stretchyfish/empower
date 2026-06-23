use std::{collections::HashMap, fs, path::PathBuf};

use crate::node_graph::NodeGraph;

pub type AssetId = i32;

pub struct Assets
{
    node_graphs: HashMap<AssetId, NodeGraph>,
}

impl Assets
{
    pub fn new() -> Self
    {
        Self
        {
            node_graphs: HashMap::new(),
        }
    }

    pub fn create_node_graph(&mut self, location: &PathBuf) -> AssetId
    {
        let new_node_graph = NodeGraph::new("unamed"); // @TODO, this approach for naming the node graph needs a second look
        
        let id = self.get_asset_id();
        self.save_node_graph(&new_node_graph, location);
        
        self.node_graphs.insert(id, new_node_graph);

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

    pub fn save_node_graph(&mut self, node_graph: &NodeGraph, location: &PathBuf)
    {
        let node_graph_json = node_graph.to_json();

        let node_graph_save_path = location.join(format!("{}.json", node_graph.name));
        
        let created_node_graph_json_file_results = std::fs::File::create(&node_graph_save_path);

        match created_node_graph_json_file_results
        {
            Ok(_) => println!("Created file succesfully"),
            Err( error ) => println!("Error, failed to create file: {}, {}", node_graph_save_path.to_string_lossy(), error.kind().to_string()),
        }

        let saving_node_graph_file_results = std::fs::write(&node_graph_save_path, node_graph_json);

        match saving_node_graph_file_results
        {
            Ok(_) => println!("Saved succesfully"),
            Err( error ) => println!("Error, failed to write: {}, {}", node_graph_save_path.to_string_lossy(), error.kind().to_string()),
        }
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
