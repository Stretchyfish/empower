use egui;
use empower_node_graph::EmpowerKey;
use empower_node_graph::Node;
use std::collections::HashMap;

mod display_node;
use display_node::DisplayNode;

mod display_port;
use display_port::DisplayPort;

pub struct DisplayNodeGraph
{
    pub display_nodes: HashMap<EmpowerKey, DisplayNode>,
    pub display_input_ports: HashMap<EmpowerKey, DisplayPort>,
    pub display_output_ports: HashMap<EmpowerKey, DisplayPort>,
}

impl DisplayNodeGraph
{
    pub fn new() -> Self
    {
        Self
        {
            display_nodes: HashMap::new(),
            display_input_ports: HashMap::new(),
            display_output_ports: HashMap::new(),
        }
    }

    pub fn add_display_node(&mut self, empower_node: &Node, position: egui::Pos2)
    {
        let display_node_title = empower_node.node_type.to_string();
        let display_node_size = egui::Vec2 { x: 450.0, y: 200.0}; // @TODO, change this depending on node type
        
        let port_gap = 50.0;
        let mut input_port_offset = 135.0;

        for input_port_keys in empower_node.input_port_keys.iter()
        {
            let new_display_port = DisplayPort { node_key: empower_node.key.clone(), relative_position: egui::Vec2::new(0.0, input_port_offset), value: String::new() }; 
            self.display_input_ports.insert(input_port_keys.clone(), new_display_port);

            input_port_offset += port_gap;
        }

        let mut output_port_offset = 135.0;
        for output_port_key in empower_node.output_port_keys.iter()
        {
            let new_display_port = DisplayPort { node_key: empower_node.key.clone(), relative_position: egui::Vec2::new(display_node_size.x, output_port_offset), value: String::new() }; 
            self.display_output_ports.insert(output_port_key.clone(), new_display_port);

            output_port_offset += port_gap;
        }

        let new_display_node = DisplayNode::new( display_node_title, position, display_node_size );
        self.display_nodes.insert(empower_node.key.clone(), new_display_node );
    }
}
