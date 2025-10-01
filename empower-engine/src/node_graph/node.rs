use crate::node_graph::NodeGraphKey;

pub mod node_kind;
pub use node_kind::NodeKind;

pub mod node_handle;
pub use node_handle::NodeHandle;

pub struct Node
{
    pub key: NodeGraphKey,
    pub kind: NodeKind,
    pub input_port_keys: Vec<NodeGraphKey>,
    pub output_port_keys: Vec<NodeGraphKey>,
}

impl Node
{
    pub fn new(key: NodeGraphKey, kind: NodeKind, input_port_keys: Vec<NodeGraphKey>, output_port_keys: Vec<NodeGraphKey>) -> Self
    {
        Self 
        { 
            key, 
            kind, 
            input_port_keys, 
            output_port_keys 
        }
    }
}