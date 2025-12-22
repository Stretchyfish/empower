use empower_engine::NodeGraphKey;
use crate::actions::Action;
use crate::graph_editor::GraphEditor;

use super::NodeAreaSelect;

mod node_widget_body;
mod node_widget_input_ports;
mod node_widget_output_ports;

pub fn show(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    graph_editor: &GraphEditor, 
    graph_viewport_title: &String, // @TODO, consider finding a way to combine these?
    node_area_select: &mut Option<NodeAreaSelect>,
    action_queue: &mut Vec<Action>,
)
{
    let debug_mode = false; // @TODO, this is temporarily hardcoded for testing purposes, set this up proper

    node_widget_body::show_node_body(ui, node_key, graph_editor, graph_viewport_title, &debug_mode, node_area_select, action_queue);
    
    let node_handle = graph_editor.node_graph.get_node_handle(node_key);

    for input_port_key in &node_handle.input_port_keys
    {
        node_widget_input_ports::show_input_port(ui, input_port_key, graph_editor, &String::from(graph_viewport_title), &debug_mode, action_queue);
    }

    for output_port_key in &node_handle.output_port_keys
    {
        node_widget_output_ports::show_output_port(ui, output_port_key, graph_editor, &String::from(graph_viewport_title), &debug_mode, action_queue);
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
