use std::fmt;
use crate::node_graph::node::{PortCompatability, PortValue};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct NumberNodeState
{
    pub desired_value: NumberNodeValueKind
}

impl NumberNodeState
{
    pub fn new() -> Self
    {
        Self 
        {  
            desired_value: NumberNodeValueKind::Automatic
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum NumberNodeValueKind
{
    #[default] Automatic,
    Integer,
    Float,
}

impl fmt::Display for NumberNodeValueKind
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}

pub fn get_name() -> &'static str
{
    "number"
}

pub fn get_number_node_input_ports_compatabilities(state: &NumberNodeState) -> Vec<PortCompatability>
{
    match state.desired_value
    {
        NumberNodeValueKind::Automatic => Vec::from( [ PortCompatability::OneOf( vec!( PortValue::Integer(0), PortValue::Float(0.0)  ) ) ]),
        NumberNodeValueKind::Integer => Vec::from( [ PortCompatability::Exatch( PortValue::Integer(0) ) ]),
        NumberNodeValueKind::Float => Vec::from( [ PortCompatability::Exatch( PortValue::Float(0.0) ) ]),
    }
}

pub fn get_number_node_output_ports_compatabilities(state: &NumberNodeState) -> Vec<PortCompatability>
{
    match state.desired_value
    {
        NumberNodeValueKind::Automatic => Vec::from( [ PortCompatability::OneOf( vec!( PortValue::Integer(0), PortValue::Float(0.0)  ) ) ]),
        NumberNodeValueKind::Integer => Vec::from( [ PortCompatability::Exatch( PortValue::Integer(0) ) ]),
        NumberNodeValueKind::Float => Vec::from( [ PortCompatability::Exatch( PortValue::Float(0.0) ) ]),
    }
}

pub fn execute_number_node(inputs: Vec<&PortValue>) -> Option<Vec<PortValue>>
{
    let output_port_value = inputs[0].clone();
    Some( Vec::from( [ output_port_value ] ) )
}
