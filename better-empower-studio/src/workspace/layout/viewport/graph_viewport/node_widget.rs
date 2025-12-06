use better_empower_engine::NodeGraphKey;

use crate::graph_editor::GraphEditor;

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
    // ChangedInputPortDisplayValue(i32, DisplayPortValue),
    // ChangedState(NodeKind, DisplayState),
    InsideSelectionRect,
}

pub fn show(
                ui: &mut egui::Ui, 
                graph_editor: &GraphEditor, 
                node_key: &NodeGraphKey,
                graph_viewport_title: &'static str,
            ) -> Option<NodeWidgetResponse>
{
    let mut node_widget_response = None;

    // Show node body
    let display_node = graph_editor.display_nodes.get(&node_key).unwrap();
    let node = graph_editor.node_graph.get_node(node_key).unwrap();

    let debug_mode = false; // @TODO, this is temporarily hardcoded for testing purposes, set this up proper

    let selected_nodes = Vec::new();
    let node_selection_rect = None;
    node_widget_body::show_node_body(ui, node, display_node, &selected_nodes, graph_viewport_title, node_key, &mut node_widget_response, &debug_mode, &node_selection_rect);

    // Show node input ports
    for input_port_key in &node.input_port_keys
    {
        let input_port = graph_editor.node_graph.get_input_port(input_port_key).unwrap();
        let display_input_port = graph_editor.display_input_ports.get(input_port_key).unwrap();

        // @TODO, these inputs can be simplified now
        node_widget_input_ports::show_input_port(ui, display_node, display_input_port, input_port, &String::from(graph_viewport_title), false, node_key, input_port_key, &mut node_widget_response, &debug_mode);
    }

    // Show node output ports
    for output_port_key in &node.output_port_keys
    {
        let output_port = graph_editor.node_graph.get_output_port(output_port_key).unwrap();
        let display_output_port = graph_editor.display_output_ports.get(output_port_key).unwrap();
        node_widget_output_ports::show_output_port(ui, display_node, display_output_port, output_port, &String::from(graph_viewport_title), node_key, output_port_key, &mut node_widget_response, &debug_mode);
    }

    node_widget_response
}
