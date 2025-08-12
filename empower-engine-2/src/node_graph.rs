use std::collections::HashMap;

type NodeGraphKey = i32;

mod node;
use node::Node;
use node::NodeHandle;
pub use node::NodeType;

mod input_port;
use input_port::InputPort;

mod output_port;
use output_port::OutputPort;

mod creator;

pub struct NodeGraph
{
    nodes: HashMap<NodeGraphKey, Node>,
    input_ports: HashMap<NodeGraphKey, InputPort>,
    output_ports: HashMap<NodeGraphKey, OutputPort>,
    connections_out: HashMap<NodeGraphKey, Vec<NodeGraphKey>>,
    connections_in: HashMap<NodeGraphKey, NodeGraphKey>,
}

impl NodeGraph
{
    pub fn new() -> Self
    {
        let mut new_node_graph = Self
        {
            nodes: HashMap::new(),
            input_ports: HashMap::new(),
            output_ports: HashMap::new(),
            connections_out: HashMap::new(),
            connections_in: HashMap::new(),
        };

        new_node_graph.add_node(NodeType::Start);

        new_node_graph
    }

    pub fn add_node(&mut self,  node_type: NodeType)
    {
        match node_type
        {
            NodeType::Start =>
            {
                creator::create_start_node(self);
            }

            _ =>
            {

            }
        }
    }
}