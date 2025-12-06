use better_empower_engine::NodeGraphKey;
use better_empower_engine::node_graph::node::Node;
use crate::graph_editor::DisplayNode; 

use super::NodeAreaSelect;

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
                        node_area_select: &Option<NodeAreaSelect>,
                    )
{
    // Determine the nodes widgets size

    let node_position = display_node.position;
    let node_size = display_node.display_kind.node_size();

    // let port_gap = 20.0; // @TODO, move this to elsewhere
    // let port_height = 50.0; // @TODO and this

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

    // let text_width_buffer = 100.0; // Adding a bit of extra gap to the text
    // let node_height = text_size.y + port_gap + node.input_port_keys.len() as f32 * (port_gap + port_height) + port_gap;
    // let node_width = text_size.x + text_width_buffer;

    let node_rect= egui::Rect::from_min_size(
        node_position,
        node_size,
        // egui::Vec2 { x: node_width, y: node_height},
    );

    // Determine and show node highlight outline

    let mut node_is_inside_selection_rect = false;
    if node_area_select.is_some()
    {
        let node_area_select = node_area_select.as_ref().unwrap();
        node_is_inside_selection_rect = node_area_select.rect.contains_rect(node_rect);
        
        if node_is_inside_selection_rect
        {
            *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::InsideSelectionRect });
        }
    }

    let node_is_selected = selected_nodes.iter().any(| selected_node_key | *selected_node_key == *node_key ); 

    let node_body_outline_margin = egui::Vec2 { x: 10.0, y: 10.0 };
    let node_outline_rect = node_rect.expand2(node_body_outline_margin);

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

    // Show title box and detect if user clicks it

    let title_box_rect = egui::Rect::from_min_size(
        node_rect.min,
        egui::Vec2 {
            x: node_rect.size().x,
            y: text_size.y * 2.0,
        },
    );

    let node_body_title_area_overlap = 12.0; // Due to the drawing of the body in 3 steps, a bit of overlap is done to smoothen

    let node_title_pos= egui::Pos2 {
        x: title_box_rect.center().x,
        y: title_box_rect.min.y
            + (title_box_rect.size().y - node_body_title_area_overlap) / 2.0,
    };

    let mut title_rect_color= egui::Color32::from_rgb(50, 50, 50);

    let node_title_reponse = ui.interact(
        title_box_rect,
        egui::Id::new(graph_viewport_title.to_owned() + "_node_body_" + node_key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Sense::click_and_drag(),
    );
  
    if node_title_reponse.hovered() // Important that this is done before clicked
    {
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }

    if node_title_reponse.clicked() || node_title_reponse.secondary_clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedTitle });
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
        egui::Align2::CENTER_CENTER,
        node_title,
        egui::FontId::proportional(title_text_font_size),
        egui::Color32::WHITE,
    );

    // Show node bottom
    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
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

    ui.painter().rect(
        node_rect_round_bottom,
        6.0,
        node_body_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

    let node_body_bottom_area_overlap = 20.0;

    // Show node body
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

    ui.painter().rect(
        node_rect_without_title_and_bottom,
        0.0,
        node_body_color,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );
 
    let mut kind_copy = node.kind.clone();

    let a = kind_copy.state(ui);

    // Show node body
    // ui.painter().rect(
    //     node_rect,
    //     0.0,
    //     egui::Color32::YELLOW,
    //     egui::Stroke::NONE,
    //         egui::StrokeKind::Inside,
    // );


    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
 
}
