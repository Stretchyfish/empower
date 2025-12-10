use std::collections::HashMap;
use better_empower_engine::node_graph::node::NodeKind;
use better_empower_engine::{NodeGraph, NodeGraphKey};

pub mod display_node;
pub use display_node::DisplayNode;
pub use display_node::DisplayPort;
pub use display_node::DisplayValue;

mod debug_info;
use debug_info::DebugInfo;

use crate::graph_editor::display_node::DisplayNodeKind;

pub struct GraphEditor
{
    pub node_graph: NodeGraph,
    pub display_nodes: HashMap<NodeGraphKey, DisplayNode>,
    pub display_input_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub display_output_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub selected_nodes: Vec<NodeGraphKey>,
    pub debug_info: DebugInfo,
}

impl GraphEditor
{
    pub fn new() -> Self
    {
       let mut graph_editor = Self
       {
        node_graph: NodeGraph::new(),
        display_nodes: HashMap::new(),
        display_input_ports: HashMap::new(),
        display_output_ports: HashMap::new(),
        selected_nodes: Vec::new(),
        debug_info: DebugInfo::new(),
       };

        let start_node_left_offset = egui::Pos2 { x: -1700.0, y: -165.0 / 2.0 }; // Half the center nodes height and oriented left

        graph_editor.add_node("start", start_node_left_offset);

        graph_editor
    }

    pub fn add_node(&mut self, node_kind: &'static str, position: egui::Pos2) -> bool
    {
        let node_handle= self.node_graph.add_node(&node_kind); 
        let node = self.node_graph.get_node(&node_handle.node_key).unwrap();

        // Create display node
        let display_node_title = node.kind.name();
        let display_node = DisplayNode::new(display_node_title, position);

        // Create display input ports
        let node_input_port_values = self.node_graph.get_node_input_port_values(&node.key);
        let display_input_ports = display_node.display_kind.display_input_ports(node_input_port_values);

        if node.input_port_keys.len() != display_input_ports.len()
        {
            panic!("Tried to create node, but display input ports doesn't match number of actual input ports");
        } 

        for index in 0..node.input_port_keys.len()
        {
            self.display_input_ports.insert(node.input_port_keys[index], display_input_ports[index].clone() );
        }

        // create display output ports
        for output_port_key in &node.output_port_keys
        {
            let output_port = self.node_graph.get_output_port(output_port_key).unwrap();
            let display_port = DisplayPort::nothing(position, &output_port.value); // The position is just defaulted here, because it will be correct in refresh display node
            self.display_output_ports.insert(*output_port_key, display_port);
        }
        self.display_nodes.insert(node_handle.node_key, display_node);

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
            let input_port = self.node_graph.get_input_port(input_port_key).unwrap();
            let display_input_port = self.display_input_ports.get_mut(input_port_key).unwrap();

            // Updating the display value is done in here to have one function with update behavior, this 
            // has the side effect of updating the display value to the last valid valid if the box is moved
            display_input_port.value = DisplayValue::from_port_value(&input_port.value);
            display_input_port.convertable = true;

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

    // @TODO, consider if functions here can be combined to reduce code re-use
    pub fn refresh_node_structure(&mut self, node_key: NodeGraphKey, node_kind: &Box<dyn NodeKind>, display_node_kind: &Box<dyn DisplayNodeKind>)
    {
        self.node_graph.refresh_node_structure(&node_key, node_kind); // @TODO, find a better name for this

        return;

        let node_handle_before_update = self.node_graph.get_node_handle(&node_key);

        let display_node = self.display_nodes.get_mut(&node_key).unwrap();
        display_node.display_kind = display_node_kind.clone();

        let updated_input_port_values = self.node_graph.get_node_input_port_values(&node_key);
        let updated_output_port_values = self.node_graph.get_node_output_port_values(&node_key);

        let updated_input_display_ports_values = display_node.display_kind.display_input_ports(updated_input_port_values);

        let node_handle_after_update = self.node_graph.get_node_handle(&node_key);

        // If the update caused there to be less input ports than before, remove the extra once
        if node_handle_before_update.input_port_keys.len() > updated_input_display_ports_values.len()
        {
            let mut index_to_remove = updated_input_display_ports_values.len();
            while index_to_remove < node_handle_before_update.input_port_keys.len() 
            {
                let key_to_remove = node_handle_before_update.input_port_keys[index_to_remove]; 
                self.display_input_ports.remove(&key_to_remove);
                index_to_remove += 1;
            }
        }

        // if node_handle_before_update.output_port_keys.len() > updated_output_display_ports_values.len()
        // {
        //     let mut index_to_remove = updated_output_display_ports_values.len();
        //     while index_to_remove < node_handle_before_update.output_port_keys.len()
        //     {
        //         let key_to_remove = node_handle_before_update.output_port_keys[index_to_remove];
        //         self.display_output_ports.remove(&key_to_remove);
        //         index_to_remove += 1;
        //     }
        // }




    }
}