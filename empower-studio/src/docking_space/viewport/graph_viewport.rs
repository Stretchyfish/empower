use std::collections::VecDeque;

use crate::{studio_context::{Cache, Log, StudioContext}, user_inputs::UserInputs};

use std::collections::{HashMap, HashSet};
use empower_engine::{assets::AssetId, node_graph::{NodeGraph, NodeGraphKey, Port, node::{NodeKind2, node_kind::{NodeState, NodeSyncResponse}}, port::{PortDirection, PortKind}}, value::Value};
use serde::{Serialize, Deserialize};

mod node_widget;
pub use node_widget::EditablePortValue;
pub use node_widget::EditableNodeState;

mod connection_widget;

mod area_select;
use area_select::AreaSelect;

mod node_picker;
use node_picker::NodePicker;

mod selected_nodes_quick_menu;
use selected_nodes_quick_menu::SelectedNodesQuickMenu;

#[derive(Clone, Serialize, Deserialize)]
pub struct GraphViewport
{
    pub graph_asset_id: AssetId,
    scene_rect: egui::Rect,

    #[serde(skip)]
    mouse_scene_position_last_frame: egui::Pos2, 

    #[serde(skip)]
    mouse_scene_delta_last_frame: egui::Vec2,

    #[serde(skip)]
    selected_nodes: HashSet<NodeGraphKey>,

    #[serde(skip)]
    selected_port: Option<NodeGraphKey>,

    #[serde(skip)]
    area_select: Option<AreaSelect>,

    #[serde(skip)]
    node_picker: NodePicker,
    
    #[serde(skip)]
    selected_nodes_quick_menu: SelectedNodesQuickMenu,
}

impl GraphViewport
{
    pub fn new( graph_asset_id: AssetId ) -> Self
    {
        Self
        {
            graph_asset_id,

            scene_rect: egui::Rect { min: egui::Pos2 { x: -650.0, y: -650.0 }, max: egui::Pos2 { x: 650.0, y: 650.0 }},

            mouse_scene_position_last_frame: egui::Pos2::ZERO,
            mouse_scene_delta_last_frame: egui::Vec2::ZERO,
            selected_nodes: HashSet::new(),
            selected_port: None,
            area_select: None,
            node_picker: NodePicker::new(),
            selected_nodes_quick_menu: SelectedNodesQuickMenu::new(),
        }
    }
}

pub fn show(graph_viewport: &mut GraphViewport, ui: &mut egui::Ui, studio_context: &mut StudioContext, viewport_name: &String, user_inputs: &UserInputs)
{
    let developer_mode = studio_context.get_settings().developer_mode;

    let (project, cache) = studio_context.get_project_and_cache_mut();

    let mut graph_viewport_actions = VecDeque::new(); // To simplify behavior, its beneficial to delay execution using actions
            
    let image_names = project.assets.get_all_image_names(); // @TODO, find a better way to get these asset values at runtime
    let node_graph_names = project.assets.get_all_node_graph_names(); // This is used later in the node widget drawing stage

    // @TODO, this is a very dangerous call, which needs a second look!
    let node_graph = project.assets.get_node_graph_naive(&graph_viewport.graph_asset_id); //.expect("graph editor tried to read a node graph but didn't get it from assets");

    if node_graph.is_none()
    {
        ui.label("unable to find node graph");
        return;
    }

    let node_graph = node_graph.unwrap();

    graph_viewport.node_picker.show(ui, &mut graph_viewport_actions, &graph_viewport.mouse_scene_position_last_frame);
    graph_viewport.selected_nodes_quick_menu.show(ui, &graph_viewport.selected_nodes, &mut graph_viewport_actions);

    graph_viewport.show_canvas(ui, &mut graph_viewport_actions, node_graph, user_inputs, cache, viewport_name, &node_graph_names, &image_names, &developer_mode);

    graph_viewport.apply_mouse_delta_to_selected_nodes(studio_context);
    graph_viewport.process_graph_viewport_actions(studio_context, user_inputs, graph_viewport_actions, viewport_name);

    if developer_mode
    {
        show_graph_viewport_debug_info(graph_viewport, ui);
    }
}

impl GraphViewport
{
    fn show_canvas(&mut self, ui: &mut egui::Ui, mut graph_viewport_actions: &mut VecDeque<GraphViewportAction>, node_graph: &NodeGraph, user_inputs: &UserInputs, cache: &mut Cache, viewport_name: &String, node_graph_names: &HashMap<AssetId, String>, image_names: &HashMap<AssetId, String>, developer_mode: &bool)
    {
        let mut drag_pan_button = egui::DragPanButtons::PRIMARY;
        if user_inputs.holding_shift // This is done to disable dragging of the scene during node area select
        {
            drag_pan_button = egui::DragPanButtons::empty();
        }

        let mouse_is_inside_viewport = ui.ui_contains_pointer();

        let cached_port_positions = &mut cache.session.cached_port_positions;
        let cached_editable_port_values = &mut cache.session.cached_editable_port_value;
        let cached_editable_node_state = &mut cache.session.cached_edtable_node_state;

        let mut scene_rect = self.scene_rect.clone(); // This is needed to avoid borrow issues
        egui::Scene::new()
        .zoom_range(0.01..=2.0)
        // .max_inner_size(egui::Vec2 { x: 200.0, y: 200.0 })
        .drag_pan_buttons(drag_pan_button)
        .show(ui, &mut scene_rect, |scene_ui|
        {
            for connection in &node_graph.connections_in
            {
                connection_widget::show(scene_ui, connection.1, connection.0, &node_graph, cached_port_positions, developer_mode);
            }

            if cache.session.instruction_highlighted_nodes.is_some()
            {
                let node_address = cache.session.instruction_highlighted_nodes.as_ref().unwrap();
                if node_address.graph_id == self.graph_asset_id
                {
                    node_widget::highlight(scene_ui, &node_address.node_key, node_graph, egui::Color32::BLUE);
                }
            }

            for highlighed_node_address in &cache.session.debug_highlighted_nodes
            {
                if highlighed_node_address.graph_id == self.graph_asset_id
                {
                    node_widget::highlight(scene_ui, &highlighed_node_address.node_key, node_graph, egui::Color32::GREEN);
                }
            }

            let node_keys: Vec<NodeGraphKey> = node_graph.nodes.keys().cloned().collect();
            for node_key in node_keys
            {
                if self.selected_nodes.contains(&node_key)
                {
                    node_widget::highlight(scene_ui, &node_key, node_graph, egui::Color32::ORANGE);
                }

                if self.area_select.is_some()
                {
                    if self.area_select.as_ref().unwrap().nodes_inside_rect.contains(&node_key)
                    {
                        node_widget::highlight(scene_ui, &node_key, node_graph, egui::Color32::ORANGE);
                    }
                }

                node_widget::show(scene_ui, &node_key, &self.graph_asset_id, node_graph, &viewport_name, &mut graph_viewport_actions, &mut self.area_select, cached_port_positions, cached_editable_port_values, cached_editable_node_state, node_graph_names, image_names, developer_mode);
            }

            if self.selected_port.is_some()
            {
                connection_widget::search_show(scene_ui, self.selected_port.as_ref().unwrap(), node_graph, &self.mouse_scene_position_last_frame, cached_port_positions);
            }

            if self.area_select.is_some()
            {
                scene_ui.painter().rect_filled(self.area_select.as_ref().unwrap().rect, 0.5, egui::Color32::from_rgba_unmultiplied(255, 140, 0, 70));
            }

            // Calculate the delta position and process background actions
            self.mouse_scene_delta_last_frame = egui::Vec2::ZERO; 
            if !mouse_is_inside_viewport // No point in processing anything below if the mouse is not inside the viewport
            {
                return;
            }

            let mouse_scene_position = self.screen_position_to_scene_position(&user_inputs.mouse_position, &scene_ui);
            self.mouse_scene_delta_last_frame = mouse_scene_position - self.mouse_scene_position_last_frame; 
            self.mouse_scene_position_last_frame = mouse_scene_position;

            self.process_canvas_background_user_inputs(user_inputs, &mut graph_viewport_actions);
        });

        self.scene_rect = scene_rect;
    }

    fn apply_mouse_delta_to_selected_nodes(&self, studio_context: &mut StudioContext)
    {
        if self.selected_nodes_quick_menu.is_active()
        {
            return;
        }

        let node_graph = studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).unwrap();

        for selected_node_key in &self.selected_nodes
        {
            let node = node_graph.nodes.get_mut(&selected_node_key ).unwrap();
            node.position += self.mouse_scene_delta_last_frame;
        }
    }

    fn screen_position_to_scene_position(&self, scene_position: &egui::Pos2, ui: &egui::Ui) -> egui::Pos2
    {
        return ui.ctx().layer_transform_from_global(ui.painter().layer_id()).unwrap() * *scene_position;
    }

    fn process_canvas_background_user_inputs(&self, user_inputs: &UserInputs, graph_viewport_actions: &mut VecDeque<GraphViewportAction>)
    {
        if !graph_viewport_actions.is_empty() // If no actions has happened yet, it must mean that whatever happens now is happening on the background
        {
            return;
        }
        
        if user_inputs.holding_primary_mouse_button && user_inputs.holding_shift
        {
            graph_viewport_actions.push_back( GraphViewportAction::DragSelecting );
            return;
        }

        if (!user_inputs.holding_primary_mouse_button || !user_inputs.holding_shift) && self.area_select.is_some()
        {
            graph_viewport_actions.push_back( GraphViewportAction::StoppedDragSelecting );
            return;
        }

        if user_inputs.clicked_primary_mouse_button
        {
            graph_viewport_actions.push_back( GraphViewportAction::PrimaryClickedBackground );
        }

        if user_inputs.clicked_secondary_mouse_button
        {
            graph_viewport_actions.push_back( GraphViewportAction::SecondaryClickedBackground );
        }
    }

    fn process_graph_viewport_actions(&mut self, studio_context: &mut StudioContext, user_inputs: &UserInputs, graph_viewport_actions: VecDeque<GraphViewportAction>, _: &String)
    {
        for action in graph_viewport_actions
        {
            match action
            {
                GraphViewportAction::ClickedNodeTitle { node_key } =>
                            {
                                if self.selected_nodes.len() == 1
                                {
                                    if self.selected_nodes.contains(&node_key)
                                    {
                                        self.selected_nodes.clear();
                                    }
                                    else
                                    {
                                        self.selected_nodes.clear();
                                        self.selected_nodes.insert(node_key);
                                    }

                                    break;
                                }

                                self.selected_nodes.insert(node_key);
                            },
                GraphViewportAction::ClickedPort { port_key } =>
                            {
                                let node_graph = studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).expect("graph viewport tried and failed to fetch node graph from assets in process graph viewport actions");

                                if node_graph.contains_connection(&port_key) // Since this should only ever be true for input ports, we do not need to check their direction
                                {
                                    let connected_output_port = node_graph.remove_connection(&port_key).unwrap();

                                    if self.selected_port.is_none()
                                    {
                                        self.selected_port = Some( connected_output_port );
                                        break;
                                    }
                                }

                                if self.selected_port.is_none()
                                {
                                    self.selected_port = Some( port_key );
                                    break;
                                }
                        
                                if self.selected_port.unwrap() == port_key
                                {
                                    self.selected_port = None;
                                    break;
                                }


                                let selected_port_direction;

                                {
                                    selected_port_direction = node_graph.ports.get(&self.selected_port.unwrap()).unwrap().direction;
                                    let clicked_port_direction = node_graph.ports.get(&port_key).unwrap().direction;

                                    if selected_port_direction == clicked_port_direction // If they are the same port direction, then they can't connect.
                                    {
                                        self.selected_port = None;
                                        break;
                                    }
                                }

                                let added_connection_result = match selected_port_direction
                                {
                                    PortDirection::Input => node_graph.add_connection(&port_key, &self.selected_port.unwrap()),
                                    PortDirection::Output => node_graph.add_connection(&self.selected_port.unwrap(), &port_key),
                                };

                                if let Err( error_text ) = added_connection_result
                                {
                                    studio_context.add_log( Log::info(error_text) );
                                }

                                self.selected_port = None;
                            },
                GraphViewportAction::DragSelecting =>
                            {
                                if self.area_select.is_none()
                                {
                                    self.area_select = Some( AreaSelect::new(self.mouse_scene_position_last_frame) );
                                    break;
                                }

                                let area_select = self.area_select.as_mut().unwrap();

                                area_select.determine_area_select_rect(&self.mouse_scene_position_last_frame);
                            },
                GraphViewportAction::StoppedDragSelecting =>
                            {
                                if self.area_select.is_some()
                                {
                                    self.selected_nodes = self.area_select.as_ref().unwrap().nodes_inside_rect.clone();
                                }
        
                                self.area_select = None;
                            },
                GraphViewportAction::PrimaryClickedBackground =>
                {
                    self.selected_nodes.clear();

                    if self.selected_nodes_quick_menu.is_active()
                    {
                        self.selected_nodes_quick_menu.off();
                    }
                    
                    self.selected_port = None;
                },
                GraphViewportAction::SecondaryClickedBackground =>
                {
                    if self.selected_nodes.is_empty()
                    {
                        self.node_picker.toggle_show( &user_inputs.mouse_position );
                        return;
                    }

                    self.selected_nodes_quick_menu.toggle_active( &user_inputs.mouse_position );
                }
                GraphViewportAction::PortEditWasChanged { port_key } => // @TODO, change to editable port value
                {
                    let editable_port_value = studio_context.get_cache().session.cached_editable_port_value.get(&port_key).unwrap().clone(); // Since this action doesn't happen so often, the clone here is not thaaat bad, but maybe find a way to simplify all of this in the future
                    let port = studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).unwrap().ports.get_mut(&port_key).unwrap();
                    port.value = editable_port_value.attempt_to_convert_to_value(&port.compatability);

                    // studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).unwrap().ports.get_mut(&port_key).unwrap().check_if_parseble(); // This has got to be the most questionable line of code I have ever written...
                },
                GraphViewportAction::NodeEditWasChanged { node_key } =>
                {
                    let (project, cache) = studio_context.get_project_mut_and_cache();
                    let editable_node_state = cache.session.cached_edtable_node_state.get(&node_key).unwrap(); 
                    let node = project.assets.get_node_graph_mut(&self.graph_asset_id).unwrap().nodes.get_mut(&node_key).unwrap();

                    let sync_response = editable_node_state.sync_with_node_state(&mut node.kind);

                    match sync_response
                    {
                        NodeSyncResponse::Nothing => {},
                        NodeSyncResponse::NodesStructureChanged =>
                        {
                            let graph = studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).unwrap();
                            graph.refresh_node(&node_key);
                        },
                        NodeSyncResponse::LoadSubgraph( node_graph_id ) =>
                        {
                            let project_path = &studio_context.get_project().location.clone();
                            let sub_graph = studio_context.get_project_mut().assets.load_node_graph(project_path, &node_graph_id).unwrap();

                            let start_node_input_ports = sub_graph.get_node_output_ports(sub_graph.start_node_key);

                            let _ = NodeState::GraphStartAndEndPorts { start_input_ports: start_node_input_ports, end_output_ports: Vec::new() };
                            
                            let graph = studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).unwrap();
                            // graph.nodes.get_mut(&node_key).unwrap().kind.sync_node_state( node_state );
                            graph.refresh_node(&node_key);
                        },
                    }
                },

                GraphViewportAction::RequestNewGraphViewportOrFocus { graph_id } =>
                {
                    studio_context.request_add_or_focus_graph_viewport( graph_id );
                }

                GraphViewportAction::CopySelectedNodes =>
                {
                    let selected_nodes_copy = self.selected_nodes.clone();
                    self.selected_nodes.clear();
                    self.selected_nodes_quick_menu.off();

                    let graph = studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).unwrap();

                    for node_key in &selected_nodes_copy  
                    {
                        let new_node_key = graph.create_node_copy(node_key); 
                        self.selected_nodes.insert(new_node_key);
                    } 
                }

                GraphViewportAction::DeleteSelectedNodes =>
                {
                    let graph = studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).unwrap();

                    for node_key in &self.selected_nodes
                    {
                        graph.remove_node(node_key);
                    }

                    self.selected_nodes.clear();
                }

                GraphViewportAction::AddNodeToGraph { node_kind, position } =>
                {
                    let node_graph = studio_context.get_project_mut().assets.get_node_graph_mut(&self.graph_asset_id).unwrap();
                    node_graph.add_node(node_kind, position);
                }
            }
        }
    }
}

pub enum GraphViewportAction
{
    ClickedNodeTitle { node_key: NodeGraphKey },
    ClickedPort { port_key: NodeGraphKey },
    CopySelectedNodes,
    DeleteSelectedNodes,
    DragSelecting,
    StoppedDragSelecting,
    PrimaryClickedBackground,
    SecondaryClickedBackground,
    NodeEditWasChanged { node_key: NodeGraphKey },
    PortEditWasChanged { port_key: NodeGraphKey },
    RequestNewGraphViewportOrFocus { graph_id: AssetId },
    AddNodeToGraph { node_kind: NodeKind2, position: Option<egui::Pos2>},
}

fn show_graph_viewport_debug_info(graph_viewport: &mut GraphViewport, ui: &mut egui::Ui)
{
    let debug_layer = egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("graph_viewport_debug_info"), // @TODO, this id needs to be unique to viewport
    );

    let viewport_rect = ui.max_rect();

    let painter = ui.layer_painter(debug_layer)
                    .with_clip_rect(viewport_rect);

    let debug_info = format!(
        "\
        Graph Viewport
        Graph id: {}
        scene_rect: {:?}
        mouse_position_last_frame: {:?}
        mouse_position_delta_last_frame: {:?}
        selected nodes: {:?}
        selected_port: {:?}
        area_select: {:?}
        node_picker: {:?}
        selected_nodes_quick_menu: {:?}"
        , graph_viewport.graph_asset_id
        , graph_viewport.scene_rect
        , graph_viewport.mouse_scene_position_last_frame
        , graph_viewport.mouse_scene_delta_last_frame
        , graph_viewport.selected_nodes
        , graph_viewport.selected_port
        , graph_viewport.area_select
        , graph_viewport.node_picker
        , graph_viewport.selected_nodes_quick_menu
    );

    painter.text(
        viewport_rect.left_top() + egui::vec2(10.0, 10.0),
        egui::Align2::LEFT_TOP,
        debug_info,
        egui::FontId::monospace(14.0),
        egui::Color32::RED,
    );
}

fn get_port_color(port: &Port) -> egui::Color32 // @TODO, move this into its own file
{
    match port.kind
    {
        PortKind::Execution => egui::Color32::WHITE,
        PortKind::Data =>
        {
            let value = if port.value.is_some()
            {
                port.value.as_ref().unwrap()
            }
            else
            {
                &port.compatability[0] // This should never be false
            };
        
            match value 
            {
                Value::Integer(_) => egui::Color32::YELLOW,
                Value::Float(_) => egui::Color32::BLUE,
                Value::Bool(_) => egui::Color32::PURPLE,
                Value::Image(_) => egui::Color32::GREEN,
                Value::Point2d(_, _) => egui::Color32::ORANGE,
                Value::List(_) => egui::Color32::PURPLE,
            }
        }
    }
}
