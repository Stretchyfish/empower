use egui;
use empower_node_graph::EmpowerKey;
use empower_node_graph::InputPort;
use empower_node_graph::OutputPort;

use super::display_node::DisplayNode;

pub struct DisplayPort
{
    // pub key: EmpowerKey,
    pub node_key: EmpowerKey,
    pub relative_position: egui::Vec2,
    pub value: String,
}

impl DisplayPort
{
    pub fn show_input_port(&mut self, ui: &mut egui::Ui, display_node: &DisplayNode, empower_input_port: &InputPort)
    {
        let input_port_position = display_node.position + self.relative_position;
        let input_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 

        let input_port_rect = egui::Rect::from_center_size(input_port_position, input_port_size);

        // if ui.interact(input_port_rect, egui::Id::from( graph_title.clone() + "_input_port_" + input_port_key.to_string().as_str()), egui::Sense::click()).clicked()
        if ui.interact(input_port_rect, egui::Id::from( String::from("test") + "_input_port_" + empower_input_port.key.to_string().as_str()), egui::Sense::click()).clicked()
        {
            // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::ClickedInputPort(input_port_key.clone()) } );
            println!("input port id from show function: {}", empower_input_port.key);
            // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::ClickedInputPort(*input_port_key)} );
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

        let mut text_edit = egui::TextEdit::singleline(&mut self.value)
        .char_limit(6)
        .font(egui::FontId::proportional(35.0));
        ui.put(input_port_value_box_rect, text_edit);
        
    }

    pub fn show_output_port(&mut self, ui: &mut egui::Ui, display_node: &DisplayNode, empower_output_port: &OutputPort)
    {
        let output_port_position = display_node.position + self.relative_position;

        let output_port_size = egui::Vec2 { x: 50.0, y: 50.0 }; 
        let output_port_rect= egui::Rect::from_center_size(output_port_position, output_port_size);

        // if ui.interact(output_port_rect, egui::Id::from( graph_title.clone() + "_output_port_" + output_port_key.to_string().as_str()), egui::Sense::click()).clicked()
        if ui.interact(output_port_rect, egui::Id::from( String::from("test") + "_output_port_" + empower_output_port.key.to_string().as_str()), egui::Sense::click()).clicked()
        {
            // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::ClickedOutputPort(output_port_key.clone()) } );
            println!("output port id in view function: {}", empower_output_port.key);
        }

        ui.painter().circle(
            output_port_position,
            25.0,
            egui::Color32::YELLOW,
            egui::Stroke::NONE,
        );
    }
}
