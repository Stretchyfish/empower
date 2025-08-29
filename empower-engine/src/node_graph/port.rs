use crate::node_graph::NodeGraphKey;

mod port_kind;
use port_kind::PortKind;

pub mod port_compatability;
pub use port_compatability::PortCompatability;

pub mod port_value;
pub use port_value::PortValue;

#[derive(Default, Clone)]
pub struct Port
{
    pub key: NodeGraphKey, 
    pub node_key: NodeGraphKey,
    pub kind: PortKind,
    pub compatability: PortCompatability,
    pub value: PortValue,
}

impl Port
{
    pub fn new(key: NodeGraphKey, node_key: NodeGraphKey, kind: PortKind, compatability: PortCompatability, value: PortValue) -> Self
    {
        Self 
        {  
            key,
            node_key,
            kind,
            compatability,
            value
        }
    }

    pub fn new_input_port(key: NodeGraphKey, node_key: NodeGraphKey, compatability: PortCompatability) -> Self
    {
        let value = compatability.get_initial_port_value();
        
        Self 
        {  
            key,
            node_key,
            kind: PortKind::Input,
            compatability,
            value,
        }
    }

    pub fn new_output_port(key: NodeGraphKey, node_key: NodeGraphKey, compatability: PortCompatability) -> Self
    {
        let value = compatability.get_initial_port_value();

        Self 
        {  
            key,
            node_key,
            kind: PortKind::Output,
            compatability,
            value,
        }
    }
}