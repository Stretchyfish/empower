use crate::{actions::Action, graph_editor::{self, GraphEditor}};


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

    pub fn show(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor, action_queue: &mut Vec<Action>)
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
                        action_queue.push( Action::StartNodeGraphExecutionFromEntry { node_key: selected_nodes[0] });
                    }
                }

                if ui.add(egui::Button::new( egui::RichText::new("Copy").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
                {
                    let mut new_node_keys = Vec::new();
                    new_node_keys.reserve(selected_nodes.len());

                    for node_key in selected_nodes.clone()
                    {
                        // @TODO, this needs to be updated based on action system

                        graph_editor.toggle_node_selection(&node_key);
                        let copied_node_key = graph_editor.create_node_copy(&node_key);
                        graph_editor.toggle_node_selection(&copied_node_key);

                        new_node_keys.push(copied_node_key);
                    }
                }

                if ui.add(egui::Button::new( egui::RichText::new("Delete").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
                {
                    // @WARNING, this could potentially cause problems with multiple windows, so be carefull
                    for selected_node_key in selected_nodes
                    {
                        action_queue.push( Action::RemoveNode { node_key: selected_node_key });
                    }
                }

                
            });
        });
    }
}
