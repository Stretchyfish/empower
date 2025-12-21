use empower_engine::NodeGraphKey;
use empower_engine::node_graph::node::NodeKind;

use crate::actions::Action;
use crate::graph_editor::display_node::DisplayNodeKind;

use crate::graph_editor::GraphEditor;
use crate::graph_editor::display_node::DisplayValue;

use super::NodeAreaSelect;

mod node_widget_body;
mod node_widget_input_ports;
mod node_widget_output_ports;

pub struct NodeWidgetResponse
{
    pub key: NodeGraphKey,
    pub kind: NodeWidgetResponseType
}

pub enum NodeWidgetResponseType
{
    ClickedTitle,
    ClickedInputPort(NodeGraphKey), 
    ClickedOutputPort(NodeGraphKey),
    ChangedInputPortDisplayValue(NodeGraphKey, DisplayValue),
    ChangedState(Box<dyn NodeKind>, Box<dyn DisplayNodeKind>),
    InsideSelectionRect,
    ToggledQuickMenu, // @TODO, this is not a great approach to detect this, and should be handled differently!
}

pub fn show(
                ui: &mut egui::Ui, 
                graph_editor: &GraphEditor, 
                node_key: &NodeGraphKey,
                graph_viewport_title: &String, // @TODO, consider finding a way to combine these?
                node_area_select: &Option<NodeAreaSelect>,
            ) -> Option<NodeWidgetResponse>
{
    let mut node_widget_response = None;

    // Show node body
    let display_node = graph_editor.display_nodes.get(&node_key).unwrap();
    let node = graph_editor.node_graph.get_node(node_key).unwrap();

    let debug_mode = false; // @TODO, this is temporarily hardcoded for testing purposes, set this up proper

    node_widget_body::show_node_body(ui, node, display_node, &graph_editor.selected_nodes, graph_viewport_title, node_key, &mut node_widget_response, &debug_mode, &node_area_select);

    // Show node input ports
    for input_port_key in &node.input_port_keys
    {
        let input_port = graph_editor.node_graph.get_input_port(input_port_key).unwrap();
        let display_input_port = graph_editor.display_input_ports.get(input_port_key).unwrap();

        let port_has_connection = graph_editor.node_graph.input_port_has_connection(input_port_key);

        // @TODO, these inputs can be simplified now
        node_widget_input_ports::show_input_port(ui, display_input_port, input_port, &String::from(graph_viewport_title), port_has_connection, node_key, input_port_key, &mut node_widget_response, &debug_mode);
    }

    // Show node output ports
    for output_port_key in &node.output_port_keys
    {
        let output_port = graph_editor.node_graph.get_output_port(output_port_key).unwrap();
        let display_output_port = graph_editor.display_output_ports.get(output_port_key).unwrap();
        node_widget_output_ports::show_output_port(ui, display_output_port, output_port, &String::from(graph_viewport_title), node_key, output_port_key, &mut node_widget_response, &debug_mode);
    }

    node_widget_response
}

pub fn show_2(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    graph_editor: &mut GraphEditor, 
    graph_viewport_title: &String, // @TODO, consider finding a way to combine these?
    node_area_select: &mut Option<NodeAreaSelect>,
    action_queue: &mut Vec<Action>,
)
{
    let debug_mode = false; // @TODO, this is temporarily hardcoded for testing purposes, set this up proper

    node_widget_body::show_node_body2(ui, node_key, graph_editor, graph_viewport_title, &debug_mode, node_area_select, action_queue);
    
    let node_handle = graph_editor.node_graph.get_node_handle(node_key);

    for input_port_key in &node_handle.input_port_keys
    {
        node_widget_input_ports::show_input_port2(ui, input_port_key, graph_editor, &String::from(graph_viewport_title), &debug_mode);
    }

    for output_port_key in &node_handle.output_port_keys
    {
        node_widget_output_ports::show_output_port2(ui, output_port_key, graph_editor, &String::from(graph_viewport_title), &debug_mode);
    }
}

pub fn highlight(
    ui: &mut egui::Ui, 
    node_key: &NodeGraphKey,
    graph_editor: &mut GraphEditor, 
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
