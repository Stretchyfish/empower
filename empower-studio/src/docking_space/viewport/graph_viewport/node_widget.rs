use std::{collections::{HashMap, VecDeque}, f32};

use empower_engine::{assets::AssetId, node_graph::{NodeGraph, NodeGraphKey}};
use crate::docking_space::viewport::graph_viewport::{area_select::AreaSelect};

use super::GraphViewportAction;

mod node_widget_body;
mod node_widget_ports;

const PORT_SIZE: egui::Vec2 = egui::Vec2 { x: 50.0, y: 50.0 };
const VERTICAL_PORT_GAB: f32 = 60.0;

pub fn show(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    graph_id: &AssetId,
    node_graph: &NodeGraph, 
    graph_viewport_title: &String, 
    graph_viewport_actions: &mut VecDeque<GraphViewportAction>,
    area_select: &mut Option<AreaSelect>,
    cached_port_positions: &mut HashMap<NodeGraphKey, egui::Pos2>,
    node_graph_names: &HashMap<AssetId, String>,
    image_names: &HashMap<AssetId, String>,
    developer_mode: &bool,
)
{
    let node = node_graph.nodes.get(node_key).unwrap();

    let node_size = node.kind.size();

    // @TODO, remove this vertical gab, and find a better way of finding the size
    let vertical_offset_before_drawing_ports = node_widget_body::show(ui, node_key, node, graph_id, graph_viewport_title, graph_viewport_actions, area_select, &node_size, node_graph_names, image_names, developer_mode);

    for (input_port_index, input_port_key) in node.input_port_keys.iter().enumerate()
    {
        let port_has_connection = node_graph.connections_in.contains_key(input_port_key);

        let input_port = node_graph.ports.get(input_port_key).unwrap();
        node_widget_ports::show(ui, &node.position, input_port_key, input_port, input_port_index, &port_has_connection, &String::from(graph_viewport_title), graph_viewport_actions, &node_size, cached_port_positions, vertical_offset_before_drawing_ports, developer_mode);
    }

    for (output_port_index, output_port_key) in node.output_port_keys.iter().enumerate()
    {
        let output_port = node_graph.ports.get(output_port_key).unwrap();
        node_widget_ports::show(ui, &node.position, output_port_key, output_port, output_port_index, &false, &String::from(graph_viewport_title), graph_viewport_actions, &node_size, cached_port_positions, vertical_offset_before_drawing_ports, developer_mode);
    }
}

pub fn highlight(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    node_graph: &NodeGraph, 
    color: egui::Color32,
    // cached_node_sizes: &mut HashMap<NodeGraphKey, egui::Vec2>, // @TODO, add this behavior back
)
{
    // if !cached_node_sizes.contains_key(node_key)
    // {
    //     return;
    // }

    let node = node_graph.nodes.get(node_key).unwrap();
    // let node_size = cached_node_sizes.get(node_key).unwrap();
    let node_size = node.kind.size();
    
    let node_rect = egui::Rect::from_min_size(
        node.position,
        node_size
    );

    let node_body_outline_margin = egui::Vec2 { x: 10.0, y: 10.0 };
    let node_outline_rect = node_rect.expand2(node_body_outline_margin); // @TODO, make this global?

    ui.painter().rect(
        node_outline_rect,
        6.0,
        color,
        egui::Stroke::NONE,
        egui::StrokeKind::Inside,
    );
}
