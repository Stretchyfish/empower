use crate::node_graph::port::{PortCompatability, PortValue};

pub fn get_name() -> &'static str
{
    "start"
}

pub fn get_start_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::new()
}

pub fn get_start_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
            [ 
                PortCompatability::Exatch( PortValue::Trigger )
            ]
        )
}

pub fn execute_start_node() -> Option<Vec<PortValue>>
{
    println!("Ran start node");
    Some( Vec::from( [ PortValue::Trigger ] ) )
}
