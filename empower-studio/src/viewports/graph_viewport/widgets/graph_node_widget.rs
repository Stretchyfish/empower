use egui;
use crate::node_graph::DisplayNode;
use crate::viewports::graph_viewport::GraphViewportState;
use crate::interactions::user::UserInputs;

pub struct NodeViewReponse
{
    pub key: i32,
    pub kind: NodeViewResponseType,
}

pub enum NodeViewResponseType
{
    Clicked,
    Hover,
    InsideSelectionArea,
}

pub fn view_graph_node_widget(ui: &mut egui::Ui, display_node: &mut DisplayNode, graph_viewport_state: &GraphViewportState, user_input: &UserInputs) -> Option<NodeViewReponse>
{
    let mut view_graph_node_reponse = Option::None;
    
    let pan_offset = graph_viewport_state.pan_zoom.pan_offset;
    let zoom_scale = graph_viewport_state.pan_zoom.zoom_scale;

    let node_size = egui::Vec2 { x: 200.0, y: 200.0 } * zoom_scale;
    
    let node_is_selected = graph_viewport_state.selected_nodes.iter().any(| selected_node_key | *selected_node_key == display_node.key );
    
    if node_is_selected
    {
        display_node.position += user_input.mouse_position_delta * (1.0 / zoom_scale);
    }

    // let screen_center = ui.max_rect().center();

    // let node_position = (display_node.position + pan_offset + screen_center.to_vec2()) * zoom_scale; // This is working great!
    let node_position = (display_node.position + pan_offset) * zoom_scale + egui::Vec2 { x: -node_size.x / 2.0, y: 0.0 }; // This fixes the new node creation problem
    // let node_position = node_position_uncentered + egui::Vec2{ x: -node_size.x / 2.0 , y: 0.0 };

    // but adds an offset problem

    let rect_margin = egui::Vec2 { x: 4.0, y: 4.0 } * zoom_scale;
    let node_outline_rect = egui::Rect::from_min_size(
        node_position,
        node_size
    );

    
    let node_rect = node_outline_rect.shrink2(rect_margin);

    let node_top_left_corner_can_be_seen_in_viewport = ui.max_rect().contains( node_rect.min ); // consider changing it to be the highlight rect
    let node_top_right_corner_can_be_seen_in_viewport = ui.max_rect().contains( egui::Pos2 { x: node_rect.max.x, y: node_rect.min.y } );
    let node_buttom_left_corner_can_be_seen_in_viewport = ui.max_rect().contains( egui::Pos2 { x: node_rect.min.x, y: node_rect.max.y } );
    let node_buttom_right_corner_can_be_seen_in_viewport = ui.max_rect().contains( node_rect.max );

    if !node_top_left_corner_can_be_seen_in_viewport &&
       !node_top_right_corner_can_be_seen_in_viewport &&
       !node_buttom_left_corner_can_be_seen_in_viewport &&
       !node_buttom_right_corner_can_be_seen_in_viewport 
    {
        return view_graph_node_reponse;
    }

    let mut node_is_inside_node_selection_rect = false;
    if graph_viewport_state.node_select_rect.is_some() // @TODO, is it possible to not check this for every node?
    {
        node_is_inside_node_selection_rect = graph_viewport_state.node_select_rect.unwrap().contains_rect(node_rect);
    }

    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
    let mut title_rect_color= egui::Color32::from_rgb(50, 50, 50);

    let text = "Centered Text";
    let text_size = ui
        .painter()
        .layout_no_wrap(
            text.to_string(),
            egui::FontId::proportional(16.0),
            egui::Color32::YELLOW,
        )
        .size() * zoom_scale;

    let title_box_rect = egui::Rect::from_min_size(
        node_rect.min,
        egui::Vec2 {
            x: node_rect.size().x,
            y: text_size.y * 2.0,
        },
    );

    let node_body_title_area_overlap = 8.0 * zoom_scale ;
    let node_body_bottom_area_overlap = 12.0 * zoom_scale ;

    let text_pos = egui::Pos2 {
        x: title_box_rect.center().x,
        y: title_box_rect.min.y
            + (title_box_rect.size().y - node_body_title_area_overlap) / 2.0,
    };
    // let text_box_color= egui::Color32::from_rgb(33, 29, 28);

    let node_rect_round_bottom = egui::Rect::from_min_size(
        egui::Pos2 {
            x: node_rect.min.x,
            y: node_rect.max.y - 20.0 * zoom_scale ,
        },
        egui::Vec2 {
            x: node_rect.size().x,
            y: 20.0 * zoom_scale ,
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

    let node_reponse = ui.interact(
        title_box_rect,
        egui::Id::new( graph_viewport_state.title.clone() + "node_body_" + display_node.key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Sense::click_and_drag(),
    );

    if node_reponse.hovered() // Important that this is done before clicked
    {
        view_graph_node_reponse = Some( NodeViewReponse { key: display_node.key, kind: NodeViewResponseType::Hover } );
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }

    if node_reponse.clicked()
    {
        view_graph_node_reponse = Some( NodeViewReponse { key: display_node.key, kind: NodeViewResponseType::Clicked } );
    }

    if node_is_inside_node_selection_rect
    {
        view_graph_node_reponse = Some( NodeViewReponse { key: display_node.key, kind: NodeViewResponseType::InsideSelectionArea } );
    }


    // Show orange outline arund selected nodes
    if node_is_selected || node_is_inside_node_selection_rect
    {
        ui.painter().rect(
            node_outline_rect,
            egui::Rounding::same(6.0),
            egui::Color32::ORANGE,
            egui::Stroke::NONE,
        );
    }


    // Show title
    ui.painter().rect(
        title_box_rect,
        egui::Rounding::same(6.0),
        title_rect_color,
        egui::Stroke::NONE,
    );

    ui.painter().text(
        text_pos,
        // egui::Align2::LEFT_TOP,
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(16.0 * zoom_scale ),
        egui::Color32::WHITE,
    );

    // Show node body
    ui.painter().rect(
        node_rect_without_title_and_bottom,
        egui::Rounding::same(0.0),
        node_body_color,
        egui::Stroke::NONE,
    );
    // let text_box_color = egui::Color32::BLACK;

    // Show node bottom
    ui.painter().rect(
        node_rect_round_bottom,
        egui::Rounding::same(6.0),
        node_body_color,
        egui::Stroke::NONE,
    );

    ui.allocate_ui_at_rect(node_rect, |ui| {
        ui.vertical(|ui| {
            ui.add_space(title_box_rect.size().y);

           
            ui.horizontal(|ui| {
                let input_port_position = ui.min_rect().max;

                ui.painter().circle(
                    input_port_position,
                    5.0 * zoom_scale ,
                    egui::Color32::YELLOW,
                    egui::Stroke::NONE,
                );

                ui.label(egui::RichText::new("Test1").font(egui::FontId::proportional(8.0 * zoom_scale )));
                // ui.label("Test 1");
            });
        });
    });

    return view_graph_node_reponse;
}
