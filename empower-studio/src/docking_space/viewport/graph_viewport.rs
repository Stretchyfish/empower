use empower_engine::{NodeGraph, NodeGraphKey};
use egui;

// use crate::{GraphEditor, actions::Action, project::Project, workspace::layout::viewport::graph_viewport::user_inputs::GraphViewportUserInputs};

use crate::{actions::Action, studio_context::{StudioContext, project::{GraphEditor, Project}}};

use super::Viewport;

mod user_inputs;
use user_inputs::GraphViewportUserInputs;

mod node_widget;
mod connection_widget;
mod debug_info_widget;

mod node_area_select;
use node_area_select::NodeAreaSelect;

mod node_select_panel;
use node_select_panel::NodeSelectionPanel;

mod quick_menu;
use quick_menu::QuickMenu;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphViewport
{
    mouse_scene_position_last_frame: egui::Pos2, // @TODO, only temporary public for debug purpose
    scene_rect: egui::Rect,
    quick_menu: Option<QuickMenu>,
    node_area_select: Option<NodeAreaSelect>,
    node_select_panel: Option<NodeSelectionPanel>,
}

#[typetag::serde]
impl Viewport for GraphViewport
{
    fn new() -> Box<dyn Viewport> 
    where
        Self: Sized {
        
        Box::new( 
            Self 
            {
                mouse_scene_position_last_frame: egui::Pos2::ZERO,
                scene_rect: egui::Rect { min: egui::Pos2 { x: -650.0, y: -650.0 }, max: egui::Pos2 { x: 650.0, y: 650.0 }},
                quick_menu: None,
                node_area_select: None,
                node_select_panel: None,
            } 
        )
    }

    fn clone_box(&self) -> Box<dyn Viewport>
    {
        Box::new( self.clone() )
    }

    fn name(&self) -> &'static str {
        "graph viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext, viewport_name: &String) {

        let project = studio_context.get_project_mut();

        let mut action_queue = Vec::new();
        let user_inputs = user_inputs::get_graph_viewport_user_inputs(ui);

        self.show_canvas(ui, &mut project.graph_editor, &user_inputs, viewport_name, &mut action_queue);
        self.process_user_inputs(&user_inputs, &mut project.graph_editor);
    }
}

impl GraphViewport
{
    fn show_canvas(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor, user_inputs: &GraphViewportUserInputs, viewport_name: &String, action_queue: &mut Vec<Action>)
    {
        
        let mut drag_pan_button = egui::DragPanButtons::PRIMARY;
        if user_inputs.left_shift_is_down
        {
            drag_pan_button = egui::DragPanButtons::empty();
        }
        
        let mut scene_rect = self.scene_rect.clone(); // This is needed to avoid borrow issues
        egui::Scene::new()
        .zoom_range(0.01..=2.0)
        .max_inner_size(egui::Vec2 { x: 200.0, y: 200.0 })
        .drag_pan_buttons(drag_pan_button)
        .show(ui, &mut scene_rect, |scene_ui|
        {
            if user_inputs.mouse_is_inside_viewport && self.quick_menu.is_none()
            {
                let mouse_scene_position = self.screen_position_to_scene_position(&user_inputs.mouse_position, &scene_ui);
                let mouse_scene_delta_position = mouse_scene_position - self.mouse_scene_position_last_frame; 

                if !graph_editor.selected_nodes.is_empty()
                {
                    graph_editor.move_selected_nodes( &mouse_scene_delta_position );
                }

                self.mouse_scene_position_last_frame = mouse_scene_position;
            }

            for node_key in graph_editor.selected_nodes.clone()
            {
                node_widget::highlight(scene_ui, &node_key, graph_editor);
            }

            if self.node_area_select.is_some()
            {
                for node_key in self.node_area_select.as_ref().unwrap().get_nodes_inside_of_area_select()
                {
                    node_widget::highlight(scene_ui, &node_key, graph_editor);
                }
            }

            let node_keys: Vec<NodeGraphKey> = graph_editor.display_nodes.keys().cloned().collect();
            for node_key in node_keys
            {
                node_widget::show(scene_ui, &node_key, graph_editor, &viewport_name, &mut self.node_area_select, action_queue);
            }

            debug_info_widget::nodes_debug_info_show(scene_ui, &graph_editor);
            
            let connection_keys = graph_editor.node_graph.get_all_connections();
            for connection in connection_keys
            {
                connection_widget::show(scene_ui, graph_editor, connection);
            }

            if self.node_area_select.is_some()
            {
                scene_ui.painter().rect_filled(self.node_area_select.as_ref().unwrap().rect, 0.5, egui::Color32::from_rgba_unmultiplied(255, 140, 0, 70));
            }

            if graph_editor.port_searcher.is_some() && user_inputs.mouse_is_inside_viewport
            {
                connection_widget::show_connection_search(scene_ui, graph_editor, &self.mouse_scene_position_last_frame);
            }
            
            if self.quick_menu.is_some()
            {
                self.quick_menu.as_mut().unwrap().show(scene_ui, graph_editor, action_queue);
            }

            if self.node_select_panel.is_some()
            {
                let added_node = self.node_select_panel.as_mut().unwrap().show(scene_ui, &self.mouse_scene_position_last_frame, action_queue);

                if added_node
                {
                    self.node_select_panel = None;
                }
            }
        });

        self.scene_rect = scene_rect;
    }

    fn process_user_inputs(&mut self, user_inputs: &GraphViewportUserInputs, graph_editor: &mut GraphEditor)
    {
        if !user_inputs.mouse_is_inside_viewport
        {
            return;
        }

        // Deselect port searcher
        if user_inputs.left_clicked && graph_editor.port_searcher.is_some()
        {
            graph_editor.stop_port_search();
            // action_queue.push( Action::StopPortSearch );
            return;
        }

        // Process quick menu behavior
        if user_inputs.right_clicked && graph_editor.selected_nodes.len() > 0 && self.quick_menu.is_none()
        {
            self.quick_menu = Some( QuickMenu::new(self.mouse_scene_position_last_frame.clone()) );
            return;
        }

        if (user_inputs.left_clicked || user_inputs.right_clicked) && self.quick_menu.is_some()
        {
            self.quick_menu = None;
            return;
        }

        // Process toggling of node selection panel
        if user_inputs.right_clicked && self.node_select_panel.is_some()
        {
            self.node_select_panel = None;
            return;
        }

        if user_inputs.right_clicked && self.node_select_panel.is_none() && self.quick_menu.is_none()
        {
            self.node_select_panel = Some( NodeSelectionPanel::new(user_inputs.mouse_position) );
            return;
        }

        // Process node area select behavior
        if user_inputs.left_is_down && user_inputs.left_shift_is_down && self.node_area_select.is_none()
        {
            // In this case its fine to use last frame, as last frame will be current frame
            self.node_area_select = Some( NodeAreaSelect::new(self.mouse_scene_position_last_frame) );
            return;
        } 

        if self.node_area_select.is_some()
        {
            let node_area_select = self.node_area_select.as_mut().unwrap();
            node_area_select.determine_area_select_rect(&self.mouse_scene_position_last_frame);
        }

        if self.node_area_select.is_some() && (!user_inputs.left_is_down || !user_inputs.left_shift_is_down)
        {
            for node_inside_area in self.node_area_select.as_ref().unwrap().get_nodes_inside_of_area_select()
            {
                graph_editor.add_node_to_selection(&node_inside_area);
            }
            // action_queue.push( Action::AddNodesToSelectedNodes { node_keys: self.node_area_select.as_ref().unwrap().get_nodes_inside_of_area_select() });
            self.node_area_select = None;
            return;
        }

        // Deselect selected nodes
        if user_inputs.left_clicked && graph_editor.selected_nodes.len() > 0
        {
            graph_editor.clear_node_selection();
            // action_queue.push( Action::ClearAllNodesFromSelectedNodes );
            return;
        }

        // Detect keyboard actions
        if user_inputs.clicked_backspace
        {
            let nodes_to_delete = graph_editor.selected_nodes.clone();

            graph_editor.clear_node_selection();
            // action_queue.push( Action::ClearAllNodesFromSelectedNodes );
            for node_key in nodes_to_delete
            {
                graph_editor.remove_node( &node_key );
                // action_queue.push( Action::DeleteNode { node_key });
            }
            return;
        }
    }

    fn screen_position_to_scene_position(&self, scene_position: &egui::Pos2, ui: &egui::Ui) -> egui::Pos2
    {
        return ui.ctx().layer_transform_from_global(ui.painter().layer_id()).unwrap() * *scene_position;
    }
}

