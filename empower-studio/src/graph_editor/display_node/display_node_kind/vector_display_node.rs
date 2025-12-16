use empower_engine::{PortValue, node_graph::node::{NodeKind, node_kind::VectorNode}, utility::alphabet_counter::AlphabetCounter};

use crate::graph_editor::DisplayPort;

use super::DisplayNodeKind;

#[derive(Clone)]
pub struct VectorDisplayNode
{
    pub show_n: bool,
    pub number_text: String,
}

impl DisplayNodeKind for VectorDisplayNode
{
    fn new() -> Box<dyn DisplayNodeKind> where
        Self: Sized {
        
        Box::new( Self { show_n: false, number_text: "2".to_string() } )
    }

    fn clone_box(&self) -> Box<dyn DisplayNodeKind> {

        Box::new( self.clone() )
    }

    fn node_size(&self, node_kind: &Box<dyn NodeKind>) -> egui::Vec2 {

        let vector_state = node_kind.as_any().downcast_ref::<VectorNode>().expect("Vector display node tried to unwrap a node_kind that is not the vector node kind");

        let n = vector_state.number_of_input_ports;
        let height = 300.0 + 70.0 * (n as f32 - 2.0); // @TODO, distance between nodes needs to be a global value
    
        egui::Vec2 { x: 450.0, y: height }
    }

    fn display_input_ports(&self, input_port_values: Vec<&PortValue>) -> Vec<DisplayPort> {

        // Probably don't need a second check here
        if input_port_values.len() < 2
        {
            panic!("Vector somehow got an impossible size");
        }

        let mut display_port_values = Vec::new();
        display_port_values.reserve(input_port_values.len());
        
        let mut alphabet_counter = AlphabetCounter::new(); 

        for input in input_port_values
        {
            let letter = alphabet_counter.next_letter();
            display_port_values.push(
                DisplayPort::new(
                                letter.to_string(), 
                                egui::pos2(0.0, 0.0), 
                                &input
                )
            );
        }

        display_port_values
    }

    fn state_size(&self) -> egui::Vec2 {
        egui::Vec2 { x: 175.0, y: 50.0 }
    }

    fn state_show(&mut self, ui: &mut egui::Ui, node_kind: &mut Box<dyn NodeKind>) -> bool {

        let vector_state = node_kind.as_any_mut().downcast_mut::<VectorNode>().expect("Vector display node tried to unwrap a node_kind that is not the vector node kind");

        let original_show_n = self.show_n.clone();
        let original_number_text = self.number_text.clone();
        let original_number_of_inputs = vector_state.number_of_input_ports;

        ui.horizontal(|ui|
        {

        ui.label("size: ");

        let mut number_to_show = vector_state.number_of_input_ports.to_string();
        if self.show_n
        {
            number_to_show = "n".to_string();
        }

        ui.menu_button(number_to_show, |ui|
        {
            if ui.button("2").clicked()
            {
                vector_state.number_of_input_ports = 2;
                self.show_n = false; 
            }
            if ui.button("3").clicked()
            {
                vector_state.number_of_input_ports = 3;
                self.show_n = false; 
            }
            if ui.button("4").clicked()
            {
                vector_state.number_of_input_ports = 4;
                self.show_n = false; 
            }
            if ui.button("5").clicked()
            {
                vector_state.number_of_input_ports = 5;
                self.show_n = false; 
            }
            if ui.button("6").clicked()
            {
                vector_state.number_of_input_ports = 6;
                self.show_n = false; 
            }
            if ui.button("n").clicked()
            {
                self.show_n = true; 
            }
        });

        if self.show_n
        {
            let parse_result = self.number_text.parse::<i32>();

            let parsed_successfully;

            let parsed_number = match parse_result
            {
                Ok( parsed_value) => 
                {
                    parsed_successfully = true;
                    parsed_value
                },
                Err(_) => 
                {
                    parsed_successfully = false;
                    vector_state.number_of_input_ports
                }
            };

            let mut text_edit_color = egui::Color32::WHITE;
            if !parsed_successfully
            {
                text_edit_color = egui::Color32::RED;
            }

            if parsed_number >= 2
            {
                vector_state.number_of_input_ports = parsed_number;
            }

            let text_edit = egui::TextEdit::singleline(&mut self.number_text)
            .char_limit(5)
            .text_color(text_edit_color);
            // .background_color(text_background_color);

            ui.add( text_edit);
        }

        });

        if original_show_n != self.show_n || 
            original_number_text != self.number_text ||
                original_number_of_inputs != vector_state.number_of_input_ports
        {
            return true;
        }

        false
    }
}