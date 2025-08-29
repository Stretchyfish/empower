use std::collections::HashMap;
use std::collections::VecDeque;

pub type NodeGraphKey = i32;

mod node;
use node::Node;
use node::NodeHandle;
pub use node::NodeKind;

mod port;
use port::Port;
use port::PortValue;

pub struct NodeGraph
{
    nodes: HashMap<NodeGraphKey, Node>,
    input_ports: HashMap<NodeGraphKey, Port>,
    output_ports: HashMap<NodeGraphKey, Port>,
    connections_out: HashMap<NodeGraphKey, Vec<NodeGraphKey>>,
    connections_in: HashMap<NodeGraphKey, NodeGraphKey>,
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
        }
    }

    pub fn add_node(&mut self,  node_kind: NodeKind) -> NodeHandle
    {
        node::node_kind::create_node(&node_kind, self)
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

    pub fn execute_node_graph(&mut self) -> bool
    {
        if self.node_count() == 0 { return false; }

        let start_node_key: NodeGraphKey = 1; // @TODO, find a better approach
        self.execute_node_graph_from_entry(&start_node_key)
    }

    pub fn execute_node_graph_from_entry(&mut self, node_key: &NodeGraphKey) -> bool
    {
        if !self.nodes.contains_key(node_key) { return false; }

        let mut node_keys_to_execute_queue: VecDeque<NodeGraphKey> = VecDeque::new();
        node_keys_to_execute_queue.push_back( *node_key );
    
        while !node_keys_to_execute_queue.is_empty()
        {
            println!("{:?}", node_keys_to_execute_queue); // @TODO, remove this and add a debug mode

            let node_to_execute_key: NodeGraphKey = node_keys_to_execute_queue[0];

            let next_nodes_to_execute: Vec<NodeGraphKey> = self.execute_node(&node_to_execute_key);

            node_keys_to_execute_queue.extend(next_nodes_to_execute);
            node_keys_to_execute_queue.pop_front();
        }

        true
    }

    pub fn execute_node(&mut self, node_key: &NodeGraphKey) -> Vec<NodeGraphKey>
    {
        node::node_kind::execute_node(node_key, self);

        let distribution_result = self.distribute_outputs(node_key); 
        match distribution_result
        {
            Ok( new_nodes_to_execute ) => new_nodes_to_execute,
            Err( error_text ) =>
            {
                println!("{}", error_text); // @TODO, printing should be part of a debug mode
                Vec::new()
            },
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

    fn distribute_outputs(&mut self, node_key: &NodeGraphKey) -> Result<Vec<NodeGraphKey>, String>
    {
        let node = match self.nodes.get(node_key)
        {
            Some( node ) => node,
            None => return Err( format!("Failed to retrieve node with key {} in distribute outputs", node_key) ),
        };

        if node.output_port_keys.is_empty()
        {
            return Ok( Vec::new() );
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
                None => return Err( format!("Failed to retrieve output port with key {} in distribute outputs", output_port_key) ),
            };

            for connected_input_port_key in connected_ports
            {
                let input_port = match self.input_ports.get_mut(connected_input_port_key)
                {
                    Some ( port ) => port,
                    None => return Err( format!("Failed to retrieve input port with key {} in distribute outputs", connected_input_port_key)),
                };

                if !input_port.compatability.contains_port_value_type(&output_port.value)
                {
                    return Err( format!("ERROR, input port {} and output port {} are connected, but have incompatable types in distribute output", input_port.key, output_port_key) );
                }

                // @TODO, find a way of detecting math / immediate nodes 
                match input_port.value
                {
                    PortValue::Trigger => next_nodes_to_execute.push(input_port.node_key),
                    _ => {},
                }

                input_port.value = output_port.value.clone();
            }
        }

        Ok( next_nodes_to_execute )
    }
}