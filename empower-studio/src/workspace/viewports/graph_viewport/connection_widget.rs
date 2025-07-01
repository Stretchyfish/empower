use egui::Options;
use empower_node_graph::{port, EmpowerKey};

use crate::{graph_editor::GraphEditor, workspace::viewports::graph_viewport::{self, port_searcher, GraphViewport}};
use super::port_searcher::{PortSearcher, PortKind};

pub fn show(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, connection_key: &EmpowerKey)
{
    let connection = graph_editor.empower_node_graph.connections.get(connection_key).unwrap();

    let display_output_port = graph_editor.display_output_ports.get(connection_key).unwrap();
    let display_node_from = graph_editor.display_nodes.get(&display_output_port.node_key).unwrap();

    let output_port_position = display_node_from.position + display_output_port.relative_position;

    for input_port_key in connection.iter()
    {
        let display_node_to = graph_editor.display_nodes.get(input_port_key).unwrap(); 
        let display_input_port = graph_editor.display_input_ports.get(input_port_key).unwrap();

        let input_port_position = display_node_to.position + display_input_port.relative_position;

        ui.painter().line_segment([output_port_position, input_port_position], egui::Stroke::new(10.0, egui::Color32::YELLOW));
    }
}

pub fn show_connection_search(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, port_searcher: Option<PortSearcher>, mouse_scene_position: &egui::Pos2)
{
    if port_searcher.is_none()
    {
        return;
    }

    let port_searcher = port_searcher.unwrap();

    let display_port;
    match port_searcher.port_kind
    {
       PortKind::InputPort =>
       {
        display_port = graph_editor.display_input_ports.get(&port_searcher.port_key).unwrap(); // @Consider simplifying this call
       },
       PortKind::OutputPort =>
       {
        display_port = graph_editor.display_output_ports.get(&port_searcher.port_key).unwrap(); // @Consider simplifying this call
        // @TODO, check that this unwrap is safe
       }, 
    }

    let display_node = graph_editor.display_nodes.get(&display_port.node_key).unwrap();
    let display_port_position = display_node.position + display_port.relative_position;

    ui.painter().line_segment([ display_port_position, *mouse_scene_position], egui::Stroke::new(10.0, egui::Color32::YELLOW));
}