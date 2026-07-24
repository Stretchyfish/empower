use std::collections::HashMap;
use empower_engine::assets::AssetId;
use serde::{Serialize, Deserialize};

use crate::docking_space::{Viewport, viewport::{ContentBrowserViewport, GraphViewport, TerminalViewport}};

use super::CONFIG_DIRECTORY;

const CONFIG_LAYOUT_FILE_NAME: &'static str = "layout.json";

#[derive(Clone, Serialize, Deserialize)]
pub struct Layout
{
    pub viewports: HashMap<String, Viewport>,
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
        
        let graph_viewport_name = new_default_layout.add_viewport( Viewport::Graph { graph_viewport: GraphViewport::new( 1 ) } ); // entry graph should always have this id

        let terminal_viewport_name = new_default_layout.add_viewport_without_docking_state( Viewport::Terminal { terminal_viewport: TerminalViewport::new() } );
        let content_browser_viewport_name = new_default_layout.add_viewport_without_docking_state( Viewport::ContentBrowser { content_browser_viewport: ContentBrowserViewport::new() });

        // This is all to place the initial docking configuration
        let graph_viewport_index = new_default_layout.docking_state.find_tab(&graph_viewport_name).expect("Unable to find initial graph viewport tab");

        new_default_layout.docking_state.main_surface_mut().split_below(graph_viewport_index.node, 0.7, vec![content_browser_viewport_name.clone()]);

        let content_browser_index = new_default_layout.docking_state.find_tab(&content_browser_viewport_name).expect("Unable to find initial content browser tab");
        new_default_layout.docking_state.main_surface_mut().split_right(content_browser_index.node, 0.6, vec![terminal_viewport_name]);

        new_default_layout
    }

    fn get_viewport_name(&self, new_viewport: &Viewport) -> String
    {
        let name = match new_viewport // @TODO, combine this with the adjust_viewport_name, to just get_viewport_name
        {
            Viewport::Graph { graph_viewport: _ } => "graph viewport",
            Viewport::Terminal { terminal_viewport: _ } => "terminal viewport",
            Viewport::ContentBrowser { content_browser_viewport: _ } => "content browser viewport",
            Viewport::Empty { empty_viewport: _ } => "empty viewport",
            Viewport::ImageViewer { image_asset_id: _ } => "image viewer viewport",
        };

        // @TODO, consider using the asset name aswell for grpah viewport, so example "graph viewport (entry graph)" and "graph viewport (entry graph) (1)
        
        let number_of_viewports_containing_the_name = self.viewports.iter().filter(|(viewport_name, _)| viewport_name.contains(name) ).count();

        if number_of_viewports_containing_the_name == 0
        {
            return String::from( name );
        }

        format!("{} ({})", name, number_of_viewports_containing_the_name)
    }

    pub fn add_viewport(&mut self, new_viewport: Viewport) -> String // @TODO, look into if this function can be written with a template instead?
    {
        let viewport_name = self.get_viewport_name(&new_viewport);
        
        self.viewports.insert(viewport_name .clone(), new_viewport);
        self.docking_state.push_to_focused_leaf(viewport_name .clone());

        viewport_name 
    }

    pub fn _add_viewport_with_custom_name(&mut self, name: String, new_viewport: Viewport) -> String // @TODO, this needs to get used
    {
        self.viewports.insert(name .clone(), new_viewport);
        self.docking_state.push_to_focused_leaf(name .clone());

        name 
    }

    pub fn add_viewport_at_first_leaf(&mut self, new_viewport: Viewport) -> String // @TODO, look into if this function can be written with a template instead?
    {
        let viewport_name = self.get_viewport_name(&new_viewport);
        
        self.viewports.insert(viewport_name .clone(), new_viewport);
        self.docking_state.push_to_first_leaf(viewport_name .clone());

        viewport_name 
    }

    // @TODO, this function only has a very specific usecase, consider if it should be a bool in the add_viewport function instead
    pub fn add_viewport_without_docking_state(&mut self, new_viewport: Viewport) -> String
    {
        let viewport_name = self.get_viewport_name(&new_viewport);
        
        self.viewports.insert(viewport_name .clone(), new_viewport);

        viewport_name 
    }

    pub fn add_or_focus_graph_viewport(&mut self, graph_id: AssetId )
    {
        for (viewport_name, viewport) in &self.viewports
        {
            match viewport
            {
                Viewport::Graph { graph_viewport } =>
                {
                    if graph_viewport.graph_asset_id == graph_id
                    {
                        let graph_viewport_index = self.docking_state.find_tab(&viewport_name).expect("Unable to find graph viewport tab");
                        self.docking_state.remove_tab( egui_dock::TabPath { surface: graph_viewport_index.surface, node: graph_viewport_index.node, tab: graph_viewport_index.tab });
                        self.docking_state.push_to_focused_leaf(viewport_name.clone());
                        return;
                    }
                },
                _ => {},
            }
        }

        println!("Got here");

        self.add_viewport( Viewport::Graph { graph_viewport: GraphViewport::new(graph_id) });
    }

    pub fn save(&self)
    {
        let file_path = CONFIG_DIRECTORY.config_dir().join(CONFIG_LAYOUT_FILE_NAME);

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
        let editor_state_path = CONFIG_DIRECTORY.config_dir().join(CONFIG_LAYOUT_FILE_NAME);

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
}
