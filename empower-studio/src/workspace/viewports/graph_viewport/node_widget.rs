// use empower_node_graph::node::NodeState;
// use empower_node_graph::EmpowerKey;

use core::panic;

use empower_engine::NodeGraphKey;
// use crate::graph_editor::display_port::DisplayPortValueRepresentation;
use crate::graph_editor::{display_port::display_port_value::DisplayPortValue, GraphEditor};
use super::GraphViewport;

mod node_widget_body;
// mod node_widget_state;
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
    ClickedInputPort(i32), // @TODO, change this to empowerkeys
    ClickedOutputPort(i32),
    ChangedInputPortDisplayValue(i32, DisplayPortValue),
    // ChangedState(NodeState),
    // ClickedQuickMenuButton(egui::Pos2),
    InsideSelectionRect,
}

pub fn show(
                ui: &mut egui::Ui, 
                graph_editor: &GraphEditor, 
                graph_viewport: &GraphViewport, 
                node_key: &NodeGraphKey,
            ) -> Option<NodeWidgetResponse>
{
    let mut node_widget_response = None;

    let display_node = graph_editor.display_nodes.get(&node_key).unwrap();

    let debug_mode = false;

    let graph_viewport_title = &graph_viewport.title;

    // @TODO, consider changing this to return a node reponse instead of taking it as input?
    node_widget_body::show_node_body(ui, display_node, &graph_editor.selected_nodes, graph_viewport_title, node_key, &mut node_widget_response, &debug_mode, &graph_viewport.node_selection_rect, &graph_editor.debug_info);

    // @TODO, consider improving this interface
    let node = match graph_editor.node_graph.get_node(node_key)
    {
        Some( node ) => node,
        None => panic!("Tried to get key {} in node graph during show node widget, but key was not present", node_key),
    };

    // @TODO, add state to show node body?
    // node_widget_state::show_node_widget_state(ui, empower_node, display_node, graph_viewport_title, &mut node_widget_response);

    let input_port_keys = &node.input_port_keys;
    for input_port_key in input_port_keys
    {
        let input_port = match graph_editor.node_graph.get_input_port(input_port_key)
        {
            Some(port) => port,
            None => panic!("Tried to get port {} in node graph during show node widget, but key was not present", input_port_key),
        };

        let display_input_port = match graph_editor.display_input_ports.get(input_port_key)
        {
            Some(display_port) => display_port,
            None => panic!("Tried to get port {} in graph editor during show node widget, but key was not present", input_port_key),
        };

        let port_has_connection = graph_editor.node_graph.input_port_has_connection(input_port_key);

        node_widget_input_ports::show_input_port(ui, display_node, display_input_port, input_port, graph_viewport_title, port_has_connection, node_key, input_port_key, &mut node_widget_response, &debug_mode);
    }

    let output_port_keys = &node.output_port_keys;
    for output_port_key in output_port_keys
    {
        let output_port = match graph_editor.node_graph.get_output_port(output_port_key) 
        {
            Some(port) => port,
            None => panic!("Tried to get port {} in node graph during show node widget, but key was not present", output_port_key),
        };
        
        let display_output_port = match graph_editor.display_output_ports.get(&output_port_key)
        {
            Some(display_port) => display_port,
            None => panic!("Tried to get port {} in graph editor during show node widget, but key was not present", output_port_key),
        };
        
        node_widget_output_ports::show_output_port(ui, display_node, display_output_port, output_port, graph_viewport_title, node_key, output_port_key, &mut node_widget_response, &debug_mode);
    }

    node_widget_response
}

