pub mod node_type;
pub use node_type::NodeType;

pub mod node_handle;
pub use node_handle::NodeHandle;

use crate::node_graph::NodeGraphKey;

pub struct Node
{
    pub key: NodeGraphKey,
    pub node_type: NodeType,
    pub input_port_keys: Vec<NodeGraphKey>,
    pub output_port_keys: Vec<NodeGraphKey>,
}

impl Node
{
    pub fn new(key: NodeGraphKey, node_type: NodeType, input_port_keys: Vec<NodeGraphKey>, output_port_keys: Vec<NodeGraphKey>) -> Self
    {
        Self 
        { 
            key, 
            node_type, 
            input_port_keys, 
            output_port_keys 
        }
    }
}