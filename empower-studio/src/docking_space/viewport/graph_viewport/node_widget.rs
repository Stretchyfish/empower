use std::{collections::{HashMap, VecDeque}, f32};

use egui::cache;
use empower_engine::{node_graph::{Node, NodeGraph, NodeGraphKey}, value::Value};
use crate::docking_space::viewport::graph_viewport::{area_select::AreaSelect};

use super::GraphViewportAction;

mod node_widget_body;
use node_widget_body::NodeWidgetBodyResponse;

mod node_widget_ports;

const PORT_SIZE: egui::Vec2 = egui::Vec2 { x: 50.0, y: 50.0 };
const VERTICAL_PORT_GAB: f32 = 60.0;

pub fn show(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    node_graph: &mut NodeGraph, 
    graph_viewport_title: &String, 
    graph_viewport_actions: &mut VecDeque<GraphViewportAction>,
    area_select: &mut Option<AreaSelect>,
    cached_node_sizes: &mut HashMap<NodeGraphKey, egui::Vec2>,
    cached_port_positions: &mut HashMap<NodeGraphKey, egui::Pos2>,
    developer_mode: &bool,
)
{
    let node = node_graph.nodes.get_mut(node_key).unwrap();

    let body_response = node_widget_body::show(ui, node_key, node, graph_viewport_title, graph_viewport_actions, area_select, cached_node_sizes, developer_mode);

    let mut largest_horizontal_port_element = 0.0;

    for (input_port_index, input_port_key) in node.input_port_keys.iter().enumerate()
    {
        let port_has_connection = node_graph.connections.contains_key(input_port_key);

        let input_port = node_graph.ports.get_mut(input_port_key).unwrap();
        let port_response = node_widget_ports::show(ui, &node.position, input_port_key, input_port, input_port_index, &port_has_connection, &String::from(graph_viewport_title), graph_viewport_actions, &cached_node_sizes, cached_port_positions, body_response.vertical_offset_before_drawing_ports, developer_mode);

        if port_response.horizontal_element_size > largest_horizontal_port_element
        {
            largest_horizontal_port_element = port_response.horizontal_element_size;
        }
    }

    for (output_port_index, output_port_key) in node.output_port_keys.iter().enumerate()
    {
        let output_port = node_graph.ports.get_mut(output_port_key).unwrap();
        node_widget_ports::show(ui, &node.position, output_port_key, output_port, output_port_index, &false, &String::from(graph_viewport_title), graph_viewport_actions, &cached_node_sizes, cached_port_positions, body_response.vertical_offset_before_drawing_ports, developer_mode);
    }

    if largest_horizontal_port_element > body_response.title_bar_width 
    {
        let cached_node_size = cached_node_sizes.get(node_key).unwrap().clone();
        cached_node_sizes.insert(*node_key, egui::vec2(largest_horizontal_port_element, cached_node_size.y));
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

fn calculate_cached_node_size(
                                cached_node_sizes: &mut HashMap<NodeGraphKey, egui::Vec2>,
                                node_key: &NodeGraphKey,
                                node: &Node,
                                node_graph: &NodeGraph,
                                node_body_response: &NodeWidgetBodyResponse,
                                cached_port_positions: &mut HashMap<NodeGraphKey, egui::Pos2>
) -> egui::Vec2
{
    if node.input_port_keys.is_empty() && node.output_port_keys.is_empty()
    {
        return egui::vec2(node_body_response.title_bar_width, node_body_response.vertical_offset_before_drawing_ports);
    }
    
    let lowest_port_key = if node.input_port_keys.len() >= node.output_port_keys.len()
    {
        node.input_port_keys.last().unwrap()
    }
    else
    {
        node.output_port_keys.last().unwrap() // Safe to do, due to check above
    };

    let lowest_port = node_graph.ports.get(lowest_port_key).unwrap();

    if lowest_port.value.is_none()
    {
        return egui::vec2(node_body_response.title_bar_width, node_body_response.vertical_offset_before_drawing_ports);
    }

    let port_horizontal_elements_size = match lowest_port.value.as_ref().unwrap()
    {
        Value::Integer(_) => 120.0,
        Value::Float(_) => 120.0,
    };

    if cached_port_positions.contains_key(&lowest_port_key)
    {
        return egui::vec2(node_body_response.title_bar_width, node_body_response.vertical_offset_before_drawing_ports);
    }

    let lowest_port_position = cached_port_positions.get(&lowest_port_key).unwrap();

    if port_horizontal_elements_size > node_body_response.title_bar_width
    {
        return egui::vec2(port_horizontal_elements_size, lowest_port_position.y + VERTICAL_PORT_GAB);
    }
    
    return egui::vec2(node_body_response.title_bar_width, lowest_port_position.y + VERTICAL_PORT_GAB);
}
