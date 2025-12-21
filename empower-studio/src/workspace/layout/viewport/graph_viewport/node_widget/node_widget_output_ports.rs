use empower_engine::{NodeGraphKey, node_graph::node::port::Port};
use crate::actions::Action;
use crate::graph_editor::GraphEditor;
use crate::graph_editor::display_node::DisplayPort;

use super::NodeWidgetResponse;
use super::NodeWidgetResponseType;

pub fn show_output_port(
                        ui: &mut egui::Ui, 
                        display_port: &DisplayPort, 
                        output_port: &Port, 
                        graph_viewport_title: &String, 
                        node_key: &NodeGraphKey, 
                        port_key: &NodeGraphKey, 
                        node_widget_response: &mut Option<NodeWidgetResponse>, 
                        debug_mode: &bool
                    )
{
    let output_port_position = display_port.position;
    let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
    let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

    let output_port_response = ui.interact(output_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_output_port_" + port_key.to_string().as_str()), egui::Sense::click());
    if output_port_response.clicked()
    {
        *node_widget_response = Some( NodeWidgetResponse { key: *node_key, kind: NodeWidgetResponseType::ClickedOutputPort(*port_key) });
    }
    output_port_response.on_hover_text( format!("{:?}, {:?}", output_port.value, output_port.compatability ));

    let port_color = display_port.color;

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

pub fn show_output_port2(
                        ui: &mut egui::Ui, 
                        output_port_key: &NodeGraphKey,
                        graph_editor: &mut GraphEditor,
                        graph_viewport_title: &String, 
                        debug_mode: &bool,
                        action_queue: &mut Vec<Action>,
                    )
{
    let output_port = graph_editor.node_graph.get_output_port(output_port_key).unwrap();
    let display_output_port = graph_editor.display_output_ports.get(output_port_key).unwrap();
  
    let output_port_position = display_output_port.position;

    // @TODO, make this const
    let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
    let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

    let output_port_response = ui.interact(output_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_output_port_" + output_port_key.to_string().as_str()), egui::Sense::click());
    if output_port_response.clicked()
    {
        action_queue.push( Action::ClickedOutputPort { port_key: *output_port_key });
    }
    output_port_response.on_hover_text( format!("{:?}, {:?}", output_port.value, output_port.compatability ));

    ui.painter().circle(
        output_port_position,
        25.0,
        display_output_port.color,
        egui::Stroke::NONE,
    );

    if *debug_mode
    {
        ui.painter().text(
            output_port_position,
            egui::Align2::CENTER_CENTER,
            output_port_key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }
}
