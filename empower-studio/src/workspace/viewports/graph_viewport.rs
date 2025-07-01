use egui::accesskit::Node;
use empower_node_graph::EmpowerKey;
use crate::{graph_editor::{self, GraphEditor}, workspace::viewports::graph_viewport::port_searcher::PortSearcher};

mod node_widget;
mod connection_widget;
mod user_input; // @TODO, find a better structure for this
mod port_searcher;
mod node_selection_panel;
use node_selection_panel::NodeSelectionPanel;

pub struct GraphViewport
{
    pub title: String,
    node_selection_panel: NodeSelectionPanel,
    mouse_scene_position_last_frame: egui::Pos2,
    port_searcher: Option<PortSearcher>, 
    scene_rect: egui::Rect,
}

impl GraphViewport
{
   pub fn new(title: String) -> Self
   {
        Self 
        { 
            title, 
            node_selection_panel: NodeSelectionPanel::new(),
            mouse_scene_position_last_frame: egui::Pos2 { x: 0.0, y: 0.0 },
            port_searcher: None,
            scene_rect: egui::Rect { min: egui::Pos2 { x: -500.0, y: -500.0 }, max: egui::Pos2 { x: 500.0, y: 500.0 }},
        }
   } 
}

pub fn show(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport)
{
    let mut scene_rect = graph_viewport.scene_rect.clone(); // This is needed to avoid borrow issues

    let mut mouse_position_in_scene = graph_viewport.mouse_scene_position_last_frame; // Set to last frame, in case there is no new position in the scene

    egui::Scene::new()
    .zoom_range(0.01..=2.0)
    .max_inner_size(egui::Vec2 { x: 8.0, y: 8.0 })
    .show(ui, &mut scene_rect, |scene_ui|
    {
        let scene_transform = scene_ui.ctx().layer_transform_from_global(scene_ui.painter().layer_id());
        let scene_latest_pos = scene_ui.input(|i| i.pointer.latest_pos());

        if scene_transform.is_some() && scene_latest_pos.is_some()
        {
            mouse_position_in_scene = scene_transform.unwrap() * scene_latest_pos.unwrap();
        }

        let mouse_scene_delta = mouse_position_in_scene - graph_viewport.mouse_scene_position_last_frame; 

        // @TODO, find a more computationally effecient way of doing this
        let connection_keys = graph_editor.empower_node_graph.connections_out.clone();
        for connection_key in connection_keys.keys()
        {
            connection_widget::show(scene_ui, graph_editor, graph_viewport, &connection_key);
        }

        let node_keys: Vec<EmpowerKey> = graph_editor.display_nodes.keys().cloned().collect(); // @TODO, find a more elegant way of writting this
        for node_key in node_keys
        {
            node_widget::show(scene_ui, graph_editor, mouse_scene_delta, graph_viewport, &node_key);
        }

        connection_widget::show_connection_search(scene_ui, graph_editor, graph_viewport.port_searcher, &mouse_position_in_scene);

        graph_viewport.mouse_scene_position_last_frame = mouse_position_in_scene;
    });

    graph_viewport.scene_rect = scene_rect;

    let user_inputs = user_input::detect_user_inputs(ui);
    node_selection_panel::show(ui, &mut graph_viewport.node_selection_panel, graph_editor, &user_inputs, &mouse_position_in_scene);
}