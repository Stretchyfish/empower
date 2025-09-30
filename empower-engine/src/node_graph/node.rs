use crate::node_graph::NodeGraphKey;

pub mod node_kind;
pub use node_kind::NodeKind2;

pub mod node_handle;
pub use node_handle::NodeHandle;

mod node_registry;
pub use node_registry::NODE_REGISTRY; // @TODO, consider improving these imports

pub struct Node
{
    pub key: NodeGraphKey,
    pub kind: NodeKind2,
    pub input_port_keys: Vec<NodeGraphKey>,
    pub output_port_keys: Vec<NodeGraphKey>,
}

impl Node
{
    pub fn new(key: NodeGraphKey, kind: NodeKind2, input_port_keys: Vec<NodeGraphKey>, output_port_keys: Vec<NodeGraphKey>) -> Self
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