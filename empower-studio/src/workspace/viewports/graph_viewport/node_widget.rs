use empower_node_graph::port;
use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerNodeGraph;
use empower_node_graph::Node;
use empower_node_graph::InputPort;
use empower_node_graph::OutputPort;
use std::collections::HashMap;

use crate::graph_editor;
use crate::graph_editor::display_node;
use crate::graph_editor::GraphEditor;
use crate::graph_editor::display_node::DisplayNode; // @TODO, simplify this include
use crate::graph_editor::display_port::DisplayPort;
use super::GraphViewport;
use super::port_searcher::{PortSearcher, PortKind};

pub fn show(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, mouse_delta: egui::Vec2, graph_viewport: &mut GraphViewport, node_key: &EmpowerKey)
{
    show_node_body(ui, graph_editor, graph_viewport, mouse_delta, node_key); // @TODO, consider changing this back to graph_viewport_title for clarity

    // @TODO find a more effecient way of getting the nodes list
    let empower_node = graph_editor.empower_node_graph.nodes.get(node_key).unwrap().clone();
    let input_port_keys = empower_node.input_port_keys.iter().cloned();
    for input_port_key in input_port_keys
    {
        show_input_port(ui, graph_editor, graph_viewport, &input_port_key);
    }
    
    let output_port_keys= empower_node.output_port_keys.iter().cloned();
    for output_port_key in output_port_keys
    {
        show_output_port(ui, graph_editor, graph_viewport, &output_port_key);
    }
}

fn show_node_body(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, mouse_delta: egui::Vec2, node_key: &EmpowerKey)
{
    // @TODO, improve this call
    let display_node = graph_editor.display_nodes.get_mut(node_key).unwrap();

    let node_key = node_key;
    let node_position = display_node.position;
    let node_screen_size= display_node.size;

    let node_rect= egui::Rect::from_min_size(
        node_position,
        node_screen_size
    );

    let node_body_color = egui::Color32::from_rgb(63, 63, 63);
    let rect_margin = egui::Vec2 { x: 10.0, y: 10.0 };

    let node_outline_rect = node_rect.expand2(rect_margin);

    let node_is_selected = graph_editor.selected_nodes.iter().any(| selected_node_key | *selected_node_key == *node_key ); // @TODO, find a reduce the computation of this check

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
        egui::Id::new(graph_viewport.title.to_owned() + "_node_body_" + node_key.to_string().as_str()), // @TODO, find a way to move title out of state
        egui::Sense::click_and_drag(),
    );

    
    if node_reponse.hovered() // Important that this is done before clicked
    {
        title_rect_color= egui::Color32::from_rgb(40, 40, 40);
    }

    if node_reponse.clicked()
    {
        if graph_editor.selected_nodes.contains(&node_key) // @TODO, figure out if this is the most performance apporaach.
        {
            graph_editor.selected_nodes.retain(|x| x != node_key );
        }
        else // @TODO, rewrite this
        {
            graph_editor.selected_nodes.push(*node_key);
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

fn show_input_port(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, port_key: &EmpowerKey)
{
    // @TODO, simplify these calls
    let display_port = graph_editor.display_input_ports.get_mut(port_key).unwrap();
    let display_node = graph_editor.display_nodes.get(&display_port.node_key).unwrap();

    let input_port_position = display_node.position + display_port.relative_position;
    let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 

    let input_port_rect = egui::Rect::from_center_size(input_port_position, input_port_size);

    // if ui.interact(input_port_rect, egui::Id::from( graph_title.clone() + "_input_port_" + input_port_key.to_string().as_str()), egui::Sense::click()).clicked()
    if ui.interact(input_port_rect, egui::Id::from( graph_viewport.title.to_owned() + "_input_port_" + port_key.to_string().as_str()), egui::Sense::click()).clicked()
    {
        input_port_interaction(&mut graph_editor.empower_node_graph, graph_viewport, port_key);
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

    let mut text_edit_color = egui::Color32::WHITE;

    let empower_port = graph_editor.empower_node_graph.input_ports.get_mut(&display_port.node_key).unwrap();


    let port_has_connection = graph_editor.empower_node_graph.connections_in.contains_key(port_key);

    if port_has_connection
    {
        display_port.value = empower_port.get_value_as_string();
        text_edit_color = egui::Color32::GRAY;
    }

    let successfully_set_port_value = empower_port.set_value_with_text(&display_port.value);
    if !successfully_set_port_value
    {
        text_edit_color = egui::Color32::RED;
    }

    let text_edit = egui::TextEdit::singleline(&mut display_port.value)
    .char_limit(6)
    .font(egui::FontId::proportional(35.0))
    .interactive(!port_has_connection)
    .text_color(text_edit_color);
    
    ui.put(input_port_value_box_rect, text_edit);
}

fn show_output_port(ui: &mut egui::Ui, graph_editor: &mut GraphEditor, graph_viewport: &mut GraphViewport, port_key: &EmpowerKey)
{
    let display_port = graph_editor.display_output_ports.get_mut(port_key).unwrap();
    let display_node = graph_editor.display_nodes.get(&display_port.node_key).unwrap();

    let output_port_position = display_node.position + display_port.relative_position;

    let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
    let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

    // if ui.interact(output_port_rect, egui::Id::from( graph_title.clone() + "_output_port_" + output_port_key.to_string().as_str()), egui::Sense::click()).clicked()
    if ui.interact(output_port_rect, egui::Id::from( graph_viewport.title.to_owned() + "_output_port_" + port_key.to_string().as_str()), egui::Sense::click()).clicked()
    {
        output_port_interaction(&mut graph_editor.empower_node_graph, graph_viewport, port_key);
    }

    ui.painter().circle(
        output_port_position,
        25.0,
        egui::Color32::YELLOW,
        egui::Stroke::NONE,
    );
}

fn input_port_interaction(empower_node_graph: &mut EmpowerNodeGraph, graph_viewport: &mut GraphViewport, port_key: &EmpowerKey) // @TODO, find a better name
{
    if graph_viewport.port_searcher.is_none()
    {
        graph_viewport.port_searcher = Some( PortSearcher { port_key: *port_key, port_kind: PortKind::InputPort });
        return;
    }

    let port_searcher = graph_viewport.port_searcher.unwrap();

    match  port_searcher.port_kind 
    {
        PortKind::InputPort =>
        {
            if port_searcher.port_key == *port_key
            {
                graph_viewport.port_searcher = None; // @TODO, expand this functionality to be more complex
                return;
            } 
        },

        PortKind::OutputPort =>
        {
            // let port_connection = connections.iter().map(|(key, vec)|
            // {
            //     if vec.contains(&empower_input_port.key)
            //     {
            //         Some(key)
            //     }
            //     else // @TODO, find a better way to write
            //     {
            //         None
            //     }
            // });

            // let connection = connections.get_mut(&port_searcher.port_key).unwrap(); 
            empower_node_graph.add_connection(*port_key, port_searcher.port_key);
            graph_viewport.port_searcher = None;
            return;
        }
    }

    println!("input port id from show function: {}", *port_key);
}

fn output_port_interaction(empower_node_graph: &mut EmpowerNodeGraph, graph_viewport: &mut GraphViewport, port_key: &EmpowerKey) // @TODO, find a better name
{
    if graph_viewport.port_searcher.is_none()
    {
        graph_viewport.port_searcher = Some( PortSearcher { port_key: *port_key, port_kind: PortKind::OutputPort });
        return;
    }

    let port_searcher = graph_viewport.port_searcher.unwrap();

    match  port_searcher.port_kind 
    {
        PortKind::InputPort =>
        {
            empower_node_graph.add_connection(port_searcher.port_key, *port_key);
            graph_viewport.port_searcher = None;
            return;
        },

        PortKind::OutputPort =>
        {
            if port_searcher.port_key == *port_key
            {
                graph_viewport.port_searcher = None;
            }
        }
    }

    println!("output port id from show function: {}", port_key);
   
}
