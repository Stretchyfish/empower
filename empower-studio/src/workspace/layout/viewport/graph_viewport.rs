use empower_engine::{NodeGraphKey, node_graph::node::NodeKind, runtime::EmpowerExecutor};
use egui;

use crate::{GraphEditor, actions::Action, graph_editor::display_node::{DisplayNodeKind, DisplayValue}, workspace::layout::viewport::graph_viewport::{node_widget::NodeWidgetResponse, user_inputs::GraphViewportUserInputs}};
use empower_engine::node_graph::node::port::PortKind; 

use super::Viewport;

mod user_inputs;

mod node_widget;
mod connection_widget;
mod debug_info_widget;

mod node_area_select;
use node_area_select::NodeAreaSelect;

mod node_select_panel;
use node_select_panel::NodeSelectionPanel;

mod quick_menu;
use quick_menu::QuickMenu;

pub struct GraphViewport
{
    mouse_scene_position_last_frame: egui::Pos2, // @TODO, only temporary public for debug purpose
    scene_rect: egui::Rect,
    quick_menu: Option<QuickMenu>,
    node_area_select: Option<NodeAreaSelect>,
    node_select_panel: Option<NodeSelectionPanel>,
}

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

    fn name(&self) -> &'static str {
        "graph viewport"
    }

    fn show(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor, viewport_name: &String, action_queue: &mut Vec<Action>) {

        let user_inputs = user_inputs::get_graph_viewport_user_inputs(ui);

        self.show_canvas(ui, graph_editor, &user_inputs, viewport_name, action_queue);

        self.process_user_inputs2(&user_inputs, graph_editor, action_queue);

        // if !action_queue.is_empty()
        // {
        //     return;
        // }
        //

        
        // let widget_responses = self.view_canvas(ui, &user_inputs, graph_editor, viewport_name, action_queue);

        // let wideget_interaction_happened = self.process_widget_responses(&widget_responses, graph_editor);
        // self.process_user_actions(&user_inputs, graph_editor, wideget_interaction_happened);
    }
}

impl GraphViewport
{
    fn show_canvas(&mut self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor, user_inputs: &GraphViewportUserInputs, viewport_name: &String, action_queue: &mut Vec<Action>)
    {
        let user_inputs = user_inputs::get_graph_viewport_user_inputs(ui);

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
                    action_queue.push( Action::MoveSelectedNodes { canvas_delta_position: mouse_scene_delta_position } );
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
                node_widget::show_2(scene_ui, &node_key, graph_editor, &viewport_name, &mut self.node_area_select, action_queue);
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
                let added_node = self.node_select_panel.as_mut().unwrap().show(scene_ui, graph_editor, &self.mouse_scene_position_last_frame, action_queue);

                if added_node
                {
                    self.node_select_panel = None;
                }
            }
        });

        self.scene_rect = scene_rect;
    }

    fn process_user_inputs2(&mut self, user_inputs: &GraphViewportUserInputs, graph_editor: &GraphEditor, action_queue: &mut Vec<Action>)
    {
        if !user_inputs.mouse_is_inside_viewport
        {
            return;
        }

        // Deselect port searcher
        if user_inputs.left_clicked && graph_editor.port_searcher.is_some()
        {
            action_queue.push( Action::StopPortSearch );
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
            action_queue.push( Action::AddNodesToSelectedNodes { node_keys: self.node_area_select.as_ref().unwrap().get_nodes_inside_of_area_select() });
            self.node_area_select = None;
            return;
        }

        // Deselect selected nodes
        if user_inputs.left_clicked && graph_editor.selected_nodes.len() > 0
        {
            action_queue.push( Action::ClearAllNodesFromSelectedNodes );
            return;
        }

        // Detect keyboard actions
        if user_inputs.clicked_backspace
        {
            let nodes_to_delete = graph_editor.selected_nodes.clone();

            action_queue.push( Action::ClearAllNodesFromSelectedNodes );
            for node_key in nodes_to_delete
            {
                action_queue.push( Action::DeleteNode { node_key });
            }
            return;
        }
    }

    fn set_input_port_value_if_display_value_can_convert(&self, port_key: &NodeGraphKey, graph_editor: &mut GraphEditor, new_value: &DisplayValue)
    {
        let input_port = graph_editor.node_graph.get_input_port_mut(port_key).unwrap();
        let display_input_port = graph_editor.display_input_ports.get_mut(port_key).unwrap();

        display_input_port.value = new_value.clone();

        let new_port_value = display_input_port.value.to_port_value(&input_port.compatability);

        if new_port_value.is_none()
        {
            display_input_port.valid = false; // @TODO, consider a better name, like "valid"
            return;
        }
        display_input_port.valid = true;

        input_port.value = new_port_value.unwrap();
    }

    fn set_state_changes(&mut self, node_key: &NodeGraphKey, graph_editor: &mut GraphEditor, new_node_kind: &Box<dyn NodeKind>, new_display_node_kind: &Box<dyn DisplayNodeKind>)
    {
        graph_editor.refresh_node_structure(*node_key, new_node_kind, new_display_node_kind);
    }

    // @TODO, find a better way to do this behavior
    fn show_quick_menu(&self, ui: &mut egui::Ui, graph_editor: &mut GraphEditor, action_queue: &mut Vec<Action>)
    {
        // if self.quick_menu.is_none()
        // {
        //     return false; // @TODO, this check is not really needed, make a decision on that
        // }

        // let menu_position = self.quick_menu.unwrap();
        // let quick_menu_rect = egui::Rect::from_min_size(menu_position, egui::Vec2::splat(500.0));

        let mut button_clicked = false;

        // let mut potentially_new_selected_nodes = Vec::new();

        // let quick_menu_ui_builder = egui::UiBuilder::new().max_rect(quick_menu_rect);
        // ui.scope_builder(quick_menu_ui_builder, |ui|
        // {
        //     egui::Frame::popup(ui.style()).show(ui, |ui| 
        //     {
        //         let selected_nodes = graph_editor.selected_nodes.clone();

        //         if selected_nodes.len() == 1
        //         {
        //             if ui.add(egui::Button::new( egui::RichText::new("Compile").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
        //             {
        //                 action_queue.push( Action::StartNodeGraphExecutionFromEntry { node_key: selected_nodes[0] });
        //                 // @TODO, move this out from here, when implementing the event system

        //                 button_clicked = true;
        //             }
        //         }

        //         if ui.add(egui::Button::new( egui::RichText::new("Copy").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
        //         {
        //             let mut new_node_keys = Vec::new();
        //             new_node_keys.reserve(selected_nodes.len());

        //             for node_key in selected_nodes.clone()
        //             {
        //                 // @TODO, this needs to be updated based on action system
        //                 let copied_node_key = graph_editor.create_node_copy(&node_key);

        //                 new_node_keys.push(copied_node_key);
        //             }

        //             potentially_new_selected_nodes = new_node_keys;
        //             println!("Number of selected nodes added: {}, {}", potentially_new_selected_nodes.len(), potentially_new_selected_nodes[0]);
        //             button_clicked = true;
        //         }

        //         if ui.add(egui::Button::new( egui::RichText::new("Delete").size(30.0)).min_size(egui::Vec2 {x: 190.0, y: 20.0})).clicked()
        //         {
        //             // @TODO, I think this will cause a crash when multiple graph viewports are open
        //             for selected_node_key in selected_nodes
        //             {
        //                 action_queue.push( Action::RemoveNode { node_key: selected_node_key });
        //             }
        //             button_clicked = true;
        //         }
        //     });
        // });

        // if button_clicked
        // {
        //     action_queue.push( Action::AddNodesToSelectedNodes { node_keys: potentially_new_selected_nodes });
        //     // graph_editor.selected_nodes = potentially_new_selected_nodes; // @TODO, find a more elegant way of doing this
        // }

        // button_clicked
    }

    // @TODO, this function is not tested
    // fn scene_to_screen(&self, scene_position: &egui::Pos2, ui: &egui::Ui) -> egui::Pos2
    // {
    //     return ui.ctx().layer_transform_to_global(ui.painter().layer_id()).unwrap() * *scene_position;
    // }

    fn screen_position_to_scene_position(&self, scene_position: &egui::Pos2, ui: &egui::Ui) -> egui::Pos2
    {
        return ui.ctx().layer_transform_from_global(ui.painter().layer_id()).unwrap() * *scene_position;
    }
}

