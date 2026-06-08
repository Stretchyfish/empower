use empower_engine::node_graph::NodeGraphKey;

use std::fmt;

pub struct UserState
{
    pub layer: String,
    pub action: UserAction,
}

impl fmt::Debug for UserState
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserState").field("layer", &self.layer).field("action", &self.action).finish()
    }
}

impl UserState
{
    pub fn from(layer_or_viewport: String, action: UserAction) -> Self
    {
        Self
        {
            layer: layer_or_viewport,
            action,
        }
    }
}

pub enum UserAction
{
    DraggingNodes { nodes: Vec<NodeGraphKey> },
}

impl fmt::Debug for UserAction
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DraggingNodes { nodes } => f.debug_struct("DraggingNodes").field("nodes", nodes).finish(),
        }
    }
}
