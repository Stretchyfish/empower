use std::collections::{HashMap, VecDeque};

use empower_engine::node_graph::port::{PortDirection, PortKind};
use empower_engine::node_graph::{NodeGraphKey, Port};

use super::GraphViewportAction;

use super::{PORT_SIZE, VERTICAL_PORT_GAB};

pub fn show(
            ui: &mut egui::Ui, 
            node_position: &egui::Pos2,
            port_key: &NodeGraphKey,
            port: &Port,
            port_index: usize,
            port_has_connection: &bool,
            graph_viewport_title: &String, 
            graph_viewport_action: &mut VecDeque<GraphViewportAction>,
            cached_node_sizes: &HashMap<NodeGraphKey, egui::Vec2>,
            cached_port_positions: &mut HashMap<NodeGraphKey, egui::Pos2>,
            vertical_offset_before_showing_ports: f32,
            developer_mode: &bool,
)
{
    let node_size = cached_node_sizes.get(&port.node_key).unwrap(); // This is safe to due, due to the cached size always being filled out first in node_widget_body::show

    let horizontal_offset = match port.direction
    {
        PortDirection::Input => 0.0,
        PortDirection::Output => node_size.x,
    };
    
    let port_position  = *node_position + egui::Vec2 { x: horizontal_offset, y: vertical_offset_before_showing_ports + VERTICAL_PORT_GAB / 2.0 + port_index as f32 * VERTICAL_PORT_GAB};
    let port_rect = egui::Rect::from_center_size(port_position, PORT_SIZE);

    let output_port_response = ui.interact(port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_port_" + port_key.to_string().as_str()), egui::Sense::click());
    if output_port_response.clicked()
    {
        graph_viewport_action.push_back( GraphViewportAction::ClickedPort { port_key: *port_key });
    }

    let port_color = port.color();

    ui.painter().circle(
        port_position,
        25.0,
        port_color,
        egui::Stroke::NONE,
    );

    if *developer_mode
    {
        ui.painter().text(
            port_position,
            egui::Align2::CENTER_CENTER,
            port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }

    cached_port_positions.insert(*port_key, port_position);
}

