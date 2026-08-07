use super::NodeGraphKey;

pub mod node_kind;
pub use node_kind::NodeKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Node
{
    pub position: egui::Pos2,
    pub input_port_keys: Vec<NodeGraphKey>,
    pub output_port_keys: Vec<NodeGraphKey>,
    pub kind: NodeKind,
}

impl Node
{
    pub fn new(position: egui::Pos2, input_port_keys: Vec<NodeGraphKey>, output_port_keys: Vec<NodeGraphKey>, kind: NodeKind) -> Self
    {
        Self
        {
            position,
            input_port_keys,
            output_port_keys,
            kind
        }
    }
}
