use std::collections::HashMap;
use crate::studio_context::layout::Viewport;

pub struct TabViewer<'a>
{
    pub viewports: &'a mut HashMap<String, Box<dyn Viewport>>,
}

impl egui_dock::TabViewer for TabViewer<'_>
{
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText
    {
        tab.clone().into()  // VERY IMPORTANT, that the titles are unique (Current implementation will have problems with this!)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) 
    {
        let tab_name: String = tab.clone();

        if !self.viewports.contains_key(&tab_name)
        {
            panic!("Requested a viewport not in the viewport that doesn't exist");
        }

        let viewport = self.viewports.get_mut(&tab_name).unwrap();
        viewport.show(ui);
    }
}
