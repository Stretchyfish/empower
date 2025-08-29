use crate::node_graph::NodeGraphKey;

pub mod node_kind;
pub use node_kind::NodeKind;

pub mod node_handle;
pub use node_handle::NodeHandle;

mod node_value;
use node_value::NodeValue;

pub struct Node
{
    pub key: NodeGraphKey,
    pub kind: NodeKind,
    pub value: NodeValue,
    pub input_port_keys: Vec<NodeGraphKey>,
    pub output_port_keys: Vec<NodeGraphKey>,
}

impl Node
{
    pub fn new(key: NodeGraphKey, kind: NodeKind, input_port_keys: Vec<NodeGraphKey>, output_port_keys: Vec<NodeGraphKey>) -> Self
    {
        let value = match kind
        {
            NodeKind::Number => NodeValue::NumberState,
            _ => NodeValue::None,
        };

        Self 
        { 
            key, 
            kind, 
            value,
            input_port_keys, 
            output_port_keys 
        }
    }
}