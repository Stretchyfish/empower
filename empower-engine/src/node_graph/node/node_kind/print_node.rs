use crate::{analyser::TextBuffer, node_graph::port::{PortCompatability, PortValue}};

pub fn get_name() -> &'static str
{
    "print"
}

pub fn get_print_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::Exatch( PortValue::Trigger ),
            PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ), PortValue::Vector( Vec::new() ) ])
       ]
    )
}

pub fn get_print_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::new()
}

pub fn execute_print_node(inputs: Vec<&PortValue>, log: &mut TextBuffer) -> Option<Vec<PortValue>> 
{
        let text_to_print = inputs[1].to_string();
        println!("PRINTING: {}", inputs[1]);
        log.add_line(&text_to_print);
        
        Some( Vec::new() )
}
