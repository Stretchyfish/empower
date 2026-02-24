use empower_engine::node_graph::node::port::Port;
use crate::actions::Action;
use crate::studio_context::project::graph_editor::DisplayPort;

pub fn show_output_port(
                        ui: &mut egui::Ui, 
                        node_position: &egui::Pos2,
                        output_port: &Port,
                        display_output_port: &DisplayPort,
                        graph_viewport_title: &String, 
                        debug_mode: &bool,
                        action_queue: &mut Vec<Action>,
                    )
{
    let output_port_position = *node_position + display_output_port.relative_position;

    // @TODO, make this const
    let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
    let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

    let output_port_response = ui.interact(output_port_rect, egui::Id::from( graph_viewport_title.to_owned() + "_output_port_" + output_port.key.to_string().as_str()), egui::Sense::click());
    if output_port_response.clicked()
    {
        action_queue.push( Action::ClickedOutputPort { port_key: output_port.key });
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
            output_port.key.to_string(),
            egui::FontId::proportional(25.0),
            egui::Color32::BLACK,
        );
    }

    let output_port_text_font_size = 35.0;
    let output_port_text_offset = egui::Vec2 { x: -40.0, y: 0.0};
    let output_port_text_position = output_port_position + output_port_text_offset;

    ui.painter().text(
        output_port_text_position,
        egui::Align2::RIGHT_CENTER,
        &display_output_port.text,
        egui::FontId::proportional(output_port_text_font_size),
        egui::Color32::WHITE,
    );
}
