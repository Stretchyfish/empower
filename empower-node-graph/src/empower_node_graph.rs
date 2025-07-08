use std::collections::HashMap;

use crate::EmpowerData;
use crate::EmpowerKey;
use crate::Node;
use crate::NodeType;
use crate::InputPort;
use crate::OutputPort;

mod node_creation;

#[derive(Default, Clone)]
pub struct EmpowerNodeGraph
{
    pub nodes: HashMap<EmpowerKey, Node>,
    pub input_ports: HashMap<EmpowerKey, InputPort>,
    pub output_ports: HashMap<EmpowerKey, OutputPort>,
    pub connections_out: HashMap<EmpowerKey, Vec<EmpowerKey>>,
    pub connections_in: HashMap<EmpowerKey, EmpowerKey>,
}

impl EmpowerNodeGraph
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

    
    pub fn add_node(&mut self, node_type: NodeType) -> EmpowerKey
    {
        let mut new_node_key = 0;
        match node_type
        {
            NodeType::Start =>
            {
                node_creation::create_start_node(&mut self.nodes, &mut self.output_ports);
                println!("Tried adding another start node, only one start node is allowed!");
            }
            NodeType::IntegerVariable =>
            {
                new_node_key = node_creation::create_integer_node(&mut self.nodes, &mut self.input_ports, &mut self.output_ports);
            }
            NodeType::Addition =>
            {
                new_node_key = node_creation::create_addition_node(&mut self.nodes, &mut self.input_ports, &mut self.output_ports);
            }
            NodeType::Print =>
            {
                new_node_key = node_creation::create_print_node(&mut self.nodes, &mut self.input_ports);
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


    // @TODO, change this function to be the other way around
    pub fn add_connection(&mut self, input_port_key: EmpowerKey, output_port_key: EmpowerKey)
    {
        let input_port_data = self.input_ports.get(&input_port_key).unwrap().value; // @TODO, find a consistency in the naming
        let output_port_data = self.output_ports.get(&output_port_key).unwrap().value;

        if input_port_data != output_port_data
        {
            return;
        }
       
        if self.connections_out.contains_key(&output_port_key)
        {
            let existing_connection = self.connections_out.get_mut(&output_port_key).unwrap();

            if existing_connection.contains(&input_port_key)
            {
                println!("Could not add connection, as it already exists");
                return;
            }

            existing_connection.push(input_port_key); // @TODO, investigate what is happening here
            self.connections_in.insert(input_port_key, output_port_key);
            return;
        }

        self.connections_out.insert(output_port_key, Vec::from([input_port_key]));
        self.connections_in.insert(input_port_key, output_port_key);
    }

    pub fn remove_connection(&mut self, input_port_key: EmpowerKey, output_port_key: EmpowerKey) -> bool
    {
        if !self.connections_in.contains_key(&input_port_key)
        {
           return false
        }

        if !self.connections_out.contains_key(&output_port_key)
        {
            return false
        }

        self.connections_in.remove(&input_port_key);

        let mut should_delete_connection = false;

        { // this scope is needed to deal with borrowing
            let connection_from_output_port = self.connections_out.get_mut(&output_port_key).unwrap();

            if let Some(index) = connection_from_output_port.iter().position(|value| *value == input_port_key) 
            {
                connection_from_output_port.swap_remove(index);
            }

            if connection_from_output_port.len() == 0
            {
                should_delete_connection = true;
            }
        }

        if should_delete_connection
        {
           self.connections_out.remove(&output_port_key); 
        }

        true
    }
}
