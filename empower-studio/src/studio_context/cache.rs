use std::{collections::VecDeque, path::PathBuf};

const CONFIG_DIRECTORY_PROJECT_NAME: &'static str = "empower-studio"; // @TODO, this created in multiple files, should be more global
const CONFIG_CACHE_FILE_NAME: &'static str = "cache.json";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cache
{
    pub previous_projects: VecDeque<PathBuf>,
}

impl Cache
{
    pub fn new() -> Self
    {
        Self
        {
            previous_projects: VecDeque::new(),
        }
    }

    pub fn save(&self)
    {
    }

    pub fn load() -> Self
    {
        let config_directory = directories::ProjectDirs::from("com", "empower", CONFIG_DIRECTORY_PROJECT_NAME).expect("Could not find a config directory");
        let cache_path = config_directory.config_dir().join(CONFIG_CACHE_FILE_NAME);

        let read_cache_result = std::fs::read_to_string(cache_path);

        match read_cache_result // If it fails to load, use new
        {
            Ok( cache_json ) =>
            {
                let cache_json_result = serde_json::from_str(&cache_json);

                match cache_json_result 
                {
                    Ok( cache ) => { return cache; },
                    Err( error ) => { println!("Error when reading stored layout, using default instead, error: {}", error.to_string()); },
                };
            },
            Err( error ) => { println!("Error when reading stored layout, using default instead, error: {}", error.to_string()); },
        };

        Self::new()
    }

    pub fn add_previous_project(&mut self, project_path: PathBuf)
    {
        if self.previous_projects.len() > 10
        {
            self.previous_projects.pop_front();
        }

        self.previous_projects.push_back(project_path);
    }
}
