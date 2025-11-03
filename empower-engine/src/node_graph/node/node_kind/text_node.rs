use crate::node_graph::port::{PortCompatability, PortValue};

pub fn get_name() -> &'static str
{
    "text"
}

pub fn get_text_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
        ]
    )
}

pub fn get_text_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::Exatch( PortValue::Text( String::new() ) ), 
        ]
    )
}

pub fn execute_text_node(inputs: Vec<&PortValue>) -> Option<Vec<PortValue>> 
{
    let output_value = inputs[0].clone();
    Some(Vec::from( [ output_value ] ))
}
