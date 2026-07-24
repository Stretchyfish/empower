use std::collections::{HashSet, VecDeque};

use empower_engine::node_graph::NodeGraphKey;

use super::GraphViewportAction;

#[derive(Default, Clone)]
pub struct SelectedNodesQuickMenu
{
    active: bool,
    mouse_position_when_quick_menu_was_activated: egui::Pos2,
}

impl SelectedNodesQuickMenu
{
    pub fn new() -> Self
    {
        Self
        {
            active: false,
            mouse_position_when_quick_menu_was_activated: egui::Pos2::ZERO,
        }
    }

    pub fn is_active(&self) -> bool
    {
        self.active
    }

    pub fn toggle_active(&mut self, mouse_position: &egui::Pos2)
    {
        self.active = !self.active;
        self.mouse_position_when_quick_menu_was_activated = *mouse_position;
    }

    pub fn off(&mut self)
    {
        self.active = false;
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _: &HashSet<NodeGraphKey>, graph_viewport_actions: &mut VecDeque<GraphViewportAction>)
    {
        if !self.active
        {
            return;
        }

        let number_of_input_actions_when_starting = graph_viewport_actions.len();

        let node_selection_window = egui::Window::new("")
                                                .current_pos(egui::Pos2 {x: self.mouse_position_when_quick_menu_was_activated.x - 100.0, y: self.mouse_position_when_quick_menu_was_activated.y - 15.0})
                                                .collapsible(false)
                                                .max_size(egui::Vec2 {x: 200.0, y: 200.0})
                                                .title_bar(false);

        node_selection_window.show(ui.ctx(), |ui|
        {
            if ui.button("Copy").clicked()
            {
                graph_viewport_actions.push_back( GraphViewportAction::CopySelectedNodes );
            }

            if ui.button("Delete").clicked()
            {
                graph_viewport_actions.push_back( GraphViewportAction::DeleteSelectedNodes );
            }
        });

        if graph_viewport_actions.len() > number_of_input_actions_when_starting // Detect if the user performed an action
        {
            self.active = false;
        }
    }
}
