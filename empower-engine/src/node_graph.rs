use std::collections::HashMap;
use std::collections::VecDeque;

pub type NodeGraphKey = i32;

pub mod node;
use node::Node;
use node::port::Port;
use node::port::PortValue;
use node::NodeHandle;
use node::node_kind::NodeKind; // @TODO, make this include better

use crate::analyser::TextBuffer;

pub mod analysis; // @TODO, decide on how to approach this

#[derive(Default)]
pub struct NodeGraph
{
    nodes: HashMap<NodeGraphKey, Node>,
    input_ports: HashMap<NodeGraphKey, Port>,
    output_ports: HashMap<NodeGraphKey, Port>,
    connections_out: HashMap<NodeGraphKey, Vec<NodeGraphKey>>,
    connections_in: HashMap<NodeGraphKey, NodeGraphKey>,
    
    last_executed_node: NodeGraphKey,
    execution_queue: VecDeque<NodeGraphKey>,
    log: TextBuffer,
}

impl NodeGraph
{
    pub fn new() -> Self
    {
        Self
        {
            nodes: HashMap::new(),
            input_ports: HashMap::new(),
            output_ports: HashMap::new(),
            connections_out: HashMap::new(),
            connections_in: HashMap::new(),
            last_executed_node: 0, // @TODO, find a better approach, its currently set to 0, because 0 is unsued
            execution_queue: VecDeque::new(),
            log: TextBuffer::new(),
        }
    }

    pub fn add_node(&mut self,  node_kind: &NodeKind) -> NodeHandle
    {    
        // let node_kind_constructor = NODE_REGISTRY.get(node_kind).unwrap();
        // let node_kind= node_kind_constructor();
        
        let input_ports_compatabilities = node_kind.input_ports_compatabilities();
        let output_ports_compatabilities = node_kind.output_ports_compatabilities();

        // let input_ports_compatabilities = node::node_kind::get_node_input_port_compatabilities(&node_kind);
        // let output_ports_compatabilities = node::node_kind::get_node_output_port_compatabilities(&node_kind);

        let new_node_key = self.get_available_node_key();

        let mut new_input_port_keys = Vec::with_capacity(input_ports_compatabilities.len());
        for input_port_compatability in input_ports_compatabilities
        {
            let new_input_port_key = self.get_available_input_port_key();
            let new_input_port = Port::new_input_port(
                                                    new_input_port_key, 
                                                    new_node_key, 
                                                    input_port_compatability,
            );
            self.input_ports.insert(new_input_port_key, new_input_port);

            new_input_port_keys.push(new_input_port_key);
        }

        let mut new_output_port_keys = Vec::with_capacity(output_ports_compatabilities.len());
        for output_port_compatability in output_ports_compatabilities
        {
            let new_output_port_key = self.get_available_output_port_key();
            let new_output_port = Port::new_input_port(
                                                    new_output_port_key, 
                                                    new_node_key, 
                                                    output_port_compatability,
            );
            self.output_ports.insert(new_output_port_key, new_output_port);

            new_output_port_keys.push(new_output_port_key);
        }

        let new_node = Node::new(
                                new_node_key, 
                                node_kind.clone(), 
                                new_input_port_keys.clone(), 
                                new_output_port_keys.clone()
        );
        self.nodes.insert(new_node_key, new_node);

        NodeHandle::new(new_node_key, new_input_port_keys, new_output_port_keys)
    }

    pub fn create_node_copy(&mut self, node_key: &NodeGraphKey) -> NodeHandle
    {
        let mut node_to_copy = self.nodes.get(node_key).expect("ERROR in create_node_copy, asked to get node with incorrect key").clone();

        let new_node_key = self.get_available_node_key();

        node_to_copy.key = new_node_key;

        let mut new_input_port_keys = Vec::new();
        for input_port_key in &node_to_copy.input_port_keys
        {
            let mut input_port_copy = self.input_ports.get(input_port_key).expect("ERROR in create_node_copy, requested to fetch input port with a key not in the input port list").clone();
            let new_copied_input_port_key = self.get_available_input_port_key();
            input_port_copy.key = new_copied_input_port_key;
            input_port_copy.node_key = new_node_key;

            new_input_port_keys.push(input_port_copy.key);
            self.input_ports.insert(input_port_copy.key, input_port_copy);

            if self.connections_in.contains_key(input_port_key)
            {
                let connection = self.connections_in.get(input_port_key).unwrap().clone();
                let add_connection_result = self.add_connection(connection, new_copied_input_port_key);

                if add_connection_result.is_err()
                {
                    println!("{:?}", add_connection_result.err());
                }
            }
        }

        let mut new_output_port_keys = Vec::new();
        for output_port_key in &node_to_copy.output_port_keys
        {
            let mut output_port_copy = self.output_ports.get(output_port_key).expect("ERROR in create_node_copy, requested to fetch output port with a key not in the output port list").clone();
            output_port_copy.key = self.get_available_output_port_key();
            output_port_copy.node_key = new_node_key;

            new_output_port_keys.push(output_port_copy.key);
            self.output_ports.insert(output_port_copy.key, output_port_copy);
        }

        node_to_copy.input_port_keys = new_input_port_keys.clone();
        node_to_copy.output_port_keys = new_output_port_keys.clone();

        self.nodes.insert(node_to_copy.key, node_to_copy);

        NodeHandle::new(new_node_key, new_input_port_keys, new_output_port_keys)
    }

    // @TODO, consider if this should return a node handle or a bool, result or similar
    pub fn remove_node(&mut self, node_key: &NodeGraphKey)
    {
        let node_handle = self.get_node_handle(node_key);

        self.nodes.remove(node_key);

        for input_port_key in node_handle.input_port_keys
        {
            self.input_ports.remove(&input_port_key);
            self.remove_input_port_connections(&input_port_key);
        }

        for output_port_key in node_handle.output_port_keys
        {
            self.output_ports.remove(&output_port_key);
            self.remove_output_port_connections(&output_port_key);
        }
    }

    pub fn update_node(&mut self, node_key: &NodeGraphKey, new_node_state: NodeKind)
    {
        // This is defined outside of the first scope to avoid borrower issues
        let mut connections_to_remove: Vec<(NodeGraphKey, NodeGraphKey)> = Vec::new();
        let mut new_input_port_keys = Vec::new();
        let mut new_output_port_keys = Vec::new();

        {
            // @TODO, take a look if this node fetch can be removed
            let node = self.nodes.get(node_key).expect("ERROR in refresh node, unable to fetch node key");

            // @TODO, see if its possible to put the node back instead of new_node_state
            let updated_input_compatabilities = new_node_state.input_ports_compatabilities();
            let updated_output_compatabilities = new_node_state.output_ports_compatabilities();
            println!("see me! {} {}", node.output_port_keys.len(), updated_output_compatabilities.len());

            // If too many input ports now exist, remove the extra ones
            if node.input_port_keys.len() > updated_input_compatabilities.len()
            {
                let mut index_to_remove= updated_input_compatabilities.len();
                while index_to_remove < node.input_port_keys.len() 
                {
                    let key_to_remove = node.input_port_keys[index_to_remove];
                    self.input_ports.remove(&key_to_remove); // This does not look correct!
                    index_to_remove += 1;
                }
            }

            // If too many output ports now exist, remove the extra ones
            if node.output_port_keys.len() > updated_output_compatabilities.len()
            {
                println!("Entered here");
                let mut index_to_remove = updated_output_compatabilities.len();
                while index_to_remove < node.output_port_keys.len()
                {
                    let key_to_remove = node.output_port_keys[index_to_remove];
                    self.output_ports.remove(&key_to_remove);
                    index_to_remove += 1;
                }
            }

            // Go though existing input ports and overwrite their values
            for (index, port_compatability) in updated_input_compatabilities.iter().enumerate()
            {
                // If there are to many new ports, add more
                if node.input_port_keys.len() - 1 < index // @TODO, this should also be doable with a contain
                {
                    let new_input_port_key = self.get_available_input_port_key();
                    let new_input_port = Port::new_input_port(
                                                        new_input_port_key, 
                                                        *node_key, 
                                                        port_compatability.clone(),
                    );
    
                    self.input_ports.insert(new_input_port_key, new_input_port);
                    new_input_port_keys.push(new_input_port_key);
                    continue;
                }

                let input_port = self.input_ports.get_mut(&node.input_port_keys[index]).unwrap();
                new_input_port_keys.push(input_port.key);
                input_port.update_compatability(port_compatability.clone() );

                // Check that the inwards connections to the input ports are still valid
                if !self.connections_in.contains_key(&input_port.key)
                {
                    continue;
                }

                let connected_output_port_key = self.connections_in.get(&input_port.key).unwrap();
                let connected_output_port = self.output_ports.get(connected_output_port_key).unwrap();

                if !connected_output_port.compatability.is_compatible_with(&port_compatability)
                {
                    connections_to_remove.push( (input_port.key, connected_output_port_key.clone() ) );
                }
            }

            // Go though existing output ports and overwrite their values
            for (index, port_compatability) in updated_output_compatabilities.iter().enumerate()
            {
                // If there are to many new ports, add more
                if node.output_port_keys.len() - 1 < index
                {
                    let new_output_port_key = self.get_available_output_port_key();
                    let new_output_port = Port::new_output_port(
                                                                    new_output_port_key, 
                                                                    *node_key, 
                                                                    port_compatability.clone()
                    );

                    self.output_ports.insert(new_output_port_key, new_output_port);
                    new_output_port_keys.push(new_output_port_key);
                    continue;
                }

                let output_port = self.output_ports.get_mut(&node.output_port_keys[index]).unwrap();
                output_port.update_compatability(port_compatability.clone() );
                new_output_port_keys.push(output_port.key);

                if !self.connections_out.contains_key(&output_port.key)
                {
                    continue;
                }

                let connected_input_port_keys = self.connections_out.get(&output_port.key).unwrap();

                for connected_input_port_key in connected_input_port_keys
                {
                    let connected_input_port = self.input_ports.get(connected_input_port_key).unwrap();
                    
                    if !connected_input_port.compatability.is_compatible_with(port_compatability)
                    {
                        connections_to_remove.push( (*connected_input_port_key, output_port.key) );
                    }
                }
            }
        }

        let node = self.nodes.get_mut(node_key).expect("Error in update node, unable to fetch node key");
        node.kind = new_node_state;

        println!("New number of output port keys: {}", new_output_port_keys.len());

        node.input_port_keys = new_input_port_keys;
        node.output_port_keys = new_output_port_keys;

        for connection in connections_to_remove
        {
            self.remove_connection(&connection.0, &connection.1);
        }
    }

    pub fn add_connection(&mut self, output_port_key: NodeGraphKey, input_port_key: NodeGraphKey) -> Result<(), String>
    {
        let output_port;

        match self.output_ports.get(&output_port_key)
        {
            Some(value) => output_port = value,
            None => return Err( format!("Output port (key: {}) requested not in node graph", output_port_key) ),
        }

        let input_port;

        match self.input_ports.get(&input_port_key)
        {
            Some(value) => input_port = value,
            None => return Err( format!("Input port (key: {}) requested not in node graph", input_port_key) ),
        }

        if output_port.node_key == input_port.node_key
        {
            return Err( "Cannot connect port to another port on the same node".to_string() );
        }

        if !input_port.compatability.is_compatible_with(&output_port.compatability)
        {
            return Err( format!( "Cannot connect two incompatible ports (key out: {}, key in: {})", output_port_key, input_port_key) );
        }
       
        if self.connections_out.contains_key(&output_port_key)
        {
            let existing_connection = self.connections_out.get_mut(&output_port_key).unwrap(); // This is a safe call due to the check above

            if existing_connection.contains(&input_port_key)
            {
                return Err( "Could not add connection, as it already exists".to_string() );
            }

            existing_connection.push(input_port_key); // @TODO, investigate what is happening here
            self.connections_in.insert(input_port_key, output_port_key);
            return Ok(());
        }

        self.connections_out.insert(output_port_key, Vec::from([input_port_key]));
        self.connections_in.insert(input_port_key, output_port_key);

        Ok(())
    }

    // @TODO, consider making this a bool?
    pub fn remove_connection(&mut self, input_port_key: &NodeGraphKey, output_port_key: &NodeGraphKey) -> bool
    {
        if !self.connections_in.contains_key(input_port_key) || !self.connections_out.contains_key(output_port_key)
        {
            return false;
        }

        self.connections_in.remove(input_port_key);

        let mut no_more_elements_in_connection_out = false;

        {
            let connections_going_out = self.connections_out.get_mut(output_port_key).expect("failed");

            let element_to_remove_index = connections_going_out.iter().position(|p| p == input_port_key).expect("remove connection was asked to remove a output port that should exist, but doesn't");

            connections_going_out.remove(element_to_remove_index);

            if connections_going_out.is_empty()
            {
                no_more_elements_in_connection_out = true;
            }
        }

        if no_more_elements_in_connection_out
        {
            self.connections_out.remove(output_port_key);
        }

        true
    }

    pub fn remove_input_port_connections(&mut self, input_port_key: &NodeGraphKey) -> bool
    {
        if !self.connections_in.contains_key(input_port_key) { return false; }

        let connected_port = self.connections_in.get(input_port_key).expect("Tried to fetch non existing connection_in").clone();
        self.remove_connection(input_port_key, &connected_port);

        true
    }

    pub fn remove_output_port_connections(&mut self, output_port_key: &NodeGraphKey) -> bool
    {
        if !self.connections_out.contains_key(output_port_key) { return false; }

        let connected_ports = self.connections_out.get(output_port_key).expect("Tried to feth non-existing connection_out").clone();

        for connected_port in connected_ports
        {
            self.remove_connection(&connected_port, output_port_key);
        }

        true
    }

    pub fn get_all_nodes(&self) -> Vec<&Node>
    {
        self.nodes.values().collect()
    }

    pub fn get_all_input_ports(&self) -> Vec<&Port>
    {
        self.input_ports.values().collect()
    }

    pub fn get_all_output_ports(&self) -> Vec<&Port>
    {
        self.output_ports.values().collect()
    }

    pub fn get_all_connections(&self) -> Vec<(NodeGraphKey, NodeGraphKey)>
    {
        let mut all_connections= Vec::new();
        
        for connection in self.connections_out.iter()
        {
            for port_in in connection.1
            {
                all_connections.push( (connection.0.clone(), port_in.clone()) );
            } 
        }

        all_connections
    }

    pub fn node_count(&self) -> usize
    {
        self.nodes.len()
    }

    pub fn connections_count(&self) -> usize
    {
        self.connections_in.len()
    }

    pub fn input_port_count(&self) -> usize
    {
        self.input_ports.len()
    }

    pub fn output_port_count(&self) -> usize
    {
        self.output_ports.len()
    }

    pub fn contains_node(&self, node_key: &NodeGraphKey) -> bool
    {
        self.nodes.contains_key(node_key)
    }

    pub fn get_node(&self, node_key: &NodeGraphKey) -> Option<&Node>
    {
        self.nodes.get(node_key)
    }

    pub fn get_input_port(&self, port_key: &NodeGraphKey) -> Option<&Port>
    {
        self.input_ports.get(port_key)
    }

    pub fn get_mut_input_port(&mut self, port_key: &NodeGraphKey) -> Option<&mut Port>
    {
        self.input_ports.get_mut(port_key)
    }

    pub fn input_port_has_connection(&self, port_key: &NodeGraphKey) -> bool
    {
        self.connections_in.contains_key(port_key)
    }

    pub fn output_port_has_connection(&self, port_key: &NodeGraphKey) -> bool
    {
        self.connections_out.contains_key(port_key)
    }

    pub fn get_output_port(&self, port_key: &NodeGraphKey) -> Option<&Port>
    {
        self.output_ports.get(port_key)
    }

    pub fn get_input_port_connection_key(&self, port_key: &NodeGraphKey) -> Option<&NodeGraphKey>
    {
        self.connections_in.get(port_key)
    }

    pub fn get_output_port_connection_keys(&self, port_key: &NodeGraphKey) -> Option<&Vec<NodeGraphKey>>
    {
        self.connections_out.get(port_key)
    }

    pub fn get_node_handle(&self, node_key: &NodeGraphKey) -> NodeHandle
    {
        let node = match self.nodes.get(node_key)
        {
            Some( node ) => node,
            None => panic!("Requested node that does not exist"),
        };

        NodeHandle { node_key: *node_key, input_port_keys: node.input_port_keys.clone(), output_port_keys: node.output_port_keys.clone() }
    }

    pub fn get_all_node_keys(&self) -> Vec<NodeGraphKey> // @TODO, consider if this is the best way to go
    {
        self.nodes.keys().cloned().collect()
    }

    pub fn start_node_graph(&mut self) 
    {
        if self.node_count() == 0 { return; }

        let start_node_key: NodeGraphKey = 1; // @TODO, find a better approach
        self.start_node_graph_from_entry(&start_node_key);
    }

    pub fn start_node_graph_from_entry(&mut self, node_key: &NodeGraphKey)
    {
        self.execution_queue.clear();

        let rouge_nodes = analysis::detect_rouge_nodes(self);

        self.execution_queue.extend(rouge_nodes);
        self.execution_queue.push_back(*node_key);
    }

    pub fn stop_node_graph(&mut self)
    {
        self.execution_queue.clear();
    }

    pub fn is_running(&self) -> bool
    {
       !self.execution_queue.is_empty() 
    }

    pub fn view_node_graph(&mut self, ctx: &egui::Context)
    {
        if self.execution_queue.is_empty() { return; }

        let node_to_execute = self.execution_queue[0]; // @TODO, consider if it should be rewritten with .front instead

        let execution_response = self.view_node(&node_to_execute, ctx);

        if execution_response.is_none()
        {
            return;
        }

        self.execution_queue.extend(execution_response.unwrap());
        self.execution_queue.pop_front();
    }

    pub fn view_node(&mut self, node_key: &NodeGraphKey, ctx: &egui::Context) -> Option<Vec<NodeGraphKey>>
    {
        let node_to_execute = self.nodes.get_mut(node_key).expect("View node tried to fetch a node that doesn't exist");

        // let input_port_values = self.get_node_input_port_values(node_key).clone(); // @Consider if there is a way to avoid this clone
        let mut input_port_values = Vec::with_capacity(node_to_execute.input_port_keys.len());
        for input_port_key in &node_to_execute.input_port_keys
        {
            let input_port = self.input_ports.get(input_port_key).unwrap();
            input_port_values.push(&input_port.value);
        } 

        if self.last_executed_node != *node_key
        {
            node_to_execute.kind.setup(input_port_values);
            self.last_executed_node = *node_key;
            return None; // Only happens once to deal with borrower issues
        }

        let mut logging = TextBuffer::new();
        let executed_output_values = node_to_execute.kind.execute(input_port_values, ctx, &mut logging);

        if executed_output_values.is_none()
        {
            return None;
        }

        let test = executed_output_values.unwrap();
        self.set_output_port_values(node_key, &test);
        
        let distribution_result = self.distribute_outputs(node_key); 
        return Some( distribution_result );
    }

    // pub fn execute_node_graph(&mut self) -> bool
    // {
    //     if self.node_count() == 0 { return false; }

    //     let start_node_key: NodeGraphKey = 1; // @TODO, find a better approach
    //     self.execute_node_graph_from_entry(&start_node_key)
    // }

    // pub fn execute_node_graph_from_entry(&mut self, node_key: &NodeGraphKey) -> bool
    // {
    //     if !self.nodes.contains_key(node_key) { return false; }

    //     let mut node_keys_to_execute_queue: VecDeque<NodeGraphKey> = VecDeque::new();

    //     // @TODO, find a better place for these, consider making a execute_node_graphies_entries for list of nodes
    //     let rouge_nodes = analysis::detect_rouge_nodes(self);
    //     println!("Rouge nodes: {:?}", rouge_nodes);
 
    //     node_keys_to_execute_queue.extend(rouge_nodes);

    //     node_keys_to_execute_queue.push_back( *node_key );
    
    //     while !node_keys_to_execute_queue.is_empty()
    //     {
    //         println!("{:?}", node_keys_to_execute_queue); // @TODO, remove this and add a debug mode

    //         let node_to_execute_key: NodeGraphKey = node_keys_to_execute_queue[0];

    //         let next_nodes_to_execute: Vec<NodeGraphKey> = self.execute_node(&node_to_execute_key);

    //         node_keys_to_execute_queue.extend(next_nodes_to_execute);
    //         node_keys_to_execute_queue.pop_front();
    //     }

    //     true
    // }

    // pub fn execute_node(&mut self, node_key: &NodeGraphKey) -> Vec<NodeGraphKey>
    // {
    //     let node_to_execute;
    //     match self.nodes.get_mut(node_key)
    //     {
    //         Some( value ) => node_to_execute = value,
    //         None => return Vec::new(),
    //     }
        
    //     let mut input_port_values = Vec::with_capacity(node_to_execute.input_port_keys.len());
    //     for input_port_key in &node_to_execute.input_port_keys
    //     {
    //         let input_port;
    //         match self.input_ports.get(input_port_key)
    //         {
    //             Some( value ) => input_port = value,
    //             None => return Vec::new(),
    //         }

    //         input_port_values.push(&input_port.value);
    //     } 

    //     let executed_output_values = node_to_execute.kind.execute(input_port_values, &mut self.log);

    //     if executed_output_values.len() != node_to_execute.output_port_keys.len()
    //     {
    //         println!("Executed output and node output does not match");
    //         return Vec::new();
    //     }

    //     for (output_port_index, output_port_key) in node_to_execute.output_port_keys.iter().enumerate()
    //     {
    //         let output_port = self.output_ports.get_mut(output_port_key).expect("ERROR in execute node, unable to fetch output port");

    //         let executed_output_value = &executed_output_values[output_port_index];

    //         if !output_port.compatability.contains_port_value_type(executed_output_value)
    //         {
    //             println!("Executed node, and tried to set output port, but types are incompatible");
    //             return Vec::new();
    //         }

    //         output_port.value = executed_output_value.clone(); // This needs to be a clone, or the value ends up on multiple input ports
    //     }

    //     let distribution_result = self.distribute_outputs(node_key); 
    //     match distribution_result
    //     {
    //         Ok( new_nodes_to_execute ) => new_nodes_to_execute,
    //         Err( error_text ) =>
    //         {
    //             println!("{}", error_text); // @TODO, printing should be part of a debug mode
    //             Vec::new()
    //         },
    //     }
    // }

    pub fn get_node_input_port_values(&self, node_key: &NodeGraphKey) -> Vec<&PortValue>
    {
        let node = match self.nodes.get(node_key)
        {
            Some( node ) => node,
            None => return Vec::new(),
        };

        let mut input_port_values = Vec::with_capacity(node.input_port_keys.len());
        for input_port_key in &node.input_port_keys
        {
            let input_port;
            match self.input_ports.get(input_port_key)
            {
                Some( value ) => input_port = value,
                None => return Vec::new(),
            }

            input_port_values.push(&input_port.value);
        } 

        input_port_values
    }

    // @TODO, simplify these input and output port functions
    pub fn get_output_port_values(&self, node_key: &NodeGraphKey) -> Vec<&PortValue> // @TODO, fix the naming now matching its neighbor function
    {
        let node = match self.nodes.get(node_key)
        {
            Some( node ) => node,
            None => return Vec::new(),
        };

        let mut output_port_values= Vec::with_capacity(node.output_port_keys.len());
        for output_port_key in &node.output_port_keys
        {
            let output_port = self.output_ports.get(output_port_key).expect("Tried to fetch output port key in get_output_port_value, but output port was not avaiable");

            output_port_values.push(&output_port.value);
        } 

        output_port_values
    }

    pub fn set_output_port_values(&mut self, node_key: &NodeGraphKey, new_values: &Vec<PortValue>)
    {
        let node = self.nodes.get(node_key).unwrap();

        if node.output_port_keys.len() != new_values.len()
        {
            println!("Executed output and node output does not match");
            return;
        }

        for (output_port_index, output_port_key) in node.output_port_keys.iter().enumerate()
        {
            let output_port = self.output_ports.get_mut(output_port_key).expect("ERROR in execute node, unable to fetch output port");

            let executed_output_value = &new_values[output_port_index];

            if !output_port.compatability.contains_port_value_type(executed_output_value)
            {
                println!("Executed node, and tried to set output port, but types are incompatible");
                return;
            }

            output_port.value = executed_output_value.clone(); // This needs to be a clone, or the value ends up on multiple input ports
        }
    }

    fn get_available_node_key(&self) -> NodeGraphKey
    {
        self.nodes.keys().max().unwrap_or(&0) + 1
    }

    fn get_available_input_port_key(&self) -> NodeGraphKey
    {
        self.input_ports.keys().max().unwrap_or(&0) + 1
    }

    fn get_available_output_port_key(&self) -> NodeGraphKey
    {
        self.output_ports.keys().max().unwrap_or(&0) + 1
    }

    fn distribute_outputs(&mut self, node_key: &NodeGraphKey) -> Vec<NodeGraphKey> 
    {
        let node = match self.nodes.get(node_key)
        {
            Some( node ) => node,
            None => return Vec::new()
        };

        if node.output_port_keys.is_empty()
        {
            return Vec::new();
        }

        let mut next_nodes_to_execute = Vec::new(); // @TODO, fill this out

        for output_port_key in &node.output_port_keys
        {
            let connected_ports = match self.connections_out.get(output_port_key)
            {
                Some( connections ) => connections,
                None => continue,
            };

            // This is a security check, but its irrelevant to check if there is no connected port
            let output_port = match self.output_ports.get(output_port_key)
            {
                Some( port ) => port,
                None => return Vec::new(),
            };

            for connected_input_port_key in connected_ports
            {
                let input_port = match self.input_ports.get_mut(connected_input_port_key)
                {
                    Some ( port ) => port,
                    None => return Vec::new(),
                };

                let mut new_input_port_value = output_port.value.clone();

                if !input_port.compatability.contains_port_value_type(&output_port.value)
                {
                    new_input_port_value = output_port.value.as_desired_value(&input_port.value).expect("Unable to convert the two desired values");

                    // return Err( format!("ERROR, input port {} and output port {} are connected, but have incompatable types in distribute output", input_port.key, output_port_key) );
                }

                // @TODO, find a way of detecting math / immediate nodes 
                // match input_port.value
                // {
                //     PortValue::Trigger => next_nodes_to_execute.push(input_port.node_key),
                //     _ => {},
                // }

                input_port.value = new_input_port_value;
                next_nodes_to_execute.push(input_port.node_key);
            }
        }

        next_nodes_to_execute
    }

    pub fn get_logs(&self) -> &TextBuffer
    {
        &self.log
    }
}