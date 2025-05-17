use std::any::Any;

use egui;
use empower_engine::EmpowerKey;
mod background;
mod widgets;
mod panels;
mod utils;
mod graph_viewport_state;
use graph_viewport_state::GraphViewportState;

use utils::PortKind;
use utils::PortSearcher;
use widgets::graph_node_widget::NodeViewReponse;

use crate::interactions;
use crate::NodeGraph;


#[derive(Default)]
pub struct GraphViewport 
{
    pub state: GraphViewportState,
}

impl GraphViewport 
{
    pub fn new(initial_title: String) -> Self 
    {
        Self 
        {
            state: GraphViewportState::new(initial_title),
        }
    }

    pub fn view(&mut self, ui: &mut egui::Ui, node_graph: &mut NodeGraph) 
    {
        self.detect_and_handle_viewport_size_change(ui);

        let user_input = interactions::user::inputs::detect_user_inputs(ui);

        let mut nodes_view_responses = Vec::new(); 
        egui::CentralPanel::default() // This extra central panel layer makes nodes out of viewport rect not spawn scrollbars, and also future proffs
        .show_inside(ui, |ui| {

            self.detect_background_interactions(ui, &user_input);
            // background::visualize_background(ui, &self.state);

            nodes_view_responses = widgets::graph_node_widget::view_nodes_and_connections(ui, node_graph, &self.state, &user_input);
            widgets::graph_connection_widget::view_connection_search(ui, node_graph, &self.state, &user_input);
        });

        self.view_scrollbars(ui);
    
        self.update_state_and_ui(ui, &user_input, nodes_view_responses, node_graph);
    }

    fn detect_and_handle_viewport_size_change(&mut self, ui: &mut egui::Ui)
    {
        let viewport_rect = ui.max_rect();
        let viewport_size = viewport_rect.size();

        let viewport_changed_size = viewport_size != self.state.window_size;

        // This will keep elements on the screen centeret as the window resizes
        if viewport_changed_size
        {
            let new_window_size_pan_offset = viewport_rect.center().to_vec2();
            self.state.pan_zoom.pan_offset -= self.state.pan_zoom.window_size_pan_offset;
            self.state.pan_zoom.window_size_pan_offset = new_window_size_pan_offset;
            self.state.pan_zoom.pan_offset += self.state.pan_zoom.window_size_pan_offset;
            self.state.window_size = viewport_size;
        }
    }

    fn detect_background_interactions(&mut self, ui: &mut egui::Ui, user_input: &interactions::user::UserInputs)
    {
        let viewport_rect = ui.max_rect();
        let background_reponse = ui.allocate_rect(viewport_rect, egui::Sense::drag());
        if background_reponse.drag_started() && user_input.left_is_down
        {
            self.state.dragging_background = true;
        }
        if background_reponse.drag_stopped() && !user_input.left_is_down
        {
            self.state.dragging_background = false;
        }
    }
    
    fn view_scrollbars(&mut self, ui: &mut egui::Ui)
    {
                // This code for scroll bars work, but needs tunning
        // ================================================================
        // egui::TopBottomPanel::bottom("horizontal_scroll_bar_panel".to_string() + self.state.title.as_str())
        // .show_separator_line(false)
        // .show(ui.ctx(), |ui|
        // {
        //     ui.spacing_mut().slider_width = ui.max_rect().size().x;
        //     ui.spacing_mut().slider_rail_height = 5.0;
        //     let mut pan_value_in_horizontal_slider = self.state.pan_zoom.pan_offset.x - self.state.pan_zoom.window_size_pan_offset.x;
        //     ui.add(
        //         egui::Slider::new(&mut pan_value_in_horizontal_slider, -500.0..=500.0)
        //         .show_value(false)
        //     );
        //     self.state.pan_zoom.pan_offset.x = pan_value_in_horizontal_slider + self.state.pan_zoom.window_size_pan_offset.x;
        // });

        // egui::SidePanel::right("vertical_scroll_bar_panel".to_string() + self.state.title.as_str())
        // .show_separator_line(false)
        // .resizable(false)
        // .max_width(30.0) // @TODO, find a better approach for max size
        // .show(ui.ctx(), |ui|
        // {
        //     ui.spacing_mut().slider_width = ui.max_rect().size().y;
        //     ui.spacing_mut().slider_rail_height = 2.0;
        //     let mut pan_value_in_vertical_slider = self.state.pan_zoom.pan_offset.y - self.state.pan_zoom.window_size_pan_offset.y;
        //     ui.add(
        //         egui::Slider::new(&mut pan_value_in_vertical_slider , -500.0..=500.0)
        //         .vertical()
        //         .handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 2.0 })
        //         .show_value(false)
        //     );
        //     self.state.pan_zoom.pan_offset.y = pan_value_in_vertical_slider + self.state.pan_zoom.window_size_pan_offset.y;
        // });
        // ================================================================
      
    }

    pub fn update_state_and_ui(&mut self, ui: &mut egui::Ui, user_input: &interactions::user::UserInputs, nodes_view_responses: Vec<NodeViewReponse>, node_graph: &mut NodeGraph)
    {
        let mut stopped_selecting_an_area_of_nodes = false;
        if self.state.node_select_rect.is_some() && (!user_input.left_is_down || !user_input.left_shift_is_down)
        {
            stopped_selecting_an_area_of_nodes = true;
        }
        let mut node_was_clicked = false;
        let mut node_was_hovered = false;
        let mut search_was_started = false;
        for node_reponse in nodes_view_responses.iter()
        {
            match node_reponse.kind
            {
                widgets::graph_node_widget::NodeViewResponseType::Clicked =>
                {
                    node_was_clicked = true;
                    if let Some(selected_node_to_remove_index) = self.state.selected_nodes.iter().position(| selected_node_key | *selected_node_key == node_reponse.key )
                    {
                        self.state.selected_nodes.remove(selected_node_to_remove_index);
                        break;
                    }
                    // self.selected_nodes.retain(f);
                   self.state.selected_nodes.push(node_reponse.key); 
                }

                widgets::graph_node_widget::NodeViewResponseType::Hover =>
                {
                    node_was_hovered = true;
                }

                widgets::graph_node_widget::NodeViewResponseType::InsideSelectionArea =>
                {
                    if stopped_selecting_an_area_of_nodes
                    {
                        self.state.selected_nodes.push(node_reponse.key);
                    }
                }

                widgets::graph_node_widget::NodeViewResponseType::ClickedInputPort(port_key) =>
                {
                    if self.state.port_search.is_some() // @TODO, rewirte this check
                    {
                        let port_search = self.state.port_search.unwrap();
                        if port_search.port_kind == PortKind::OutputPort
                        {
                            node_graph.add_connection(port_key, port_search.port_key);
                            self.state.port_search = Option::None;
                        }

                    }
                    else
                    {
                        self.state.port_search = Some( PortSearcher { port_key: port_key, port_kind: PortKind::InputPort });
                        search_was_started = true;                    
                    }
                }

                widgets::graph_node_widget::NodeViewResponseType::ClickedOutputPort(port_key) =>
                {
                    if self.state.port_search.is_some() // @TODO, rewrite this check
                    {
                        let port_search = self.state.port_search.unwrap();
                        if port_search.port_kind == PortKind::InputPort
                        {
                            node_graph.add_connection( port_search.port_key, port_key);
                            self.state.port_search = Option::None;
                        }
                    }
                    else
                    {
                        self.state.port_search = Some( PortSearcher { port_key: port_key, port_kind: PortKind::OutputPort });
                        search_was_started = true;                    
                    }
                }

            }
        }

        if node_was_hovered
        {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
        }

        let has_selected_nodes = self.state.selected_nodes.len() > 0;
        if has_selected_nodes
        {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        }

        let detact_selected_nodes = self.state.selected_nodes.len() > 0 && (user_input.right_clicked || (user_input.left_clicked && !node_was_clicked));
        if detact_selected_nodes
        {
            self.state.selected_nodes.clear();    
        }

        if !detact_selected_nodes && user_input.right_clicked // Find a more elegant approach for this
        {
            self.state.node_selection_panel_state.show = !self.state.node_selection_panel_state.show;
            self.state.node_selection_panel_state.mouse_position_when_node_select_menu_was_activated = Option::Some(user_input.mouse_position );
        }
        
        if self.state.node_selection_panel_state.show
        {
            // This can be tidied up
            panels::node_selection_panel::view_node_selector(&mut self.state.node_selection_panel_state, node_graph, ui, &user_input, &mut self.state.pan_zoom);
        }
        
        if self.state.dragging_background
        {
            if !user_input.left_shift_is_down
            {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Move);
                self.state.pan_zoom.update_pan(&user_input);
            }
            
            if user_input.left_shift_is_down
            {
                if self.state.node_select_rect != Option::None
                {
                    self.state.node_select_rect = Option::Some( egui::Rect::from_min_max( self.state.node_select_rect.unwrap().min, user_input.mouse_position) );
                    ui.painter().rect(self.state.node_select_rect.unwrap(), 0.0, egui::Color32::ORANGE.gamma_multiply(0.2), egui::Stroke::NONE, egui::StrokeKind::Inside);
                }

                if self.state.node_select_rect == Option::None
                {
                    let initial_node_selection_start_pos = user_input.mouse_position.clone();
                    let initial_node_selection_rect = egui::Rect::from_min_size(initial_node_selection_start_pos, egui::Vec2::new(0.0, 0.0)); 
                    self.state.node_select_rect = Option::Some(initial_node_selection_rect);
                }
            }
        }

        if stopped_selecting_an_area_of_nodes
        {
            self.state.node_select_rect = Option::None;
        }


        if self.state.port_search.is_some() && user_input.left_clicked && search_was_started == false
        { 
            self.state.port_search = Option::None;
        }
        
        self.state.pan_zoom.update_zoom(&user_input);
    }
}

