use crate::node_graph::NodeGraphKey;

pub struct NodeHandle
{
    pub node_key: NodeGraphKey,
    pub input_port_keys: Vec<NodeGraphKey>,
    pub output_port_keys: Vec<NodeGraphKey>,
}
