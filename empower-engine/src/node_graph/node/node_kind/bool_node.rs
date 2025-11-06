use crate::node_graph::node::{PortCompatability, PortValue};

pub fn get_name() -> &'static str
{
    "bool"
}

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

pub fn execute_bool_node(inputs: Vec<&PortValue>) -> Option<Vec<PortValue>> 
{
    let output_value = inputs[0].clone();
    Some( Vec::from( [ output_value ] ) )
}
