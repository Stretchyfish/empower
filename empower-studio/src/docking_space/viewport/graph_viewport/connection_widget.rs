use std::collections::HashMap;

use empower_engine::node_graph::{NodeGraph, NodeGraphKey};


pub fn show(ui: &mut egui::Ui, port_from: &NodeGraphKey, port_to: &NodeGraphKey, cached_port_position: &HashMap<NodeGraphKey, egui::Pos2>)
{
    if !cached_port_position.contains_key(&port_from) || !cached_port_position.contains_key(&port_to) // First time drawn, these values will not be there due to being drawn before node widget
    {
        return;
    }

    let port_from_position = cached_port_position.get(&port_from).unwrap();
    let port_to_position = cached_port_position.get(&port_to).unwrap();
    
    ui.painter().line_segment([ *port_from_position, *port_to_position], egui::Stroke::new(10.0, egui::Color32::YELLOW));
}

pub fn search_show(ui: &mut egui::Ui, selected_port: &NodeGraphKey, node_graph: &mut NodeGraph, mouse_scene_position: &egui::Pos2, cached_port_position: &HashMap<NodeGraphKey, egui::Pos2>)
{
    if !cached_port_position.contains_key(selected_port)
    {
        return;
    }

    let port_position = cached_port_position.get(selected_port).unwrap();
    
    ui.painter().line_segment([ *port_position, *mouse_scene_position], egui::Stroke::new(10.0, egui::Color32::YELLOW));
}
