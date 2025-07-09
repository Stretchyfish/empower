pub mod viewport_type;
use viewport_type::ViewportType;

pub mod empty_viewport;
use empty_viewport::EmptyViewport;

pub mod graph_viewport;
use graph_viewport::GraphViewport;

pub mod terminal_viewport;
use terminal_viewport::TerminalViewport;

use crate::graph_editor::GraphEditor;

pub struct Viewports
{
    pub empty_viewports: Vec<EmptyViewport>, // @TODO, consider using hash tables instead
    pub graph_viewports: Vec<GraphViewport>, // @TODO, consider if these should be private
    pub terminal_viewports: Vec<TerminalViewport>,
}

impl Viewports
{
    pub fn new() -> Self
    {
        Self
        {
            empty_viewports: Vec::new(),
            graph_viewports: Vec::new(),
            terminal_viewports: Vec::new(),
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

            ViewportType::TerminalViewport =>
            {
                let tab_index = self.terminal_viewports.len();
                
                let mut tab_affix = String::new();
                if tab_index > 0
                {
                    tab_affix = format!("({})", tab_index);
                }
                
                new_tab_name = String::from("Terminal Viewport") + &tab_affix; // @TODO, reduce the code in each case

                let new_terminal_viewport = TerminalViewport::new(new_tab_name.clone());
                self.terminal_viewports.push( new_terminal_viewport );
            }
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

    for terminal_viewport in viewports.terminal_viewports.iter_mut()
    {
        if terminal_viewport.title == viewport_title
        {
            terminal_viewport::show(ui, terminal_viewport);
            return;
        }
    }
}