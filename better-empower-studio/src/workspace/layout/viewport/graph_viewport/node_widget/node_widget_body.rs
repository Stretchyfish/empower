use better_empower_engine::NodeGraphKey;
use better_empower_engine::node_graph::node::Node;
use crate::graph_editor::DisplayNode; 

use super::NodeWidgetResponse;
use super::NodeWidgetResponseType;

pub fn show_node_body(
                        ui: &mut egui::Ui, 
                        node: &Node,
                        display_node: &DisplayNode, 
                        selected_nodes: &Vec<NodeGraphKey>, 
                        graph_viewport_title: &'static str, 
                        node_key: &NodeGraphKey, 
                        node_widget_response: &mut Option<NodeWidgetResponse>, 
                        debug_mode: &bool, 
                        node_selection_rect: &Option<egui::Rect>,
                    )
{
    
}
