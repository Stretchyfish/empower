use egui;
use empower_node_graph::EmpowerKey;
use crate::interactions::user::{inputs::detect_user_inputs, UserInputs};
use crate::StudioContext;

mod graph_viewport_state;
use graph_viewport_state::GraphViewportState;

mod port_searcher;
use port_searcher::PortSearcher;

mod port_kind;
use port_kind::PortKind;

mod graph_node_widget;
use graph_node_widget::{NodeViewReponse, NodeViewResponseType}; // @TODO, consider if these need better namespaces?

mod node_selection_panel;
mod graph_connection_widget;

pub struct GraphViewport
{
    pub title: String,
    pub state: GraphViewportState,
    scene_rect: egui::Rect,
}

impl GraphViewport
{
    pub fn new(title: String) -> Self
    {
        Self
        {
            title,
            state: GraphViewportState::new(),
            scene_rect: egui::Rect { min: egui::Pos2 { x: -500.0, y: -500.0 }, max: egui::Pos2 { x: 500.0, y: 500.0 }},
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, studio_context: &mut StudioContext)
    {
        let user_inputs = detect_user_inputs(ui);
        
        let mut nodes_view_responses = Vec::new();         

        let mut mouse_position_in_scene = self.state.mouse_scene_position_last_frame; // Set to last frame, in case there is no new position in the scene

        egui::Scene::new()
        .zoom_range(0.01..=2.0)
        .max_inner_size(egui::Vec2 { x: 8.0, y: 8.0 })
        .show(ui, &mut self.scene_rect, |scene_ui|
        {
            let scene_transform = scene_ui.ctx().layer_transform_from_global(scene_ui.painter().layer_id());
            let scene_latest_pos = scene_ui.input(|i| i.pointer.latest_pos());

            if scene_transform.is_some() && scene_latest_pos.is_some()
            {
                mouse_position_in_scene = scene_transform.unwrap() * scene_latest_pos.unwrap();
            }
          
            // let mut user_input = detect_user_inputs(ui);
            // println!("A: {}, {}", user_input.mouse_position.x, user_input.mouse_position.y);

            // let mut nodes_view_responses = Vec::new(); // Change to an optional?
            let display_node_keys: Vec<EmpowerKey> = studio_context.display_node_graph.display_nodes.keys().cloned().collect(); // @TODO, find a more elegant way of writting this

            for display_node_key  in display_node_keys
            {
                studio_context.show_node(scene_ui, &display_node_key);
            }
            // nodes_view_responses = graph_node_widget::show_nodes( scene_ui,  studio_context, &mut self.state, self.title.clone());

            graph_connection_widget::view_connection_search(scene_ui, studio_context, &self.state, &mouse_position_in_scene);
        });

        if self.state.node_selection_panel.show
        {
            node_selection_panel::view_node_selector(&mut self.state.node_selection_panel, studio_context, ui, &user_inputs, &mouse_position_in_scene);
        }

        let mouse_delta_position_in_scene = mouse_position_in_scene - self.state.mouse_scene_position_last_frame; 
        self.state.mouse_scene_position_last_frame = mouse_position_in_scene;

        self.update_state_based_on_node_responses(&nodes_view_responses, studio_context);
        // update nodes based on state
        self.update_nodes_based_on_state_and_user_actions(studio_context, mouse_delta_position_in_scene, user_inputs);
        // Update state based on user
    }

    fn update_state_based_on_node_responses(&mut self, nodes_view_responses: &Vec<NodeViewReponse>, studio_context: &mut StudioContext)
    {
        for node_reponse in nodes_view_responses
        {
            match node_reponse.kind
            {
                NodeViewResponseType::Clicked =>
                {
                    println!("Does this appear 2");
                    if let Some(selected_node_to_remove_index) = self.state.selected_nodes.iter().position(| selected_node_key | *selected_node_key == node_reponse.key )
                    {
                        self.state.selected_nodes.remove(selected_node_to_remove_index);
                        break;
                    }

                   self.state.selected_nodes.push(node_reponse.key); 
                }

                NodeViewResponseType::ClickedInputPort(port_key) =>
                {
                    if self.state.port_search.is_some() // @TODO, rewirte this check
                    {
                        let port_search = self.state.port_search.unwrap();
                        if port_search.port_kind == PortKind::OutputPort
                        {
                            studio_context.add_connection(node_reponse.key, port_search.port_key );
                            // node_graph.add_connection(node_reponse.key, port_search.port_key); // @TODO, add connections back
                            self.state.port_search = Option::None;
                        }

                    }
                    else
                    {
                    self.state.port_search = Some( PortSearcher { port_key: port_key, port_kind: PortKind::InputPort });
                        // search_was_started = true;                    
                    }
                    println!("Input port clicked: {}", port_key);
                }

                NodeViewResponseType::ClickedOutputPort(port_key) =>
                {
                    if self.state.port_search.is_some() // @TODO, rewrite this check
                    {
                        let port_search = self.state.port_search.unwrap();
                        if port_search.port_kind == PortKind::InputPort
                        {
                            studio_context.add_connection(port_search.port_key, port_key );
                            // node_graph.add_connection( port_search.port_key, port_key); // @TODO, add connections back
                            self.state.port_search = Option::None;
                        }
                    }
                    else
                    {
                        self.state.port_search = Some( PortSearcher { port_key: port_key, port_kind: PortKind::OutputPort });
                    }
                    println!("Output port clicked: {}", port_key);
                }

                _ =>
                {
                    
                }
            }
        }
    }

    fn update_nodes_based_on_state_and_user_actions(&mut self, studio_context: &mut StudioContext, mouse_delta_position_in_scene: egui::Vec2, user_inputs: UserInputs)
    {
        for selected_nodes_key in self.state.selected_nodes.iter()
        {
            let display_node = studio_context.display_node_graph.display_nodes.get_mut(&selected_nodes_key).unwrap(); // @TODO, simplify this call
            // display_node.position = self.state.mouse_scene_position_last_frame - self.state.grap_vector.unwrap();
            display_node.position += mouse_delta_position_in_scene;
        }

        if self.state.port_search.is_some() && user_inputs.right_clicked
        {
            self.state.port_search = None;
        }

        if user_inputs.right_clicked
        {
            self.state.node_selection_panel.show = !self.state.node_selection_panel.show;

            if self.state.node_selection_panel.show // @TODO, find a more elegant way of writting this
            {
                self.state.node_selection_panel.mouse_position_when_node_select_menu_was_activated = Some(user_inputs.mouse_position);
            }
        }
    }
}
