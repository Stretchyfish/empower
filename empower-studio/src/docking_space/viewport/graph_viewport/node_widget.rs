use std::collections::{HashMap, VecDeque};

use empower_engine::node_graph::{NodeGraph, NodeGraphKey};
use crate::docking_space::viewport::graph_viewport::area_select::AreaSelect;

use super::GraphViewportAction;

mod node_widget_body;
mod node_widget_input_ports;
mod node_widget_output_ports;

const PORT_SIZE: egui::Vec2 = egui::Vec2 { x: 50.0, y: 50.0 };
const VERTICAL_PORT_GAB: f32 = 60.0;

pub fn show(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    node_graph: &mut NodeGraph, 
    graph_viewport_title: &String, 
    // node_area_select: &mut Option<NodeAreaSelect>,
    graph_viewport_actions: &mut VecDeque<GraphViewportAction>,
    area_select: &mut Option<AreaSelect>,
    cached_node_sizes: &mut HashMap<NodeGraphKey, egui::Vec2>,
    cached_port_positions: &mut HashMap<NodeGraphKey, egui::Pos2>,
    developer_mode: &bool,
)
{
    let node = node_graph.nodes.get_mut(node_key).unwrap();

    let vertical_offset_before_showing_ports = node_widget_body::show(ui, node_key, node, graph_viewport_title, graph_viewport_actions, area_select, cached_node_sizes);

    for (input_port_index, input_port_key) in node.input_port_keys.iter().enumerate()
    {
        let input_port = node_graph.ports.get(input_port_key).unwrap();
        node_widget_input_ports::show(ui, &node.position, input_port_key, input_port, input_port_index, &true, &String::from(graph_viewport_title), graph_viewport_actions, cached_port_positions, vertical_offset_before_showing_ports, developer_mode);
    }

    for (output_port_index, output_port_key) in node.output_port_keys.iter().enumerate()
    {
        let output_port = node_graph.ports.get(output_port_key).unwrap();
        node_widget_output_ports::show(ui, &node.position, output_port_key, output_port, output_port_index, &true, &String::from(graph_viewport_title), graph_viewport_actions, &cached_node_sizes, cached_port_positions, vertical_offset_before_showing_ports, developer_mode);
    }
}

pub fn highlight(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    node_graph: &mut NodeGraph, 
    cached_node_sizes: &mut HashMap<NodeGraphKey, egui::Vec2>,
)
{
    if !cached_node_sizes.contains_key(node_key)
    {
        return;
    }

    let node = node_graph.nodes.get(node_key).unwrap();
    let node_size = cached_node_sizes.get(node_key).unwrap();
    
    let node_rect = egui::Rect::from_min_size(
        node.position,
        *node_size
    );

    let node_body_outline_margin = egui::Vec2 { x: 10.0, y: 10.0 };
    let node_outline_rect = node_rect.expand2(node_body_outline_margin); // @TODO, make this global?

    ui.painter().rect(
        node_outline_rect,
        6.0,
        egui::Color32::ORANGE,
        egui::Stroke::NONE,
        egui::StrokeKind::Inside,
    );
}
