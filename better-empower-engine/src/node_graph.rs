use std::collections::HashMap;
use std::collections::VecDeque;

pub type NodeGraphKey = i32;

mod node;
use node::Node;
use node::NodeHandle;
use node::node_kind::NODE_REGISTRY;
use node::port::Port;

use crate::node_graph::node::port::PortCompatability;

pub mod analysis;

pub struct NodeGraph
{
    nodes: HashMap<NodeGraphKey, Node>,
    input_ports: HashMap<NodeGraphKey, Port>,
    output_ports: HashMap<NodeGraphKey, Port>,
    connections_out: HashMap<NodeGraphKey, Vec<NodeGraphKey>>,
    connections_in: HashMap<NodeGraphKey, NodeGraphKey>,
    
    last_executed_node: NodeGraphKey,
    execution_queue: VecDeque<NodeGraphKey>,
    // log: TextBuffer,
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
            // log: TextBuffer::new(),
        }
    }

    pub fn add_node(&mut self, node_name: &'static str) -> NodeHandle 
    {
        let node_kind_constructor = match NODE_REGISTRY.get(node_name)
        {
           Some( constructor ) => constructor,
           None => panic!("Requested a non-existing node name"), 
        };

        let node_kind = node_kind_constructor();

        let input_ports_compatabilities = node_kind.input_compatabilities();
        let output_ports_compatabilities = node_kind.output_compatabilities();

        let new_node_key = self.get_available_node_key();

        let new_input_ports_keys = self.add_input_ports(&new_node_key, input_ports_compatabilities);
        let new_output_ports_keys = self.add_output_ports(&new_node_key, output_ports_compatabilities);

        let new_node = Node::new(
                                new_node_key, 
                                node_kind, 
                                new_input_ports_keys.clone(), 
                                new_output_ports_keys.clone()
        );
        self.nodes.insert(new_node_key, new_node);

        NodeHandle::new(new_node_key, new_input_ports_keys, new_output_ports_keys)
    }

    fn add_input_ports(&mut self, node_key: &NodeGraphKey, input_ports_compatabilities: Vec<PortCompatability>) -> Vec<NodeGraphKey>
    {
        // @TODO Consider merging these into 1 function that takes both inputs and output using the port kind
        let mut new_input_port_keys = Vec::with_capacity(input_ports_compatabilities.len());
        for input_port_compatability in input_ports_compatabilities
        {
            let new_input_port_key = self.get_available_input_port_key();
            let new_input_port = Port::new_input_port(
                                                    new_input_port_key, 
                                                    *node_key, 
                                                    input_port_compatability,
            );
            self.input_ports.insert(new_input_port_key, new_input_port);

            new_input_port_keys.push(new_input_port_key);
        }

        new_input_port_keys
    }

    fn add_output_ports(&mut self, node_key: &NodeGraphKey, output_ports_compatabilities: Vec<PortCompatability>) -> Vec<NodeGraphKey>
    {
        let mut new_output_port_keys = Vec::with_capacity(output_ports_compatabilities.len());
        for output_port_compatability in output_ports_compatabilities
        {
            let new_output_port_key = self.get_available_output_port_key();
            let new_output_port = Port::new_input_port(
                                                    new_output_port_key, 
                                                    *node_key, 
                                                    output_port_compatability,
            );
            self.output_ports.insert(new_output_port_key, new_output_port);

            new_output_port_keys.push(new_output_port_key);
        }

        new_output_port_keys
    }

    pub fn add_connection(&mut self, output_port_key: NodeGraphKey, input_port_key: NodeGraphKey) -> Result<(), String>
    {
        let output_port = self.output_ports.get(&output_port_key).unwrap();
        let input_port = self.input_ports.get(&input_port_key).unwrap();

        if output_port.node_key == input_port.node_key
        {
            return Err( "Cannot connect port to another port on the same node".to_string() );
        }

        if !input_port.compatability.is_compatible_with(&output_port.compatability)
        {
            return Err( format!( "Cannot connect two incompatible ports (key out: {}, key in: {})", output_port_key, input_port_key) );
        }

        // If the current output port already has atleast one connection, add add to the existing one instead
        if self.connections_out.contains_key(&output_port_key)
        {
            let current_output_connections = self.connections_out.get_mut(&output_port_key).unwrap(); // This is a safe call due to the check above

            if current_output_connections.contains(&input_port_key)
            {
                return Err( "Could not add connection, as it already exists".to_string() );
            }

            current_output_connections.push(input_port_key); // @TODO, investigate what is happening here
            self.connections_in.insert(input_port_key, output_port_key);
            return Ok(());
        }

        self.connections_out.insert(output_port_key, Vec::from([input_port_key]));
        self.connections_in.insert(input_port_key, output_port_key);

        Ok(())
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

}