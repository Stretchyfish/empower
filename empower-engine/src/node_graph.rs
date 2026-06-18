use std::collections::{HashMap, HashSet};

pub mod node;
pub use node::Node;
pub use node::node_kind::NodeEdit;

pub mod port;
pub use port::Port;

use crate::node_graph::{node::node_kind::NODE_KIND_REGISTRY, port::PortDefinition};

pub type NodeGraphKey = i32;

pub struct NodeGraph
{
    pub name: &'static str,
    pub input_nodes: Vec<NodeGraphKey>,
    pub output_nodes: Vec<NodeGraphKey>,
    
    pub nodes: HashMap<NodeGraphKey, Node>,
    pub ports: HashMap<NodeGraphKey, Port>,

    pub connections_in: HashMap<NodeGraphKey, NodeGraphKey>, // inputs -> outputs
    pub connections_out: HashMap<NodeGraphKey, HashSet<NodeGraphKey>>, // outputs -> inputs
}

impl NodeGraph
{
    pub fn new(name: &'static str) -> Self
    {
        Self
        {
            name,
            input_nodes: Vec::new(),
            output_nodes: Vec::new(),
            
            nodes: HashMap::new(),
            ports: HashMap::new(),

            connections_out: HashMap::new(),
            connections_in: HashMap::new(),
        }
    }

    pub fn new_entry_graph() -> Self
    {
        let mut node_graph = NodeGraph::new("entry graph");

        let start_node_key = node_graph.add_node("start", None);
        node_graph.input_nodes.push(start_node_key);

        let _ = node_graph.add_node("print", Some( egui::Pos2{ x: 300.0, y: 0.0 } ));
        let _ = node_graph.add_node("list", Some( egui::Pos2{ x: 400.0, y: 150.0 } ));
        let _ = node_graph.add_node("number", Some( egui::Pos2{ x: 0.0, y: 150.0 } ));

        node_graph
    }

    pub fn add_node(&mut self, node_name: &'static str, position: Option<egui::Pos2>) -> NodeGraphKey
    {
        let node_kind_constructor = match NODE_KIND_REGISTRY.get(node_name)
        {
           Some( constructor ) => constructor,
           None => panic!("Requested a non-existing node name"), 
        };

        let node_kind = node_kind_constructor();

        let input_port_definitions = node_kind.input_port_definitions();
        let output_port_definitions = node_kind.output_port_definitions();

        let node_key = self.get_available_node_key();

        let input_port_keys = self.add_ports(&node_key, input_port_definitions);
        let output_port_keys = self.add_ports(&node_key, output_port_definitions);

        let new_node = Node::new(position.unwrap_or(egui::Pos2::default()), input_port_keys, output_port_keys, node_kind);

        self.nodes.insert(node_key, new_node);

        node_key
    }

    pub fn refresh_node(&mut self, node_key: &NodeGraphKey)
    {
        // @TODO, this function is still far from finished
        
        let mut ports_to_replace = Vec::new();
        let mut ports_to_add = Vec::new();
        let mut ports_to_remove = Vec::new();

        {
            let (node, input_ports, output_ports) = self.get_node_input_output(*node_key).unwrap();

            let new_input_port_definitions = node.kind.input_port_definitions();
            let new_output_port_definitions = node.kind.output_port_definitions();

            for (index, new_input_port_definition) in new_input_port_definitions.iter().enumerate()
            {
                let port = input_ports.get(index);

                if port.is_none()
                {
                    ports_to_add.push(new_input_port_definition.clone());
                    continue;
                }

                let port = port.as_ref().unwrap();
            
                if port.compatability == new_input_port_definition.compatability
                {
                    continue;
                }

                ports_to_replace.push( ( port.key, new_input_port_definition.clone()) );
            }

            if new_input_port_definitions.len() < node.input_port_keys.len()
            {
                ports_to_remove = node.input_port_keys[new_input_port_definitions.len()..].to_vec();
            }
        }

        for (port_key, port_definition) in ports_to_replace
        {
            self.ports.insert(port_key, Port::new(port_key, *node_key, port_definition));
        }

        let mut new_input_port_keys = Vec::new();
        for port_definition in ports_to_add
        {
            let new_port_key = self.get_available_port_key();
            self.ports.insert(new_port_key, Port::new(new_port_key, *node_key, port_definition));

            new_input_port_keys.push(new_port_key);
        }

        let node = self.nodes.get_mut(node_key).unwrap();
        node.input_port_keys.extend(new_input_port_keys);

        for port_key in &ports_to_remove
        {
            node.input_port_keys.retain(|e| e != port_key);
            self.ports.remove(&port_key);
        }

        for port_key in ports_to_remove
        {
            self.remove_connection(&port_key);
        }
    } 

    fn add_ports(&mut self, node_key: &NodeGraphKey, port_definitions: Vec<PortDefinition>) -> Vec<NodeGraphKey>
    {
        let mut new_port_keys = Vec::with_capacity(port_definitions.len());
        for port_definition in port_definitions
        {
            let new_port_key = self.get_available_port_key();
            let new_port = Port::new(new_port_key, *node_key, port_definition);
            self.ports.insert(new_port_key, new_port);

            new_port_keys.push(new_port_key);
        }

        new_port_keys
    }

    pub fn add_connection(&mut self, from_port_key: &NodeGraphKey, to_port_key: &NodeGraphKey) -> bool
    {
        if from_port_key == to_port_key
        {
            return false;
        }

        let from_port = self.ports.get(from_port_key).unwrap(); // This is slightly dangerous, but with the current editor implementation it should be safe (maybe revise in the future though)
        let to_port = self.ports.get(to_port_key).unwrap();

        if !from_port.compatible_with(to_port)
        {
            return false;
        }

        self.connections_in.insert(*to_port_key, *from_port_key); // The ports are switched upon insert as output ports has an 1:N relation and inputs have a 1:1 relation to other ports, the order is then reversed during compilation.
        self.connections_out.entry(*from_port_key).or_default().insert(*to_port_key);

        true
    }

    pub fn remove_connection(&mut self, to_port_key: &NodeGraphKey) -> Option<NodeGraphKey>
    {
        if !self.connections_in.contains_key(to_port_key)
        {
            return None;
        }
        
        let from_port_key = self.connections_in.remove(to_port_key).unwrap();

        let remove_connection_out = 
        {
            let to_port_keys = self.connections_out.get_mut(&from_port_key).unwrap();
            to_port_keys.remove(to_port_key);

            to_port_keys.is_empty()
        };

        if remove_connection_out
        {
            self.connections_out.remove(&from_port_key);
        }

        Some( from_port_key )
    }

    pub fn contains_connection(&mut self, to_port_key: &NodeGraphKey) -> bool
    {
        self.connections_in.contains_key(to_port_key) // We don't need to check both connections, as they should be macthing
    }

    fn get_available_node_key(&self) -> NodeGraphKey
    {
        self.nodes.keys().max().unwrap_or(&0) + 1
    }

    fn get_available_port_key(&self) -> NodeGraphKey
    {
        self.ports.keys().max().unwrap_or(&0) + 1
    }

    pub fn get_node_input_output(&self, node_key: NodeGraphKey) -> Option<(&Node, Vec<&Port>, Vec<&Port>)>
    {
        let node = self.nodes.get(&node_key).unwrap();
        let mut inputs = Vec::new();
        let mut ouptuts = Vec::new();

        for port_key in &node.input_port_keys
        {
            let port = self.ports.get(port_key).unwrap();
            inputs.push(port);
        }

        for port_key in &node.output_port_keys
        {
            let port = self.ports.get(port_key).unwrap();
            ouptuts.push(port);
        }
        
        Some( (node, inputs, ouptuts) )
    }

    pub fn get_node_input_output_mut(&mut self, node_key: NodeGraphKey) -> Option<(&mut Node, Vec<&mut Port>, Vec<&mut Port>)>
    {
        let node = self.nodes.get_mut(&node_key).unwrap();
        let mut inputs = Vec::new();
        let mut ouptuts = Vec::new();

        // let port = self.ports.get_disjoint_mut(node.input_port_keys).unwrap();
        // for port_key in &node.input_port_keys
        // {
        //     inputs.push(port);
        // }

        for port_key in &node.output_port_keys
        {
            // let port = self.ports.get_mut(port_key).unwrap();
            // ouptuts.push(port);
        }
        
        Some( (node, inputs, ouptuts) )
    }

    pub fn get_connected_exec_nodes(&self, node_key: NodeGraphKey) -> Vec<NodeGraphKey>
    {
        let node = self.nodes.get(&node_key).unwrap();

        if node.output_port_keys.is_empty()
        {
            return Vec::new();
        }

        let exec_port = self.ports.get(&node.output_port_keys[0]).unwrap();
        
        match exec_port.kind
        {
            port::PortKind::Execution => {},
            port::PortKind::Data => { return Vec::new(); },
        }


        Vec::new()

        

        

        

        

        


        
        
    }
}
