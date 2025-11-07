use crate::node_graph::NodeGraphKey;

#[derive(Clone)]
pub struct NodeHandle
{
    pub node_key: NodeGraphKey,
    pub input_port_keys: Vec<NodeGraphKey>,
    pub output_port_keys: Vec<NodeGraphKey>,
}

impl NodeHandle
{
    pub fn new(node_key: NodeGraphKey, input_port_keys: Vec<NodeGraphKey>, output_port_keys: Vec<NodeGraphKey>) -> Self
    {
        Self { node_key, input_port_keys, output_port_keys }
    }

    pub fn empty() -> Self
    {
        Self { node_key: 0, input_port_keys: Vec::new(), output_port_keys: Vec::new() }
    }
}