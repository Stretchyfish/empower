use better_empower_engine::NodeGraphKey;

use crate::graph_editor::GraphEditor;

mod node_widget_body;

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

    let display_node = graph_editor.display_nodes.get(&node_key).unwrap();
    let node = graph_editor.node_graph.get_node(node_key).unwrap();

    let debug_mode = false; // @TODO, this is temporarily hardcoded for testing purposes, set this up proper

    let selected_nodes = Vec::new();
    let node_selection_rect = None;
    node_widget_body::show_node_body(ui, node, display_node, &selected_nodes, graph_viewport_title, node_key, &mut node_widget_response, &debug_mode, &node_selection_rect);

    node_widget_response
}
