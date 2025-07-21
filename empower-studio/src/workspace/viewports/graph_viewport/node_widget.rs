use empower_node_graph::node::NodeState;
use empower_node_graph::EmpowerKey;

use crate::graph_editor::display_port::DisplayPortValueRepresentation;
use crate::graph_editor::GraphEditor;
use super::GraphViewport;

mod node_widget_body;
mod node_widget_state;
mod node_widget_input_ports;
mod node_widget_output_ports;

pub struct NodeWidgetResponse
{
    pub key: EmpowerKey,
    pub kind: NodeWidgetResponseType
}

pub enum NodeWidgetResponseType
{
    ClickedTitle,
    ClickedInputPort(i32), // @TODO, change this to empowerkeys
    ClickedOutputPort(i32),
    ChangedInputPortValueRepresentation(i32, DisplayPortValueRepresentation),
    ChangedState(NodeState),
}

pub fn show(ui: &mut egui::Ui, graph_editor: &GraphEditor, graph_viewport: &GraphViewport, node_key: &EmpowerKey) -> Option<NodeWidgetResponse>
{
    let mut node_widget_response = None;

    let display_node = graph_editor.display_nodes.get(&node_key).unwrap();

    let debug_mode = true;

    let graph_viewport_title = &graph_viewport.title;

    // @TODO, consider changing this to return a node reponse instead of taking it as input?
    node_widget_body::show_node_body(ui, display_node, &graph_editor.selected_nodes, graph_viewport_title, node_key, &mut node_widget_response, &debug_mode);

    let empower_node = graph_editor.empower_node_graph.nodes.get(&node_key).unwrap();

    // @TODO, add state to show node body?
    node_widget_state::show_node_widget_state(ui, empower_node, display_node, &mut node_widget_response);

    let input_port_keys = &empower_node.input_port_keys;
    for input_port_key in input_port_keys
    {
        let input_port = graph_editor.empower_node_graph.input_ports.get(input_port_key).unwrap(); 
        let display_input_port = graph_editor.display_input_ports.get(input_port_key).unwrap();

        let port_has_connection = graph_editor.empower_node_graph.connections_in.contains_key(input_port_key);

        node_widget_input_ports::show_input_port(ui, display_node, display_input_port, input_port, graph_viewport_title, port_has_connection, node_key, input_port_key, &mut node_widget_response, &debug_mode);
    }

    let output_port_keys = &empower_node.output_port_keys;
    for output_port_key in output_port_keys
    {
        let output_port = graph_editor.empower_node_graph.output_ports.get(&output_port_key).unwrap(); 
        let display_output_port = graph_editor.display_output_ports.get(&output_port_key).unwrap();
        node_widget_output_ports::show_output_port(ui, display_node, display_output_port, output_port, graph_viewport_title, node_key, output_port_key, &mut node_widget_response, &debug_mode);
    }

    node_widget_response
}

