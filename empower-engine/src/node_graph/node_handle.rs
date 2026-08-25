use serde::{Deserialize, Serialize};

use crate::node_graph::NodeGraphKey;

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeHandle // @TODO, maybe combine node address and node handle?
{
    pub node_key: NodeGraphKey,
    pub input_port_keys: Vec<NodeGraphKey>,
    pub output_port_keys: Vec<NodeGraphKey>,
}
