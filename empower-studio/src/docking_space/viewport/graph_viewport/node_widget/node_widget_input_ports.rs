use std::collections::{HashMap, VecDeque};

use empower_engine::node_graph::{NodeGraphKey, Port};

use super::GraphViewportAction;

use super::{PORT_SIZE, VERTICAL_PORT_GAB};

pub fn show(
            ui: &mut egui::Ui, 
            node_position: &egui::Pos2,
            input_port_key: &NodeGraphKey,
            input_port: &Port,
            port_index: usize,
            port_has_connection: &bool,
            graph_viewport_title: &String, 
            graph_viewport_action: &mut VecDeque<GraphViewportAction>,
            cached_port_positions: &mut HashMap<NodeGraphKey, egui::Pos2>,
            vertical_offset_before_showing_ports: f32,
            developer_mode: &bool,
)
{
    let input_port_position = *node_position + egui::Vec2 { x: 0.0, y: vertical_offset_before_showing_ports + VERTICAL_PORT_GAB / 2.0 + VERTICAL_PORT_GAB * port_index as f32 };
    let input_port_rect = egui::Rect::from_center_size(input_port_position, PORT_SIZE);

    let input_port_response = ui.interact(input_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_input_port_" + input_port_key.to_string().as_str()), egui::Sense::click());
    if input_port_response.clicked()
    {
        graph_viewport_action.push_back( GraphViewportAction::ClickedPort { port_key: *input_port_key });
    }

    let port_color = egui::Color32::YELLOW;

    ui.painter().circle(
        input_port_position,
        25.0,
        port_color,
        egui::Stroke::NONE,
    );

    if *developer_mode
    {
        ui.painter().text(
            input_port_position,
            egui::Align2::CENTER_CENTER,
            input_port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }
    
    cached_port_positions.insert(*input_port_key, input_port_position);
}
