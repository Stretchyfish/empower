use empower_engine::node_graph::node::NodeKind;
use empower_engine::NodeGraph;
use empower_engine::NodeGraphKey;

use std::collections::HashMap;

pub mod display_node;
use display_node::DisplayNode;

pub mod display_port;
use display_port::DisplayPort;

pub mod debug_info;
pub use debug_info::DebugInfo;

use crate::graph_editor::display_node::display_node_kind;
use crate::graph_editor::display_node::display_node_kind::DisplayState;
use crate::graph_editor::display_port::display_port_value::DisplayPortValue;

pub struct GraphEditor
{
    pub(crate) display_nodes: HashMap<NodeGraphKey, DisplayNode>, 
    pub(crate) display_input_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub(crate) display_output_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub(crate) node_graph: NodeGraph, 
    pub(crate) selected_nodes: Vec<NodeGraphKey>,
    pub(crate) debug_info: DebugInfo,
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
            node_graph: NodeGraph::new(),
            selected_nodes: Vec::new(),
            debug_info: DebugInfo::new(),
        };

        // let new_node_type = NodeKind::Start;
        let new_node_type = NodeKind::Start;

        // @TODO, find a better approach to centering the start node
        let start_node_left_offset = egui::Pos2 { x: -1700.0, y: -165.0 / 2.0 }; // Half the center nodes height and oriented left

        graph_editor.add_node(new_node_type, start_node_left_offset);

        graph_editor
    }

    pub fn add_node(&mut self, node_kind: NodeKind, position: egui::Pos2) -> bool
    {
        // @TODO, find a better way of assigning keys
        let node_handle= self.node_graph.add_node(&node_kind); 

        if self.display_nodes.contains_key(&node_handle.node_key) // @TODO, simplify these calls
        {
            println!("ERROR, attempted to add engine node key already in node graph (denied)"); // This should never happen, only if something wrongly implemented
            return false;
        }

        let node = match self.node_graph.get_node(&node_handle.node_key)
        {
            Some( node ) => node,
            None => 
            {
                println!("Add node tried to fetch node {} that doesn't exist", node_handle.node_key);
                return false;
            }
        };

        
        let input_port_values = self.node_graph.get_node_input_port_values(&node_handle.node_key);
        let output_port_values = self.node_graph.get_output_port_values(&node_handle.node_key);

        // let display_node_constructor_option = DISPLAY_NODE_REGISTRY.get(node.kind.name());

        // if display_node_constructor_option.is_none() // @TODO, look more into this approach
        // {
        //     println!("Unable to find a matching display node constructor in DISPLAY NODE REGISTRY");
        //     return false;
        // }

        let display_node_title = node.kind.name();

        // let display_node_constructor = display_node_constructor_option.unwrap();

        // let display_node_kind = display_node_constructor();

        let display_node_size = display_node_kind::get_display_node_size(&node_kind);
        // let display_node_value_offset = display_node::display_node_kind::get_display_node_value_offset(&node_kind);
        let display_node_value_offset = display_node_kind::get_state_size(&node_kind).y;
        let input_port_display_values = display_node_kind::get_display_input_ports(&node_kind, input_port_values);
        let output_port_display_values = display_node_kind::get_display_output_ports(&node_kind, output_port_values);

        if input_port_display_values.len() != node.input_port_keys.len()
        {
            println!("ERROR, the size of node in graph editor (add_node) input port and display input ports does not match");
            return false;
        }
        if output_port_display_values.len() != node.output_port_keys.len()
        {
            println!("ERROR, the size of node in graph editor (add_node) output and display output ports does not match");
            return false;
        }
              
        let port_gap = 60.0;
        let mut input_port_offset = 120.0 + display_node_value_offset;
        let mut output_port_offset = input_port_offset.clone();

        for (input_port_key_index, input_port_key) in node.input_port_keys.iter().enumerate()
        {
            // let display_port_value_representation = self.get_input_port_value_representation(input_port_key);
            let new_display_port = DisplayPort { 
                                                        node_key: node.key.clone(), 
                                                        relative_position: egui::Vec2::new(0.0, input_port_offset), 
                                                        display_value: input_port_display_values[input_port_key_index].clone(),
                                                        // text: input_port_names[input_port_key_index].clone(),
                                                        // value_representation: DisplayPortValueRepresentation::None, 
                                                        // value_representation_valid: true 
                                                        }; 
            self.display_input_ports.insert(input_port_key.clone(), new_display_port);

            input_port_offset += port_gap;
        }

        for (output_port_key_index, output_port_key) in node.output_port_keys.iter().enumerate()
        {
            // @TODO, figure out if setting display port values is needed for output
            let new_display_port = DisplayPort { 
                                                    node_key: node.key.clone(), 
                                                    relative_position: egui::Vec2::new(display_node_size.x, output_port_offset), 
                                                    display_value: output_port_display_values[output_port_key_index].clone(),
                                                    // value_representation: DisplayPortValueRepresentation::Text( String::new() ), 
                                                    // value_representation: DisplayPortValueRepresentation::None, 
                                                    // value_representation_valid: false // @TODO, decide if this should be true?
                                                    }; 
            self.display_output_ports.insert(output_port_key.clone(), new_display_port);

            output_port_offset += port_gap;
        }

        let display_state = display_node_kind::get_display_state(&node_kind);

        let new_display_node = DisplayNode::new( display_node_title, position, display_node_size, display_state);
        self.display_nodes.insert(node.key.clone(), new_display_node );

        true
    }

    pub fn create_node_copy(&mut self, node_key: &NodeGraphKey) -> NodeGraphKey
    {
        let original_node_handle = self.node_graph.get_node_handle(node_key);

        let copied_node_handle= self.node_graph.create_node_copy(node_key);

        let mut copied_display_node = self.display_nodes.get(&original_node_handle.node_key).unwrap().clone();

        // @TODO, consider if this is a good approach
        copied_display_node.position.y += 20.0;
        self.display_nodes.insert(copied_node_handle.node_key, copied_display_node);

        for (index, input_port_key) in original_node_handle.input_port_keys.iter().enumerate()
        {
            let mut copied_display_input_port = self.display_input_ports.get(&input_port_key).unwrap().clone();
            copied_display_input_port.node_key = copied_node_handle.node_key;
            self.display_input_ports.insert(copied_node_handle.input_port_keys[index], copied_display_input_port);
        }

        for (index, output_port_key) in original_node_handle.output_port_keys.iter().enumerate()
        {
            let mut copied_display_output_port = self.display_output_ports.get(&output_port_key).unwrap().clone();
            copied_display_output_port.node_key = copied_node_handle.node_key;
            self.display_output_ports.insert(copied_node_handle.output_port_keys[index], copied_display_output_port);
        }

        copied_node_handle.node_key.clone()
    }

    pub fn remove_node(&mut self, node_key: &NodeGraphKey)
    {
        let node_handle = self.node_graph.get_node_handle(node_key);

        self.node_graph.remove_node(node_key);

        for input_port_key in node_handle.input_port_keys
        {
            self.display_input_ports.remove(&input_port_key);
        }

        for output_port_key in node_handle.output_port_keys
        {
            self.display_output_ports.remove(&output_port_key);
        }

        self.display_nodes.remove(node_key);
    }

    pub fn set_input_port_value(&mut self, input_port_key: &NodeGraphKey, display_port_value: DisplayPortValue)
    {
        let input_port = match self.node_graph.get_mut_input_port(input_port_key)
        {
            Some( port ) => port,
            None => panic!("ERROR, tried to set display input port in set input port value, but input port doesn't exist."),
        };

        let display_input_port = match self.display_input_ports.get_mut(input_port_key)
        {
            Some( port ) => port,
            None => panic!("ERROR, tried to get display input port in set input port value, but input port doesn't exist."),
        };

        display_input_port.display_value = display_port_value.clone();

        // @TODO, consider if this should be a result instead of an optional
        let new_input_port_value = match display_port_value.to(&input_port.compatability)
        {
            Some( input_port_value) => 
            {
                display_input_port.display_value.display_value_valid = true;
                input_port_value
            }
            None => 
            {
                display_input_port.display_value.display_value_valid = false;
                return
            }
        };

        input_port.value = new_input_port_value;
    }

    // @TODO, this whole function could use a cleanup
    pub fn update_node(&mut self, node_key: &NodeGraphKey, new_node_state: NodeKind, new_display_state: DisplayState)
    {
        let node_handle_before_update = self.node_graph.get_node_handle(node_key);

        self.node_graph.update_node( node_key, new_node_state.clone() );

        let display_node = self.display_nodes.get_mut(node_key).unwrap();
        display_node.size = display_node_kind::get_display_node_size(&new_node_state);
        display_node.display_state = new_display_state;

        let updated_input_port_values = self.node_graph.get_node_input_port_values(node_key);
        let updated_output_port_values = self.node_graph.get_output_port_values(node_key);

        let updated_input_display_ports_values = display_node_kind::get_display_input_ports(&new_node_state, updated_input_port_values);
        let updated_output_display_ports_values = display_node_kind::get_display_output_ports(&new_node_state, updated_output_port_values);

        let node_handle_after_update = self.node_graph.get_node_handle(node_key);

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

        if node_handle_before_update.output_port_keys.len() > updated_output_display_ports_values.len()
        {
            let mut index_to_remove = updated_output_display_ports_values.len();
            while index_to_remove < node_handle_before_update.output_port_keys.len()
            {
                let key_to_remove = node_handle_before_update.output_port_keys[index_to_remove];
                self.display_output_ports.remove(&key_to_remove);
                index_to_remove += 1;
            }
        }

        let port_gap = 60.0; // @TODO, find a better approach for this value, its defined twice now

        for (index, input_port_key) in node_handle_after_update.input_port_keys.iter().enumerate()
        {
            if !self.display_input_ports.contains_key( &input_port_key )
            {
                let previous_display_port = self.display_input_ports.get( &(node_handle_after_update.input_port_keys[index - 1]) ).unwrap();

                let new_display_port = DisplayPort
                {
                    node_key: node_handle_after_update.node_key,
                    relative_position: previous_display_port.relative_position + egui::Vec2 { x: 0.0, y: port_gap },
                    display_value: updated_input_display_ports_values[index].clone(),
                };

                self.display_input_ports.insert(*input_port_key, new_display_port);
                continue;
            }

            let display_port_to_update = self.display_input_ports.get_mut(input_port_key).unwrap();

            display_port_to_update.display_value = updated_input_display_ports_values[index].clone();
        }

        for (index, output_port_key) in node_handle_after_update.output_port_keys.iter().enumerate()
        {
            if !self.display_output_ports.contains_key( &output_port_key )
            {
                let previous_display_port = self.display_output_ports.get( &(node_handle_after_update.output_port_keys[index - 1]) ).unwrap();

                let new_display_port = DisplayPort
                {
                    node_key: node_handle_after_update.node_key,
                    relative_position: previous_display_port.relative_position + egui::Vec2 { x: 0.0, y: port_gap },
                    display_value: updated_input_display_ports_values[index].clone(),
                };

                self.display_input_ports.insert(*output_port_key, new_display_port);
                continue;
            }

            let display_port_to_update = self.display_output_ports.get_mut(output_port_key).unwrap();
            display_port_to_update.display_value = updated_output_display_ports_values[index].clone();
        }
    }



    pub fn refresh_all_node_display(&mut self)
    {
        // @TODO, try to find a better approach for this
        let node_keys = self.node_graph.get_all_node_keys();
        for node_key in node_keys
        {
            self.refresh_node_display(&node_key);
        }
     }

    pub fn refresh_node_display(&mut self, node_key: &NodeGraphKey)
    {
        let node_handle = self.node_graph.get_node_handle(node_key);

        for input_port_key in node_handle.input_port_keys
        {
            let input_port= self.node_graph.get_input_port(&input_port_key).expect("Failed to fetch input port");
            let display_input_port = self.display_input_ports.get_mut(&input_port_key).expect("Failed to fetch display input port");

            // display_input_port.display_value.value_type = DisplayPortValueType::new(&input_port.value);
            display_input_port.display_value.update(&input_port.value);
        }

        for output_port_key in node_handle.output_port_keys
        {
            let output_port= self.node_graph.get_output_port(&output_port_key).expect("Failed to fetch output port");
            let display_output_port= self.display_output_ports.get_mut(&output_port_key).expect("Failed to fetch display output port");

            // display_output_port.display_value.value_type = DisplayPortValueType::new(&output_port.value);
            display_output_port.display_value.update(&output_port.value);
        }
    }
}
