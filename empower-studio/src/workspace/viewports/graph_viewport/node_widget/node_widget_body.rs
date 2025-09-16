use crate::graph_editor::display_node::DisplayNode; use egui::Vec2;
// @TODO, simplify this include
// use empower_node_graph::EmpowerKey;
use empower_engine::NodeGraphKey;

use super::NodeWidgetResponse;
use super::NodeWidgetResponseType;

use crate::graph_editor::DebugInfo;

pub fn show_node_body(
                        ui: &mut egui::Ui, 
                        display_node: &DisplayNode, 
                        selected_nodes: &Vec<NodeGraphKey>, 
                        graph_viewport_title: &String, 
                        node_key: &NodeGraphKey, 
                        node_widget_response: &mut Option<NodeWidgetResponse>, 
                        debug_mode: &bool, 
                        node_selection_rect: &Option<egui::Rect>,
                        debug_info: &DebugInfo,
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
    let mut node_title = display_node.title.clone();

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

    // @TODO, simplify this a bit by having the sizes saves seperate to avoid uneeded size call? 
    // let node_quick_menu_button_position = node_position + egui::Vec2 { x: title_box_rect.size().x - 30.0, y: (title_box_rect.size().y - node_body_title_area_overlap) / 2.0 } ; 
    // let node_quick_menu_button_size = 15.0;
    // let node_quick_menu_button_rect = egui::Rect::from_center_size(
    //     node_quick_menu_button_position, 
    //     Vec2::splat(node_quick_menu_button_size * 2.0));

    // let node_quick_menu_button_response = ui.interact(
    //     node_quick_menu_button_rect, 
    //     egui::Id::new(graph_viewport_title.to_owned() + "_node_quick_menu_button_" + node_key.to_string().as_str()), 
    //     egui::Sense::click()
    // );

    // let mut node_quick_menu_button_color = egui::Color32::DARK_GRAY;
    // if node_quick_menu_button_response.hovered()
    // {
    //     node_quick_menu_button_color = egui::Color32::BLACK;
    // }

    // if node_quick_menu_button_response.clicked()
    // {
    //     *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedQuickMenuButton( node_quick_menu_button_position ) });
    // }

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

    // ui.painter().circle(
    //     node_quick_menu_button_position, 
    //     node_quick_menu_button_size, 
    //     node_quick_menu_button_color, 
    //     egui::Stroke::NONE
    // );

    // Show node body
    ui.painter().rect(
        node_rect_without_title_and_bottom,
        0.0,
        node_body_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

    // let close_button_widget = egui::Button::new(egui::RichText::new("x").size(14.0));

    // let close_button_position = egui::Pos2 { x: title_box_rect.max.x - 30.0, y: title_box_rect.min.y };
    // let close_button_size = egui::Vec2 { x: 28.0, y: 28.0 };

    // if ui.put(egui::Rect::from_min_size(close_button_position, close_button_size), close_button_widget).clicked()
    // {

    // }

    // Show node bottom
    ui.painter().rect(
        node_rect_round_bottom,
        6.0,
        node_body_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

}
