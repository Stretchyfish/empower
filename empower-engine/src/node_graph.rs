use std::collections::{HashMap, HashSet};

mod node;
use node::Node;

mod port;
use port::Port;

mod connection;
use connection::Connection;

use crate::node_graph::{node::node_kind::NODE_KIND_REGISTRY, port::PortDefinition};

pub type NodeGraphKey = i32;

pub struct NodeGraph
{
    input_nodes: HashSet<NodeGraphKey>,
    output_nodes: HashSet<NodeGraphKey>,
    
    nodes: HashMap<NodeGraphKey, Node>,
    ports: HashMap<NodeGraphKey, Port>,
    connections: HashMap<NodeGraphKey, Connection>
}

impl NodeGraph
{
    pub fn new() -> Self
    {
        Self
        {
            input_nodes: HashSet::new(),
            output_nodes: HashSet::new(),
            
            nodes: HashMap::new(),
            ports: HashMap::new(),
            connections: HashMap::new()
        }
    }

    pub fn new_entry_graph() -> Self
    {
        let mut node_graph = NodeGraph::new();

        let start_node_key = node_graph.add_node("start", None);
        node_graph.input_nodes.insert(start_node_key);

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

    fn add_ports(&mut self, node_key: &NodeGraphKey, port_definitions: Vec<PortDefinition>) -> Vec<NodeGraphKey>
    {
        let mut new_port_keys = Vec::with_capacity(port_definitions.len());
        for port_definition in port_definitions
        {
            let new_port_key = self.get_available_port_key();
            let new_port = Port::new(*node_key, port_definition);
            self.ports.insert(new_port_key, new_port);

            new_port_keys.push(new_port_key);
        }

        new_port_keys
    }

    fn get_available_node_key(&self) -> NodeGraphKey
    {
        self.nodes.keys().max().unwrap_or(&0) + 1
    }

    fn get_available_port_key(&self) -> NodeGraphKey
    {
        self.ports.keys().max().unwrap_or(&0) + 1
    }
}
