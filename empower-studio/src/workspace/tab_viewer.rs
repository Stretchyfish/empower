use crate::graph_editor::GraphEditor;
use super::viewports::{self, Viewports};

pub struct TabsViewer<'a>
{
    pub graph_editor: &'a mut GraphEditor,
    pub viewports: &'a mut Viewports,
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
        viewports::show_single_viewport(ui, self.graph_editor, self.viewports, tab_name);
    }
}
