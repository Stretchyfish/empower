use empower_engine::PortValue;
use empower_engine::node_graph::Variables;
use empower_engine::node_graph::node::NodeKind;
use empower_engine::node_graph::node::node_kind::{WaitNode, WaitTimeIntervals};
use crate::studio_context::project::graph_editor::display_node::DisplayNodeStateResponse;

use super::DisplayNodeKind;
use super::super::DisplayPort;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DisplayWaitNode
{
    pub number_text: String,
}

#[typetag::serde]
impl DisplayNodeKind for DisplayWaitNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {

        Box::new( Self { number_text: String::from("1") } )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {
        Box::new( self.clone() )
    }

    fn node_size(&self, _: &Box<dyn NodeKind>) -> egui::Vec2 {
        egui::Vec2 { x: 350.0, y: 230.0 }
    }

    fn display_input_ports(&self, input_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {
        vec![
            DisplayPort::new(
                        "A".to_string(), 
                        egui::vec2(0.0, 0.0), 
                        &input_port_values[0]
                    )
        ]
    }

    fn display_output_ports(&self, output_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {
        vec![
            DisplayPort::nothing(egui::vec2(0.0, 0.0), &output_port_values[0])
        ]
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 300.0, y: 50.0 }
    }

    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>, _: &Variables) -> DisplayNodeStateResponse {

        let wait_node = node_kind.as_any_mut().downcast_mut::<WaitNode>().expect("Wait display node tried to unwrap a node_kind that is not the wait node kind");


        ui.horizontal(|ui|
        {
            let button_text = format!("{}", wait_node.time_interval_type);
            ui.menu_button(button_text, |ui|
            {
                if ui.button("Miliseconds").clicked()
                {
                    wait_node.time_interval_type = WaitTimeIntervals::Miliseconds;
                }
                if ui.button("Seconds").clicked()
                {
                    wait_node.time_interval_type = WaitTimeIntervals::Seconds;
                }
                if ui.button("Minutes").clicked()
                {
                    wait_node.time_interval_type = WaitTimeIntervals::Minutes;
                }
                if ui.button("Hours").clicked()
                {
                    wait_node.time_interval_type = WaitTimeIntervals::Hours;
                }
            });

            // @TODO, modify this to allow for floating numbers?

            let text_edit_color;
            match self.number_text.parse::<u64>()
            {
                Ok( parsed_value) => 
                {
                    text_edit_color = egui::Color32::WHITE;
                    wait_node.wait_time = parsed_value;
                },
                Err(_) => 
                {
                    text_edit_color = egui::Color32::RED;
                }
            };
            
            let text_edit = egui::TextEdit::singleline(&mut self.number_text)
            .char_limit(5)
            .text_color(text_edit_color);

            ui.add( text_edit);
        });

        DisplayNodeStateResponse::NoChange
    }
}
