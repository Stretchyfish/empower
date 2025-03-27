// use empower_engine::EmpowerEngine;

use std::collections::HashMap;

pub struct NodeGraph
{
    pub display_nodes: HashMap<i32, DisplayNode>,
    // pub engine: EmpowerEngine,
}

impl NodeGraph
{
    pub fn new() -> Self
    {
        Self
        {
            display_nodes: HashMap::new(),
            // engine: EmpowerEngine::new(),
        }
    }

    pub fn add_node(&mut self)
    {
        // let new_node_type = empower_engine::nodes::NodeType::IntVariable;
        // let engine_node_key = self.engine.add_node(new_node_type); // @TODO, add this back

        let new_node_key = self.display_nodes.len() as i32; // @TODO, change this back to the engine
        self.display_nodes.insert(new_node_key, DisplayNode::new_with_key(new_node_key));
    }

    pub fn add_node_at_position(&mut self, position: egui::Pos2)
    {
        // let new_node_type = empower_engine::nodes::NodeType::IntVariable;
        // let engine_node_key = self.engine.add_node(new_node_type); // @TODO, add this back

        let new_node_key = self.display_nodes.len() as i32; // @TODO, change this back to the engine
        self.display_nodes.insert(new_node_key, DisplayNode::new_with_key_and_position(new_node_key, position));
        
    }
}

pub struct DisplayNode
{
    pub key: i32,
    pub position: egui::Pos2,
}

impl DisplayNode
{
    pub fn new() -> Self
    {
        Self
        {
            key: 0,
            position: egui::Pos2::new(0.0, 0.0),
        }
    }

    pub fn new_with_key(new_key: i32) -> Self
    {
        Self
        {
            key: new_key,
            position: egui::Pos2::new(0.0, 0.0),
        }
    }

    pub fn new_with_key_and_position(new_key: i32, new_position: egui::Pos2) -> Self
    {
        Self
        {
            key: new_key,
            position: new_position,
        }
    }
}
