use std::collections::VecDeque;

use crate::studio_context::project::graph_editor::GraphEditor;

use super::GraphViewportAction;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickMenu
{
    mouse_position_when_quick_menu_was_activated: egui::Pos2,
}

impl QuickMenu
{
    pub fn new(mouse_position_when_quick_menu_was_activated: egui::Pos2) -> Self
    {
        Self
        {
            mouse_position_when_quick_menu_was_activated,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, graph_editor: &GraphEditor, graph_viewport_actions: &mut VecDeque<GraphViewportAction>)
    {
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
                        // action_queue.push( Action::StartNodeGraphExecutionFromEntry { node_key: selected_nodes[0] });
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
    }
}
