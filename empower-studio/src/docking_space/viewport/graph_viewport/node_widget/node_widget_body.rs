use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;

use empower_engine::node_graph::Variable;
use empower_engine::node_graph::Variables;
use empower_engine::node_graph::node::Node;
use crate::studio_context::project::graph_editor::display_node::DisplayNodeStateResponse;
use crate::studio_context::project::graph_editor::DisplayNode;

use super::NodeAreaSelect;
use super::GraphViewportAction;

const NODE_BODY_COLOR: egui::Color32 = egui::Color32::from_rgb(63, 63, 63);

pub fn show_node_body(
                        ui: &mut egui::Ui, 
                        node: &mut Node,
                        display_node: &mut DisplayNode,
                        variables: &Variables,
                        graph_viewport_title: &String, 
                        debug_mode: &bool, 
                        node_area_select: &mut Option<NodeAreaSelect>,
                        graph_viewport_actions: &mut VecDeque<GraphViewportAction>
                    )
{
    let node_position = display_node.position;
    let node_size = display_node.display_kind.node_size(&node.kind);
    let node_rect = egui::Rect::from_min_size(
        node_position,
        node_size,
    );

    if node_area_select.is_some()
    {
        node_area_select.as_mut().unwrap().check_if_node_is_inside_area_select_and_add_if_it_is(&node.key, &node_rect);
    }

    let title_text_font_size = 40.0;

    let mut node_title = display_node.title.to_string();
    if *debug_mode
    {
       node_title = format!("{} [{}]", node_title, node.key.to_string());
    }
    
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
        egui::Id::new(graph_viewport_title.to_owned() + "_node_body_" + node.key.to_string().as_str()),
        egui::Sense::click_and_drag(),
    );

    if node_title_reponse.hovered() // Important that this is done before clicked
    {
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }
    
    if node_title_reponse.clicked()
    {
        graph_viewport_actions.push_back( GraphViewportAction::ClickedNodeTitle { node_key: node.key });
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
        NODE_BODY_COLOR,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

    let node_body_bottom_area_overlap = 20.0;
    
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
        NODE_BODY_COLOR,
        egui::Stroke::NONE,
            egui::StrokeKind::Inside,
    );

    let state_size = display_node.display_kind.state_size();
    
    // @TODO, move this to const?
    let node_state_margin = 5.0;
    let state_top_left_corner = egui::Pos2 { x: display_node.position.x + node_state_margin, y: title_box_rect.max.y + node_state_margin };
    let state_max_rect = egui::Rect::from_min_size(state_top_left_corner, state_size); 

    let state_ui_builder = egui::UiBuilder::new()
    .max_rect(state_max_rect);

    // let mut node_kind_copy = node.kind.clone();
    // let mut display_node_kind_copy = display_node.display_kind.clone();

    let mut display_node_state_response = DisplayNodeStateResponse::NoChange;
    ui.scope_builder(state_ui_builder, |ui|
    {
        // This is to ensure the styles and sizes match the rest of the UI
        let style = ui.style_mut();
        style.override_font_id = Some ( egui::FontId::proportional(35.0));

        display_node_state_response = display_node.display_kind.state_show(ui, &mut node.kind, variables);
    });

    match display_node_state_response
    {
        DisplayNodeStateResponse::NoChange => (),
        DisplayNodeStateResponse::RefreshNodeStructure =>
        {
            graph_viewport_actions.push_back( GraphViewportAction::RefreshedNodeStructure { node_key:  node.key });
        }
    }
}
