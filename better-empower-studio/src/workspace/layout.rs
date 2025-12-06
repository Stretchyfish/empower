use std::collections::HashMap;

mod viewport;
pub use viewport::{Viewport, VIEWPORT_REGISTRY};

pub struct Layout
{
    pub debug_window_active: bool, // @TODO, find a better name or make a collected object for multiple windows
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
            viewports: HashMap::new(),            
            docking_state: egui_dock::DockState::new(Vec::new()), 
        };

        let graph_viewport = new_layout.add_viewport("graph viewport");
        let terminal_viewport = new_layout.add_viewport("empty viewport");

        // This is all to place the initial docking configuration
        let graph_viewport_index = new_layout.docking_state.find_tab(&graph_viewport).expect("Unable to find initial graph viewport tab");
        let terminal_viewport_index = new_layout.docking_state.find_tab(&terminal_viewport).expect("Unable to find initial terminal viewport tab");

        new_layout.docking_state.remove_tab(terminal_viewport_index).expect("Terminal viewport missing from docking state");
        new_layout.docking_state.main_surface_mut().split_below(graph_viewport_index.1, 0.7, vec![terminal_viewport]);

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
        let new_viewport_constructor= match VIEWPORT_REGISTRY.get(new_viewport_name)
        {
           Some( constructor ) => constructor,
           None => panic!("Tried to create a non-existing viewport name"), 
        };

        let adjusted_viewport_name = self.adjust_viewport_name( new_viewport_name );

        let new_viewport = new_viewport_constructor();

        self.viewports.insert(adjusted_viewport_name.clone(), new_viewport);
        self.docking_state.push_to_focused_leaf(adjusted_viewport_name.clone());

        adjusted_viewport_name
    }
}