use std::collections::HashMap;

use empower_engine::NodeGraphKey;

use crate::project::graph_editor::GraphEditor;

pub fn nodes_debug_info_show(ui: &mut egui::Ui, graph_editor: &GraphEditor)
{
    if !graph_editor.debug_info.show_node_execution_order
    {
        return;
    }

    let node_execution_order_keys = graph_editor.debug_info.node_execution_order.clone();

    // @TODO, find a better name
    // This is used to keep track of how many times the node appears in the execution order.
    let mut nodes_count_in_execution_tracker: HashMap<NodeGraphKey, i32> = HashMap::new();

    for (node_index, node_key) in node_execution_order_keys.iter().enumerate()
    {
        // Check that the nodes is still available, could have been removed
        if !graph_editor.display_nodes.contains_key(&node_key)
        {
            continue;
        }

        let node = graph_editor.node_graph.get_node(&node_key).unwrap();
        let display_node = graph_editor.display_nodes.get(&node_key).unwrap();

        let mut i = 0; // Bad naming
        // @TODO, consider a better way of writting this
        if nodes_count_in_execution_tracker.contains_key(node_key)
        {
            let number_of_times_node_is_in_execution = nodes_count_in_execution_tracker.get_mut(node_key).unwrap();
            i = *number_of_times_node_is_in_execution;
            *number_of_times_node_is_in_execution += 1;
        }
        else 
        {
            nodes_count_in_execution_tracker.insert(*node_key, 1);
        }

        let number_box_lengths = 30.0;

        let number_box_horizontal_offset = i as f32 * 30.0;

        let number_rect_top_left_corner = egui::Pos2
        {
            x: display_node.position.x + display_node.display_kind.node_size(&node.kind).x - number_box_lengths - number_box_horizontal_offset,
            y: display_node.position.y
        };
        let number_rect_bottom_right_corner = egui::Pos2
        {
            x: display_node.position.x + display_node.display_kind.node_size(&node.kind).x - number_box_horizontal_offset,
            y: display_node.position.y + number_box_lengths,
        };

        let number_rect = egui::Rect::from_min_max(number_rect_top_left_corner, number_rect_bottom_right_corner);

        ui.painter().rect(
                        number_rect, 
                        5.0, 
                        egui::Color32::LIGHT_GREEN, 
                        egui::Stroke::NONE, 
                        egui::StrokeKind::Inside
        );

        let number_rect_center = number_rect_top_left_corner + ( (number_rect_bottom_right_corner - number_rect_top_left_corner ) / 2.0 ); 

        ui.painter().text(
            number_rect_center,
            egui::Align2::CENTER_CENTER,
            node_index + 1,
            egui::FontId::proportional(20.0),
            egui::Color32::WHITE,
        );
    }
}
