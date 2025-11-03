use crate::node_graph::port::{PortCompatability, PortValue};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct FilePathState
{
    pub path: String,
}

impl FilePathState
{
    pub fn new() -> Self
    {
        Self 
        { 
            path: String::new() 
        }
    }
}

pub fn get_name() -> &'static str
{
    "file path"
}

pub fn get_node_input_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::new()
}

pub fn get_node_output_ports_compatabilities() -> Vec<PortCompatability>
{
    Vec::from(
            [ 
                PortCompatability::Exatch( PortValue::Text( String::new() ))
            ]
        )
}

pub fn execute(state: &FilePathState) -> Option<Vec<PortValue>>
{
    Some(
    Vec::from(
        [
            PortValue::Text( state.path.clone() )
        ]
    ))
}
