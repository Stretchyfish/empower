use egui;
use empower_node_graph::EmpowerKey;
use empower_node_graph::{InputPort, OutputPort};
use empower_node_graph::Node;

use crate::studio_context;
use crate::StudioContext;
use crate::studio_context::display_node_graph::DisplayNode;
use crate::studio_context::display_node_graph::DisplayPort;

use super::graph_node_widget; // @TODO, consider better structure of these

pub struct NodeWidgetResponse
{
    key: EmpowerKey,
    kind: NodeWidgetResponseType,
}

pub enum NodeWidgetResponseType
{
    Clicked,
    Hover,
    InsideSelectionArea,
    ClickedInputPort(EmpowerKey),
    ClickedOutputPort(EmpowerKey)
}

pub fn show_node_widget(ui: &mut egui::Ui, studio_context: &mut StudioContext, node_key: &EmpowerKey, graph_viewport_title: &String) -> Option<NodeWidgetResponse>
{
    let mut node_widget_response: Option<NodeWidgetResponse> = None;

    let empower_node = studio_context.empower_node_graph.nodes.get_mut(node_key).unwrap();
    let display_node = studio_context.display_node_graph.display_nodes.get_mut(node_key).unwrap();

    show_node_body(ui, empower_node, display_node, &mut node_widget_response, &graph_viewport_title);

    let input_port_keys = empower_node.input_port_keys.iter().cloned();
    for input_port_key in input_port_keys
    {
        let display_input_port = studio_context.display_node_graph.display_input_ports.get_mut(&input_port_key).unwrap();
        let empower_input_port = studio_context.empower_node_graph.input_ports.get_mut(&input_port_key).unwrap();

        show_input_port(ui, display_node, display_input_port, empower_input_port, &graph_viewport_title);
    }
    
    let output_port_keys= empower_node.output_port_keys.iter().cloned();
    for output_port_key in output_port_keys
    {
        let display_output_port = studio_context.display_node_graph.display_output_ports.get_mut(&output_port_key).unwrap();
        let empower_output_port = studio_context.empower_node_graph.output_ports.get_mut(&output_port_key).unwrap();

        show_output_port(ui, display_node, display_output_port, empower_output_port, &graph_viewport_title);
    }

    node_widget_response
}

fn show_node_body(ui: &mut egui::Ui, empower_node: &mut Node, display_node: &mut DisplayNode, node_widget_response: &mut Option<NodeWidgetResponse>, graph_viewport_title: &String) // @TODO, decide if NodeWidgetResponse should be returned instead of passed 
{
    let node_position = display_node.position;
    let node_screen_size= display_node.size;

    let node_rect= egui::Rect::from_min_size(
        node_position,
        node_screen_size
    );

    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
    let rect_margin = egui::Vec2 { x: 10.0, y: 10.0 };

    let node_outline_rect = node_rect.expand2(rect_margin);

    // let node_is_selected = graph_viewport_state.selected_nodes.iter().any(| selected_node_key | *selected_node_key == display_node_key ); // @TODO, find a reduce the computation of this check
    let title_text_font_size = 40.0;
    let node_title = display_node.title.clone();
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

    let node_reponse = ui.interact(
        title_box_rect,
        // egui::Id::new( graph_title.clone() + "_node_body_" + display_node_key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Id::new(graph_viewport_title.to_owned() + "_node_body_" + empower_node.key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Sense::click_and_drag(),
    );

    
    if node_reponse.hovered() // Important that this is done before clicked
    {
        // view_graph_node_reponse = Some( NodeViewReponse { key: display_node.key, kind: NodeViewResponseType::Hover } );
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }

    if node_reponse.clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: empower_node.key, kind: NodeWidgetResponseType::Clicked } );
    }


    // if node_is_selected // @TODO, add this back
    // {
    //     ui.painter().rect(
    //         node_outline_rect,
    //         6.0,
    //         egui::Color32::ORANGE,
    //         egui::Stroke::NONE,
    //         egui::StrokeKind::Inside,
    //     );
    // }


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
}

fn show_input_port(ui: &mut egui::Ui, display_node: &DisplayNode, display_port: &mut DisplayPort, empower_input_port: &InputPort, graph_viewport_title: &String)
{
    let input_port_position = display_node.position + display_port.relative_position;
    let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 

    let input_port_rect = egui::Rect::from_center_size(input_port_position, input_port_size);

    // if ui.interact(input_port_rect, egui::Id::from( graph_title.clone() + "_input_port_" + input_port_key.to_string().as_str()), egui::Sense::click()).clicked()
    if ui.interact(input_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_input_port_" + empower_input_port.key.to_string().as_str()), egui::Sense::click()).clicked()
    {
        // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::ClickedInputPort(input_port_key.clone()) } );
        println!("input port id from show function: {}", empower_input_port.key);
        // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::ClickedInputPort(*input_port_key)} );
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

    let mut text_edit = egui::TextEdit::singleline(&mut display_port.value)
    .char_limit(6)
    .font(egui::FontId::proportional(35.0));
    ui.put(input_port_value_box_rect, text_edit);
}

fn show_output_port(ui: &mut egui::Ui, display_node: &DisplayNode, display_port: &mut DisplayPort, empower_output_port: &OutputPort, graph_viewport_title: &String)
{
    let output_port_position = display_node.position + display_port.relative_position;

    let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
    let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

    // if ui.interact(output_port_rect, egui::Id::from( graph_title.clone() + "_output_port_" + output_port_key.to_string().as_str()), egui::Sense::click()).clicked()
    if ui.interact(output_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_output_port_" + empower_output_port.key.to_string().as_str()), egui::Sense::click()).clicked()
    {
        // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::ClickedOutputPort(output_port_key.clone()) } );
        println!("output port id in view function: {}", empower_output_port.key);
    }

    ui.painter().circle(
        output_port_position,
        25.0,
        egui::Color32::YELLOW,
        egui::Stroke::NONE,
    );
}
