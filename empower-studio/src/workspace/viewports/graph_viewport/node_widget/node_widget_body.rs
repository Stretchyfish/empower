use crate::graph_editor::display_node::display_node_kind;
use crate::graph_editor::display_node::DisplayNode; 
use empower_engine::NodeGraphKey;
use empower_engine::node_graph::node::Node;

use super::NodeWidgetResponse;
use super::NodeWidgetResponseType;

pub fn show_node_body(
                        ui: &mut egui::Ui, 
                        node: &Node,
                        display_node: &DisplayNode, 
                        selected_nodes: &Vec<NodeGraphKey>, 
                        graph_viewport_title: &String, 
                        node_key: &NodeGraphKey, 
                        node_widget_response: &mut Option<NodeWidgetResponse>, 
                        debug_mode: &bool, 
                        node_selection_rect: &Option<egui::Rect>,
                    )
{
    let node_position = display_node.position;
    let node_screen_size= display_node.size;

    let node_rect= egui::Rect::from_min_size(
        node_position,
        node_screen_size
    );

    let mut node_is_inside_selection_rect = false;
    if node_selection_rect.is_some()
    {
        node_is_inside_selection_rect = node_selection_rect.unwrap().contains_rect(node_rect);
        
        if node_is_inside_selection_rect
        {
            *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::InsideSelectionRect });
        }
    }

    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
    let rect_margin = egui::Vec2 { x: 10.0, y: 10.0 };

    let node_outline_rect = node_rect.expand2(rect_margin);

    // @TODO, consider making this a bool that is taken as input to the function instead
    let node_is_selected = selected_nodes.iter().any(| selected_node_key | *selected_node_key == *node_key ); // @TODO, find a reduce the computation of this check
  
    let title_text_font_size = 40.0;
    let mut node_title = display_node.title.to_string();

    if *debug_mode
    {
       node_title = format!("{} [{}]", node_title, node_key.to_string());
    }

    let text_size = ui
        .painter()
        .layout_no_wrap(
            node_title.to_string(),
            egui::FontId::proportional(title_text_font_size),
            egui::Color32::YELLOW,
        )
        .size();

    let title_box_rect = egui::Rect::from_min_size(
        node_rect.min,
        egui::Vec2 {
            x: node_rect.size().x,
            y: text_size.y * 2.0,
        },
    );

    let node_body_title_area_overlap = 12.0;
    let node_body_bottom_area_overlap = 20.0;

    let node_title_pos= egui::Pos2 {
        x: title_box_rect.center().x,
        y: title_box_rect.min.y
            + (title_box_rect.size().y - node_body_title_area_overlap) / 2.0,
    };

    let node_rect_round_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.max.y - 20.0,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: 20.0,
        },
    );

    let node_rect_without_title_and_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.min.y + title_box_rect.size().y - node_body_title_area_overlap,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: node_rect.size().y - title_box_rect.size().y - node_rect_round_bottom.size().y
                + node_body_bottom_area_overlap,
        },
    );

    let mut title_rect_color= egui::Color32::from_rgb(50, 50, 50);

    // @TODO, change the name
    let node_reponse = ui.interact(
        title_box_rect,
        egui::Id::new(graph_viewport_title.to_owned() + "_node_body_" + node_key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Sense::click_and_drag(),
    );
  
    if node_reponse.hovered() // Important that this is done before clicked
    {
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }

    if node_reponse.clicked() || node_reponse.secondary_clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedTitle });
    }

    if node_is_selected || node_is_inside_selection_rect
    {
        ui.painter().rect(
            node_outline_rect,
            6.0,
            egui::Color32::ORANGE,
            egui::Stroke::NONE,
            egui::StrokeKind::Inside,
        );
    }

    ui.painter().rect(
        title_box_rect,
        6.0,
        title_rect_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

    ui.painter().text(
        node_title_pos,
        // egui::Align2::LEFT_TOP,
        egui::Align2::CENTER_CENTER,
        node_title,
        // egui::FontId::monospace(40.0),
        egui::FontId::proportional(title_text_font_size),
        egui::Color32::WHITE,
    );

    // Show node body
    ui.painter().rect(
        node_rect_without_title_and_bottom,
        0.0,
        node_body_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

    // Show node bottom
    ui.painter().rect(
        node_rect_round_bottom,
        6.0,
        node_body_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );


    // @TODO, try and find a way to precompute the state and calculate this distance first, then show it later

    let node_state_margin = 5.0;
    let state_size = display_node_kind::get_state_size(&node.kind);
    // let state_top_left_corner = egui::Pos2 { x: display_node.position.x + display_node.size.x / 2.0 - state_size.x / 2.0, y: title_box_rect.max.y + node_state_margin };
    let state_top_left_corner = egui::Pos2 { x: display_node.position.x + node_state_margin, y: title_box_rect.max.y + node_state_margin };
    let state_max_rect = egui::Rect::from_min_size(state_top_left_corner, state_size); 

    let state_ui_builder = egui::UiBuilder::new()
    .max_rect(state_max_rect);

    let mut modified_state = None;
    ui.scope_builder(state_ui_builder, |ui|
    {
        let style = ui.style_mut();
        style.override_font_id = Some ( egui::FontId::proportional(35.0));

        modified_state = display_node_kind::show(ui, &node.kind, &display_node.display_state);
    });

    if modified_state.is_some()
    {
        let modifications = modified_state.unwrap();
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ChangedState(modifications.0, modifications.1)});
    }
}
