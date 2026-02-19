use std::collections::HashMap;
use crate::{Action, Project, Settings, user_inputs::UserInputs, UserState};

use super::Viewport;

pub struct TabViewer<'a>
{
    pub project: &'a mut Project,
    pub settings: &'a Settings,
    pub viewports: &'a mut HashMap<String, Box<dyn Viewport>>,
    pub user_state: &'a UserState,
    pub user_inputs: &'a UserInputs,
    pub action_queue: &'a mut Vec<Action>,
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
        viewport.show(ui, self.project, &tab_name, self.action_queue);
    }
}
