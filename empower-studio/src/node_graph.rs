use empower_engine::{EmpowerEngine, EmpowerKey};

use std::collections::HashMap;

pub struct NodeGraph
{
    pub display_nodes: HashMap<EmpowerKey, DisplayNode>,
    pub display_input_ports: HashMap<EmpowerKey, DisplayPort>,
    pub display_output_ports: HashMap<EmpowerKey, DisplayPort>,
    pub engine: EmpowerEngine,
}

impl NodeGraph
{
    pub fn new() -> Self
    {
        Self
        {
            display_nodes: HashMap::new(),
            display_input_ports: HashMap::new(),
            display_output_ports: HashMap::new(),
            engine: EmpowerEngine::new(),
        }
    }

    pub fn add_node(&mut self, position: egui::Pos2) -> EmpowerKey 
    {
        let new_node_type = empower_engine::NodeType::IntegerVariable;
        // let new_node_type = empower_engine::nodes::NodeType::IntVariable;
        let engine_node_key: EmpowerKey = self.engine.add_node(new_node_type); 

        if self.display_nodes.contains_key(&engine_node_key)
        {
            println!("ERROR, attempted to add engine node key already in node graph (denied)");
            return engine_node_key;
        }

        let node_size = egui::Vec2 { x: 200.0, y: 100.0}; // @TODO, change this depending on node type

        let port_gap = 50.0;
        let mut input_port_offset = 50.0;

        for input_port_keys in self.engine.nodes.get(&engine_node_key).unwrap().input_port_keys.iter()
        {
            let new_display_port = DisplayPort { key: input_port_keys.clone(), node_key: engine_node_key.clone(), relative_position: egui::Vec2::new(0.0, input_port_offset) }; 
            self.display_input_ports.insert(input_port_keys.clone(), new_display_port);

            input_port_offset += port_gap;
        }

        let mut output_port_offset = 50.0;
        for output_port_key in self.engine.nodes.get(&engine_node_key).unwrap().output_port_keys.iter()
        {
            let new_display_port = DisplayPort { key: output_port_key.clone(), node_key: engine_node_key.clone(), relative_position: egui::Vec2::new(node_size.x, output_port_offset) }; 
            self.display_output_ports.insert(output_port_key.clone(), new_display_port);

            output_port_offset += port_gap;
        }
        
        // let new_node_key = self.display_nodes.len() as i32;
        self.display_nodes.insert(engine_node_key, DisplayNode::new_with_key_and_position(engine_node_key, position, node_size));

        engine_node_key
    }

    pub fn add_connection(&mut self, input_port_key: EmpowerKey, output_port_key: EmpowerKey)
    {
        self.engine.add_connection(input_port_key, output_port_key);
    }
}

pub struct DisplayNode
{
    pub key: EmpowerKey,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
}

impl DisplayNode
{
    pub fn new() -> Self
    {
        Self
        {
            key: 0,
            position: egui::Pos2::new(0.0, 0.0),
            size: egui::Vec2 { x: 100.0, y: 100.0 }
        }
    }

    pub fn new_with_key(new_key: EmpowerKey) -> Self
    {
        Self
        {
            key: new_key,
            position: egui::Pos2::new(0.0, 0.0),
            size: egui::Vec2 { x: 100.0, y: 100.0 }
        }
    }

    pub fn new_with_key_and_position(new_key: EmpowerKey, new_position: egui::Pos2, new_size: egui::Vec2) -> Self
    {
        Self
        {
            key: new_key,
            position: new_position,
            size: new_size        
        }
    }
}

pub struct DisplayPort
{
    pub key: EmpowerKey,
    pub node_key: EmpowerKey,
    pub relative_position: egui::Vec2,
}
