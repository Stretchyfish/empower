use empower_engine::NodeGraphKey;

use crate::{graph_editor::GraphEditor, workspace::viewports::graph_viewport::{self, port_searcher, GraphViewport}};
use super::port_searcher::{PortSearcher, PortKind};

pub fn show(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, connection: (NodeGraphKey, NodeGraphKey))
{
    let display_output_port = graph_editor.display_output_ports.get(&connection.0).expect("Tried to show connection but output port was not available");
    let display_input_port = graph_editor.display_input_ports.get(&connection.1).expect("Tried to show connection but output port was not available");

    let display_output_port_node = graph_editor.display_nodes.get(&display_output_port.node_key).expect("Tried to show connection but output port node was not available");
    let display_input_port_node = graph_editor.display_nodes.get(&display_input_port.node_key).expect("Tried to show connection but inut port node was not available");

    let output_port_position = display_output_port_node.position + display_output_port.relative_position;
    let input_port_position = display_input_port_node.position + display_input_port.relative_position;
    let connection_color = display_output_port.display_value.color;

    ui.painter().line_segment([output_port_position, input_port_position], egui::Stroke::new(10.0, connection_color));

    // for input_port_key in connection.iter()
    // {
    //     let display_input_port = graph_editor.display_input_ports.get(input_port_key).unwrap();
    //     let display_node_to = graph_editor.display_nodes.get(&display_input_port.node_key).unwrap(); 

    //     let input_port_position = display_node_to.position + display_input_port.relative_position;

        // @TODO, find a better way to do this
        // let input_port_data = graph_editor.empower_node_graph.input_ports.get(&input_port_key).unwrap().value.clone();

        // match input_port_data
        // {
        //     EmpowerData::Trigger =>
        //     {
        //         connection_color = egui::Color32::WHITE;
        //     },
        //     _ =>
        //     {
        //         connection_color = egui::Color32::YELLOW;
        //     }
        // }

    // }
}

pub fn show_connection_search(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, port_searcher: Option<PortSearcher>, mouse_scene_position: &egui::Pos2)
{
    if port_searcher.is_none()
    {
        return;
    }

    let port_searcher = port_searcher.unwrap();

    let display_port = match port_searcher.port_kind // @TODO, make this more safe
    {
       PortKind::InputPort => graph_editor.display_input_ports.get(&port_searcher.port_key).expect("Tried to input port in port searcher connection, but input port was not present"),
       PortKind::OutputPort => graph_editor.display_output_ports.get(&port_searcher.port_key).unwrap(), // @TODO, replace with expect
    };

    // let port_searcher_data; // @TODO, consider placing this differently, so its not calculated every loop 
    // let display_port;
    // match port_searcher.port_kind
    // {
    //    PortKind::InputPort =>
    //    {
    //     display_port = graph_editor.display_input_ports.get(&port_searcher.port_key).unwrap(); // @Consider simplifying this call
    //     // port_searcher_data = graph_editor.node_graph.input_ports.get(&port_searcher.port_key).unwrap().value.clone();
    //     port_searcher_data = graph_editor.node_graph.input_ports.get(&port_searcher.port_key).unwrap().value.clone();
    //    },
    //    PortKind::OutputPort =>
    //    {
    //     port_searcher_data = graph_editor.empower_node_graph.output_ports.get(&port_searcher.port_key).unwrap().value.clone();
    //     display_port = graph_editor.display_output_ports.get(&port_searcher.port_key).unwrap(); // @Consider simplifying this call
    //     // @TODO, check that this unwrap is safe
    //    }, 
    // }

    // let search_color: egui::Color32;
    // match port_searcher_data 
    // {
    //     EmpowerData::Trigger =>
    //     {
    //         search_color = egui::Color32::WHITE;
    //     }
    //     _ =>
    //     {
    //         search_color = egui::Color32::YELLOW;

    //     }
    // }

    let display_node = graph_editor.display_nodes.get(&display_port.node_key).unwrap();
    let display_port_position = display_node.position + display_port.relative_position;

    ui.painter().line_segment([ display_port_position, *mouse_scene_position], egui::Stroke::new(10.0, display_port.display_value.color));
}
