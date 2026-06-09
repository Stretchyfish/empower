use std::collections::{HashMap, VecDeque};

use empower_engine::node_graph::port::{PortDirection, PortEdit, PortKind};
use empower_engine::node_graph::{NodeGraphKey, Port};

use super::GraphViewportAction;

use super::{PORT_SIZE, VERTICAL_PORT_GAB};

const PORT_AND_TEXT_HORIZONTAL_BUFFER: f32 = 40.0;
const PORT_TEXT_FONT_SIZE: f32 = 35.0;
const TEXT_AND_EDIT_HORIZONTAL_BUFFER: f32 = 20.0; 

const INTEGER_EDIT_BOX_LENGTH: f32 = 120.0;

pub struct NodeWidgetPortResponse
{
    pub port_vertical_position: f32,
    pub horizontal_element_size: f32,
}

pub fn show(
            ui: &mut egui::Ui, 
            node_position: &egui::Pos2,
            port_key: &NodeGraphKey,
            port: &mut Port,
            port_index: usize,
            port_has_connection: &bool,
            graph_viewport_title: &String, 
            graph_viewport_action: &mut VecDeque<GraphViewportAction>,
            cached_node_sizes: &HashMap<NodeGraphKey, egui::Vec2>,
            cached_port_positions: &mut HashMap<NodeGraphKey, egui::Pos2>,
            vertical_offset_before_showing_ports: f32,
            developer_mode: &bool,
) -> NodeWidgetPortResponse
{
    let node_size = cached_node_sizes.get(&port.node_key).unwrap(); // This is safe to do, due to the cached size always being filled out first in node_widget_body::show

    let horizontal_offset = match port.direction
    {
        PortDirection::Input => 0.0,
        PortDirection::Output => node_size.x,
    };
    
    let port_position  = *node_position + egui::Vec2 { x: horizontal_offset, y: vertical_offset_before_showing_ports + VERTICAL_PORT_GAB / 2.0 + port_index as f32 * VERTICAL_PORT_GAB};
    let port_rect = egui::Rect::from_center_size(port_position, PORT_SIZE);

    let output_port_response = ui.interact(port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_port_" + port_key.to_string().as_str()), egui::Sense::click());
    if output_port_response.clicked()
    {
        graph_viewport_action.push_back( GraphViewportAction::ClickedPort { port_key: *port_key });
    }

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
        return NodeWidgetPortResponse { port_vertical_position: port_position.y, horizontal_element_size: 0.0 };
    }

    if port.direction == PortDirection::Output
    {
        return NodeWidgetPortResponse { port_vertical_position: port_position.y, horizontal_element_size: 0.0 };
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
        return NodeWidgetPortResponse { port_vertical_position: port_position.y, horizontal_element_size: PORT_AND_TEXT_HORIZONTAL_BUFFER + painted_text.size().x };
    }

    let painted_text_size = painted_text.size();
    let port_edit_position = port_text_position + egui::Vec2 { x: painted_text_size.x + TEXT_AND_EDIT_HORIZONTAL_BUFFER, y: - painted_text_size.y / 2.0 };

    let text_edit_color = if port.parseble { egui::Color32::WHITE } else { egui::Color32::RED };
    let edit_was_changed = match &mut port.edit
    {
        PortEdit::None => false,
        PortEdit::Interger( integer_string ) =>
        {
            let port_edit_box_size = egui::Vec2{ x: INTEGER_EDIT_BOX_LENGTH, y: painted_text_size.y };
            let input_port_value_box_rect = egui::Rect::from_min_size(port_edit_position, port_edit_box_size);

            let text_edit = egui::TextEdit::singleline(integer_string)
            // .char_limit(5)
            .font(egui::FontId::proportional(35.0))
            // .interactive(!port_has_connection)
            .text_color(text_edit_color)
            .background_color(egui::Color32::BLACK);


            let response = ui.put(input_port_value_box_rect, text_edit);
            response.changed()
        },
        PortEdit::Float(_) => todo!(),
    };

    if edit_was_changed
    {
        graph_viewport_action.push_back( GraphViewportAction::PortEditWasChanged { port_key: *port_key } );
    }

    let edit_width = match port.edit
    {
        PortEdit::None => PORT_AND_TEXT_HORIZONTAL_BUFFER + painted_text_size.x,
        PortEdit::Interger(_) => 150.0,
        PortEdit::Float(_) => 150.0,
    };

    NodeWidgetPortResponse { port_vertical_position: port_position.y, horizontal_element_size: PORT_AND_TEXT_HORIZONTAL_BUFFER + painted_text.size().x + edit_width }
}

