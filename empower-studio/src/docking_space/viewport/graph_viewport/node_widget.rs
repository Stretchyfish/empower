use std::collections::VecDeque;

use empower_engine::NodeGraphKey;
use crate::studio_context::project::graph_editor::GraphEditor;

use super::NodeAreaSelect;
use super::GraphViewportAction;

mod node_widget_body;
mod node_widget_input_ports;
mod node_widget_output_ports;

pub fn show(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    graph_editor: &mut GraphEditor, 
    graph_viewport_title: &String, // @TODO, consider finding a way to combine these?
    node_area_select: &mut Option<NodeAreaSelect>,
    show_ids: bool,
    graph_viewport_actions: &mut VecDeque<GraphViewportAction>,
)
{
    // let debug_mode = graph_editor.debug_info.show_keys; 
    let debug_mode = show_ids; 

    let node = graph_editor.node_graph.get_node_mut(node_key).unwrap(); // THIS IS THE ONLY PLACE WITH MUTABLE ACCESS TO GRAPH EDITOR OUTSIDE OF STUDIO_CONTEXT!
    let display_node = graph_editor.display_nodes.get_mut(node_key).unwrap();

    node_widget_body::show_node_body(ui, node, display_node, graph_viewport_title, &debug_mode, node_area_select, graph_viewport_actions);

    let node_handle = graph_editor.node_graph.get_node_handle(node_key);

    for input_port_key in &node_handle.input_port_keys
    {
        let port_has_connection = graph_editor.node_graph.input_port_has_connection(input_port_key); // This needs to be placed here for the borrow checker 

        let input_port = graph_editor.node_graph.get_input_port(input_port_key).unwrap();
        let display_input_port = graph_editor.display_input_ports.get(input_port_key).unwrap();

        node_widget_input_ports::show_input_port(ui, &display_node.position, input_port, display_input_port, &port_has_connection, &String::from(graph_viewport_title), &debug_mode, graph_viewport_actions);
    }

    for output_port_key in &node_handle.output_port_keys
    {
        let output_port = graph_editor.node_graph.get_output_port(output_port_key).unwrap();
        let display_output_port = graph_editor.display_output_ports.get(output_port_key).unwrap();

        node_widget_output_ports::show_output_port(ui, &display_node.position, output_port, display_output_port, &String::from(graph_viewport_title), &debug_mode, graph_viewport_actions);
    }
}

pub fn highlight(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    graph_editor: &GraphEditor, 
)
{
    let node = graph_editor.node_graph.get_node(node_key).unwrap();
    let display_node = graph_editor.display_nodes.get(node_key).unwrap();
    
    let node_rect= egui::Rect::from_min_size(
        display_node.position,
        display_node.display_kind.node_size(&node.kind), // @TODO, find a way to fix this node size implementation
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
