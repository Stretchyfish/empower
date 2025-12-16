use empower_engine::NodeGraphKey;

pub struct DebugInfo
{
    pub show_node_execution_order: bool,
    pub node_execution_order: Vec<NodeGraphKey>,
}

impl DebugInfo
{
    pub fn new() -> Self
    {
        Self
        {
            show_node_execution_order: false,
            node_execution_order: Vec::new(),
        }
    }
}