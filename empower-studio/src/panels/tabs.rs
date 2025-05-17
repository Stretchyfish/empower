use egui;
use egui_dock;

//use crate::graph_editor::graph_state::GraphState;
//use crate::graph_editor::graph_viewport::{update_graph_view_port, GraphViewportState};

use crate::graph;

use crate::viewports;
use crate::user;


//use crate::graph;
//mod graph_viewport;
//use crate::graph_viewport::;
//use crate::graph_viewport::{update_graph_view_port, GraphViewportState};

pub struct Tabs // @TODO, decide on this name, it was named context before, should it be changed back?
{
    pub graph_viewport_tabs: Vec<viewports::GraphViewport>,
    pub new_graph_viewport_tabs: Vec<viewports::NewGraphViewport>,
    pub graph_state: graph::GraphState,
}

impl Tabs
{
    pub fn new() -> Self
    {
        Self
        {
            graph_viewport_tabs: Vec::new(), // @TODO, this should be a hashtable
            new_graph_viewport_tabs: Vec::new(),
            graph_state: graph::GraphState::new(),
        }
    }

    pub fn create_new_tab(&mut self) -> String
    {
        // @TODO, Later add multiple different kinds of tabs
        let mut tab_name = String::from("graph state");

        if self.graph_viewport_tabs.len() > 0
        {
            tab_name += " (";
            tab_name += self.graph_viewport_tabs.len().to_string().as_str();
            tab_name += ")";
        }

        // self.graph_viewport_tabs.push( viewports::GraphViewport::new(tab_name.clone()) );
        self.new_graph_viewport_tabs.push( viewports::NewGraphViewport::new(tab_name.clone()) );

        println!("{}", tab_name);
        return tab_name;
    }
}

impl egui_dock::TabViewer for Tabs
{
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText
    {
        tab.clone().into()  // VERY IMPORTANT, that the titles are unique (Current implementation will have problems with this!)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) 
    {
        let tab_name: String = tab.clone().into();
        let user_inputs = user::inputs::detect_user_inputs(ui);        

        // Leaving this code here, might be interesting later
        //let tab_top_left_corner_position = ui.cursor().min; // not 100% top left corner, build shouldn't matter?
        //ui.painter().circle(tab_top_left_corner_position, 5.0, egui::Color32::YELLOW, egui::Stroke::new(2.0, egui::Color32::YELLOW));

        for graph_viewport_tab in self.graph_viewport_tabs.iter_mut()
        {
            if tab_name == graph_viewport_tab.title
            {
                graph_viewport_tab.update_graph_viewport(ui, &mut self.graph_state, &user_inputs);
                return;
            }
        }

        for new_graph_viewport_tab in self.new_graph_viewport_tabs.iter_mut()
        {
            if tab_name == new_graph_viewport_tab.title
            {
                new_graph_viewport_tab.show(ui);
            }
        }

        ui.label(format!("Tab was not setup correctly, tree title is '{tab_name}'"));
    }
}
