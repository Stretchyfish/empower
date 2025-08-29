use crate::node_graph::{node::node_value::NodeValue, port::{PortCompatability, PortValue}};

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

pub fn execute_print_node(inputs: Vec<&PortValue>) -> Vec<PortValue>
{
    println!("PRINTING: {}", inputs[1]);
    Vec::new()
}
