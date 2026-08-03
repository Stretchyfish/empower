use std::{collections::VecDeque, path::PathBuf};

use super::CONFIG_DIRECTORY;

use empower_engine::node_graph::NodeAddress;
use serde::{Deserialize, Serialize};

mod persistent_cache;
pub use persistent_cache::PersistentCache;

mod session_cache;
pub use session_cache::SessionCache;


const CONFIG_CACHE_FILE_NAME: &'static str = "cache.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Cache
{
    pub persistent: PersistentCache,

    #[serde(skip)]
    pub session: SessionCache,
}

impl Cache
{
    pub fn new() -> Self
    {
        Self
        {
            persistent: PersistentCache::new(),
            session: SessionCache::new(),
        }
    }

    pub fn save(&self)
    {
        // @TODO, this work is done multiple times, maybe define it better
        let created_config_directory = std::fs::create_dir(CONFIG_DIRECTORY.config_dir());
        match created_config_directory
        {
            Ok(_) => {},
            Err( error ) => match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {},
                _ => {
                    panic!("Failing to save cache because : {}", error.kind().to_string());
                }
            },
        }

        let file_path = CONFIG_DIRECTORY.config_dir().join(CONFIG_CACHE_FILE_NAME);
        let cache_json = serde_json::to_string_pretty(self).unwrap();

        let save_cache_result = std::fs::write(file_path, cache_json);

        match save_cache_result
        {
            Ok(_) => {},
            Err( error ) =>
            {
                println!("Error when saving cache : {}",error.kind().to_string());
            },
        }
    }

    pub fn load() -> Self
    {
        let cache_path = CONFIG_DIRECTORY.config_dir().join(CONFIG_CACHE_FILE_NAME);

        let read_cache_result = std::fs::read_to_string(cache_path);

        match read_cache_result // If it fails to load, use new
        {
            Ok( cache_json ) =>
            {
                let cache_json_result = serde_json::from_str(&cache_json);

                match cache_json_result 
                {
                    Ok( cache ) => { return cache; },
                    Err( error ) => { println!("Error when reading stored cache, using default instead, error: {}", error.to_string()); },
                };
            },
            Err( error ) => { println!("Error when reading stored cache, using default instead, error: {}", error.to_string()); },
        };

        Self::new()
    }

    pub fn add_previous_project(&mut self, project_path: PathBuf)
    {
        let mut project_index_to_remove = None;
        for (index, previous_project) in self.persistent.previous_projects.iter().enumerate()
        {
            if previous_project.to_string_lossy().to_string() == project_path.to_string_lossy().to_string()
            {
                project_index_to_remove = Some( index );
            }
        }

        if project_index_to_remove.is_some()
        {
            self.persistent.previous_projects.remove(project_index_to_remove.unwrap());
        }
        
        if self.persistent.previous_projects.len() > 10
        {
            self.persistent.previous_projects.pop_front();
        }

        self.persistent.previous_projects.push_back(project_path);
    }
}
