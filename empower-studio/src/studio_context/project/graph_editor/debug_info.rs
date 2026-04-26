use empower_engine::NodeGraphKey;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DebugInfo
{
    pub debug_mode: bool,
    pub delay_between_each_execution: f32,
    pub show_keys: bool,
    pub show_node_execution_order: bool,
    pub node_execution_order: Vec<NodeGraphKey>,
}

impl DebugInfo
{
    pub fn new() -> Self
    {
        Self
        {
            debug_mode: true,
            delay_between_each_execution: 0.0,
            show_keys: false,
            show_node_execution_order: false,
            node_execution_order: Vec::new(),
        }
    }
}
