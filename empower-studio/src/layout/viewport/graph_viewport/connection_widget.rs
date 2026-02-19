use empower_engine::NodeGraphKey;
use empower_engine::node_graph::node::port::PortKind;

use crate::project::graph_editor::GraphEditor;

pub fn show(ui: &mut egui::Ui, graph_editor: &GraphEditor, connection: (NodeGraphKey, NodeGraphKey))
{
    let display_output_port = graph_editor.display_output_ports.get(&connection.0).expect("Tried to show connection but output port was not available");
    let display_input_port = graph_editor.display_input_ports.get(&connection.1).expect("Tried to show connection but output port was not available");
    
    let output_port = graph_editor.node_graph.get_output_port(&connection.0).unwrap(); // @TODO, investigate a good way to remove this step
    let input_port = graph_editor.node_graph.get_input_port(&connection.1).unwrap();

    let display_output_port_node = graph_editor.display_nodes.get(&output_port.node_key).unwrap();
    let display_input_port_node = graph_editor.display_nodes.get(&input_port.node_key).unwrap();

    let output_port_position = display_output_port_node.position + display_output_port.relative_position;
    let input_port_position = display_input_port_node.position + display_input_port.relative_position;

    let connection_color = display_output_port.color;

    ui.painter().line_segment([output_port_position, input_port_position], egui::Stroke::new(10.0, connection_color));
}

pub fn show_connection_search(ui: &mut egui::Ui, graph_editor: &GraphEditor, mouse_scene_position: &egui::Pos2)
{
    let port_searcher = graph_editor.port_searcher.as_ref().unwrap();

    let display_port_position;
    let display_port_color;
    
    match port_searcher.port_kind
    {
        PortKind::Input =>
        {
            let display_input_port = graph_editor.display_input_ports.get(&port_searcher.port_key).unwrap();
            let input_port = graph_editor.node_graph.get_input_port(&port_searcher.port_key).unwrap();
            let display_input_port_node = graph_editor.display_nodes.get(&input_port.node_key).unwrap();

            display_port_position = display_input_port_node.position + display_input_port.relative_position;
            display_port_color = display_input_port.color;
        },
        PortKind::Output =>
        {
            let display_output_port = graph_editor.display_output_ports.get(&port_searcher.port_key).unwrap();
            let output_port = graph_editor.node_graph.get_output_port(&port_searcher.port_key).unwrap();
            let display_output_port_node = graph_editor.display_nodes.get(&output_port.node_key).unwrap();

            display_port_position = display_output_port_node.position + display_output_port.relative_position;
            display_port_color = display_output_port.color;
        },
    };
 
    ui.painter().line_segment([ display_port_position, *mouse_scene_position], egui::Stroke::new(10.0, display_port_color));
}
