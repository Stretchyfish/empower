use std::collections::HashMap;
use better_empower_engine::{NodeGraph, NodeGraphKey};

pub mod display_node;
pub use display_node::DisplayNode;

pub mod display_port;
pub use display_port::DisplayPort;

use crate::graph_editor::display_value::DisplayValue;

pub mod display_value;

pub struct GraphEditor
{
    pub node_graph: NodeGraph,
    pub display_nodes: HashMap<NodeGraphKey, DisplayNode>,
    pub display_input_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub display_output_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub selected_nodes: Vec<NodeGraphKey>,
}

impl GraphEditor
{
    pub fn new() -> Self
    {
       Self
       {
        node_graph: NodeGraph::new(),
        display_nodes: HashMap::new(),
        display_input_ports: HashMap::new(),
        display_output_ports: HashMap::new(),
        selected_nodes: Vec::new(),
       } 
    }

    pub fn add_node(&mut self, node_kind: &'static str, position: egui::Pos2) -> bool
    {
        let node_handle= self.node_graph.add_node(&node_kind); 
        let node = self.node_graph.get_node(&node_handle.node_key).unwrap();

        // Create display node
        let display_node_title = node.kind.name();
        let display_node = DisplayNode::new(display_node_title, position);


        let display_input_ports = display_node.display_kind.display_inputs(&node.kind);

        if node.input_port_keys.len() != display_input_ports.len()
        {
            panic!("Tried to create node, but display input ports doesn't match number of actual input ports");
        } 


        for index in 0..node.input_port_keys.len()
        {
            self.display_input_ports.insert(node.input_port_keys[index], display_input_ports[index].clone() );
        }

        // for input_port_key in &node.input_port_keys
        // {
        //     self.display_input_ports.insert(*input_port_key, display_port);
        // }



        self.display_nodes.insert(node_handle.node_key, display_node);



        // create display ports
        for input_port_key in &node.input_port_keys
        {
            let input_port = self.node_graph.get_input_port(input_port_key).unwrap();
            let display_port = DisplayPort::new("a", position); // The position is just defaulted here, because it will be correct in refresh display node
            self.display_input_ports.insert(*input_port_key, display_port);
        }

        // create display ports
        for output_port_key in &node.output_port_keys
        {
            let output_port = self.node_graph.get_input_port(output_port_key).unwrap();
            let display_port = DisplayPort::new("a", position); // The position is just defaulted here, because it will be correct in refresh display node
            self.display_output_ports.insert(*output_port_key, display_port);
        }

        // Correct position and etc to avoid unessesary code duplication
        self.refresh_display_node(node_handle.node_key);

        true
    }

    pub fn refresh_display_node(&mut self, node_key: NodeGraphKey)
    {
        let node = self.node_graph.get_node(&node_key).unwrap();
        let display_node = self.display_nodes.get_mut(&node_key).unwrap();
        let display_node_size = display_node.display_kind.node_size();
        let display_node_state_size = display_node.display_kind.state_size();

        let vertical_offset = 120.0;
        let mut input_ports_vertical_offset = vertical_offset;

        for input_port_key in &node.input_port_keys
        {
            let display_input_port = self.display_input_ports.get_mut(input_port_key).unwrap();
            display_input_port.position = display_node.position + egui::Vec2 { x: 0.0, y: input_ports_vertical_offset + display_node_state_size.y };

            input_ports_vertical_offset += 70.0; // Same as port_gap in node_widet_body (should be made global)
        }

        let mut output_ports_vertical_offset = vertical_offset;
        for output_port_key in &node.output_port_keys
        {
            let display_output_port = self.display_output_ports.get_mut(output_port_key).unwrap();
            display_output_port.position = display_node.position + egui::Vec2 { x: display_node_size.x, y: output_ports_vertical_offset + display_node_state_size.y };

            output_ports_vertical_offset += 70.0; // Same as port_gap in node_widet_body (should be made global)
        }
    }

    pub fn set_input_port_value(&mut self, port_key: &NodeGraphKey, display_value: &DisplayValue)
    {
        let input_port = self.node_graph.get_mut_input_port(port_key).unwrap();
        let display_input_port = self.display_input_ports.get_mut(port_key).unwrap();




    }
}