use empower_node_graph::EmpowerKey;
use empower_node_graph::Node;
use empower_node_graph::InputPort;
use empower_node_graph::OutputPort;
use std::collections::HashMap;

use crate::graph_editor;
use crate::graph_editor::GraphEditor;
use crate::graph_editor::display_node::DisplayNode; // @TODO, simplify this include
use crate::graph_editor::display_port::DisplayPort;
use super::GraphViewport;
use super::port_searcher::{PortSearcher, PortKind};

pub fn show(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, mouse_delta: egui::Vec2, graph_viewport: &mut GraphViewport, node_key: &EmpowerKey)
{
    // @TODO, consider making the access to the nodes here easier
    let empower_node = graph_editor.empower_node_graph.nodes.get_mut(node_key).unwrap();
    let display_node = graph_editor.display_nodes.get_mut(node_key).unwrap();
    let selected_nodes = &mut graph_editor.selected_nodes;

    show_node_body(ui, empower_node, display_node, selected_nodes, mouse_delta, graph_viewport); // @TODO, consider changing this back to graph_viewport_title for clarity

    let input_port_keys = empower_node.input_port_keys.iter().cloned();
    for input_port_key in input_port_keys
    {
        let display_input_port = graph_editor.display_input_ports.get_mut(&input_port_key).unwrap();
        let empower_input_port = graph_editor.empower_node_graph.input_ports.get_mut(&input_port_key).unwrap();

        // @TODO, find a better way to write this function
        show_input_port(ui, display_node, display_input_port, empower_input_port, graph_viewport, &mut graph_editor.empower_node_graph.connections);
    }
    
    let output_port_keys= empower_node.output_port_keys.iter().cloned();
    for output_port_key in output_port_keys
    {
        let display_output_port = graph_editor.display_output_ports.get_mut(&output_port_key).unwrap();
        let empower_output_port = graph_editor.empower_node_graph.output_ports.get_mut(&output_port_key).unwrap();

        show_output_port(ui, display_node, display_output_port, empower_output_port, graph_viewport);
    }
}

fn show_node_body(ui: &mut egui::Ui, empower_node: &mut Node, display_node: &mut DisplayNode, selected_nodes: &mut Vec<EmpowerKey>, mouse_delta: egui::Vec2, graph_viewport: &mut GraphViewport)
{
    let node_key = empower_node.key;
    let node_position = display_node.position;
    let node_screen_size= display_node.size;

    let node_rect= egui::Rect::from_min_size(
        node_position,
        node_screen_size
    );

    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
    let rect_margin = egui::Vec2 { x: 10.0, y: 10.0 };

    let node_outline_rect = node_rect.expand2(rect_margin);

    let node_is_selected = selected_nodes.iter().any(| selected_node_key | *selected_node_key == node_key ); // @TODO, find a reduce the computation of this check

    if node_is_selected // @TODO, find a better place to put this
    {
        display_node.position += mouse_delta;
    }
    
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
        egui::Id::new(graph_viewport.title.to_owned() + "_node_body_" + empower_node.key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Sense::click_and_drag(),
    );

    
    if node_reponse.hovered() // Important that this is done before clicked
    {
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }

    if node_reponse.clicked()
    {
        if selected_nodes.contains(&node_key) // @TODO, figure out if this is the most performance apporaach.
        {
            selected_nodes.retain(|x| x != &node_key );
        }
        else // @TODO, rewrite this
        {
            selected_nodes.push(node_key);
        }
    }


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
}

fn show_input_port(ui: &mut egui::Ui, display_node: &DisplayNode, display_port: &mut DisplayPort, empower_input_port: &InputPort, graph_viewport: &mut GraphViewport, connections: &mut HashMap<EmpowerKey, Vec<EmpowerKey>>)
{
    let input_port_position = display_node.position + display_port.relative_position;
    let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 

    let input_port_rect = egui::Rect::from_center_size(input_port_position, input_port_size);

    // if ui.interact(input_port_rect, egui::Id::from( graph_title.clone() + "_input_port_" + input_port_key.to_string().as_str()), egui::Sense::click()).clicked()
    if ui.interact(input_port_rect, egui::Id::from( graph_viewport.title.to_owned() + "_input_port_" + empower_input_port.key.to_string().as_str()), egui::Sense::click()).clicked()
    {
        input_port_interaction(graph_viewport, connections, empower_input_port);
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

fn show_output_port(ui: &mut egui::Ui, display_node: &DisplayNode, display_port: &mut DisplayPort, empower_output_port: &OutputPort, graph_viewport: &mut GraphViewport)
{
    let output_port_position = display_node.position + display_port.relative_position;

    let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
    let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

    // if ui.interact(output_port_rect, egui::Id::from( graph_title.clone() + "_output_port_" + output_port_key.to_string().as_str()), egui::Sense::click()).clicked()
    if ui.interact(output_port_rect, egui::Id::from( graph_viewport.title.to_owned() + "_output_port_" + empower_output_port.key.to_string().as_str()), egui::Sense::click()).clicked()
    {
        output_port_interaction(graph_viewport, empower_output_port);
    }

    ui.painter().circle(
        output_port_position,
        25.0,
        egui::Color32::YELLOW,
        egui::Stroke::NONE,
    );
}

fn input_port_interaction(graph_viewport: &mut GraphViewport, connections: &mut HashMap<EmpowerKey, Vec<EmpowerKey>>, empower_input_port: &InputPort) // @TODO, find a better name
{
    if graph_viewport.port_searcher.is_none()
    {
        graph_viewport.port_searcher = Some( PortSearcher { port_key: empower_input_port.key, port_kind: PortKind::InputPort });
        return;
    }

    let port_searcher = graph_viewport.port_searcher.unwrap();

    match  port_searcher.port_kind 
    {
        PortKind::InputPort =>
        {
            if port_searcher.port_key == empower_input_port.key
            {
                graph_viewport.port_searcher = None; // @TODO, expand this functionality to be more complex
                return;
            } 
        },

        PortKind::OutputPort =>
        {
            let port_connection = connections.iter().map(|(key, vec)|
            {
                if vec.contains(&empower_input_port.key)
                {
                    Some(key)
                }
                else // @TODO, find a better way to write
                {
                    None
                }
            });

            let connection = connections.get_mut(&port_searcher.port_key).unwrap(); 

            graph_viewport.port_searcher = Some( PortSearcher { port_key: empower_input_port.key, port_kind: PortKind::InputPort });
        }

    }

    println!("input port id from show function: {}", empower_input_port.key);
}

fn output_port_interaction(graph_viewport: &mut GraphViewport, empower_output_port: &OutputPort) // @TODO, find a better name
{
    if graph_viewport.port_searcher.is_none()
    {
        graph_viewport.port_searcher = Some( PortSearcher { port_key: empower_output_port.key, port_kind: PortKind::OutputPort });
        return;
    }

    let port_searcher = graph_viewport.port_searcher.unwrap();

    match  port_searcher.port_kind 
    {
        PortKind::InputPort =>
        {
            graph_viewport.port_searcher = Some( PortSearcher { port_key: empower_output_port.key, port_kind: PortKind::InputPort });
        },

        PortKind::OutputPort =>
        {
            if port_searcher.port_key == empower_output_port.key
            {
                graph_viewport.port_searcher = None;
            }
        }
    }

    println!("output port id from show function: {}", empower_output_port.key);
   
}
