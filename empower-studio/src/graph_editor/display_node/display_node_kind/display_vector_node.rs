use core::num;

use egui::{self, output};

use crate::graph_editor::display_port::display_port_value::DisplayPortValue;
use empower_engine::node_graph::{node::{node_kind::VectorState, NodeKind}, port::PortValue};

#[derive(Clone, PartialEq, Eq)]
pub struct DisplayVectorState
{
    pub show_n: bool,
    pub number_text: String,
}

impl DisplayVectorState
{
    pub fn new() -> Self
    {
        Self 
        { 
            show_n: false, 
            number_text: "2".to_string() 
        }
    }
}

pub fn get_display_node_size( state: &VectorState ) -> egui::Vec2
{
    let n = state.number_of_input_ports;
    let height = 300.0 + 60.0 * (n as f32 - 2.0);
 
    egui::Vec2 { x: 450.0, y: height }
}

pub fn get_display_node_state_size() -> egui::Vec2
{
    egui::Vec2 { x: 175.0, y: 50.0 }
}

pub fn get_display_node_input_ports(inputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    // Probably don't need a second check here
    if inputs.len() < 2
    {
        panic!("Vector somehow got an impossible size");
    }

    let mut display_port_values = Vec::new();
    display_port_values.reserve(inputs.len());

    for input in inputs
    {
        display_port_values.push( DisplayPortValue::from("a".to_string(), input ));
    }

    display_port_values

    // @TODO, find a better approach for this?
    // match inputs.len()
    // {
    //     2 =>
    //     {
    //         vec![ 
    //             DisplayPortValue::from("a".to_string(), inputs[0]),
    //             DisplayPortValue::from("b".to_string(), inputs[1]),
    //         ]
    //     }
    //     3 =>
    //     {
    //         vec![ 
    //             DisplayPortValue::from("a".to_string(), inputs[0]),
    //             DisplayPortValue::from("b".to_string(), inputs[1]),
    //             DisplayPortValue::from("c".to_string(), inputs[1]),
    //         ]
    //     }
    //     4 =>
    //     {
    //         vec![ 
    //             DisplayPortValue::from("a".to_string(), inputs[0]),
    //             DisplayPortValue::from("b".to_string(), inputs[1]),
    //             DisplayPortValue::from("c".to_string(), inputs[1]),
    //             DisplayPortValue::from("d".to_string(), inputs[1]),
    //         ]
    //     }
    //     5 =>
    //     {
    //         vec![ 
    //             DisplayPortValue::from("a".to_string(), inputs[0]),
    //             DisplayPortValue::from("b".to_string(), inputs[1]),
    //             DisplayPortValue::from("c".to_string(), inputs[1]),
    //             DisplayPortValue::from("d".to_string(), inputs[1]),
    //             DisplayPortValue::from("e".to_string(), inputs[1]),
    //         ]
    //     }
    //     6 =>
    //     {
    //         vec![ 
    //             DisplayPortValue::from("a".to_string(), inputs[0]),
    //             DisplayPortValue::from("b".to_string(), inputs[1]),
    //             DisplayPortValue::from("c".to_string(), inputs[1]),
    //             DisplayPortValue::from("d".to_string(), inputs[1]),
    //             DisplayPortValue::from("e".to_string(), inputs[1]),
    //             DisplayPortValue::from("f".to_string(), inputs[1]),
    //         ]
    //     }
    //     _ => panic!("Vector somehow got an impossible size"),
    // }
}

pub fn get_display_node_output_ports(outputs: Vec<&PortValue>) -> Vec<DisplayPortValue>
{
    vec![ 
        DisplayPortValue::from("a".to_string(), outputs[0]),
    ]
}

pub fn show(ui: &mut egui::Ui, state: &mut VectorState, display_state: &mut DisplayVectorState)
{
    ui.horizontal(|ui|
    {
        ui.label("size: ");

        let mut number_to_show = state.number_of_input_ports.to_string();
        if display_state.show_n
        {
            number_to_show = "n".to_string();
        }

        ui.menu_button(number_to_show, |ui|
        {
            if ui.button("2").clicked()
            {
                state.number_of_input_ports = 2;
                display_state.show_n = false; 
            }
            if ui.button("3").clicked()
            {
                state.number_of_input_ports = 3;
                display_state.show_n = false; 
            }
            if ui.button("4").clicked()
            {
                state.number_of_input_ports = 4;
                display_state.show_n = false; 
            }
            if ui.button("5").clicked()
            {
                state.number_of_input_ports = 5;
                display_state.show_n = false; 
            }
            if ui.button("6").clicked()
            {
                state.number_of_input_ports = 6;
                display_state.show_n = false; 
            }
            if ui.button("n").clicked()
            {
                display_state.show_n = true; 
            }
        });

        if display_state.show_n
        {
            let parse_result = display_state.number_text.parse::<i32>();

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
                    state.number_of_input_ports
                }
            };


            let mut text_edit_color = egui::Color32::WHITE;
            if !parsed_successfully
            {
                text_edit_color = egui::Color32::RED;
            }

            if parsed_number >= 2
            {
                state.number_of_input_ports = parsed_number;
            }

            let text_edit = egui::TextEdit::singleline(&mut display_state.number_text)
            .char_limit(5)
            .text_color(text_edit_color);
            // .background_color(text_background_color);

            ui.add( text_edit);
            
        }
    });
}

