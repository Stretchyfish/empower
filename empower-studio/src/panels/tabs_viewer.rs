use egui_dock;

use crate::NodeGraph;
use crate::panels;

pub struct TabsViewer<'a>
{
    pub node_graph: &'a mut NodeGraph,
    pub workspace: &'a mut panels::Workspace, 
}

impl egui_dock::TabViewer for TabsViewer<'_>
{
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText
    {
        tab.clone().into()  // VERY IMPORTANT, that the titles are unique (Current implementation will have problems with this!)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) 
    {
        let tab_name: String = tab.clone().into();
        self.workspace.view_single_viewport(ui, tab_name, self.node_graph);
    }
}
