use std::collections::HashMap;

mod viewport;
use egui_dock::Split;
pub use viewport::{Viewport, VIEWPORT_REGISTRY};

pub struct Layout
{
    pub debug_window_active: bool, // @TODO, find a better name or make a collected object for multiple windows
    pub project_setting_window: bool,
    pub project_name_window: Option<String>,
    pub execution_history_window_active: bool, 
    pub viewports: HashMap<String, Box<dyn Viewport>>,
    pub docking_state: egui_dock::DockState<String>,
}

impl Layout
{
    pub fn new() -> Self
    {
        let mut new_layout = Self
        {
            debug_window_active: false,
            project_setting_window: false,
            project_name_window: None,
            execution_history_window_active: false,
            viewports: HashMap::new(),            
            docking_state: egui_dock::DockState::new(Vec::new()), 
        };

        let graph_viewport_name = new_layout.add_viewport("graph viewport");
        let content_browser_viewport_name = new_layout.add_viewport_without_docking_state("content browser viewport");
        let terminal_viewport_name = new_layout.add_viewport_without_docking_state("terminal viewport");

        // This is all to place the initial docking configuration
        let graph_viewport_index = new_layout.docking_state.find_tab(&graph_viewport_name).expect("Unable to find initial graph viewport tab");

        new_layout.docking_state.main_surface_mut().split_below(graph_viewport_index.1, 0.7, vec![content_browser_viewport_name, terminal_viewport_name]);

        new_layout
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
