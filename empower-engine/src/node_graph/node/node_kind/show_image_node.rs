use std::thread;

use crate::node_graph::node::{PortCompatability, PortValue};

pub fn get_name() -> &'static str
{
    "show image"
}

pub fn get_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::Exatch( PortValue::Trigger ), 
            PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
        ]
    )
}

pub fn get_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::new()
}

pub fn execute(inputs: Vec<&PortValue>) -> Option<Vec<PortValue>>
{
    let input0 = inputs[1].clone();

    thread::spawn(move ||
    {
        let file_path = match input0
        {
            PortValue::Text(text) => text,
            _ => panic!("ERROR"),
        };

        let show_result = open::that(file_path);

        if show_result.is_err()
        {
            println!("{:?}", show_result.err());
        }
    });
    
    Some( Vec::new() )
}

// struct ImageShow
// {

// }

// impl ImageShow
// {
//     pub fn new() -> Self
//     {
//         Self {  }
//     }
// }

// impl eframe::App for ImageShow
// {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
//     {
//         println!("Does this appear 1?");
//         egui::CentralPanel::default()
//         .show(ctx, |ui|
//         {
//             println!("Does this appear 2?");
//             ui.heading("My egui Application");
//         });
//     }
// }
