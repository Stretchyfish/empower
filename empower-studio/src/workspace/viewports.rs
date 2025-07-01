pub mod viewport_type;
use viewport_type::ViewportType;

pub mod empty_viewport;
use empty_viewport::EmptyViewport;

pub mod graph_viewport;
use graph_viewport::GraphViewport;

use crate::graph_editor::GraphEditor;

pub struct Viewports
{
    empty_viewports: Vec<EmptyViewport>, // @TODO, consider using hash tables instead
    graph_viewports: Vec<GraphViewport>,
}

impl Viewports
{
    pub fn new() -> Self
    {
        Self
        {
            empty_viewports: Vec::new(),
            graph_viewports: Vec::new(),
        }
    }

    pub fn add_viewport(&mut self, viewport_type: ViewportType) -> String
    {
        let new_tab_name;
        match viewport_type
        {
            ViewportType::EmptyViewport =>
            {
                let tab_index = self.empty_viewports.len();
                
                let mut tab_affix = String::new();
                if tab_index > 0
                {
                    tab_affix = format!("({})", tab_index);
                }
                
                new_tab_name = String::from("Empty Viewport") + &tab_affix;

                let new_empty_viewport = EmptyViewport::new(new_tab_name.clone());
                self.empty_viewports.push( new_empty_viewport );
            },

            ViewportType::GraphViewport =>
            {
                let tab_index = self.graph_viewports.len();
                
                let mut tab_affix = String::new();
                if tab_index > 0
                {
                    tab_affix = format!("({})", tab_index);
                }
                
                new_tab_name = String::from("Graph Viewport") + &tab_affix;

                let new_graph_viewport = GraphViewport::new(new_tab_name.clone());
                self.graph_viewports.push( new_graph_viewport );
            },
        }

        new_tab_name
    }
}

pub fn show_single_viewport(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, viewports: &mut Viewports, viewport_title: String)
{
    for empty_viewport in viewports.empty_viewports.iter_mut()
    {
        if empty_viewport.title == viewport_title
        {
            empty_viewport::show(ui);
            return;
        }
    }

    for graph_viewport in viewports.graph_viewports.iter_mut()
    {
        if graph_viewport.title == viewport_title
        {
            graph_viewport::show(ui, graph_editor, graph_viewport);
            return;
        }
    }
}