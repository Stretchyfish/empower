use std::collections::HashMap;

use crate::EmpowerData;
use crate::EmpowerKey;
use crate::Node;
use crate::NodeType;
use crate::InputPort;
use crate::OutputPort;

mod node_creation;

#[derive(Default)]
pub struct EmpowerNodeGraph
{
    pub nodes: HashMap<EmpowerKey, Node>,
    pub input_ports: HashMap<EmpowerKey, InputPort>,
    pub output_ports: HashMap<EmpowerKey, OutputPort>,
    pub connections: HashMap<EmpowerKey, Vec<EmpowerKey>>,
}

impl EmpowerNodeGraph
{
    pub fn add_node(&mut self, node_type: NodeType) -> EmpowerKey
    {
        let mut new_node_key = 0;
        match node_type
        {
            NodeType::Start =>
            {
                println!("Tried adding another start node, only one start node is allowed!");
            }
            NodeType::IntegerVariable =>
            {
                new_node_key = node_creation::create_integer_node(&mut self.nodes, &mut self.input_ports, &mut self.output_ports);
            }
        }

        new_node_key
    }

    pub fn remove_node(&mut self, key: EmpowerKey)
    {
        self.nodes.remove(&key); // @TODO, remove input and output ports aswell
    }

    pub fn get_node_input_port_keys(&mut self, node_key: EmpowerKey) -> Option<Vec<EmpowerKey>>
    {
        if !self.nodes.contains_key(&node_key)
        {
            println!("Tried to get input port of node key not in nodes");
            return None;
        }

        let node = self.nodes.get(&node_key).unwrap();

        Some( node.input_port_keys.clone() )
    }
    pub fn get_node_output_port_keys(&mut self, node_key: EmpowerKey) -> Option<Vec<EmpowerKey>>
    {
        if !self.nodes.contains_key(&node_key)
        {
            println!("Tried to get output port of node key not in nodes");
            return None;
        }

        let node = self.nodes.get(&node_key).unwrap();

        Some( node.output_port_keys.clone() )
    }

    pub fn get_input_port_value(&mut self, input_port_key: EmpowerKey)
    {
        if !self.input_ports.contains_key(&input_port_key)
        {
            println!("Get input port value tried to access a non-existend input port");
            return;
        }

        let input_port = self.input_ports.get(&input_port_key).unwrap();
        
        println!("Input port ({}) value: {}", input_port_key, input_port.value);
    }

    pub fn get_output_port_value(&mut self, output_port_key: EmpowerKey)
    {
        if !self.output_ports.contains_key(&output_port_key)
        {
            println!("Get output port value tried to access a non-existend input port");
            return;
        }

        let output_port = self.output_ports.get(&output_port_key).unwrap();
        
        println!("Output port ({}) value: {}", output_port_key, output_port.value);
    }

    pub fn set_input_port_value(&mut self, input_port_key: EmpowerKey, value: EmpowerData)
    {
        if !self.input_ports.contains_key(&input_port_key)
        {
            println!("Set input port value tried to access a non-existend input port");
            return;
        }

        let input_port = self.input_ports.get_mut(&input_port_key).unwrap();
        input_port.value = value;
    }


    pub fn add_connection(&mut self, input_port_key: EmpowerKey, output_port_key: EmpowerKey)
    {
        if self.connections.contains_key(&output_port_key)
        {
            let existing_connection = self.connections.get_mut(&output_port_key).unwrap();

            if existing_connection.contains(&input_port_key)
            {
                println!("Could not add connection, as it already exists");
                return;
            }

            existing_connection.push(input_port_key);
            return;
        }

        self.connections.insert(output_port_key, Vec::from([input_port_key]));
    }
}
