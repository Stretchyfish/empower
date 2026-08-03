use empower_engine::node_graph::NodeAddress;

#[derive(Clone, Default)]
pub struct SessionCache
{
    pub instruction_highlighted_nodes: Option<NodeAddress>, 
    pub debug_highlighted_nodes: Vec<NodeAddress>,
    pub outputs: Vec<String>,
}

impl SessionCache
{
    pub fn new() -> Self
    {
        Self
        {
            instruction_highlighted_nodes: None,
            debug_highlighted_nodes: Vec::new(),
            outputs: Vec::new(),
        }
    }
}
