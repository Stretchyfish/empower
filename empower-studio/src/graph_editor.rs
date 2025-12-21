use std::collections::HashMap;
use empower_engine::node_graph::node::port::PortKind;
use empower_engine::{NodeGraph, NodeGraphKey};
use empower_engine::runtime::EmpowerExecutor;

pub mod display_node;
pub use display_node::DisplayNode;
pub use display_node::DisplayPort;
pub use display_node::DisplayValue;

mod debug_info;
use debug_info::DebugInfo;

mod port_searcher;
use port_searcher::PortSearcher;

pub struct GraphEditor
{
    pub node_graph: NodeGraph,
    pub display_nodes: HashMap<NodeGraphKey, DisplayNode>,
    pub display_input_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub display_output_ports: HashMap<NodeGraphKey, DisplayPort>,
    pub selected_nodes: Vec<NodeGraphKey>,
    pub port_searcher: Option<PortSearcher>,
    pub debug_info: DebugInfo,
    pub executor: Option<EmpowerExecutor>,
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
        port_searcher: None,
        debug_info: DebugInfo::new(),
        executor: None,
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
        // let node_output_port_values = self.node_graph.get_node_output_port_values(&node.key);
        let display_input_ports = display_node.display_kind.display_input_ports(node_input_port_values);
        // let display_output_ports = display_node.display_kind.display_output_ports(node_output_port_values);

        if node.input_port_keys.len() != display_input_ports.len()
        {
            panic!("Tried to create node, but display input ports doesn't match number of actual input ports");
        } 

        for index in 0..node.input_port_keys.len()
        {
            self.display_input_ports.insert(node.input_port_keys[index], display_input_ports[index].clone() );
        }

        // for index in 0..node.output_port_keys.len()
        // {
        //     self.display_output_ports.insert(node.output_port_keys[index], display_output_ports[index].clone() );
        // }

        // create display output ports
        for output_port_key in &node.output_port_keys
        {
            let output_port = self.node_graph.get_output_port(output_port_key).unwrap();

            let display_port = DisplayPort::nothing(position, &output_port.value); // The position is just defaulted here, because it will be correct in refresh display node
            self.display_output_ports.insert(*output_port_key, display_port);
        }
        // @TODO, this function needs a second look!
        self.display_nodes.insert(node_handle.node_key, display_node);

        // Correct position and etc to avoid unessesary code duplication
        self.refresh_display_node(node_handle.node_key);

        true
    }

    pub fn create_node_copy(&mut self, node_key: &NodeGraphKey) -> NodeGraphKey
    {
        let original_node_handle = self.node_graph.get_node_handle(node_key);

        let copied_node_handle= self.node_graph.create_node_copy(node_key);

        let copied_display_node = self.display_nodes.get(&original_node_handle.node_key).unwrap().clone();

        self.display_nodes.insert(copied_node_handle.node_key, copied_display_node);

        for (index, input_port_key) in original_node_handle.input_port_keys.iter().enumerate()
        {
            let copied_display_input_port = self.display_input_ports.get(&input_port_key).unwrap().clone();
            self.display_input_ports.insert(copied_node_handle.input_port_keys[index], copied_display_input_port);
        }

        for (index, output_port_key) in original_node_handle.output_port_keys.iter().enumerate()
        {
            let copied_display_output_port = self.display_output_ports.get(&output_port_key).unwrap().clone();
            self.display_output_ports.insert(copied_node_handle.output_port_keys[index], copied_display_output_port);
        }

        self.refresh_display_node(copied_node_handle.node_key);

        copied_node_handle.node_key.clone()
    }

    pub fn remove_node(&mut self, node_key: &NodeGraphKey)
    {
        // First check and remove the node key from selected nodes
        if self.selected_nodes.contains(node_key)
        {
            self.selected_nodes.retain(|x| x != node_key ); // Removes all elements with this value
        }

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

    pub fn refresh_display_node(&mut self, node_key: NodeGraphKey)
    {
        let node = self.node_graph.get_node(&node_key).unwrap();
        let display_node = self.display_nodes.get_mut(&node_key).unwrap();
        let display_node_size = display_node.display_kind.node_size(&node.kind);
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
            display_input_port.valid  = true;

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

    pub fn update_node_structure(&mut self, node_key: &NodeGraphKey)
    {
        // @TODO, this whole thing is a mess... Needs to be redone, and add output ports
        let node_handle_before_update = self.node_graph.get_node_handle(&node_key);

        let node_kind_copy = self.node_graph.get_node(node_key).unwrap().kind.clone();

        self.node_graph.refresh_node_structure(&node_key, &node_kind_copy); // @TODO, find a better name for this

        let node_handle_after_update = self.node_graph.get_node_handle(&node_key);

        let display_node = self.display_nodes.get_mut(&node_key).unwrap();
        // display_node.display_kind = display_node_kind.clone();

        let updated_input_port_values = self.node_graph.get_node_input_port_values(&node_key);
        let updated_input_display_ports_values = display_node.display_kind.display_input_ports(updated_input_port_values);

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

        for (index, display_port_key) in node_handle_after_update.input_port_keys.iter().enumerate()
        {
            if !self.display_input_ports.contains_key(display_port_key)
            {
                let new_display_input_port = updated_input_display_ports_values[index].clone();
                self.display_input_ports.insert(*display_port_key, new_display_input_port);
                continue;
            }

            *self.display_input_ports.get_mut(&display_port_key).unwrap() = updated_input_display_ports_values[index].clone();
        }

        self.refresh_display_node(*node_key);
    }

    pub fn add_node_to_selection(&mut self, node_key: &NodeGraphKey)
    {
        if self.selected_nodes.contains(node_key)
        {
            return;
        }

        self.selected_nodes.push( *node_key );
    }

    pub fn toggle_node_selection(&mut self, node_key: &NodeGraphKey)
    {
        if self.selected_nodes.contains(node_key)
        {
            self.selected_nodes.retain(|x| x != node_key ); // Removes all elements with this value
            return;
        }

        self.selected_nodes.push( *node_key );
    }

    pub fn clear_node_selection(&mut self)
    {
        self.selected_nodes.clear();
    }

    pub fn move_selected_nodes(&mut self, canvas_delta_position: &egui::Vec2)
    {
        for node_key in &self.selected_nodes.clone() // This is needed for borrow with refresh display node
        {
            let display_node = self.display_nodes.get_mut(node_key).unwrap();
            display_node.position += *canvas_delta_position;

            // @TODO, this whole refresh needs a rework
            self.refresh_display_node(*node_key);
        }
    }

    pub fn clicked_input_port(&mut self, port_key: &NodeGraphKey)
    {
        if self.port_searcher.is_none()
        {
            // First check if the input port already has a connection, remove that connection, and either convert that to a port search or overtake it
            if self.node_graph.input_port_has_connection(port_key)
            {
                let connect_output_port_key = self.node_graph.get_input_port_connection_key(port_key).expect("Tried to access ouptut port in connection-in, not available").clone();
                self.node_graph.remove_connection(port_key, &connect_output_port_key);

                self.port_searcher = Some( PortSearcher::output_port_searching(connect_output_port_key) );
                return;
            }

            self.port_searcher = Some( PortSearcher::input_port_searching(*port_key) );
            return;
        }

        let port_searcher = self.port_searcher.as_ref().unwrap();

        match port_searcher.port_kind
        {
            PortKind::Input => // Detect if user clicked another input port while port searching from input
            {
                if port_searcher.port_key == *port_key
                {
                    self.port_searcher = None; // @TODO, expand this functionality to be more complex
                }
            },
            PortKind::Output => // Detect if ports can be connected
            {
                let add_connection_result = self.node_graph.add_connection(port_searcher.port_key, *port_key);

                match add_connection_result
                {
                    Ok(()) => println!("Added connection: {}, {}", port_searcher.port_key, *port_key),
                    Err( text ) => println!("Failed to add connection because: {}", text),
                }

                self.port_searcher = None;
            },
        }
        
    }

    pub fn clicked_output_port(&mut self, port_key: &NodeGraphKey)
    {
        if self.port_searcher.is_none()
        {
            self.port_searcher = Some( PortSearcher::output_port_searching(*port_key) );
            return;
        }

        let port_searcher = self.port_searcher.as_ref().unwrap(); 

        match  port_searcher.port_kind 
        {
            PortKind::Output =>
            {
                if port_searcher.port_key == *port_key
                {
                    self.port_searcher = None;
                }
            }
            PortKind::Input =>
            {
                let add_connection_result = self.node_graph.add_connection(*port_key, port_searcher.port_key); 

                // @TODO, remove this, currently its mostly debug info
                match add_connection_result
                {
                    Ok(()) => println!("Added connection: {}, {}", *port_key, port_searcher.port_key),
                    Err( text ) => println!("Failed to add connection because: {}", text),
                }

                self.port_searcher = None;
            },
        }
    }

    // @TODO, find a better name
    pub fn set_input_port_value_if_display_value_can_convert(&mut self, port_key: &NodeGraphKey, new_value: &DisplayValue)
    {
        let input_port = self.node_graph.get_input_port_mut(port_key).unwrap();
        let display_input_port = self.display_input_ports.get_mut(port_key).unwrap();

        display_input_port.value = new_value.clone();

        let new_port_value = display_input_port.value.to_port_value(&input_port.compatability);

        if new_port_value.is_none()
        {
            display_input_port.valid = false; // @TODO, consider a better name, like "valid"
            return;
        }
        display_input_port.valid = true;

        input_port.value = new_port_value.unwrap();
    }

    pub fn stop_port_search(&mut self)
    {
        self.port_searcher = None;
    }
}
