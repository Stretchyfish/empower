use better_empower_engine::{NodeGraphKey, node_graph::node::port::Port};
use crate::graph_editor::display_node::DisplayNode; 
use crate::graph_editor::display_node::DisplayPort;
use crate::graph_editor::display_node::DisplayValue;

use super::NodeWidgetResponse;
use super::NodeWidgetResponseType;

// @TODO, find a way to reduce the number of inputs in this function?
pub fn show_input_port(
                        ui: &mut egui::Ui, 
                        display_node: &DisplayNode, 
                        display_port: &DisplayPort, 
                        input_port: &Port, 
                        graph_viewport_title: &String, 
                        port_has_connection: bool, 
                        node_key: &NodeGraphKey, 
                        port_key: &NodeGraphKey, 
                        node_widget_response: &mut Option<NodeWidgetResponse>, 
                        debug_mode: &bool,
                    )
{
    let input_port_position = display_port.position;
    let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
 
    let input_port_rect = egui::Rect::from_center_size(display_port.position, input_port_size);

    let input_port_response = ui.interact(input_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_input_port_" + port_key.to_string().as_str()), egui::Sense::click());
    if input_port_response.clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedInputPort(*port_key) })
    }
    input_port_response.on_hover_text( format!("{:?}, {:?}", input_port.value, input_port.compatability ));

    // let port_color = display_port.display_value.color;
    let port_color = egui::Color32::YELLOW;

    ui.painter().circle(
        input_port_position,
        25.0,
        port_color,
        egui::Stroke::NONE,
    );

    if *debug_mode
    {
        ui.painter().text(
            input_port_position,
            egui::Align2::CENTER_CENTER,
            port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }

    let input_port_text_offset = egui::Vec2 { x: 40.0, y: 0.0};
    let input_port_text_position = display_port.position + input_port_text_offset;

    let input_port_text_font_size = 35.0;

    let painted_text = ui.painter().text(
        input_port_text_position,
        egui::Align2::LEFT_CENTER,
        &display_port.text,
        egui::FontId::proportional(input_port_text_font_size),
        egui::Color32::WHITE,
    );

    let painted_text_size = painted_text.size();
    let text_and_display_value_buffer = 20.0;

    let input_port_value_position = input_port_text_position + egui::Vec2 { x: painted_text_size.x + text_and_display_value_buffer, y: -painted_text_size.y / 2.0 };

    let mut potentially_modified_display_value= display_port.value.clone(); // @TODO, find a better name
    match &mut potentially_modified_display_value
    {
        DisplayValue::Nothing => {},
        DisplayValue::Text( text ) => 
        {
            let input_port_value_box_size = egui::Vec2{ x: 120.0, y: painted_text_size.y };
            let input_port_value_box_rect = egui::Rect::from_min_size(input_port_value_position, input_port_value_box_size);

            let mut text_edit_color = egui::Color32::WHITE;
            let mut text_background_color = egui::Color32::BLACK;

            if port_has_connection
            {
                text_edit_color = egui::Color32::GRAY;
                text_background_color = egui::Color32::TRANSPARENT;
            }

            if !display_port.convertable
            {
                text_edit_color = egui::Color32::RED;
            }

            let text_edit = egui::TextEdit::singleline(text)
            .char_limit(5)
            .font(egui::FontId::proportional(35.0))
            .interactive(!port_has_connection)
            .text_color(text_edit_color)
            .background_color(text_background_color);

            ui.put(input_port_value_box_rect, text_edit);
        },
        DisplayValue::Checkbox( toggle ) => 
        {
            let input_port_checkbox_size = egui::Vec2{ x: 120.0, y: 0.0 };
            let input_port_checkbox_rect = egui::Rect::from_min_size(input_port_value_position, input_port_checkbox_size);

            // @TODO, improve this, and fix box size
            let checkbox = egui::Checkbox::new(
                                                    toggle, 
                                                        egui::RichText::new("").font(egui::FontId::proportional(35.0)));
            ui.put(input_port_checkbox_rect, checkbox);
        },
    }

    if potentially_modified_display_value != display_port.value
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ChangedInputPortDisplayValue(*port_key, potentially_modified_display_value ) } );
    }
}