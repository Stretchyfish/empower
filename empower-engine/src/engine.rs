use std::collections::HashMap;

use crate::node;
use crate::EmpowerKey;
use crate::InputPort;
use crate::Node;
use crate::NodeType;
use crate::OutputPort;

mod node_creation;

#[derive(Default)]
pub struct EmpowerEngine
{
    nodes: HashMap<EmpowerKey, Node>,
    input_ports: HashMap<EmpowerKey, InputPort>,
    output_ports: HashMap<EmpowerKey, OutputPort>
}

impl EmpowerEngine
{
    pub fn new() -> Self
    {
        Self 
        {  
            nodes: HashMap::new(),
            input_ports: HashMap::new(),
            output_ports: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_type: NodeType)
    {
        match node_type
        {
            NodeType::Integer =>
            {
                node_creation::create_integer_node(&mut self.nodes, &mut self.input_ports, &mut self.output_ports);
            }
        }
                
    }

    pub fn remove_node(&mut self, key: EmpowerKey)
    {
        self.nodes.remove(&key); // @TODO, remove input and output ports aswell
    }

    pub fn compile(&mut self)
    {
        for node in self.nodes.iter()
        {
            let value = node.1.value;
            println!("Value in node: {}", value);
        }
        
        println!("Compiled nodes");
    }
    
}
