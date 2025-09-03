// use empower_node_graph::EmpowerData;
// use empower_node_graph::InputPort;
// use empower_node_graph::EmpowerKey;
// use empower_node_graph::OutputPort;

use empower_engine::node_graph::port::Port;
use empower_engine::NodeGraphKey;
// use empower_engine::node_graph::
use crate::graph_editor::display_node::DisplayNode; use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
// @TODO, simplify this include
use crate::graph_editor::display_port::DisplayPort;
// use crate::graph_editor::display_port::DisplayPortValueRepresentation;
use crate::graph_editor::display_port::display_port_value;

use super::NodeWidgetResponse;
use super::NodeWidgetResponseType;


// @TODO, find a way to reduce the number of inputs in this function?
pub fn show_input_port(
                        ui: &mut egui::Ui, 
                        display_node: &DisplayNode, 
                        display_port: &DisplayPort, 
                        input_port: &Port, 
                        graph_viewport_title: &String, 
                        port_has_coonection: bool, 
                        node_key: &NodeGraphKey, 
                        port_key: &NodeGraphKey, 
                        node_widget_response: &mut Option<NodeWidgetResponse>, 
                        debug_mode: &bool
                    )
{
    let input_port_position = display_node.position + display_port.relative_position;
    let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 

    let input_port_rect = egui::Rect::from_center_size(input_port_position, input_port_size);

    let input_port_response = ui.interact(input_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_input_port_" + port_key.to_string().as_str()), egui::Sense::click());
    if input_port_response.clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedInputPort(*port_key) })
    }

    input_port_response.on_hover_text( format!("{:?}, {:?}", input_port.value, input_port.compatability ));

    // let port_color;
    let port_color = display_port_value::get_display_port_value_color(&display_port.display_value);
    
    // match input_port.value 
    // {
    //     EmpowerData::Trigger =>
    //     {
    //         // port_text = "";
    //         port_color = egui::Color32::WHITE;
    //     }
    //     EmpowerData::Integer(_) =>
    //     {

    //         // port_text = "int";
    //         port_color = egui::Color32::YELLOW;
    //     }
    //     EmpowerData::Undefined(_) =>
    //     {
    //         // port_text = ""; // @TODO, consider getting rid of the port text
    //         port_color = egui::Color32::YELLOW;
    //     }
    //     _ =>
    //     {
    //         // port_text = "float";
    //         port_color = egui::Color32::YELLOW; 
    //     }
    // }

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
    // display_port_value::show_display_port_value_type(&display_port.display_value.value_type, &input_port_text_position, port_has_coonection);
    let potentially_modified_display_value = display_port_value::show_display_port_value(&display_port.display_value, &input_port_position, port_has_coonection, ui);

    if potentially_modified_display_value != display_port.display_value
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ChangedInputPortDisplayValue(*port_key, potentially_modified_display_value ) } );
    }

    // match &display_port.value_representation
    // {
    //     DisplayPortValueRepresentation::Text( value_text) =>
    //     {
    //         let input_port_value_box_position = input_port_text_position + egui::Vec2 { x: 50.0, y: 0.0 };
    //         let input_port_value_box_size = egui::Vec2{ x: 120.0, y: 40.0 };
    //         let input_port_value_box_rect = egui::Rect::from_min_size(input_port_value_box_position + egui::Vec2 { x: 50.0, y: -20.0 }, input_port_value_box_size);

    //         let mut text_edit_color = egui::Color32::WHITE;
    //         let mut text_background_color = egui::Color32::BLACK;

    //         if port_has_coonection
    //         {
    //             text_edit_color = egui::Color32::GRAY;
    //             text_background_color = egui::Color32::TRANSPARENT;
    //         }

    //         if !display_port.value_representation_valid
    //         {
    //             text_edit_color = egui::Color32::RED;
    //         }

    //         let mut display_port_text = value_text.clone();

    //         let text_edit = egui::TextEdit::singleline(&mut display_port_text)
    //         .char_limit(5)
    //         .font(egui::FontId::proportional(35.0))
    //         .interactive(!port_has_coonection)
    //         .text_color(text_edit_color)
    //         .background_color(text_background_color);
            
    //         ui.put(input_port_value_box_rect, text_edit);

    //         if display_port_text != *value_text
    //         {
    //             // @TODO, consider how to change this for other than text
    //             *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ChangedInputPortValueRepresentation(*port_key, DisplayPortValueRepresentation::Text( display_port_text )) } );
    //         }
    //     },

    //     DisplayPortValueRepresentation::Checkbox( boolean ) =>
    //     {
    //         let input_port_checkbox_position = input_port_text_position + egui::Vec2 { x: 50.0, y: 0.0 };
    //         let input_port_checkbox_size = egui::Vec2{ x: 120.0, y: 0.0 };
    //         let input_port_checkbox_rect = egui::Rect::from_min_size(input_port_checkbox_position + egui::Vec2 { x: 50.0, y: -20.0 }, input_port_checkbox_size);

    //         let mut check = boolean.clone();
            
    //         let checkbox = egui::Checkbox::new(&mut check, egui::RichText::new("check").font(egui::FontId::proportional(35.0)));
    //         ui.put(input_port_checkbox_rect, checkbox);

    //         if check != *boolean
    //         {
    //             *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ChangedInputPortValueRepresentation( *port_key, DisplayPortValueRepresentation::Checkbox( check ) ) });
    //         }
    //     },

    //     DisplayPortValueRepresentation::None =>
    //     {

    //     },
    // }
}

