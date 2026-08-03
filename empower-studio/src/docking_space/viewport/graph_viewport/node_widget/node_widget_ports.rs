use std::collections::{HashMap, VecDeque};

use empower_engine::node_graph::port::{PortDirection, PortEdit, PortKind};
use empower_engine::node_graph::{NodeGraphKey, Port};

use super::GraphViewportAction;

use super::{PORT_SIZE, VERTICAL_PORT_GAB};

const PORT_AND_TEXT_HORIZONTAL_BUFFER: f32 = 40.0;
const PORT_TEXT_FONT_SIZE: f32 = 35.0;
const TEXT_AND_EDIT_HORIZONTAL_BUFFER: f32 = 20.0; 

const INTEGER_EDIT_BOX_LENGTH: f32 = 120.0;

pub fn show(
            ui: &mut egui::Ui, 
            node_position: &egui::Pos2,
            port_key: &NodeGraphKey,
            port: &Port,
            port_index: usize,
            port_has_connection: &bool,
            graph_viewport_title: &String, 
            graph_viewport_action: &mut VecDeque<GraphViewportAction>,
            node_size: &egui::Vec2,
            cached_port_positions: &mut HashMap<NodeGraphKey, egui::Pos2>,
            vertical_offset_before_showing_ports: f32,
            developer_mode: &bool,
)
{
    let horizontal_offset = match port.direction
    {
        PortDirection::Input => 0.0,
        PortDirection::Output => node_size.x,
    };
    
    let port_position  = *node_position + egui::Vec2 { x: horizontal_offset, y: vertical_offset_before_showing_ports + VERTICAL_PORT_GAB / 2.0 + port_index as f32 * VERTICAL_PORT_GAB};
    let port_rect = egui::Rect::from_center_size(port_position, PORT_SIZE);

    let port_response = ui.interact(port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_port_" + port_key.to_string().as_str()), egui::Sense::click());
    if port_response.clicked()
    {
        graph_viewport_action.push_back( GraphViewportAction::ClickedPort { port_key: *port_key });
    }

    let type_text; 
    if !*developer_mode
    {
        type_text = if port.value.is_some() // @TODO, take a second look at this, should only ever fail when its exec port
        {
            port.value.as_ref().unwrap().type_string()
        }
        else if !port.compatability.is_empty() {
            port.compatability[0].type_string()
        }
        else
        {
            "".to_string()
        };
    }
    else
    {
        type_text = if port.value.is_some()
        {
            port.value.as_ref().unwrap().type_string() + " (" + port.value.as_ref().unwrap().to_string().as_str() + ")"
        }
        else if !port.compatability.is_empty()
        {
            "undefined, will default to: ".to_string() + port.compatability[0].type_string().as_str() + " (" + port.compatability[0].to_string().as_str() + ")"
        }
        else
        {
            "undefined".to_string()
        }
    }

    let compatible_type_text: Vec<String> = port.compatability.iter().map(|e| e.type_string()).collect();

    port_response.on_hover_text( format!("{} : {:?}", type_text, compatible_type_text ));

    let port_color = port.color();

    ui.painter().circle(
        port_position,
        25.0,
        port_color,
        egui::Stroke::NONE,
    );

    if *developer_mode
    {
        ui.painter().text(
            port_position,
            egui::Align2::CENTER_CENTER,
            port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }

    cached_port_positions.insert(*port_key, port_position);

    if port.kind == PortKind::Execution
    {
        return;
    }

    if port.direction == PortDirection::Output
    {
        return;
    }

    let port_text_position = port_position + egui::Vec2 { x: PORT_AND_TEXT_HORIZONTAL_BUFFER, y: 0.0 };

    let painted_text = ui.painter().text(
        port_text_position,
        egui::Align2::LEFT_CENTER,
        &port.name,
        egui::FontId::proportional(PORT_TEXT_FONT_SIZE),
        egui::Color32::WHITE,
    );

    if *port_has_connection // This only ever applies to the input ports
    {
        return;
    }

    let painted_text_size = painted_text.size();
    let port_edit_position = port_text_position + egui::Vec2 { x: painted_text_size.x + TEXT_AND_EDIT_HORIZONTAL_BUFFER, y: - painted_text_size.y / 2.0 };

    let text_edit_color = if port.value.is_some() { egui::Color32::WHITE } else { egui::Color32::RED };
    // let edit_was_changed = match &mut port.edit
    // {
    //     PortEdit::None => false,
    //     PortEdit::Text( text ) =>
    //     {
    //         let port_edit_box_size = egui::Vec2{ x: INTEGER_EDIT_BOX_LENGTH, y: painted_text_size.y };
    //         let input_port_value_box_rect = egui::Rect::from_min_size(port_edit_position, port_edit_box_size);

    //         let text_edit = egui::TextEdit::singleline(text)
    //         // .char_limit(5)
    //         .font(egui::FontId::proportional(35.0))
    //         // .interactive(!port_has_connection)
    //         .text_color(text_edit_color)
    //         .background_color(egui::Color32::BLACK);

    //         let response = ui.put(input_port_value_box_rect, text_edit);
    //         response.changed()
    //     },
    //     PortEdit::CheckBox( toggle ) =>
    //     {
    //         let input_port_checkbox_size = egui::Vec2{ x: 120.0, y: 0.0 };
    //         let input_port_checkbox_rect = egui::Rect::from_min_size(port_edit_position, input_port_checkbox_size);

    //         // @TODO, improve this, and fix box size
    //         let checkbox = egui::Checkbox::new(
    //                                         toggle, 
    //                                         egui::RichText::new("").font(egui::FontId::proportional(35.0))
    //         );

    //         ui.put(input_port_checkbox_rect, checkbox).changed()
    //     }
    //     PortEdit::TwoBox( text1, text2 ) =>
    //     {
    //         let port_value_box_size = egui::Vec2{ x: 50.0, y: painted_text_size.y };

    //         let mut box_one_changed = false;
    //         let mut box_two_changed = false;
    //         ui.horizontal(|ui|
    //         {
    //             let text_edit = egui::TextEdit::singleline(text1).font(egui::FontId::proportional(35.0));
    //             box_one_changed = ui.put(
    //                 egui::Rect::from_min_size(port_edit_position, port_value_box_size),
    //                 text_edit
    //             ).changed();

    //             let text_edit = egui::TextEdit::singleline(text2).font(egui::FontId::proportional(35.0));
    //             box_two_changed = ui.put(
    //                 egui::Rect::from_min_size(port_edit_position + egui::Vec2 { x: 60.0, y: 0.0 }, port_value_box_size),
    //                 text_edit
    //             ).changed();

    //             // let text_edit = egui::TextEdit::singleline(to).font(egui::FontId::proportional(35.0));
    //             // ui.put(
    //             //     egui::Rect::from_min_size(port_edit_position + egui::Vec2 { x: 120.0, y: 0.0 }, input_port_value_box_size),
    //             //     text_edit);
    //         });


    //         box_one_changed || box_two_changed
    //     }
    // };

    // if edit_was_changed
    // {
    //     graph_viewport_action.push_back( GraphViewportAction::PortEditWasChanged { port_key: *port_key } );
    // }

    // let edit_width = match port.edit
    // {
    //     PortEdit::None => PORT_AND_TEXT_HORIZONTAL_BUFFER + painted_text_size.x,
    //     PortEdit::Text(_) => 150.0,
    //     PortEdit::CheckBox(_) => 150.0,
    // };
}

