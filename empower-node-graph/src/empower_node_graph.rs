use std::collections::HashMap;

use crate::EmpowerData;
use crate::EmpowerKey;
use crate::Node;
use crate::NodeType;
use crate::InputPort;
use crate::OutputPort;

mod node_creation;
mod node_handle;
use node_handle::NodeHandle;

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

    
    pub fn add_node(&mut self, node_type: NodeType) -> NodeHandle
    {
        let new_node_key;
        let new_input_port_keys;
        let new_output_port_keys;
        match node_type
        {
            NodeType::Start =>
            {
                // @TODO, convert these to optionals?
                (new_node_key, new_input_port_keys, new_output_port_keys) = node_creation::create_start_node(&mut self.nodes, &mut self.output_ports);
            },
            NodeType::IntegerVariable =>
            {
                (new_node_key, new_input_port_keys, new_output_port_keys) = node_creation::create_integer_node(&mut self.nodes, &mut self.input_ports, &mut self.output_ports);
            },
            NodeType::Number =>
            {
                (new_node_key, new_input_port_keys, new_output_port_keys) = node_creation::create_number_node(&mut self.nodes, &mut self.input_ports, &mut self.output_ports);
            },
            NodeType::Addition =>
            {
                (new_node_key, new_input_port_keys, new_output_port_keys) = node_creation::create_addition_node(&mut self.nodes, &mut self.input_ports, &mut self.output_ports);
            },
            NodeType::Print =>
            {
                (new_node_key, new_input_port_keys, new_output_port_keys) = node_creation::create_print_node(&mut self.nodes, &mut self.input_ports);
            },
        }

        NodeHandle { node_key: new_node_key,  input_port_keys: new_input_port_keys, output_port_keys: new_output_port_keys}
    }

    pub fn remove_node(&mut self, key: EmpowerKey)
    {
        let node = self.nodes.get(&key).expect("Tried to delete a node not in the empower node graph");

        let input_port_keys = node.input_port_keys.clone();
        let output_port_keys = node.output_port_keys.clone();

        for input_port_key in input_port_keys
        {
            self.input_ports.remove(&input_port_key);
            self.remove_connections_to_input_port(&input_port_key);
        }

        for output_port_key in output_port_keys
        {
            self.output_ports.remove(&output_port_key);
            self.remove_connection_to_output_port(&output_port_key);
        }

        self.nodes.remove(&key);
    }

    // @TODO, consider removing the option?
    pub fn get_node_input_port_keys(&mut self, node_key: &EmpowerKey) -> Option<Vec<EmpowerKey>>
    {
        if !self.nodes.contains_key(node_key)
        {
            println!("Tried to get input port of node key not in nodes");
            return None;
        }

        let node = self.nodes.get(node_key).unwrap();

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

    pub fn set_node_input_port_value(&mut self, node_key: &EmpowerKey, index: usize, value: EmpowerData) -> bool
    {
        if !self.nodes.contains_key(node_key)
        {
            println!("Tried to set input ports value with node key {}, but node key is not in the graph.", node_key);
            return false;
        }

        let input_port_keys = self.get_node_input_port_keys(node_key).unwrap();
        
        if input_port_keys.len() - 1 > index
        {
            return false;
        }

        let input_port_to_change_key = input_port_keys[index];

        self.set_input_port_value(input_port_to_change_key, value);

        true
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
    pub fn add_connection(&mut self, output_port_key: EmpowerKey, input_port_key: EmpowerKey)
    {
        let output_port = self.output_ports.get(&output_port_key).unwrap();
        let input_port = self.input_ports.get(&input_port_key).unwrap();

        let output_port_data = output_port.value.clone();
        let input_port_data = input_port.value.clone();

        let input_port_type = &input_port.port_type;

        if !input_port_type.is_compatible_with(&output_port_data)
        {
            println!("Tried to add connection between two incompatable ports (out: {}) & (in: {})", output_port_key, input_port_key);
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

        self.input_ports.get_mut(&input_port_key).unwrap().reset(); // @TODO, find a better way of doing this

        true
    }

    pub fn remove_connections_to_input_port(&mut self, input_port_key: &EmpowerKey) // Untested
    {
        if !self.connections_in.contains_key(input_port_key)
        {
            return;
        }

        let output_port_key = self.connections_in.get(input_port_key).unwrap();

        self.remove_connection(*input_port_key, *output_port_key);
    }

    pub fn remove_connection_to_output_port(&mut self, output_port_key: &EmpowerKey)
    {
        if !self.connections_out.contains_key(output_port_key)
        {
            return;
        }

        let input_port_keys = self.connections_out.get(output_port_key).unwrap();
        for input_port_key in input_port_keys
        {
            self.connections_in.remove(input_port_key);
        }
        self.connections_out.remove(output_port_key);
    }

}
