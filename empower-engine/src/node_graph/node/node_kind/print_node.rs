use crate::{analyser::TextBuffer, node_graph::port::{PortCompatability, PortValue}};

use super::NodeKind2;

pub struct PrintNode
{

}

impl NodeKind2 for PrintNode
{
    fn name(&self) -> &'static str {
        "print"
    }

    fn input_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::from(
            [
                PortCompatability::Exatch( PortValue::Trigger ),
                PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ) ]),
            ]
        )
    }

    fn output_ports_compatabilities(&self) -> Vec<PortCompatability> {
        Vec::new()
    }

    fn execute(&self, inputs: Vec<&PortValue>, log: &mut TextBuffer) -> Vec<PortValue> {
        let text_to_print = inputs[1].to_string();
        println!("PRINTING: {}", inputs[1]);
        log.add_line(&text_to_print);
        
        Vec::new()
    }
}

pub fn get_print_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::Exatch( PortValue::Trigger ),
            PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ) ]),
        ]
    )
}

pub fn get_print_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::new()
}

pub fn execute_print_node(inputs: Vec<&PortValue>, log: &mut TextBuffer) -> Vec<PortValue>
{
    let text_to_print = inputs[1].to_string();
    println!("PRINTING: {}", inputs[1]);
    log.add_line(&text_to_print);
    
    Vec::new()
}
