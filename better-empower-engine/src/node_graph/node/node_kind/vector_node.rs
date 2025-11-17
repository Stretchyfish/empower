use crate::node_graph::node::{NodeFunction, port::PortCompatability, port::PortValue};

use super::NodeKind;

#[derive(Clone)]
pub struct VectorNode
{
    show_n: bool, // This bool is tied to visualization, probably can't get rid off
    number_text: String, // Same for this
    number_of_input_ports: i32,
}

impl NodeKind for VectorNode
{
    fn new() -> Box<dyn NodeKind> where
        Self: Sized {
        
        Box::new( Self { number_of_input_ports: 2, show_n: false, number_text: String::new() } )
    }

    fn name(&self) -> &'static str {
        "vector"
    }

    fn clone_box(&self) -> Box<dyn NodeKind> {
        Box::new( self.clone() )
    }

    fn function(&self) -> NodeFunction {
        NodeFunction::Instant
    }

    fn input_compatabilities(&self) -> Vec<PortCompatability> {

        if self.number_of_input_ports < 2
        {
            panic!("Vector somehow got an impossible size");
        }

        vec![ PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ) ]); self.number_of_input_ports as usize]
    }

    fn output_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Vector( Vec::new() )),
            ]
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn state(&mut self, ui: &mut egui::Ui) {

        ui.horizontal(|ui|
        {
            ui.horizontal_centered(|ui|
            {
                ui.label("size: ");

                let mut number_to_show = self.number_of_input_ports.to_string();
                if self.show_n
                {
                    number_to_show = "n".to_string();
                }

                ui.menu_button(number_to_show, |ui|
                {
                    if ui.button("2").clicked()
                    {
                        self.number_of_input_ports = 2;
                        self.show_n = false; 
                    }
                    if ui.button("3").clicked()
                    {
                        self.number_of_input_ports = 3;
                        self.show_n = false; 
                    }
                    if ui.button("4").clicked()
                    {
                        self.number_of_input_ports = 4;
                        self.show_n = false; 
                    }
                    if ui.button("5").clicked()
                    {
                        self.number_of_input_ports = 5;
                        self.show_n = false; 
                    }
                    if ui.button("6").clicked()
                    {
                        self.number_of_input_ports = 6;
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
                            self.number_of_input_ports
                        }
                    };

                    let mut text_edit_color = egui::Color32::WHITE;
                    if !parsed_successfully
                    {
                        text_edit_color = egui::Color32::RED;
                    }

                    if parsed_number >= 2
                    {
                        self.number_of_input_ports = parsed_number;
                    }

                    let text_edit = egui::TextEdit::singleline(&mut self.number_text)
                    .char_limit(5)
                    .text_color(text_edit_color);
                    // .background_color(text_background_color);

                    ui.add( text_edit);
                    
                }
            });
        });
    }

    fn setup(&mut self, inputs: Vec<&PortValue>) -> Option<Vec<PortValue>> {
        let mut port_values_vector = Vec::new();
        port_values_vector.reserve(inputs.len());

        for port_value in inputs
        {
            port_values_vector.push(port_value.clone());
        }

        Some( vec![ PortValue::Vector( port_values_vector ) ] )
 
    }

    fn update(&mut self) -> Option<Vec<PortValue>> {
        todo!()
    }

    fn execute(&mut self, _: &mut egui::Ui) {
        todo!()
    }
}