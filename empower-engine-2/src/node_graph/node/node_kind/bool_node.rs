use crate::node_graph::port::{PortCompatability, PortValue};

pub fn get_bool_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
           PortCompatability::Exatch( PortValue::Bool( false )), 
        ]
    )
}

pub fn get_bool_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
           PortCompatability::Exatch( PortValue::Bool( false )), 
        ]
    )
}

pub fn execute_bool_node(inputs: Vec<&PortValue>) -> Vec<PortValue>
{
    let output_value = inputs[0].clone();
    Vec::from( [ output_value ] )
}
