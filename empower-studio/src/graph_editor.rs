use empower_node_graph::EmpowerKey;
use empower_node_graph::EmpowerNodeGraph;
use empower_node_graph::Node;
use empower_node_graph::NodeType;

use std::collections::HashMap;

pub mod display_node;
use display_node::DisplayNode;

pub mod display_port;
use display_port::DisplayPort;

#[derive(Default, Clone)]
pub struct GraphEditor
{
    pub display_nodes: HashMap<EmpowerKey, DisplayNode>, // @TODO, consider simplifying this to be a display node graph instead of seperate hash tables
    pub display_input_ports: HashMap<EmpowerKey, DisplayPort>,
    pub display_output_ports: HashMap<EmpowerKey, DisplayPort>,
    pub empower_node_graph: EmpowerNodeGraph, // @TODO, should make these private
    pub selected_nodes: Vec<EmpowerKey>,
}

impl GraphEditor
{
    pub fn new() -> Self
    {
        let mut graph_editor = Self
        {
            display_nodes: HashMap::new(),
            display_input_ports: HashMap::new(),
            display_output_ports: HashMap::new(),
            empower_node_graph: EmpowerNodeGraph::new(),
            selected_nodes: Vec::new(),
        };


        let new_node_type = NodeType::IntegerVariable;
        graph_editor.add_node(new_node_type, egui::Pos2 { x: 0.0, y: 0.0 });

        graph_editor
    }

    pub fn add_node(&mut self, new_node_type: NodeType, position: egui::Pos2)
    {
        let empower_node_key: EmpowerKey = self.empower_node_graph.add_node(new_node_type); 

        if self.display_nodes.contains_key(&empower_node_key) // @TODO, simplify these calls
        {
            println!("ERROR, attempted to add engine node key already in node graph (denied)"); // This should never happen, only if something wrongly implemented
            return;
        }

        let empower_node = self.empower_node_graph.nodes.get(&empower_node_key).unwrap(); // This should never fail @TODO, consider simplifying this call

        let display_node_title = empower_node.node_type.to_string();
        let display_node_size = egui::Vec2 { x: 450.0, y: 200.0}; // @TODO, change this depending on node type
        
       
        let port_gap = 50.0;
        let mut input_port_offset = 135.0;


        for input_port_key in empower_node.input_port_keys.iter()
        {
            let display_port_value = self.empower_node_graph.input_ports.get(input_port_key).unwrap().value.to_string();
            let new_display_port = DisplayPort { node_key: empower_node.key.clone(), relative_position: egui::Vec2::new(0.0, input_port_offset), value: display_port_value, value_text_valid: true }; 
            self.display_input_ports.insert(input_port_key.clone(), new_display_port);

            input_port_offset += port_gap;
        }

        let mut output_port_offset = 135.0;
        for output_port_key in empower_node.output_port_keys.iter()
        {
            // @TODO, figure out if setting display port values is needed for output
            let new_display_port = DisplayPort { node_key: empower_node.key.clone(), relative_position: egui::Vec2::new(display_node_size.x, output_port_offset), value: String::new(), value_text_valid: false }; 
            self.display_output_ports.insert(output_port_key.clone(), new_display_port);

            output_port_offset += port_gap;
        }

        let new_display_node = DisplayNode::new( display_node_title, position, display_node_size );
        self.display_nodes.insert(empower_node.key.clone(), new_display_node );
    }

    // @TODO, figure out where the best place to put these functions are
    // This will very likely need to be removed!
    pub fn get_input_port_value(&mut self, port_key: EmpowerKey) -> String
    {
        // @TODO, make a check here
        let input_port = self.empower_node_graph.input_ports.get(&port_key).unwrap();
        let input_port_data = input_port.value;

        input_port_data.to_string()
    }

    pub fn set_input_port_value() -> bool
    {
        false
    }

    pub fn refresh_display_port_values(&mut self)
    {
        for input_port_key in self.empower_node_graph.input_ports.keys()
        {
            let input_port = self.empower_node_graph.input_ports.get(input_port_key).unwrap();
            let input_port_value_as_text = input_port.get_value_as_string();

            let display_port = self.display_input_ports.get_mut(input_port_key).unwrap();
            display_port.value = input_port_value_as_text;
        }
    }
}
