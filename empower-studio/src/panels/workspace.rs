use egui;

use crate::viewports;
use crate::StudioContext;

#[derive(Default)]
pub struct Workspace
{
    pub empty_viewports: Vec<viewports::EmptyViewport>,
    pub graph_viewports: Vec<viewports::GraphViewport>,
}

impl Workspace
{
    pub fn new() -> Self
    {
        Self
        {   
            empty_viewports: Vec::new(),
            graph_viewports: Vec::new(),
        }
    }

    pub fn create_viewport(&mut self, viewport_type: viewports::ViewportTypes) -> String
    {
        let tab_name; 
        match viewport_type
        {
            viewports::ViewportTypes::Empty =>
            {
                let tab_index = self.empty_viewports.len();
                
                let mut tab_affix = String::new();
                if tab_index > 0
                {
                    tab_affix = format!("({})", tab_index);
                }
                
                tab_name = String::from("Empty Viewport") + &tab_affix;

                let new_empty_viewport = viewports::EmptyViewport::new(tab_name.clone());
                self.empty_viewports.push(new_empty_viewport );
            }

            viewports::ViewportTypes::GraphViewport =>
            {
                let tab_index = self.graph_viewports.len();
                
                let mut tab_affix = String::new();
                if tab_index > 0
                {
                    tab_affix = format!("({})", tab_index);
                }
                
                tab_name = String::from("Graph Viewport") + &tab_affix;

                let new_graph_viewport = viewports::GraphViewport::new(tab_name.clone());
                self.graph_viewports.push(new_graph_viewport);
            }
        }

        tab_name
    }

    pub fn view_single_viewport(&mut self, ui: &mut egui::Ui, viewport_title: String, studio_context: &mut StudioContext)
    {
        for empty_viewport in self.empty_viewports.iter_mut()
        {
            if empty_viewport.title == viewport_title
            {
                empty_viewport.view(ui);
                return;
            }
        }

        for graph_viewport in self.graph_viewports.iter_mut()
        {
            if graph_viewport.title == viewport_title
            {
                graph_viewport.show(ui, studio_context);                
                return;
            }
        }
    }
}

