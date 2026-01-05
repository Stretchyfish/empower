use empower_engine::NodeGraphKey;
use crate::actions::Action;
use crate::graph_editor::GraphEditor;

pub fn show_output_port(
                        ui: &mut egui::Ui, 
                        output_port_key: &NodeGraphKey,
                        graph_editor: &GraphEditor,
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
