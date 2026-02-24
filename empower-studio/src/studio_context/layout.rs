use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use crate::docking_space::viewport::{Viewport, VIEWPORT_REGISTRY};
 
const CONFIG_DIRECTORY_PROJECT_NAME: &'static str = "empower-studio";
const CONFIG_LAYOUT_FILE_NAME: &'static str = "layout.json";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Layout
{
    pub viewports: HashMap<String, Box<dyn Viewport>>,
    pub docking_state: egui_dock::DockState<String>,
}

impl Layout
{
    pub fn new() -> Self
    {
        Self
        {
            viewports: HashMap::new(),
            docking_state: egui_dock::DockState::new(Vec::new()),
        }
    }

    pub fn default_layout() -> Self
    {
        let mut new_default_layout = Self::new();
        
        let graph_viewport_name = new_default_layout.add_viewport("graph viewport");
        let content_browser_viewport_name = new_default_layout.add_viewport_without_docking_state("content browser viewport");
        let terminal_viewport_name = new_default_layout.add_viewport_without_docking_state("terminal viewport");

        // This is all to place the initial docking configuration
        let graph_viewport_index = new_default_layout.docking_state.find_tab(&graph_viewport_name).expect("Unable to find initial graph viewport tab");

        new_default_layout.docking_state.main_surface_mut().split_below(graph_viewport_index.1, 0.7, vec![content_browser_viewport_name, terminal_viewport_name]);

        new_default_layout
    }

    pub fn load() -> Self
    {
        let config_directory = directories::ProjectDirs::from("com", "empower", CONFIG_DIRECTORY_PROJECT_NAME).expect("Could not find a config directory");
        let editor_state_path = config_directory.config_dir().join(CONFIG_LAYOUT_FILE_NAME);

        let read_editor_state_result = std::fs::read_to_string(editor_state_path);

        match read_editor_state_result // If it fails to load, use default
        {
            Ok( editor_state_json ) =>
            {
                let read_json_result = serde_json::from_str(&editor_state_json);

                match read_json_result
                {
                    Ok( editor_state ) => { return editor_state; },
                    Err( error ) => { println!("Error when reading stored layout, using default instead, error: {}", error.to_string()); },
                };
            },
            Err( error ) => { println!("Error when reading stored layout, using default instead, error: {}", error.to_string()); },
        };

        Self::default_layout()
    }

    pub fn save(&self)
    {
        // @TODO, find a better way to match the save and load paths
        let config_directory = directories::ProjectDirs::from("com", "empower", CONFIG_DIRECTORY_PROJECT_NAME).expect("Could not find a config directory");

        let created_config_directory = std::fs::create_dir(config_directory.config_dir());
        match created_config_directory
        {
            Ok(_) => {},
            Err( error ) => match error.kind()
            {
                std::io::ErrorKind::AlreadyExists => {},
                _ => {
                    panic!("Failing to save layout because : {}", error.kind().to_string());
                }
            },
        }

        let file_path = config_directory.config_dir().join(CONFIG_LAYOUT_FILE_NAME);
        let editor_json = serde_json::to_string_pretty(self).unwrap();

        let save_editor_state_result = std::fs::write(file_path, editor_json);

        match save_editor_state_result
        {
            Ok(_) => {},
            Err( error ) =>
            {
                println!("Error when saving editor state : {}",error.kind().to_string());
            },
        }
    }

    fn adjust_viewport_name(&self, name: &'static str) -> String
    {
        let number_of_viewports_containing_the_name = self.viewports.iter().filter(|(viewport_name, _)| viewport_name.contains(name) ).count();

        if number_of_viewports_containing_the_name == 0
        {
            return String::from( name );
        }

        format!("{} ({})", name, number_of_viewports_containing_the_name)
    }

    pub fn add_viewport(&mut self, new_viewport_name: &'static str) -> String
    {
        let new_viewport_constructor = match VIEWPORT_REGISTRY.get(new_viewport_name)
        {
           Some( constructor ) => constructor,
           None => panic!("Tried to create a non-existing viewport name : {}", new_viewport_name), 
        };

        let adjusted_viewport_name = self.adjust_viewport_name( new_viewport_name );

        let new_viewport = new_viewport_constructor();

        self.viewports.insert(adjusted_viewport_name.clone(), new_viewport);
        self.docking_state.push_to_focused_leaf(adjusted_viewport_name.clone());

        adjusted_viewport_name
    }

    // @TODO, this function only has a very specific usecase, consider if it should be a bool in the add_viewport function instead
    pub fn add_viewport_without_docking_state(&mut self, new_viewport_name: &'static str) -> String
    {
        
        let new_viewport_constructor = match VIEWPORT_REGISTRY.get(new_viewport_name)
        {
           Some( constructor ) => constructor,
           None => panic!("Tried to create a non-existing viewport name : {}", new_viewport_name), 
        };

        let adjusted_viewport_name = self.adjust_viewport_name( new_viewport_name );

        let new_viewport = new_viewport_constructor();

        self.viewports.insert(adjusted_viewport_name.clone(), new_viewport);

        adjusted_viewport_name
    }
}
