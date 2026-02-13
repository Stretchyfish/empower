use std::{collections::{HashMap, VecDeque}, path::PathBuf};

mod viewport;
pub use viewport::{Viewport, VIEWPORT_REGISTRY};

mod previous_project;
use previous_project::PreviousProject;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Layout // @TODO, consider renaming editor?
{
    pub debug_window_active: bool, // @TODO, find a better name or make a collected object for multiple windows
    pub project_setting_window: bool,
    pub project_name_window: Option<String>,
    pub execution_history_window_active: bool, 
    pub viewports: HashMap<String, Box<dyn Viewport>>,
    pub docking_state: egui_dock::DockState<String>,
    pub previous_projects: VecDeque<PreviousProject>,
}

impl Layout
{
    pub fn new() -> Self // @TODO, maybe better naming here is needed?
    {
        Self::load()
    }

    pub fn clear() -> Self
    {
        Self
        {
            debug_window_active: false,
            project_setting_window: false,
            project_name_window: None,
            execution_history_window_active: false,
            viewports: HashMap::new(),            
            docking_state: egui_dock::DockState::new(Vec::new()), 
            previous_projects: VecDeque::new(),
        }
    }

    pub fn default_layout() -> Self
    {
        let mut new_default_layout = Self::clear();
        
        let graph_viewport_name = new_default_layout.add_viewport("graph viewport");
        let content_browser_viewport_name = new_default_layout.add_viewport_without_docking_state("content browser viewport");
        let terminal_viewport_name = new_default_layout.add_viewport_without_docking_state("terminal viewport");

        // This is all to place the initial docking configuration
        let graph_viewport_index = new_default_layout.docking_state.find_tab(&graph_viewport_name).expect("Unable to find initial graph viewport tab");

        new_default_layout.docking_state.main_surface_mut().split_below(graph_viewport_index.1, 0.7, vec![content_browser_viewport_name, terminal_viewport_name]);

        new_default_layout
    }

    pub fn save(&self)
    {
        let config_directory = directories::ProjectDirs::from("com", "empower", "studio").expect("Could not find a config directory");

        std::fs::create_dir(config_directory.config_dir());

        let file_path = config_directory.config_dir().join("studio_editor.json");
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

    pub fn load() -> Self
    {
        let config_directory = directories::ProjectDirs::from("com", "empower", "studio").expect("Could not find a config directory");
        let editor_state_path = config_directory.config_dir().join("studio_editor.json");

        let read_editor_state_result = std::fs::read_to_string(editor_state_path);

        match read_editor_state_result
        {
            Ok( editor_state_json ) =>
            {
                serde_json::from_str(&editor_state_json).unwrap()
            },
            Err(_) =>
            {
                Self::default_layout()
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

    pub fn add_previous_project(&mut self, project_name: &String, project_path: &PathBuf)
    {

        let mut project_to_remove = None;

        for (index, project) in self.previous_projects.iter().enumerate()
        {
        
            if project.project_location == *project_path
            {
                project_to_remove = Some( index );
            }
        }

        if project_to_remove.is_some()
        {
            self.previous_projects.remove(project_to_remove.unwrap());
        }

        let new_previous_project = PreviousProject { project_name: project_name.clone(), project_location: project_path.clone() };
        
        self.previous_projects.push_front(new_previous_project);

        if self.previous_projects.len() > 10
        {
            self.previous_projects.pop_back();
        }
    }
}
