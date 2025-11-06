use crate::node_graph::node::{PortCompatability, PortValue};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct VectorState 
{
    pub number_of_input_ports: i32,
}

impl VectorState
{
    pub fn new() -> Self
    {
        Self 
        {  
            number_of_input_ports: 2,
        }
    }
}

pub fn get_name() -> &'static str
{
    "vector"
}

pub fn get_input_ports_compatabilities(state: &VectorState) -> Vec<PortCompatability>
{
    if state.number_of_input_ports < 2
    {
        panic!("Vector somehow got an impossible size");
    }

    vec![ PortCompatability::OneOf( vec![PortValue::Integer(0), PortValue::Float(0.0), PortValue::Text( String::new() ), PortValue::Bool( false ) ]); state.number_of_input_ports as usize]
}

pub fn get_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
        [
            PortCompatability::Exatch( PortValue::Vector( Vec::new() )),
        ]
    )
}

pub fn execute_vector_node(inputs: Vec<&PortValue>) -> Option<Vec<PortValue>>
{
    let mut port_values_vector = Vec::new();
    port_values_vector.reserve(inputs.len());

    for port_value in inputs
    {
        port_values_vector.push(port_value.clone());
    }

    Some( vec![ PortValue::Vector( port_values_vector ) ] )
}




