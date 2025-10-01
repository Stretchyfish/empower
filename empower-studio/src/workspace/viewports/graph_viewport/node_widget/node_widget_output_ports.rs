use empower_engine::node_graph::port::Port;
use empower_engine::NodeGraphKey;

use crate::graph_editor::display_node::DisplayNode; 

// @TODO, simplify this include
use crate::graph_editor::display_port::DisplayPort;

use super::NodeWidgetResponse;
use super::NodeWidgetResponseType;

pub fn show_output_port(
                        ui: &mut egui::Ui, 
                        display_node: &DisplayNode, 
                        display_port: &DisplayPort, 
                        output_port: &Port, 
                        graph_viewport_title: &String, 
                        node_key: &NodeGraphKey, 
                        port_key: &NodeGraphKey, 
                        node_widget_response: &mut Option<NodeWidgetResponse>, 
                        debug_mode: &bool
                    )
{
    let output_port_position = display_node.position + display_port.relative_position;

    let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
    let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

    // if ui.interact(output_port_rect, egui::Id::from( graph_title.clone() + "_output_port_" + output_port_key.to_string().as_str()), egui::Sense::click()).clicked()

    let output_port_response = ui.interact(output_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_output_port_" + port_key.to_string().as_str()), egui::Sense::click());
    if output_port_response.clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedOutputPort(*port_key) });
        // output_port_interaction(&mut graph_editor.empower_node_graph, graph_viewport, port_key, node_widget_response);
    }
    output_port_response.on_hover_text( format!("{:?}, {:?}", output_port.value, output_port.compatability ));

    let port_color = display_port.display_value.color;
    // let port_color;
    // match output_port.value 
    // {
    //     EmpowerData::Trigger =>
    //     {
    //         port_color = egui::Color32::WHITE;
    //     }
    //     _ =>
    //     {
    //         port_color = egui::Color32::YELLOW;
    //     }
    // }

    let output_port_text_offset = egui::Vec2 { x: -40.0, y: 0.0};
    let output_port_text_position = output_port_position + output_port_text_offset;
    let port_text = &display_port.display_value.text;

    ui.painter().text(
        output_port_text_position,
        egui::Align2::RIGHT_CENTER,
        port_text,
        egui::FontId::proportional(35.0),
        egui::Color32::WHITE,
    );

    ui.painter().circle(
        output_port_position,
        25.0,
        port_color,
        egui::Stroke::NONE,
    );


    if *debug_mode
    {
        ui.painter().text(
            output_port_position,
            egui::Align2::CENTER_CENTER,
            port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }
}
