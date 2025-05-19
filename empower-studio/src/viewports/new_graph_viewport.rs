use std::borrow::BorrowMut;

use egui;
use crate::{node_graph, NodeGraph};
use empower_engine::EmpowerKey;
use crate::interactions;

pub struct NewGraphViewport
{
    pub title: String,
    pub state: NewGraphViewportState,
    scene_rect: egui::Rect,
}

impl NewGraphViewport
{
    pub fn new(title: String) -> Self
    {
        Self
        {
            title,
            state: NewGraphViewportState::new(),
            scene_rect: egui::Rect { min: egui::Pos2 { x: -500.0, y: -500.0 }, max: egui::Pos2 { x: 500.0, y: 500.0 }},
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, node_graph: &mut NodeGraph)
    {
        let mut nodes_view_responses = Vec::new();         

        let scene_response= egui::Scene::new()
        .zoom_range(0.01..=2.0)
        .show(ui, &mut self.scene_rect, |scene_ui|
        {
            nodes_view_responses = show_nodes( scene_ui,  node_graph, &mut self.state, self.title.clone());
        });

        self.state.mouse_scene_position_second_last_frame = self.state.mouse_scene_position_last_frame;
        self.state.mouse_scene_position_last_frame = scene_response.response.hover_pos().unwrap_or( self.state.mouse_scene_position_second_last_frame );

        for node_reponse in nodes_view_responses
        {
            match node_reponse.kind
            {
                NodeViewResponseType::Clicked =>
                {
                    if let Some(selected_node_to_remove_index) = self.state.selected_nodes.iter().position(| selected_node_key | *selected_node_key == node_reponse.key )
                    {
                        self.state.selected_nodes.remove(selected_node_to_remove_index);
                        break;
                    }
                    // self.selected_nodes.retain(f);
                   self.state.selected_nodes.push(node_reponse.key); 
                    // self.state.selected_nodes = Vec::from([node_reponse.key]);
                }

                _ =>
                {
                    
                }
            }
        }
    }

}

struct NewGraphViewportState
{
    pub selected_nodes: Vec<EmpowerKey>,
    pub mouse_scene_position_last_frame: egui::Pos2,
    pub mouse_scene_position_second_last_frame: egui::Pos2,
}

impl NewGraphViewportState
{
    pub fn new() -> Self
    {
        Self
        {
            selected_nodes: Vec::new(),
            mouse_scene_position_last_frame: egui::Pos2 { x: 0.0, y: 0.0 },
            mouse_scene_position_second_last_frame: egui::Pos2 { x: 0.0, y: 0.0 },
        }
    }
}

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
    ClickedInputPort(EmpowerKey),
    ClickedOutputPort(EmpowerKey)
}


fn show_nodes(ui: &mut egui::Ui, node_graph: &mut NodeGraph, graph_viewport_state: &mut NewGraphViewportState, graph_title: String) -> Vec<NodeViewReponse>
{
    let mut user_input = interactions::user::inputs::detect_user_inputs(ui);
    // println!("A: {}, {}", user_input.mouse_position.x, user_input.mouse_position.y);

    let mut nodes_view_responses = Vec::new(); // Change to an optional?
    let display_node_keys: Vec<EmpowerKey> = node_graph.display_nodes.keys().cloned().collect(); 
    
    for display_node_key  in display_node_keys
    {
        let response= show_node(display_node_key, ui, &user_input, node_graph, graph_viewport_state, graph_title.clone());

        if let Some(view_response) = response
        {
            nodes_view_responses.push(view_response);
        }
    }

    nodes_view_responses
}

fn show_node(display_node_key: EmpowerKey, ui: &mut egui::Ui, user_input: &interactions::user::UserInputs, node_graph: &mut NodeGraph, graph_viewport_state: &mut NewGraphViewportState, graph_title: String) -> Option<NodeViewReponse>
{
    let mut view_graph_node_reponse = Option::None;

    let display_node = node_graph.display_nodes.get_mut(&display_node_key).unwrap();

    let node_position = display_node.position;
    let node_screen_size= display_node.size;

    let node_rect= egui::Rect::from_min_size(
        node_position,
        node_screen_size
    );
    
    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
    let rect_margin = egui::Vec2 { x: 10.0, y: 10.0 };

    let node_outline_rect = node_rect.expand2(rect_margin);

    let node_is_selected = graph_viewport_state.selected_nodes.iter().any(| selected_node_key | *selected_node_key == display_node.key );

    if node_is_selected
    {
        // display_node.position += user_input.mouse_position_delta;
        display_node.position += graph_viewport_state.mouse_scene_position_last_frame - graph_viewport_state.mouse_scene_position_second_last_frame;
    }

    let title_text_font_size = 40.0;
    let node_title = "Centered Text";
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

    // let node_rect_without_title_and_bottom = egui::Rect::from_min_size(
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

    let node_reponse = ui.interact(
        title_box_rect,
        egui::Id::new( String::from("missing_") + "_node_body_" + display_node.key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Sense::click_and_drag(),
    );

    if node_reponse.hovered() // Important that this is done before clicked
    {
        // view_graph_node_reponse = Some( NodeViewReponse { key: display_node.key, kind: NodeViewResponseType::Hover } );
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }

    if node_reponse.clicked()
    {
        view_graph_node_reponse = Some( NodeViewReponse { key: display_node.key, kind: NodeViewResponseType::Clicked } );
    }

    // Show orange outline arund selected nodes
    // if node_is_selected || node_is_inside_node_selection_rect
    if node_is_selected
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


    let engine_node = node_graph.engine.nodes.get_mut(&display_node.key).unwrap();

    for input_port_key in engine_node.input_port_keys.iter() 
    {
        let display_input_port = node_graph.display_input_ports.get_mut(input_port_key).unwrap();// @TODO, change this back to being borrowed once the value is read from engine instead of display port

        let input_port_position = node_position + display_input_port.relative_position;
        let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 

        let input_port_rect = egui::Rect::from_center_size(input_port_position, input_port_size);

        if ui.interact(input_port_rect, egui::Id::from(String::from("missing_") + "_input_port_" + input_port_key.to_string().as_str()), egui::Sense::click()).clicked()
        {
            // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::ClickedInputPort(input_port_key.clone()) } );
            println!("input port id: {}", input_port_key);
        }


        ui.painter().circle(
            input_port_position,
            25.0,
            egui::Color32::YELLOW,
            egui::Stroke::NONE,
        );

        let input_port_text_offset = egui::Vec2 { x: 40.0, y: 0.0};
        let input_port_text_position = input_port_position + input_port_text_offset;

        ui.painter().text(
            input_port_text_position,
            // egui::Align2::LEFT_TOP,
            egui::Align2::LEFT_CENTER,
            "value",
            egui::FontId::proportional(35.0),
            egui::Color32::WHITE,
        );

        let input_port_value_box_position = input_port_text_position + egui::Vec2 { x: 50.0, y: 0.0 };
        let input_port_value_box_size = egui::Vec2{ x: 120.0, y: 40.0 };
        let input_port_value_box_rect = egui::Rect::from_min_size(input_port_value_box_position + egui::Vec2 { x: 50.0, y: -20.0 }, input_port_value_box_size);

        // let input_port_value_box_screen_position = input_port_text_position + input_port_value_box_offset;

        let mut text_edit = egui::TextEdit::singleline(&mut display_input_port.value)
        .char_limit(6)
        .font(egui::FontId::proportional(35.0));
        ui.put(input_port_value_box_rect, text_edit);
    }

    for output_port_key in engine_node.output_port_keys.iter()
    {
        let display_output_port = node_graph.display_output_ports.get(output_port_key).unwrap();
        let output_port_position = node_position + display_output_port.relative_position;

        let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
        let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

        if ui.interact(output_port_rect, egui::Id::from(String::from("missing_") + "_output_port_" + output_port_key.to_string().as_str()), egui::Sense::click()).clicked()
        {
            // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::ClickedOutputPort(output_port_key.clone()) } );
            println!("output port id: {}", output_port_key);
        }

        ui.painter().circle(
            output_port_position,
            25.0,
            egui::Color32::YELLOW,
            egui::Stroke::NONE,
        );

        if !node_graph.engine.connections.contains_key(output_port_key)
        {
            continue;
        }

        let connected_port_keys = node_graph.engine.connections.get(output_port_key).unwrap();

        for connected_port_key in connected_port_keys
        {
            if !node_graph.display_input_ports.contains_key(connected_port_key)
            {
                println!("ERROR, while trying to draw connection, key not in hashtable");
                continue; 
            }

            let connected_display_port = node_graph.display_input_ports.get(connected_port_key).unwrap();
            let connected_display_node = node_graph.display_nodes.get(&connected_display_port.node_key).unwrap();

            // let connected_port_world_position = connected_display_node.position + node_centering_offset + connected_display_port.relative_position;
            let connected_port_position = connected_display_node.position + connected_display_port.relative_position;

            //let connected_node_draw_position= (connected_display_node.position + pan_offset + node_centering_offset); 
            //let connected_port_draw_position = connected_node_draw_position + connected_display_port.relative_position * zoom_scale;

            ui.painter().line_segment([ output_port_position, connected_port_position], egui::Stroke::new(10.0, egui::Color32::YELLOW));
        }
    }

    // let node_rect_1= node_rect.min;
    // let node_rect_2= node_rect.max;
    // ui.painter().circle_filled(node_rect_1, 14.0, egui::Color32::GREEN);
    // ui.painter().circle_filled(node_rect_2, 14.0, egui::Color32::GREEN);

    // let title_box_rect_1= title_box_rect.min;
    // let title_box_rect_2= title_box_rect.max;
    // ui.painter().circle_filled(title_box_rect_1, 13.0, egui::Color32::RED);
    // ui.painter().circle_filled(title_box_rect_2, 13.0, egui::Color32::RED);

    // let node_center_box_rect_1= node_rect_without_title_and_bottom.min;
    // let node_center_box_rect_2= node_rect_without_title_and_bottom.max;
    // ui.painter().circle_filled(node_center_box_rect_1, 12.0, egui::Color32::BLUE);
    // ui.painter().circle_filled(node_center_box_rect_2, 12.0, egui::Color32::BLUE);

    // let node_buttom_box_rect_1= node_rect_round_bottom.min;
    // let node_buttom_box_rect_2= node_rect_round_bottom.max;
    // ui.painter().circle_filled(node_buttom_box_rect_1, 11.0, egui::Color32::PURPLE);
    // ui.painter().circle_filled(node_buttom_box_rect_2, 11.0, egui::Color32::PURPLE);
    view_graph_node_reponse 
    
}

