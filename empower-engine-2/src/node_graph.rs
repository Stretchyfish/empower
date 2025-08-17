use std::collections::HashMap;

type NodeGraphKey = i32;

mod node;
use node::Node;
use node::NodeHandle;
pub use node::NodeType;

mod port;
use port::Port;
use port::PortCompatability;
use port::PortValue;

mod creator;

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

    pub fn add_node(&mut self,  node_type: NodeType) -> NodeHandle
    {
        match node_type
        {
            NodeType::Start => creator::create_start_node(self),
            NodeType::Number => creator::create_number_node(self),
            NodeType::Bool => NodeHandle::empty(),
            NodeType::Text => NodeHandle::empty(),
            NodeType::Print => creator::create_print_node(self),
            NodeType::Addition => NodeHandle::empty(),
            NodeType::Multiply => NodeHandle::empty(),
        }
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

    pub fn node_count(&self) -> usize
    {
        self.nodes.len()
    }

    pub fn input_port_count(&self) -> usize
    {
        self.input_ports.len()
    }

    pub fn output_port_count(&self) -> usize
    {
        self.output_ports.len()
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
}