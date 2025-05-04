use egui;
use empower_engine::EmpowerKey;
mod background;
mod widgets;
mod panels;

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

        let user_input = interactions::user::inputs::detect_user_inputs(ui);
        // let viewport_bounds_rect = ui.available_rect_before_wrap();

        // For this to work, no other element should depend on dragging

        // egui::SidePanel::left("my left panel") // This is good code to save for later
        // .show_inside(ui, |ui|
        // {
            
        // });

        let mut stopped_selecting_an_area_of_nodes = false;
        if self.state.node_select_rect.is_some() && (!user_input.left_is_down || !user_input.left_shift_is_down)
        {
            stopped_selecting_an_area_of_nodes = true;
        }

        let mut nodes_view_responses = Vec::new();
        egui::CentralPanel::default() // This extra central panel layer makes nodes out of viewport rect not spawn scrollbars, and also future proffs
        .show_inside(ui, |ui| {

        let background_reponse = ui.allocate_rect(viewport_rect, egui::Sense::drag());
        if background_reponse.drag_started() && user_input.left_is_down
        {
            self.state.dragging_background = true;
        }
        if background_reponse.drag_stopped() && !user_input.left_is_down
        {
            self.state.dragging_background = false;
        }
        // background::visualize_background(ui, &self.state);

        let display_node_keys: Vec<EmpowerKey> = node_graph.display_nodes.keys().cloned().collect(); 

        // for (_display_node_key, display_node) in node_graph.display_nodes.iter_mut()
        // {
        //     let reponse = widgets::graph_node_widget::view_graph_node_widget(ui, display_node, &node_graph, &self.state, &user_input);

        //     if let Some(view_response) = reponse
        //     {
        //         nodes_view_responses.push(view_response);
        //     }
        // }
        for display_node_key in display_node_keys        
        {
            let reponse = widgets::graph_node_widget::view_graph_node_widget(ui, display_node_key, node_graph, &self.state, &user_input);

            if let Some(view_response) = reponse
            {
                nodes_view_responses.push(view_response);
            }
        }

        });

        if self.state.port_search.is_some()
        {
            let port_search = self.state.port_search.unwrap();

            let node_key;
            let port_relative_position;
            match port_search.port_kind
            {
                PortKind::InputPort => 
                {
                    let display_input_port = node_graph.display_input_ports.get(&port_search.port_key).unwrap();
                    port_relative_position = display_input_port.relative_position;                   
                    node_key = display_input_port.node_key;
                }

                PortKind::OutputPort =>
                {
                    let display_output_port= node_graph.display_output_ports.get(&port_search.port_key).unwrap();
                    port_relative_position = display_output_port.relative_position;                   
                    node_key = display_output_port.node_key;
                }
            }

            // @TODO A crash can happen here if the node gets removed, needs to be fixed
            let display_node = node_graph.display_nodes.get(&node_key).unwrap();
            // let node_position = node_graph.display_nodes.get(&node_key).unwrap().position;

            let node_centering_offset = egui::Vec2{ x: -display_node.size.x / 2.0 , y: 0.0 }; // This value is used to center node around its middle, instead of around its top left corner
            let port_position = display_node.position + port_relative_position + node_centering_offset;
            // let port_position = node_position;

            let port_draw_position = self.state.pan_zoom.world_to_screen(&port_position);

            ui.painter().line_segment([ port_draw_position, user_input.mouse_position], egui::Stroke::new(5.0 * self.state.pan_zoom.zoom_scale, egui::Color32::YELLOW));
        }
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
                    ui.painter().rect(self.state.node_select_rect.unwrap(), 0.0, egui::Color32::ORANGE.gamma_multiply(0.2), egui::Stroke::NONE);
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

#[derive(Default)]
pub struct GraphViewportState
{
    pub title: String,
    pub selected_nodes: Vec<i32>,
    pub node_select_rect: Option<egui::Rect>, // Consider improving the naming
    pub window_size: egui::Vec2,
    pub pan_zoom: PanZoom,
    pub dragging_background: bool,
    pub node_selection_panel_state: panels::NodeSelectionPanelState,
    port_search: Option<PortSearcher>,
}

impl GraphViewportState
{
    pub fn new(initial_title: String) -> Self
    {
        Self
        {
            title: initial_title,
            selected_nodes: Vec::new(),
            node_select_rect: Option::None,
            window_size: egui::Vec2 { x: 0.0, y: 0.0 },
            pan_zoom: PanZoom::new(),
            dragging_background: false,
            node_selection_panel_state: panels::NodeSelectionPanelState::new(),
            port_search: Option::None,
        }
    }    
}

// struct NodeSelectRect
// {
//     pub rect: egui::Rect,
// }

pub struct PanZoom
{
    pub zoom_scale: f32,
    pub pan_offset: egui::Vec2,
    pub window_size_pan_offset: egui::Vec2,
    zoom_speed: f32,
}

impl Default for PanZoom
{
    fn default() -> Self
    {
        Self
        {
            zoom_scale: 1.0,
            pan_offset: egui::Vec2 { x: 0.0, y: 0.0 },
            window_size_pan_offset: egui::Vec2 { x: 0.0, y: 0.0 },
            zoom_speed: 0.01,
        }
    }
}

impl PanZoom
{
    pub fn new() -> Self
    {
        Self
        {
            zoom_scale: 1.0,
            pan_offset: egui::Vec2 { x: 0.0, y: 0.0 },
            window_size_pan_offset: egui::Vec2 { x: 0.0, y: 0.0 },
            zoom_speed: 0.001,
        }
    }

    // pub fn update_pan_zoom(&mut self, ui: &egui::Ui, user_input: &interactions::user::UserInputs, background_clicked: bool)
    // {
    //     if user_input.left_is_down && background_clicked
    //     {
    //         self.pan_offset += user_input.mouse_position_delta / self.zoom_scale;
    //     }


    //     let mouse_position_world_space_before_zoom = self.screen_to_world(&user_input.mouse_position);

    //     self.zoom_scale += user_input.scroll_delta * self.zoom_speed;
    //     self.zoom_scale = self.zoom_scale.clamp(0.1, 10.0);
        
    //     let mouse_position_world_space_after_zoom = self.screen_to_world(&user_input.mouse_position);

    //     self.pan_offset -= mouse_position_world_space_before_zoom - mouse_position_world_space_after_zoom;
      
    // }

    pub fn update_pan(&mut self, user_input: &interactions::user::UserInputs)
    {
        self.pan_offset += user_input.mouse_position_delta / self.zoom_scale;
    }

    pub fn update_zoom(&mut self, user_input: &interactions::user::UserInputs)
    {
        let mouse_position_world_space_before_zoom = self.screen_to_world(&user_input.mouse_position);

        self.zoom_scale += user_input.scroll_delta * self.zoom_speed;
        self.zoom_scale = self.zoom_scale.clamp(0.1, 10.0);
        
        let mouse_position_world_space_after_zoom = self.screen_to_world(&user_input.mouse_position);

        self.pan_offset -= mouse_position_world_space_before_zoom - mouse_position_world_space_after_zoom;
    }
    
    pub fn world_to_screen(&self, world_position: &egui::Pos2) -> egui::Pos2
    {
        (*world_position + self.pan_offset) * self.zoom_scale
    }

    pub fn screen_to_world(&self, screen_position: &egui::Pos2) -> egui::Pos2
    {
        // Should never happen, but make sure to add a divide by 0 check here
        // *screen_position / self.zoom_scale + self.pan_offset
        *screen_position / self.zoom_scale - self.pan_offset
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PortKind
{
    InputPort,
    OutputPort,
}

#[derive(Clone, Copy)]
struct PortSearcher
{
    port_key: EmpowerKey,
    port_kind: PortKind,
}
