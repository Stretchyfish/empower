use better_empower_engine::NodeGraphKey;
use better_empower_engine::node_graph::node::port::PortKind;

use crate::graph_editor::GraphEditor;

use super::PortSearcher;

pub fn show(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, connection: (NodeGraphKey, NodeGraphKey))
{
    let display_output_port = graph_editor.display_output_ports.get(&connection.0).expect("Tried to show connection but output port was not available");
    let display_input_port = graph_editor.display_input_ports.get(&connection.1).expect("Tried to show connection but output port was not available");

    let connection_color = display_output_port.color;

    ui.painter().line_segment([display_output_port.position, display_input_port.position], egui::Stroke::new(10.0, connection_color));
}

pub fn show_connection_search(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, port_searcher: &PortSearcher, mouse_scene_position: &egui::Pos2)
{
    let display_port = match port_searcher.port_kind // @TODO, make this more safe
    {
        PortKind::Input => graph_editor.display_input_ports.get(&port_searcher.port_key).unwrap(),
        PortKind::Output => graph_editor.display_output_ports.get(&port_searcher.port_key).unwrap(),
    };
 
    ui.painter().line_segment([ display_port.position, *mouse_scene_position], egui::Stroke::new(10.0, display_port.color));
}