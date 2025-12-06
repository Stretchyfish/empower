use better_empower_engine::{NodeGraphKey, node_graph::node::port::Port};
use crate::graph_editor::display_node::DisplayNode; 
use crate::graph_editor::display_port::DisplayPort;

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
}