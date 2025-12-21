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

mod port_searcher;
use port_searcher::PortSearcher;

mod quick_menu;
use quick_menu::QuickMenu;

pub struct GraphViewport
{
    mouse_scene_position_last_frame: egui::Pos2, // @TODO, only temporary public for debug purpose
    scene_rect: egui::Rect,
    port_searcher: Option<PortSearcher>,
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
                port_searcher: None,
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

        // @TODO, this is not a great way to approach user inputs, so fix in the future!
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
            if user_inputs.mouse_is_inside_viewport
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

            if self.port_searcher.is_some()
            {
                connection_widget::show_connection_search(scene_ui, graph_editor, &self.port_searcher.as_ref().unwrap(), &self.mouse_scene_position_last_frame);
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
            action_queue.push( Action::RemoveAllNodesFromSelectedNodes );
            return;
        }

        // Detect keyboard actions
        if user_inputs.clicked_backspace
        {
            let nodes_to_delete = graph_editor.selected_nodes.clone();

            action_queue.push( Action::RemoveAllNodesFromSelectedNodes );
            for node_key in nodes_to_delete
            {
                action_queue.push( Action::RemoveNode { node_key });
            }
            return;
        }
    }

    // @TODO, these functions can be simplified down using Canvas structs for example
    fn view_canvas(&mut self, ui: &mut egui::Ui, user_inputs: &GraphViewportUserInputs, graph_editor: &mut GraphEditor, viewport_name: &String, action_queue: &mut Vec<Action>) -> Vec<NodeWidgetResponse>
    {
        let mut scene_rect = self.scene_rect.clone(); // This is needed to avoid borrow issues

        let mut mouse_position_in_scene = self.mouse_scene_position_last_frame; // Set to last frame, in case there is no new position in the scene
        let mut mouse_scene_delta = egui::Vec2::ZERO; // @TODO, take another look at this placement

        let mouse_pointer_inside_viewport = ui.rect_contains_pointer(ui.min_rect());

        let mut drag_pan_button = egui::DragPanButtons::PRIMARY;
        if user_inputs.left_shift_is_down
        {
            drag_pan_button = egui::DragPanButtons::empty();
        }

        let mut widget_responses = Vec::new();

        egui::Scene::new()
        .zoom_range(0.01..=2.0)
        .max_inner_size(egui::Vec2 { x: 200.0, y: 200.0 })
        .drag_pan_buttons(drag_pan_button)
        .show(ui, &mut scene_rect, |scene_ui|
        {
            // @TODO, take another investigation into this
            if mouse_pointer_inside_viewport // Is needed to avoid applying double delta position to selected nodes
            {
                let scene_latest_pos = scene_ui.input(|i| i.pointer.latest_pos());

                // @TODO, this whole if statement can be simplified!
                if scene_latest_pos.is_some()
                {
                    mouse_position_in_scene = self.screen_position_to_scene_position(&mut scene_latest_pos.unwrap(), scene_ui);
                }
            }

            mouse_scene_delta = mouse_position_in_scene - self.mouse_scene_position_last_frame; 
            // self.mouse_scene_delta = mouse_scene_delta;

            let node_keys: Vec<NodeGraphKey> = graph_editor.display_nodes.keys().cloned().collect(); // @TODO, find a more elegant way of writting this
            for node_key in node_keys
            {
                let node_widget_response = node_widget::show(scene_ui, graph_editor, &node_key, &viewport_name, &self.node_area_select);

                if node_widget_response.is_some()
                {
                    widget_responses.push(node_widget_response.unwrap());
                }
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

            if self.port_searcher.is_some()
            {
                connection_widget::show_connection_search(scene_ui, graph_editor, &self.port_searcher.as_ref().unwrap(), &mouse_position_in_scene);
            }

            if self.quick_menu.is_some()
            {
                let clicked_quick_menu_button = self.show_quick_menu(scene_ui, graph_editor, &mut Vec::new());

                // if clicked_quick_menu_button
                // {
                //     // @TODO, this is a really bad way to detect the interaction in the loop, find a better way!
                //     widget_responses.push( NodeWidgetResponse { key: 0, kind: node_widget::NodeWidgetResponseType::ToggledQuickMenu });
                //     self.quick_menu = None;
                // }
            }

            self.mouse_scene_position_last_frame = mouse_position_in_scene;
        });
        self.scene_rect = scene_rect;

        if self.node_select_panel.is_some()
        {
            let added_node = self.node_select_panel.as_mut().unwrap().show(ui, graph_editor, &self.mouse_scene_position_last_frame, &mut Vec::new());

            if added_node
            {
                self.node_select_panel = None;
            }
        }

        widget_responses
    }


    fn process_widget_responses(&mut self, widget_responses: &Vec<NodeWidgetResponse>, graph_editor: &mut GraphEditor) -> bool
    {
        let mut interaction_happened = false;

        for response in widget_responses
        {
            match &response.kind
            {
                node_widget::NodeWidgetResponseType::ClickedTitle => self.toggle_node_in_selected_nodes(&response.key, graph_editor),
                node_widget::NodeWidgetResponseType::ClickedInputPort( port_key ) => self.toggle_input_port_search_or_add_connection(&port_key, graph_editor),
                node_widget::NodeWidgetResponseType::ClickedOutputPort( port_key ) => self.toggle_output_port_search_or_add_connection(&port_key, graph_editor),
                node_widget::NodeWidgetResponseType::ChangedInputPortDisplayValue( port_key, modified_port_value) => self.set_input_port_value_if_display_value_can_convert(port_key, graph_editor, modified_port_value),
                node_widget::NodeWidgetResponseType::ChangedState(new_node_kind, new_display_node_kind) => self.set_state_changes(&response.key, graph_editor, &new_node_kind, &new_display_node_kind),
                node_widget::NodeWidgetResponseType::InsideSelectionRect => self.check_or_add_node_to_nodes_inside_selection_area(&response.key),
                node_widget::NodeWidgetResponseType::ToggledQuickMenu => self.quick_menu = None,
            }

            interaction_happened = true; // This just detect any widget has been interacted with
        }

        interaction_happened
    }

    fn process_user_actions(&mut self, user_inputs: &GraphViewportUserInputs, graph_editor: &mut GraphEditor, widget_interaction_happened_same_loop: bool)
    {
        // @TODO This action now can potentially be applied double!
        for selected_node_key in graph_editor.selected_nodes.clone()
        {
            if self.quick_menu.is_some() // @TODO, this is not a great apporach to stop movement
            {
                continue;
            }
            
            let display_node = graph_editor.display_nodes.get_mut(&selected_node_key).unwrap();
            // display_node.position += self.mouse_scene_delta; 

            graph_editor.refresh_display_node(selected_node_key);
        }

        if user_inputs.left_is_down && user_inputs.left_shift_is_down && self.node_area_select.is_none()
        {
            // In this case its fine to use last frame, as last frame will be current frame
            // self.node_selection_rect = Some( egui::Rect::from_min_max(self.mouse_scene_position_last_frame, self.mouse_scene_position_last_frame) );
            self.node_area_select = Some( NodeAreaSelect { start_point: self.mouse_scene_position_last_frame, rect: egui::Rect::ZERO, nodes_inside_rect: Vec::new() } );
            return;
        } 

        if self.node_area_select.is_some() && (!user_inputs.left_is_down || !user_inputs.left_shift_is_down)
        {
            graph_editor.selected_nodes = self.node_area_select.clone().unwrap().nodes_inside_rect;
            self.node_area_select = None;
            return;
        }

        if self.node_area_select.is_some()
        {
            let node_area_select = self.node_area_select.as_mut().unwrap();
            node_area_select.determine_area_select_rect(&self.mouse_scene_position_last_frame);

            return;
        }

        if widget_interaction_happened_same_loop
        {
            return;
        }

        // @TODO, this is not the desired behavior, but will work for now
        if user_inputs.right_clicked && graph_editor.selected_nodes.len() >= 1 && self.quick_menu.is_none()
        {
            // self.quick_menu = Some( self.mouse_scene_position_last_frame );
            return;
        }

        if user_inputs.left_clicked && self.port_searcher.is_some()
        {
            self.port_searcher = None;
            return;
        }

        if user_inputs.left_clicked
        {
            graph_editor.selected_nodes = Vec::new();
            return;
        }

        if user_inputs.right_clicked && self.node_select_panel.is_none()
        {
            // self.node_select_panel = Some( NodeSelectionPanel::new(self.mouse_scene_position_last_frame) );
            self.node_select_panel = Some( NodeSelectionPanel::new(user_inputs.mouse_position) );
            return;
        }

        if user_inputs.right_clicked && self.node_select_panel.is_some()
        {
            self.node_select_panel = None;
            return;
        }

    }

    // @TODO, consider where this function should be (maybe it should be in graph editor?)
    fn toggle_node_in_selected_nodes(&self, node_key: &NodeGraphKey, graph_editor: &mut GraphEditor)
    {
        if graph_editor.selected_nodes.contains(node_key) // @TODO, figure out if this is the most performance apporaach.
        {
            graph_editor.selected_nodes.retain(|x| x != node_key );
        }
        else // @TODO, rewrite this
        {
            graph_editor.selected_nodes.push(*node_key);
        }
    }

    fn check_or_add_node_to_nodes_inside_selection_area(&mut self, node_key: &NodeGraphKey)
    {
        if self.node_area_select.is_none()
        {
            return;
        }

        let node_area_select = self.node_area_select.as_mut().unwrap();

        if node_area_select.nodes_inside_rect.contains(node_key)
        {
            return;
        }

        node_area_select.nodes_inside_rect.push(*node_key);
    }

    fn toggle_input_port_search_or_add_connection(&mut self, port_key: &NodeGraphKey, graph_editor: &mut GraphEditor)
    {   
        if self.port_searcher.is_none()
        {
            // First check if the input port already has a connection, remove that connection, and either convert that to a port search or overtake it
            if graph_editor.node_graph.input_port_has_connection(port_key)
            {
                let connect_output_port_key = graph_editor.node_graph.get_input_port_connection_key(port_key).expect("Tried to access ouptut port in connection-in, not available").clone();
                graph_editor.node_graph.remove_connection(port_key, &connect_output_port_key);

                self.port_searcher = Some( PortSearcher::output_port_searching(connect_output_port_key) );
                return;
            }

            self.port_searcher = Some( PortSearcher::input_port_searching(*port_key) );
            return;
        }

        let port_searcher = self.port_searcher.as_ref().unwrap();

        match port_searcher.port_kind
        {
            PortKind::Input => // Detect if user clicked another input port while port searching from input
            {
                if port_searcher.port_key == *port_key
                {
                    self.port_searcher = None; // @TODO, expand this functionality to be more complex
                }
            },
            PortKind::Output => // Detect if ports can be connected
            {
                let add_connection_result = graph_editor.node_graph.add_connection(port_searcher.port_key, *port_key);

                match add_connection_result
                {
                    Ok(()) => println!("Added connection: {}, {}", port_searcher.port_key, *port_key),
                    Err( text ) => println!("Failed to add connection because: {}", text),
                }

                self.port_searcher = None;
            },
        }

    }

    // @TODO, figure out if some of these functions should get added to graph_editor instead
    fn toggle_output_port_search_or_add_connection(&mut self, port_key: &NodeGraphKey, graph_editor: &mut GraphEditor)
    {
        if self.port_searcher.is_none()
        {
            self.port_searcher = Some( PortSearcher::output_port_searching(*port_key) );
            return;
        }

        let port_searcher = self.port_searcher.as_ref().unwrap(); 

        match  port_searcher.port_kind 
        {
            PortKind::Output =>
            {
                if port_searcher.port_key == *port_key
                {
                    self.port_searcher = None;
                }
            }
            PortKind::Input =>
            {
                let add_connection_result = graph_editor.node_graph.add_connection(*port_key, port_searcher.port_key); 

                match add_connection_result
                {
                    Ok(()) => println!("Added connection: {}, {}", *port_key, port_searcher.port_key),
                    Err( text ) => println!("Failed to add connection because: {}", text),
                }

                self.port_searcher = None;
            },
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

