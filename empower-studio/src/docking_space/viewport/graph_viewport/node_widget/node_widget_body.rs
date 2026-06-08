use std::{cmp, collections::{HashMap, VecDeque}};

use empower_engine::node_graph::{Node, NodeGraphKey};

use crate::docking_space::viewport::graph_viewport::{GraphViewportAction, area_select::AreaSelect, node_widget::VERTICAL_PORT_GAB};

const NODE_BODY_COLOR: egui::Color32 = egui::Color32::from_rgb(63, 63, 63);
const TITLE_TEXT_HORIZONTAL_OFFSET_PUFFER: f32 = 75.0;

const NODE_BOTTOM_RECT_HEIGHT: f32 = 20.0;

const NODE_BODY_TITLE_AREA_OVERLAP: f32 = 12.0; // Due to the drawing of the body in 3 steps, a bit of overlap is done to smoothen
const NODE_BODY_BUTTON_AREA_OVERLAP: f32 = 12.0;

pub fn show(
            ui: &mut egui::Ui, 
            node_key: &NodeGraphKey,
            node: &mut Node,
            graph_viewport_title: &String, 
            // node_area_select: &mut Option<NodeAreaSelect>,
            graph_viewport_actions: &mut VecDeque<GraphViewportAction>,
            area_select: &mut Option<AreaSelect>,
            cached_node_sizes: &mut HashMap<NodeGraphKey, egui::Vec2>,
        ) -> f32
{
    let node_size = cached_node_sizes.get(node_key).copied().unwrap_or_default();
    let node_rect = egui::Rect::from_min_size(node.position, node_size);

    if area_select.is_some()
    {
        area_select.as_mut().unwrap().check_if_node_is_inside_area_select_and_add_if_it_is(&node_key, &node_rect);
    }

    let title_text_font_size = 40.0;

    let node_title = node.kind.name().to_string();

    let text_size = ui.painter()
                        .layout_no_wrap(
                            node_title.to_string(),
                            egui::FontId::proportional(title_text_font_size),
                            egui::Color32::YELLOW,
                        )
                        .size();
    
        
    let title_box_rect = egui::Rect::from_min_size(
        node_rect.min,
        egui::Vec2 {
            x: text_size.x + TITLE_TEXT_HORIZONTAL_OFFSET_PUFFER * 2.0,
            y: text_size.y * 2.0,
        },
    );
    
    let node_title_pos = egui::Pos2 {
        x: title_box_rect.center().x,
        y: title_box_rect.min.y
            + (title_box_rect.size().y - NODE_BODY_TITLE_AREA_OVERLAP) / 2.0,
    };
    
    let mut title_rect_color = egui::Color32::from_rgb(50, 50, 50);

    let node_title_reponse = ui.interact(
        title_box_rect,
        egui::Id::new(graph_viewport_title.to_owned() + "_node_body_" + node_key.to_string().as_str()),
        egui::Sense::click_and_drag(),
    );

    if node_title_reponse.hovered() // Important that this is done before clicked
    {
        title_rect_color = egui::Color32::from_rgb(40, 40, 40);
    }

    if node_title_reponse.clicked()
    {
        graph_viewport_actions.push_back( GraphViewportAction::ClickedNodeTitle { node_key: *node_key });
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

    let node_rect_round_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.max.y - NODE_BOTTOM_RECT_HEIGHT,
            // y: node_rect.max.y,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: NODE_BOTTOM_RECT_HEIGHT,
        },
    );

    ui.painter().rect(
        node_rect_round_bottom,
        6.0,
        NODE_BODY_COLOR,
        egui::Stroke::NONE,
        egui::StrokeKind::Inside,
    );
    
    let node_rect_without_title_and_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.min.y + title_box_rect.size().y - NODE_BODY_TITLE_AREA_OVERLAP,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: node_rect.size().y
                                    - title_box_rect.size().y + NODE_BODY_TITLE_AREA_OVERLAP
                                    - node_rect_round_bottom.size().y + NODE_BODY_BUTTON_AREA_OVERLAP,
        },
    );

    ui.painter().rect(
        node_rect_without_title_and_bottom,
        0.0,
        NODE_BODY_COLOR,
        egui::Stroke::NONE,
        egui::StrokeKind::Inside,
    );

    let max_number_of_ports = cmp::max(node.input_port_keys.len(), node.output_port_keys.len());
    let vertical_size = title_box_rect.size().y - NODE_BODY_TITLE_AREA_OVERLAP + max_number_of_ports as f32 * VERTICAL_PORT_GAB;

    cached_node_sizes.insert(*node_key, egui::Vec2 { x: title_box_rect.size().x, y: vertical_size });

    // node_rect_without_title_and_bottom.min.y
    title_box_rect.size().y - NODE_BODY_TITLE_AREA_OVERLAP
}
