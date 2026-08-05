use std::collections::HashMap;

use empower_engine::node_graph::{NodeAddress, NodeGraphKey};
use crate::docking_space::viewport::graph_viewport::EditablePortValue;

#[derive(Clone, Default)]
pub struct SessionCache
{
    pub cached_port_positions: HashMap<NodeGraphKey, egui::Pos2>,
    pub cached_editable_port_value: HashMap<NodeGraphKey, EditablePortValue>, 
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
            cached_port_positions: HashMap::new(),
            cached_editable_port_value: HashMap::new(),
            instruction_highlighted_nodes: None,
            debug_highlighted_nodes: Vec::new(),
            outputs: Vec::new(),
        }
    }
}
