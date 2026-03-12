use std::collections::VecDeque;

use crate::studio_context::project::graph_editor::GraphEditor;

use super::GraphViewportAction;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickMenu
{
    show: bool,
    mouse_position_when_quick_menu_was_activated: egui::Pos2,
}

impl QuickMenu
{
    pub fn new() -> Self
    {
        Self
        {
            show: false,
            mouse_position_when_quick_menu_was_activated: egui::Pos2::ZERO,
        }
    }

    pub fn on(&mut self, mouse_position: &egui::Pos2)
    {
        self.show = true;
        self.mouse_position_when_quick_menu_was_activated = *mouse_position;
    }

    pub fn off(&mut self)
    {
        self.show = false;
    }

    pub fn is_on(&self) -> bool
    {
        self.show
    }

    pub fn show(&mut self, ui: &mut egui::Ui, graph_editor: &GraphEditor, graph_viewport_actions: &mut VecDeque<GraphViewportAction>)
    {
        let number_of_input_actions_when_starting =graph_viewport_actions.len();
        let quick_menu_rect = egui::Rect::from_min_size(self.mouse_position_when_quick_menu_was_activated, egui::Vec2::splat(500.0));

        let quick_menu_ui_builder = egui::UiBuilder::new().max_rect(quick_menu_rect);
        ui.scope_builder(quick_menu_ui_builder, |ui|
        {
            egui::Frame::popup(ui.style()).show(ui, |ui| 
            {
                let selected_nodes = graph_editor.selected_nodes.clone();

                if selected_nodes.len() == 1
                {
                    if ui.add(egui::Button::new( egui::RichText::new("Compile").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
                    {
                        graph_viewport_actions.push_back( GraphViewportAction::StartExecutionFromEntry { node_key: selected_nodes[0] });
                    }
                }

                if ui.add(egui::Button::new( egui::RichText::new("Copy").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
                {
                    graph_viewport_actions.push_back( GraphViewportAction::CopySelectedNodes );
                }

                if ui.add(egui::Button::new( egui::RichText::new("Delete").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
                {
                    graph_viewport_actions.push_back( GraphViewportAction::DeleteSelectedNodes );
                }
            });
        });

        if graph_viewport_actions.len() > number_of_input_actions_when_starting // Detect if the user performed an action
        {
            self.show = false;
        }
    }
}
